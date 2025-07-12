# Relogic Tunnel

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-blue?style=for-the-badge)](https://github.com/0xReLogic/Relogic_Tunnel/releases)

**Relogic Tunnel** adalah solusi tunnel HTTP sederhana, ringan, dan open source, terinspirasi dari ngrok — tapi tanpa batasan, tanpa login, dan tanpa biaya bulanan.

---

## Motivasi

Banyak layanan tunnel seperti ngrok membatasi user dengan kuota, login, atau biaya langganan.  
**Relogic Tunnel** hadir untuk developer, maker, dan siapa saja yang ingin:

- **Akses aplikasi lokal dari mana saja** tanpa ribet
- **Tanpa login, tanpa dashboard, tanpa limit**
- **100% open source** dan bisa di-host sendiri
- **Distribusi mudah**: cukup download, edit token, langsung pakai

---

## Fitur Utama

- Tanpa login, tanpa dashboard
- Unlimited tunnel (selama server kamu hidup)
- Distribusi mudah: cukup download exe, edit token, langsung connect
- Simple authentication (token-based, tinggal edit `config.txt`)
- Bisa dijalankan di VPS, cloud, atau lokal
- Ringan: pure binary, tanpa dependency ribet
- No telemetry, no tracking, no nonsense

---

## Kapan Pakai Relogic Tunnel?

### **Demo & Presentasi**
Client minta akses langsung ke aplikasi kamu? Tapi project masih di localhost?  
**Solusi**: Relogic Tunnel bikin localhost kamu bisa diakses dari mana saja dalam 30 detik.

### **Client Testing**
Client butuh test fitur baru tapi kamu belum deploy ke production?  
**Solusi**: Share tunnel URL, client bisa test langsung tanpa ribet setup.

### **Development & Debugging**
Remote work tapi butuh akses ke aplikasi di komputer rumah?  
**Solusi**: Tunnel dari rumah ke kantor, akses aplikasi dari mana saja.

### **Mobile Testing**
Butuh test aplikasi web di HP tapi cuma jalan di laptop?  
**Solusi**: Tunnel laptop, akses dari HP via WiFi atau data.

### **Quick Prototyping**
Ada ide baru, mau share ke temen tapi belum deploy?  
**Solusi**: Tunnel localhost, share URL, temen bisa langsung coba.

### **Private Testing**
Mau test aplikasi di internet tapi gak mau pake ngrok yang ribet?  
**Solusi**: Relogic Tunnel = ngrok tanpa login, tanpa limit, tanpa ribet.

---

## Cara Pakai

1. **Download** `tunnel-server.exe` dan `tunnel-client.exe` dari [Releases](https://github.com/0xReLogic/Relogic_Tunnel/releases)
2. **Edit** file `config.txt` dan isi dengan token yang sama dengan server
3. **Jalankan server** di VPS/cloud:
   ```
   tunnel-server.exe --port 9000 --token whatsyourapple
   ```
4. **Jalankan client** di komputer lokal:
   ```
   tunnel-client.exe --remote-host <alamat-server>:9000
   ```
5. **Akses aplikasi lokal kamu** dari internet via server

---

## Keamanan

- Token adalah “password” untuk tunnel kamu.
- Jangan share token ke orang yang tidak dipercaya.
- Ganti token secara berkala untuk keamanan ekstra.

---



## Made with ❤️ by Relogic

Dibuat dengan semangat open source. Silakan fork, modifikasi, dan gunakan untuk kebutuhan apapun.

---

## Lisensi

MIT License — bebas digunakan untuk komersial maupun non-komersial.

---

> **Relogic Tunnel**: Mirip ngrok, tapi unlimited, tanpa login, dan sepenuhnya milikmu. 