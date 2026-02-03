# Tunly

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-blue?style=for-the-badge)](https://github.com/0xReLogic/Tunly/releases)
[![Download Client](https://img.shields.io/badge/Download-Client-2ea44f?style=for-the-badge&logo=github)](https://github.com/0xReLogic/Tunly/releases)

[![CodeQL](https://github.com/0xReLogic/Tunly/actions/workflows/codeql.yml/badge.svg)](https://github.com/0xReLogic/Tunly/actions/workflows/codeql.yml)
[![Rust CI](https://github.com/0xReLogic/Tunly/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/0xReLogic/Tunly/actions/workflows/rust-ci.yml)
[![Frontend CI](https://github.com/0xReLogic/Tunly/actions/workflows/frontend-ci.yml/badge.svg)](https://github.com/0xReLogic/Tunly/actions/workflows/frontend-ci.yml)
[![Release](https://github.com/0xReLogic/Tunly/actions/workflows/release.yml/badge.svg)](https://github.com/0xReLogic/Tunly/actions/workflows/release.yml)

**Tunly** is a simple, lightweight, and open-source HTTP tunnel solution inspired by ngrok but without limitations, login requirements, or monthly fees.

---

## Motivation

Many tunnel services like ngrok limit users with quotas, login requirements, or subscription fees.  
**Tunly** is here for developers, makers, and anyone who wants:

- **Access local applications from anywhere** without hassle
- **No login, no dashboard, no limits**
- **100% open source** and self-hostable
- **Easy distribution**: double‑click, paste token, and use immediately

---

## Key Features

- No login, no dashboard
- Unlimited tunnels (as long as your server is running)
- Double‑click UX: prompt token (get it from `https://tunly.online`), then prompt local address, then Tunly prints your Public URL
- Simple authentication: paste token when prompted; advanced users can still use `config.txt`/ENV flags
- Can run on VPS, cloud, or locally (self-host)
- Lightweight: pure binary, no complex dependencies
- No telemetry, no tracking, no nonsense
- Clear logs: server prints `PROXY` per request, client prints `LOCAL` per proxied request
- Built-in session log page: `/s/:sid/_log` shows recent accessed paths (e.g., `/`, `/api`, `/blog`)

---

## When to Use Tunly?

### **Demo & Presentations**
Client wants direct access to your app? But your project is still on localhost?  
**Solution**: Tunly makes your localhost accessible from anywhere in 30 seconds.

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
**Solution**: Tunly = ngrok without login, without limits, without hassle.

---

## How to Use

### Modes
- **Hosted (recommended)**: Use our backend at `app.tunly.online` and landing page at `tunly.online` to get tokens.
- **Self-host (advanced)**: Run your own server and point the client to it.

### Hosted (Recommended)

1. **Download** `tunly-client` for your OS from [Releases](https://github.com/0xReLogic/Tunly/releases)
2. **Double‑click** `tunly-client` to run
3. When prompted, open `https://tunly.online`, copy the token shown in the UI, then paste it into the client
4. Enter your local address when prompted (default: `127.0.0.1:80`)
5. The client will print your **Public URL**, e.g., `https://app.tunly.online/s/<session>/` — share this URL
6. View the session log: `https://app.tunly.online/s/<session>/_log`

> Notes:
> - Long flags use kebab-case (e.g., `--remote-host`, `--token-url`, `--allow-token-query`).
> - Default auth uses header `Authorization: Bearer <token>`. Query `?token=...` works only if server enables `--allow-token-query`.
> - For self-host without TLS, pass `--use-wss=false` so the client uses `ws://` (the flag accepts an explicit boolean, e.g., `--use-wss=false`).

### Quick start (from source, via Cargo)

If building from source:

- **Hosted (recommended)** — interactive (no flags):
  ```
  cargo run --bin tunly-client --
  ```
  Then follow the prompts: paste the token from `https://tunly.online` and enter your local address.

- **Self-host (advanced)** — run your own server, then point client to it.

  1) Start server (ephemeral token mode):
  ```
  cargo run --bin tunly-server -- --bind 0.0.0.0:9000
  ```

  2a) Start client (interactive, custom server):
  ```
  cargo run --bin tunly-client -- --remote-host <server-ip-or-host>:9000 --use-wss=false --local 127.0.0.1:8080
  ```

  2b) Start client (auto-fetch token; advanced):
  ```
  cargo run --bin tunly-client -- --remote-host <server-ip-or-host>:9000 \
    --use-wss=false \
    --local 127.0.0.1:8080 \
    --token-url http://<server-ip-or-host>:9000/token
  ```

  3) Open the Public URL printed by the client, e.g.:
  ```
  http://<server-ip-or-host>:9000/s/<session>/
  ```

  4) Check recent paths accessed by visitors for that session:
  ```
  http://<server-ip-or-host>:9000/s/<session>/_log
  ```

#### Local offline test (no TLS)

For a quick local test without internet:

1) Start server with a fixed token:
```
cargo run --bin tunly-server -- --bind 127.0.0.1:9000 --token devtoken
```

2) Start client (interactive) and connect over ws:
```
cargo run --bin tunly-client -- --remote-host 127.0.0.1:9000 --use-wss=false
```
When prompted, enter `devtoken`, then your local app address (e.g., `127.0.0.1:8080`).

### Loginless / Ephemeral Token Mode (no dashboard, no signup)

If you don't want to manage a static token, run the server without `--token` and without env `TUNLY_TOKEN`. The server will issue one-time tokens bound to the requester's IP via `/token`.

- **Start server (ephemeral mode)**
  ```
  tunly-server.exe --port 9000
  ```
- **Client auto-fetch token (advanced)**
  ```
  tunly-client.exe --remote-host <vps-address>:9000 --token-url http://<vps-address>:9000/token
  ```
  The client fetches a token from `/token` (JSON or plain text) and connects via WebSocket using that token.

Notes:
- Tokens are one-time use, may be bound to the requester IP, and expire in ~5 minutes.
- Default auth is via header `Authorization: Bearer <token>`; `?token=` query is disabled unless `--allow-token-query` is set on the server.
- If you prefer a fixed token, set `--token <value>` or env `TUNLY_TOKEN` on the server and keep using `config.txt` or env on the client.

### Fixed vs Ephemeral Tokens

- **Fixed Token**
  - Server: run with `--token <value>` or env `TUNLY_TOKEN`.
  - Client: paste token when prompted, or set it via `config.txt`/`TUNLY_TOKEN`.
  - Best for interactive UX testing and simple setups.

- **Ephemeral Token**
  - Server: run without `--token` (issues one-time tokens via `/token`, tied to `session`+IP, TTL ~5 minutes).
  - Client: use `--token-url http://<server>:<port>/token` so the token matches the current `sid` automatically.
  - Manual prompt is not compatible with Ephemeral mode (will be rejected as invalid).

### Server Alternatives
- **Cheap VPS**: DigitalOcean, Vultr, Linode ($5/month)
- **Free cloud**: Oracle Cloud Free Tier, Google Cloud Free Tier
- **Local testing**: Can use ngrok first for testing, then move to VPS

---

## Environment & Deploy

- **Server env**:
  - `PORT` (from platform, e.g., Render, Koyeb) — server listens on this port automatically.
  - `TUNLY_TOKEN` — optional; if set, server uses fixed-token mode. If not set and `--token` is not provided, server uses ephemeral mode with `/token` issuance.
- **Client config**:
  - `config.txt` with `token: <value>` (tolerant to `token=`/`token:`/`tokenn`).
  - Or env `TUNLY_TOKEN`.
  - Or runtime fetch via `--token-url http://<server>:<port>/token` (ephemeral mode).
- **Frontend env**:
  - `BACKEND_BASE_URL` — base URL of your Tunly backend (e.g., `https://<your-app>.koyeb.app` or your custom domain). Used by the Next.js proxy route `app/api/token/route.ts` to call `/token`.
- **Deploy on Koyeb**:
  - Source: Docker → Dockerfile path: `backend/Dockerfile`
  - Health check: `GET /healthz`
  - Environment:
    - `TUNLY_TOKEN` (optional): set for Fixed mode; leave empty for Ephemeral mode (`/token` enabled)
    - `PORT`: injected automatically by Koyeb (no need to set)
  - Optional: add a custom domain; Koyeb will provision TLS automatically

---

## Security & Limits

- `/token` rate limit: 10 requests per 60 seconds per IP
- Ephemeral token TTL: ~5 minutes; single use; bound to requester's IP and session id
- Proxy request body limit: 2 MB
- Session idle TTL: ~10 minutes (inactive sessions are garbage-collected)

---

## Logs & Observability

- **Server logs** each proxied request:
  ```
  PROXY GET / -> 200 in 16ms (sid=abc123)
  ```
- **Client logs** each local request it performs:
  ```
  LOCAL GET /api -> 200 in 8ms
  ```
- **Session log page** lists the last ~50 requests for a session:
  - URL: `http://<server>/s/<session>/_log`
  - Shows: Method, URI, Status, Duration (ms)
  - Includes quick links to `/, /api, /blog` for quick checks

## API Endpoints

- `GET /healthz` — health check
- `GET /token` — issue ephemeral token (available only in Ephemeral mode)
- `GET /ws?sid=<session>` — WebSocket entrypoint (use `Authorization: Bearer <token>` header)
- `GET /s/:sid/_log` — recent paths accessed for the session
- `ANY /s/:sid/<...>` — proxied traffic routed to the connected client

## Troubleshooting

- **“Token is invalid or has expired.”**
  - Cause: Server is in Ephemeral mode but client used manual token prompt (token doesn’t match `sid`).
  - Fix: Use `--token-url http://<server>:<port>/token`, or run server with a fixed token (`--token <value>`) and then use manual prompt.

- **Cannot connect over wss on localhost/self-host**
  - Cause: No TLS certificate for your self-hosted server.
  - Fix: Add `--use-wss=false` to use plain `ws://` during local testing.

## Security

- Token is the "password" for your tunnel.
- Don't share tokens with untrusted people.
- Change tokens regularly for extra security.

---

## License

MIT License — free to use for commercial and non-commercial purposes.
 
