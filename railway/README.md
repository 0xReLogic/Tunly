# Tunnel Rust

Proyek tunnel sederhana yang terdiri dari dua binary Rust untuk membuat tunnel HTTP melalui VPS menggunakan TCP socket murni tanpa async runtime.

## Komponen

### 1. tunnel-server
Server TCP yang berjalan di VPS (port default 9000) yang:
- Menerima koneksi HTTP dari browser
- Menunggu koneksi aktif dari tunnel-client
- Meneruskan permintaan HTTP dari browser ke tunnel-client
- Meneruskan response dari tunnel-client kembali ke browser
- Menampilkan halaman error HTML jika client tidak tersedia

### 2. tunnel-client
Client yang berjalan di komputer lokal yang:
- Menyambung ke tunnel-server
- Menunggu permintaan dari server
- Untuk setiap permintaan yang diterima, diteruskan ke localhost:<local_port> (misalnya localhost:3000)
- Mengambil response dan mengirim kembali ke server

## Fitur Utama

- **TCP socket murni** tanpa async runtime
- **Command line arguments** yang fleksibel
- **Simple authentication** dengan token Bearer
- **Failover message** dengan halaman HTML error yang user-friendly
- **Logging sederhana** ke stdout untuk debugging
- **Workspace Cargo** dengan dua binary terpisah
- **Portable binary** untuk Linux, Windows, dan macOS
- **Docker support** untuk deployment container
- **Error handling** yang robust

## Cara Penggunaan

### 1. Build Proyek
```bash
# Build semua binary
cargo build --release

# Atau build individual
cargo build --release -p tunnel-server
cargo build --release -p tunnel-client
```

### 2. Jalankan tunnel-server di VPS
```bash
# Di VPS dengan port default 9000
./target/release/tunnel-server

# Atau dengan port custom
./target/release/tunnel-server --port 9001

# Dengan authentication token
./target/release/tunnel-server --port 9000 --token rahasia123
```

### 3. Jalankan tunnel-client di komputer lokal
```bash
# Di komputer lokal dengan port default 3000
./target/release/tunnel-client --remote-host myvps.com:9000

# Atau dengan konfigurasi custom
./target/release/tunnel-client --local-port 3000 --remote-host myvps.com:9000

# Dengan authentication token
./target/release/tunnel-client --local-port 3000 --remote-host myvps.com:9000 --token rahasia123

# Demo mode - auto disconnect setelah request pertama
./target/release/tunnel-client --local-port 3000 --remote-host myvps.com:9000 --once

# Disable auto reconnect
./target/release/tunnel-client --local-port 3000 --remote-host myvps.com:9000 --no-retry

# Enable debug logging
./target/release/tunnel-server --port 9000 --debug
./target/release/tunnel-client --local-port 3000 --remote-host myvps.com:9000 --debug
```

### 4. Akses melalui browser
Setelah kedua service berjalan, Anda bisa mengakses aplikasi lokal melalui:
```
http://myvps.com:9000
```

## Command Line Arguments

### tunnel-server
- `--port, -p`: Port untuk server (default: 9000)
- `--token`: Token otentikasi (opsional, tapi wajib jika ingin auth)
- `--debug`: Enable debug logging dengan request details

### tunnel-client
- `--local-port, -l`: Port aplikasi lokal (default: 3000)
- `--remote-host, -r`: Host dan port remote server (format: host:port) - **wajib**
- `--token`: Token otentikasi (opsional, tapi wajib jika server pakai token)
- `--once`: Auto disconnect setelah request pertama (demo mode)
- `--no-retry`: Disable auto reconnect (default: enabled)
- `--debug`: Enable debug logging dengan request details

## Build dan Distribusi

### Build Static Binary untuk Server

#### Linux (Static, cocok untuk Docker/Alpine)
```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release -p tunnel-server --target x86_64-unknown-linux-musl
# Hasil: ./target/x86_64-unknown-linux-musl/release/tunnel-server
```

#### Windows
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release -p tunnel-server --target x86_64-pc-windows-gnu
# Hasil: ./target/x86_64-pc-windows-gnu/release/tunnel-server.exe
```

#### macOS
```bash
cargo build --release -p tunnel-server
# Hasil: ./target/release/tunnel-server
```

### Build Client Executable untuk Distribusi

#### Windows Client
```bash
# Build untuk Windows
build-client-exe.bat
# Hasil: dist/tunnel-client.exe
```

#### Cross-Platform Build
```bash
# Build untuk semua platform
build-all-platforms.bat
# Hasil: 
# - dist/tunnel-client-windows.exe
# - dist/tunnel-client-linux
# - dist/tunnel-client-macos
```

#### Create Release Package
```bash
# Buat package lengkap untuk distribusi
create-release.bat
# Hasil: release/ folder dengan semua file yang diperlukan
```

### Distribusi Client

Setelah build, client executable bisa didistribusikan ke user tanpa perlu install Rust:

1. **Windows**: User tinggal download `tunnel-client-windows.exe` dan jalankan
2. **Linux**: User download `tunnel-client-linux`, beri permission execute, dan jalankan
3. **macOS**: User download `tunnel-client-macos`, beri permission execute, dan jalankan

Lihat `CLIENT_USAGE.md` untuk panduan lengkap penggunaan client executable.

## Deployment Options

### 1. Fly.io Deployment (Recommended)

#### Build Static Binary
```bash
# Build static binary untuk deployment
rustup target add x86_64-unknown-linux-musl
cargo build --release -p tunnel-server --target x86_64-unknown-linux-musl

# Atau gunakan script yang sudah disediakan
build-static.bat
```

#### Deploy ke Fly.io
```bash
# 1. Install Fly CLI
# Download dari https://fly.io/docs/hands-on/install-flyctl/

# 2. Login ke Fly.io
fly auth login

# 3. Edit fly.toml sesuai kebutuhan
# - Ganti "your-tunnel-app-name" dengan nama app yang diinginkan
# - Set AUTH_TOKEN yang aman

# 4. Deploy
fly deploy

# Atau gunakan script yang sudah disediakan
deploy-fly.bat
```

#### Konfigurasi Client untuk Fly.io
```bash
# Setelah deploy, client akan connect ke:
./target/release/tunnel-client --remote-host your-app-name.fly.dev:443 --token your-secret-token
```

### 2. Docker Deployment

#### Build Docker Image
```bash
# Build static binary (Linux)
rustup target add x86_64-unknown-linux-musl
cargo build --release -p tunnel-server --target x86_64-unknown-linux-musl

# Build Docker image
docker build -f Dockerfile.fly -t tunnel-server .
```

#### Jalankan Container
```bash
# Tanpa authentication
docker run -p 9000:9000 tunnel-server

# Dengan authentication
docker run -p 9000:9000 -e AUTH_TOKEN=rahasia123 tunnel-server
```

## Struktur Proyek
```
tunnel_rust/
├── Cargo.toml (workspace)
├── README.md
├── README.en.md
├── CLIENT_USAGE.md
├── Dockerfile
├── Dockerfile.fly
├── fly.toml
├── build-static.bat
├── build-client-exe.bat
├── build-all-platforms.bat
├── create-release.bat
├── deploy-fly.bat
├── run-all.bat
├── run_tunnel.bat
├── .gitignore
├── dist/ (generated)
├── release/ (generated)
├── tunnel-server/
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   └── static/
│       └── error.html
└── tunnel-client/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## Dependencies
- `clap`: Command line argument parsing
- `httparse`: HTTP parsing (built-in parsing juga digunakan)

## Contoh Penggunaan Lengkap

### VPS (Ubuntu/Debian)
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone dan build proyek
git clone <repository>
cd tunnel-rust
cargo build --release

# Jalankan server dengan authentication
./target/release/tunnel-server --port 9000 --token rahasia123
```

### Komputer Lokal
```bash
# Build proyek
cargo build --release

# Jalankan aplikasi lokal (contoh: Node.js server)
node app.js  # Berjalan di port 3000

# Jalankan tunnel client dengan authentication
./target/release/tunnel-client --local-port 3000 --remote-host myvps.com:9000 --token rahasia123
```

### Akses dari Browser
```
http://myvps.com:9000
```

## Troubleshooting

### Error: "No tunnel client available for host"
- Pastikan tunnel-client sudah terhubung ke server
- Periksa apakah host yang diakses sesuai dengan yang didaftarkan client
- Periksa log server untuk informasi detail

### Error: "Failed to connect to local application"
- Pastikan aplikasi lokal berjalan di port yang dikonfigurasi (default: 3000)
- Periksa firewall lokal
- Periksa apakah port tidak digunakan oleh aplikasi lain

### Error: "Connection refused"
- Pastikan tunnel-server berjalan di VPS
- Periksa firewall VPS dan port forwarding
- Periksa apakah port server tidak diblokir

### Error: "Client rejected: invalid or missing token"
- Pastikan token yang digunakan client sama dengan token server
- Periksa format token (tidak boleh ada spasi atau karakter khusus)
- Pastikan argumen --token diberikan dengan benar

## Keamanan

- Gunakan token authentication untuk mencegah akses tidak sah
- Pastikan firewall dikonfigurasi dengan benar
- Gunakan HTTPS reverse proxy jika diperlukan untuk production
- Monitor log server secara berkala

## Catatan
- Pastikan aplikasi lokal berjalan di port yang dikonfigurasi (default: 3000)
- Tunnel server harus bisa diakses dari internet
- Firewall harus mengizinkan koneksi ke port tunnel server
- Tunnel client akan otomatis mendaftarkan dirinya ke server dengan host `localhost:<local_port>`
- Log akan ditampilkan di stdout untuk memantau alur data
- Halaman error HTML akan ditampilkan jika client tidak tersedia 