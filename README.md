# Relogic Tunnel

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-blue?style=for-the-badge)](https://github.com/0xReLogic/Relogic_Tunnel/releases)

**Relogic Tunnel** is a simple, lightweight, and open-source HTTP tunnel solution inspired by ngrok but without limitations, login requirements, or monthly fees.

---

## Motivation

Many tunnel services like ngrok limit users with quotas, login requirements, or subscription fees.  
**Relogic Tunnel** is here for developers, makers, and anyone who wants:

- **Access local applications from anywhere** without hassle
- **No login, no dashboard, no limits**
- **100% open source** and self-hostable
- **Easy distribution**: just download, edit token, and use immediately

---

## Key Features

- No login, no dashboard
- Unlimited tunnels (as long as your server is running)
- Easy distribution: just download exe, edit token, connect immediately
- Simple authentication (token-based, just edit `config.txt`)
- Can run on VPS, cloud, or locally
- Lightweight: pure binary, no complex dependencies
- No telemetry, no tracking, no nonsense

---

## When to Use Relogic Tunnel?

### **Demo & Presentations**
Client wants direct access to your app? But your project is still on localhost?  
**Solution**: Relogic Tunnel makes your localhost accessible from anywhere in 30 seconds.

### **Client Testing**
Client needs to test new features but you haven't deployed to production yet?  
**Solution**: Share tunnel URL, client can test immediately without complex setup.

### **Development & Debugging**
Remote work but need access to apps on your home computer?  
**Solution**: Tunnel from home to office, access applications from anywhere.

### **Mobile Testing**
Need to test web apps on phone but they only run on laptop?  
**Solution**: Tunnel laptop, access from phone via WiFi or data.

### **Quick Prototyping**
Have a new idea, want to share with friends but haven't deployed yet?  
**Solution**: Tunnel localhost, share URL, friends can try immediately.

### **Private Testing**
Want to test apps on the internet but don't want to use ngrok's complexity?  
**Solution**: Relogic Tunnel = ngrok without login, without limits, without hassle.

---

## How to Use

### Prerequisites
- **VPS/Server**: You need a VPS or server accessible from the internet
- **Open port**: Make sure the chosen port (default: 9000) is open in VPS firewall

### Setup Steps

1. **Download** `tunnel-server.exe` and `tunnel-client.exe` from [Releases](https://github.com/0xReLogic/Relogic_Tunnel)
2. **Upload server** to your VPS and run:
   ```
   tunnel-server.exe --port 9000 --token whatsyourapple
   ```
3. **Edit config** on local computer, fill `config.txt` with the same token
4. **Run client** on local computer:
   ```
   tunnel-client.exe --remote-host <vps-address>:9000
   ```
5. **Access local app** from internet via `http://vps-address:9000`

### Server Alternatives
- **Cheap VPS**: DigitalOcean, Vultr, Linode ($5/month)
- **Free cloud**: Oracle Cloud Free Tier, Google Cloud Free Tier
- **Local testing**: Can use ngrok first for testing, then move to VPS

---

## Security

- Token is the "password" for your tunnel.
- Don't share tokens with untrusted people.
- Change tokens regularly for extra security.

---

## Made with ❤️ by Relogic

Built with open source spirit. Feel free to fork, modify, and use for any purpose.

---

## License

MIT License — free to use for commercial and non-commercial purposes.

---

> **Relogic Tunnel**: Like ngrok, but unlimited, no login required, and completely yours. 
