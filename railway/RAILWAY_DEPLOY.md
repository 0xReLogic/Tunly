# Deploy Tunnel Server ke Railway.app

## Prerequisites
- GitHub account
- Railway.app account (gratis)

## Step 1: Upload ke GitHub
1. Buat repository baru di GitHub
2. Upload semua file project
3. Pastikan folder `fly_deploy/` ada di root

## Step 2: Deploy ke Railway
1. Buka https://railway.app
2. Login dengan GitHub
3. Klik "New Project"
4. Pilih "Deploy from GitHub repo"
5. Pilih repository kamu
6. Railway akan auto-detect Dockerfile di `fly_deploy/`

## Step 3: Set Environment Variables
Di Railway dashboard, set environment variables:
```
PORT=8080
AUTH_TOKEN=whatsyourapple
```

## Step 4: Deploy
1. Railway akan auto-build dan deploy
2. Tunggu sampai status "Deployed"
3. Catat URL yang diberikan (misal: `https://your-app.railway.app`)

## Step 5: Test
1. Buka URL Railway di browser
2. Seharusnya muncul error page (karena belum ada client connect)
3. Client bisa connect ke URL Railway dengan token `whatsyourapple`

## Troubleshooting
- Pastikan Dockerfile ada di `fly_deploy/Dockerfile`
- Pastikan binary `tunnel-server` ada di `fly_deploy/`
- Pastikan environment variables sudah diset
- Cek logs di Railway dashboard jika ada error

## Keuntungan Railway
- ✅ Gratis 500 jam/bulan
- ✅ Tidak butuh credit card
- ✅ Auto-deploy dari GitHub
- ✅ Support Docker
- ✅ Custom domain (opsional) 