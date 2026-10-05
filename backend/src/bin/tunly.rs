use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

use clap::{Args, Parser, Subcommand};

const DEFAULT_LABEL: &str = "dev.allenarch.tunly.client";

#[derive(Parser, Debug)]
#[command(name = "tunly", about = "Tunly macOS setup and diagnostics")]
struct Cli {
    #[command(subcommand)]
    command: CommandKind,
}

#[derive(Subcommand, Debug)]
enum CommandKind {
    /// Check the local Tunly installation and macOS runtime prerequisites.
    Doctor,
    /// Install the Tunly client as a macOS LaunchAgent.
    Install(InstallArgs),
    /// Remove the Tunly client LaunchAgent.
    Uninstall(UninstallArgs),
}

#[derive(Args, Debug)]
struct InstallArgs {
    /// Tunly server host[:port], for example tunly.allenarch.dev:8080.
    #[arg(long)]
    remote_host: String,

    /// Local application host:port.
    #[arg(long, default_value = "127.0.0.1:3000")]
    local: String,

    /// URL that returns the ephemeral token JSON or plain-text token.
    #[arg(long)]
    token_url: Option<String>,

    /// Use secure WebSocket transport.
    #[arg(long, action = clap::ArgAction::Set, default_value_t = true)]
    use_wss: bool,

    /// LaunchAgent label, useful when running more than one client.
    #[arg(long, default_value = DEFAULT_LABEL)]
    label: String,
}

#[derive(Args, Debug)]
struct UninstallArgs {
    /// LaunchAgent label to remove.
    #[arg(long, default_value = DEFAULT_LABEL)]
    label: String,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    match Cli::parse().command {
        CommandKind::Doctor => doctor(),
        CommandKind::Install(args) => install(args),
        CommandKind::Uninstall(args) => uninstall(args),
    }
}

fn doctor() -> io::Result<()> {
    let executable = env::current_exe()?;
    let client = executable
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("tunly-client");

    println!("Tunly doctor");
    println!("  platform: {} / {}", env::consts::OS, env::consts::ARCH);
    println!("  manager: {}", executable.display());
    println!(
        "  client: {} ({})",
        client.display(),
        if client.exists() { "found" } else { "missing" }
    );

    if env::consts::OS != "macos" {
        println!("  launchd: unavailable (macOS only)");
        return Ok(());
    }

    let launchctl = Command::new("launchctl").arg("version").output()?;
    println!(
        "  launchctl: {}",
        if launchctl.status.success() {
            "available"
        } else {
            "not responding"
        }
    );

    let agent_dir = launch_agents_dir()?;
    println!(
        "  LaunchAgents: {} ({})",
        agent_dir.display(),
        if agent_dir.exists() {
            "found"
        } else {
            "missing"
        }
    );
    println!("  status: ready for `tunly install --remote-host ...`");
    Ok(())
}

fn install(args: InstallArgs) -> io::Result<()> {
    require_macos()?;
    let executable = env::current_exe()?;
    let client = executable
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("tunly-client");
    if !client.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("tunly-client not found next to {}", executable.display()),
        ));
    }

    let agent_dir = launch_agents_dir()?;
    fs::create_dir_all(&agent_dir)?;
    let plist_path = agent_dir.join(format!("{}.plist", args.label));
    let log_dir = home_dir()?.join("Library/Logs/Tunly");
    fs::create_dir_all(&log_dir)?;

    let mut program_arguments = vec![
        client.display().to_string(),
        "--remote-host".to_string(),
        args.remote_host,
        "--local".to_string(),
        args.local,
        "--use-wss".to_string(),
        args.use_wss.to_string(),
    ];
    if let Some(token_url) = args.token_url {
        program_arguments.extend(["--token-url".to_string(), token_url]);
    }

    let plist = launch_agent_plist(&args.label, &program_arguments, &log_dir);
    File::create(&plist_path)?.write_all(plist.as_bytes())?;
    let domain = format!("gui/{}", unsafe { libc::getuid() });

    let _ = Command::new("launchctl")
        .args(["bootout", &domain, plist_path.to_str().unwrap_or_default()])
        .status();
    let status = Command::new("launchctl")
        .args([
            "bootstrap",
            &domain,
            plist_path.to_str().unwrap_or_default(),
        ])
        .status()?;
    if !status.success() {
        return Err(io::Error::other("launchctl bootstrap failed"));
    }

    println!("Installed LaunchAgent {}", args.label);
    println!("  plist: {}", plist_path.display());
    println!("  logs: {}", log_dir.display());
    Ok(())
}

fn uninstall(args: UninstallArgs) -> io::Result<()> {
    require_macos()?;
    let plist_path = launch_agents_dir()?.join(format!("{}.plist", args.label));
    let domain = format!("gui/{}", unsafe { libc::getuid() });
    let _ = Command::new("launchctl")
        .args(["bootout", &domain, plist_path.to_str().unwrap_or_default()])
        .status();
    if plist_path.exists() {
        fs::remove_file(&plist_path)?;
        println!("Removed LaunchAgent {}", args.label);
    } else {
        println!("LaunchAgent {} was not installed", args.label);
    }
    Ok(())
}

fn launch_agent_plist(label: &str, arguments: &[String], log_dir: &Path) -> String {
    let args_xml = arguments
        .iter()
        .map(|argument| format!("        <string>{}</string>", xml_escape(argument)))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<dict>\n    <key>Label</key>\n    <string>{label}</string>\n    <key>ProgramArguments</key>\n    <array>\n{args_xml}\n    </array>\n    <key>RunAtLoad</key>\n    <true/>\n    <key>KeepAlive</key>\n    <true/>\n    <key>StandardOutPath</key>\n    <string>{}/client.log</string>\n    <key>StandardErrorPath</key>\n    <string>{}/client.error.log</string>\n</dict>\n</plist>\n",
        xml_escape(&log_dir.display().to_string()),
        xml_escape(&log_dir.display().to_string()),
    )
}

fn launch_agents_dir() -> io::Result<PathBuf> {
    Ok(home_dir()?.join("Library/LaunchAgents"))
}

fn home_dir() -> io::Result<PathBuf> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "HOME is not set"))
}

fn require_macos() -> io::Result<()> {
    if env::consts::OS == "macos" {
        Ok(())
    } else {
        Err(io::Error::other("this command is only supported on macOS"))
    }
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
