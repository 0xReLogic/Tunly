# Tunly Roadmap — Beyond

## 1. Gambaran Umum

Tunly adalah platform tunneling **open-source dan self-hosted**.

Tunly tidak membatasi penggunaannya pada satu jenis user atau use case. Pengguna bebas memakai Tunly untuk development, webhook, internal service, homelab, production workload, networking, testing, atau kebutuhan lainnya.

Tunly tidak harus menjadi hosted SaaS.

Fokus proyek:

* Mudah di-deploy sendiri
* Mudah dikonfigurasi
* Reliable ketika koneksi bermasalah
* Aman secara default
* Mudah di-debug
* Mendukung HTTP/HTTPS dan networking lanjutan
* Mudah diintegrasikan dengan infrastructure lain
* Tetap ringan dan sederhana
* Open-source dan self-hosted first

---

# 2. Prinsip Pengembangan

## 2.1 Open Source First

Fitur inti Tunly harus tersedia untuk deployment self-hosted.

Tidak ada ketergantungan terhadap hosted Tunly service untuk fungsi dasar.

## 2.2 Self-Hosted First

Deployment self-hosted harus menjadi jalur utama.

Target deployment:

* VPS
* Bare metal
* Docker
* Docker Compose
* systemd
* Kubernetes

## 2.3 Use-Case Agnostic

Tunly tidak perlu menentukan pengguna akan memakai tunnel untuk apa.

Kita membangun capability.

User bebas menentukan penggunaannya.

## 2.4 Reliability Before Complexity

Prioritaskan:

1. Correctness
2. Reliability
3. Security
4. Observability
5. Performance
6. UX
7. Advanced features
8. Scalability

## 2.5 Secure by Default

Fitur security tidak boleh menjadi afterthought.

Minimal harus tersedia:

* Authentication
* Rate limiting
* Connection limits
* Request limits
* Timeout
* TLS support
* Token revocation

## 2.6 Backward Compatibility

Perubahan protocol/config/API harus sebisa mungkin backward-compatible.

Breaking change harus didokumentasikan.

## 2.7 Community Driven

Roadmap dapat berubah berdasarkan:

* GitHub issues
* Pull requests
* User feedback
* Deployment experience
* Benchmark
* Security findings

---

# 3. Status Saat Ini

## 3.1 Dashboard & Monitoring UI

**Status: BASELINE SUDAH ADA**

Dashboard sudah dibuat dan tidak perlu dibangun ulang dari nol.

Dashboard menjadi fondasi untuk pengembangan monitoring berikutnya.

Baseline dashboard:

* Tunnel status
* Tunnel metadata
* Active connections
* Request information
* Bandwidth/performance information
* Basic statistics

Pengembangan berikutnya bersifat incremental.

Target akhir dashboard:

* Real-time tunnel monitoring
* Request inspection
* Search/filter
* Tunnel grouping
* Connection details
* Traffic statistics
* Health status
* Admin actions
* Logs
* Metrics visualization

**Acceptance Criteria:**

* Dashboard dapat melihat tunnel yang aktif
* Status tunnel diperbarui secara real-time atau near real-time
* Request dapat dilihat
* Statistik dapat dilihat
* Dashboard tidak menyebabkan overhead besar pada tunnel server
* Dashboard tetap berfungsi ketika tidak ada tunnel aktif
* Error API ditampilkan dengan jelas

---

# 4. Phase 1 — Core Features & Reliability

## Target

Membuat Tunly stabil untuk digunakan sehari-hari.

**Prioritas: SANGAT TINGGI**

---

## 4.1 TUNLY-001 — Multi-Tunnel Configuration

### Tujuan

Pengguna dapat menjalankan banyak tunnel dari satu configuration file.

### Scope

Support:

* `.tunly.yml`
* `.tunly.yaml`
* `.tunly.toml`

Configuration dapat berada di:

* current directory
* `~/.tunly/config.yml`
* path custom melalui CLI

Satu configuration dapat berisi banyak tunnel.

### Contoh

```yaml
version: "1"

server:
  remote_host: "tunly.example.com:8080"
  use_wss: true
  token_url: "https://tunly.example.com/token"

tunnels:
  - name: "api"
    local: "127.0.0.1:3000"
    domain: "api.example.com"

  - name: "frontend"
    local: "127.0.0.1:5173"
    domain: "app.example.com"

  - name: "admin"
    local: "127.0.0.1:8080"
    domain: "admin.example.com"
    auth_required: true
```

### CLI

```bash
tunly-client --config ~/.tunly/config.yml
```

Tambahkan command yang lebih ergonomis jika cocok dengan CLI:

```bash
tunly up
tunly up api
tunly up api frontend
tunly ls
tunly down api
```

### Acceptance Criteria

* Configuration dapat diparsing
* Invalid configuration menghasilkan error yang jelas
* Multiple tunnel dapat dijalankan dari satu config
* CLI arguments tetap dapat digunakan
* Config dapat memilih tunnel tertentu
* Environment variable dapat override configuration
* Tidak boleh ada secret yang dicetak ke log secara default

### Dependencies

* Existing client
* Existing server/session infrastructure

### Effort

1-3 hari

---

## 4.2 TUNLY-002 — Stable Tunnel Identity

### Tujuan

Tunnel tetap memiliki identity yang konsisten setelah reconnect/restart.

### Scope

Tambahkan:

* `tunnel_id`
* `tunnel_name`

Server menyimpan:

* id
* name
* created_at
* last_seen
* status
* local address
* public endpoint
* token metadata

### CLI

```bash
tunly connect 3000 --name api
```

atau:

```bash
tunly connect 3000 --tunnel-id my-api
```

### Acceptance Criteria

* Tunnel memiliki ID unik
* Tunnel dapat memiliki nama
* Reconnect tidak membuat tunnel baru
* Server dapat lookup tunnel berdasarkan ID
* Server dapat lookup tunnel berdasarkan name
* Public endpoint dapat direuse jika konfigurasi memungkinkan
* Identity tetap konsisten setelah client restart

### Persistence

Implementasi awal:

* In-memory

Implementasi berikutnya:

* SQLite atau storage abstraction

Jangan langsung membuat dependency database wajib jika belum diperlukan.

### Effort

1-2 hari

---

## 4.3 TUNLY-003 — Automatic Reconnect

### Tujuan

Tunnel otomatis pulih ketika koneksi terputus.

### Scope

* Automatic reconnect
* Exponential backoff
* Jitter
* Configurable reconnect interval
* Max reconnect delay
* Re-authentication
* Re-register tunnel
* Preserve tunnel identity

### Contoh

```text
CONNECTED
   |
   v
DISCONNECTED
   |
   v
WAIT 1s
   |
   v
RETRY
   |
   +---- success ---> CONNECTED
   |
   +---- fail ------> WAIT 2s
                         |
                         v
                      RETRY
```

### Acceptance Criteria

* Network interruption tidak membuat client mati
* Client mencoba reconnect otomatis
* Backoff digunakan
* Tidak terjadi reconnect storm
* Tunnel identity tetap sama
* Log menunjukkan state transition
* Setelah server kembali hidup, tunnel dapat kembali online

### Effort

2-3 hari

---

## 4.4 TUNLY-004 — Heartbeat & Half-Open Detection

### Tujuan

Mendeteksi koneksi yang secara teknis masih terbuka tetapi sebenarnya sudah mati.

### Scope

* Ping/pong
* Heartbeat interval
* Connection timeout
* Last seen
* Dead connection detection

### Acceptance Criteria

* Dead WebSocket dapat terdeteksi
* Tunnel berubah menjadi `offline/degraded`
* Reconnect dapat dipicu
* Tidak terlalu agresif memutus koneksi normal
* Interval dapat dikonfigurasi

### Effort

1-2 hari

### Dependency

Automatic reconnect

---

## 4.5 TUNLY-005 — Rate Limiting & Abuse Protection

### Tujuan

Melindungi server dari abuse dan resource exhaustion.

### Scope

Rate limit berdasarkan:

* IP
* Token
* Tunnel

Limit berdasarkan:

* requests/sec
* concurrent connections
* request body size
* response size
* connection duration
* tunnel count

### Behavior

```text
429 Too Many Requests
```

untuk request yang melewati rate limit.

Gunakan backpressure bila relevan.

### Acceptance Criteria

* Per-IP rate limit bekerja
* Per-token rate limit bekerja
* Connection limit bekerja
* Request size limit bekerja
* Timeout bekerja
* Limit dapat dikonfigurasi
* Metrics limit tersedia
* Tidak ada memory growth tak terbatas

### Effort

2-3 hari

---

## 4.6 TUNLY-006 — Request History

### Tujuan

Memudahkan debugging request.

### Scope

Simpan metadata:

* Method
* Path
* Query
* Status code
* Request size
* Response size
* Latency
* Timestamp
* Tunnel
* Connection ID

Header dapat disimpan secara configurable.

### Storage

Gunakan bounded storage.

Contoh:

```text
100 requests
500 requests
1000 requests
```

Jangan menggunakan unbounded memory.

### Acceptance Criteria

* Request dapat dilihat
* Request dapat difilter
* Storage memiliki maximum size
* Request lama otomatis dibuang
* Dashboard dapat membaca history
* Request history tidak mempengaruhi forwarding secara signifikan

### Effort

2 hari

---

## 4.7 TUNLY-007 — Secure Request Replay

### Tujuan

Mengulangi request untuk debugging webhook/API.

### Scope

* Replay request
* Filter request
* Replay ke tunnel yang sama
* Optional replay ke endpoint berbeda
* Show replay result

### Security

Secara default redact:

* `Authorization`
* `Cookie`
* API key
* Access token
* Secret headers

Request body recording sebaiknya configurable.

### Acceptance Criteria

* Request dapat direplay
* User dapat memilih request tertentu
* Secret tidak direplay secara default jika telah di-redact
* Maximum body size diterapkan
* Replay memiliki timeout
* Replay mempunyai audit/log event

### Effort

2-3 hari

---

# 5. Phase 1B — Testing & Reliability

## 5.1 TUNLY-008 — Unit Testing

Coverage minimal untuk:

* Auth
* Config parser
* Tunnel registry
* Rate limiter
* Routing
* Request lifecycle
* Protocol parsing
* Error handling

---

## 5.2 TUNLY-009 — Integration Testing

Test:

* Client → server
* Server → tunnel
* Tunnel → local service
* Authentication
* Multiple tunnel
* Reconnect
* Request forwarding
* WebSocket
* Large payload
* Concurrent requests

---

## 5.3 TUNLY-010 — Failure Testing

Simulasikan:

* Server mati
* Client mati
* Network disconnect
* Packet loss
* High latency
* Local service mati
* TLS failure
* Authentication failure
* Token revoked
* Rate limit exceeded

### Acceptance Criteria

Setiap failure menghasilkan:

* predictable behavior
* useful error
* recovery jika memungkinkan
* tidak menyebabkan process crash

---

## 5.4 TUNLY-011 — Load Testing

Test:

* Requests/sec
* Concurrent tunnels
* Concurrent connections
* Large response
* Long-lived connections
* WebSocket
* Streaming
* Memory usage
* CPU usage

Simpan baseline benchmark untuk dibandingkan di release berikutnya.

### Priority

SANGAT TINGGI

### Effort

Ongoing

---

# 6. Phase 2 — Production Hardening (v0.5.0)

---

## 6.1 TUNLY-012 — Health Checks

### Scope

* Tunnel health
* Server health
* Client health
* Last seen
* Connection state
* Error rate
* Uptime

### API

```text
GET /api/tunnels/{name}/health
GET /health
GET /ready
```

### Acceptance Criteria

Health endpoint tidak boleh bergantung pada request user biasa.

---

## 6.2 TUNLY-013 — Token Management

### Scope

Token memiliki:

* ID
* created_at
* created_by
* last_used
* expiry
* permissions
* status

Support:

* Create
* List
* Revoke
* Rotate
* Expire

### API

```text
GET  /admin/tokens
POST /admin/tokens
POST /admin/tokens/revoke
POST /admin/tokens/rotate
```

### Acceptance Criteria

* Token dapat direvoke tanpa restart server
* Expired token ditolak
* Token dapat dibatasi scope-nya
* Secret token tidak ditampilkan kembali secara plaintext setelah creation

---

## 6.3 TUNLY-014 — Observability

### Scope

Structured logging:

```json
{
  "timestamp": "...",
  "level": "INFO",
  "event": "tunnel_connected",
  "tunnel_id": "...",
  "tunnel_name": "...",
  "remote": "...",
  "request_id": "..."
}
```

### Metrics

Minimal:

```text
tunly_tunnels_total
tunly_tunnels_online
tunly_connections_total
tunly_active_connections
tunly_requests_total
tunly_request_errors_total
tunly_request_latency
tunly_bytes_in_total
tunly_bytes_out_total
tunly_reconnects_total
tunly_auth_failures_total
tunly_rate_limited_total
```

### Acceptance Criteria

* Metrics tersedia
* Logs mudah dibaca
* Sensitive values tidak masuk log
* Metrics tidak menambah overhead besar

---

## 6.4 TUNLY-015 — Performance Optimization

### Scope

* HTTP keep-alive
* Connection reuse
* TLS session reuse
* Reduce allocation
* Reduce unnecessary copying
* Optimize request path
* Optimize WS handling
* Backpressure

### Optional

Multiplex beberapa request melalui satu transport jika architecture mendukung.

### Acceptance Criteria

* Benchmark sebelum dan sesudah
* Tidak ada regression signifikan
* Memory usage terukur
* High concurrency tetap stabil

---

## 6.5 TUNLY-016 — Production Deployment

### Support

* Docker
* Docker Compose
* systemd
* Bare metal
* VPS

### Files

Contoh:

```text
deploy/
├── docker/
├── compose/
├── systemd/
└── examples/
```

### Acceptance Criteria

User dapat mengikuti documentation dan menjalankan server tanpa perlu memahami internal architecture Tunly.

---

# 7. Phase 2B — Self-Hosting Experience

## 7.1 TUNLY-017 — Easy Bootstrap

### Tujuan

Mengurangi friction untuk deployment pertama.

### Scope

* Quickstart command
* Example config generator
* Environment validation
* Configuration validation
* Server setup guide
* Docker Compose quickstart

Contoh:

```bash
tunly init
tunly check
tunly server
```

---

## 7.2 TUNLY-018 — `tunly doctor`

### Tujuan

Membantu user mencari penyebab tunnel gagal.

### Check

```text
Server reachable
DNS resolution
TCP connectivity
TLS certificate
Authentication
WebSocket handshake
Local origin reachable
Port availability
IPv4
IPv6
Proxy configuration
```

### Contoh output

```text
Tunly Doctor

[OK] Local service          127.0.0.1:3000
[OK] DNS                    tunly.example.com
[OK] TCP                    port 443
[OK] TLS                    certificate valid
[OK] Authentication
[FAIL] WebSocket handshake

Reason:
Server rejected connection with HTTP 401.

Suggestion:
Check TUNLY_TOKEN.
```

### Acceptance Criteria

* Error harus actionable
* Jangan hanya menampilkan `connection failed`
* Tidak mencetak secret
* Exit code mencerminkan status

### Priority

TINGGI

---

## 7.3 TUNLY-019 — Automatic TLS / ACME

### Tujuan

Mempermudah self-hosted HTTPS.

### Scope

* ACME
* Let's Encrypt
* Automatic certificate issuance
* Automatic renewal
* Certificate expiry monitoring
* HTTP-01 support
* DNS-01 support jika dibutuhkan
* Wildcard certificate support

### Acceptance Criteria

* Certificate dapat diperoleh otomatis
* Renewal otomatis
* Failure renewal terlihat jelas
* Certificate tidak bocor ke log
* HTTPS dapat digunakan tanpa konfigurasi manual yang berlebihan

### Priority

TINGGI

---

## 7.4 TUNLY-020 — Network Compatibility

### Scope

* IPv4
* IPv6
* IPv4/IPv6 detection
* Bind address configuration
* Connection fallback
* Proxy-aware connection
* Configurable transport

### Acceptance Criteria

* Tunnel bekerja pada IPv4
* Tunnel bekerja pada IPv6 jika environment mendukung
* Failure IPv6 tidak menyebabkan failure total jika IPv4 tersedia
* Error network menjelaskan penyebabnya

---

# 8. Phase 3 — Dashboard Improvements

Dashboard baseline sudah tersedia. Fase ini hanya menambah capability.

## 8.1 TUNLY-021 — Tunnel Management

Tambahkan:

* Search
* Filter
* Sort
* Status filter
* Tunnel tags
* Grouping
* Detail page
* Connection details

---

## 8.2 TUNLY-022 — Request Inspection

Tambahkan:

* Request detail
* Request headers
* Response headers
* Latency
* Status
* Request/response size
* Replay action
* Copy request information

---

## 8.3 TUNLY-023 — Real-Time Events

Tampilkan:

* Tunnel connected
* Tunnel disconnected
* Reconnected
* Authentication failure
* Rate limit event
* Error event

---

## 8.4 TUNLY-024 — Metrics & Analytics

Tambahkan:

* Requests/sec
* Bandwidth
* Latency
* Error rate
* Active connections
* Tunnel uptime

Dashboard harus dapat memilih time range.

---

## 8.5 TUNLY-025 — Admin Actions

Jika auth/permission sudah tersedia:

* Revoke token
* Disable tunnel
* Restart/reconnect request
* View health
* View logs

Semua admin action harus tercatat pada audit log jika audit system telah tersedia.

---

# 9. Phase 4 — Advanced Networking (v0.6.0+)

---

## 9.1 TUNLY-026 — Host & Path Routing

### Scope

Host routing:

```text
api.example.com      -> api tunnel
app.example.com      -> frontend tunnel
admin.example.com    -> admin tunnel
```

Path routing:

```text
/api/*      -> api
/static/*   -> static
```

### Acceptance Criteria

* Rules dapat dikonfigurasi
* Rule priority jelas
* Conflicting rules menghasilkan behavior deterministic
* Invalid route configuration ditolak

---

## 9.2 TUNLY-027 — Request Filtering

### Scope

* IP allowlist
* IP blocklist
* User-Agent filtering
* Header matching
* Query parameter matching
* Optional regex

### Security

Filtering harus dilakukan sebelum forwarding jika memungkinkan.

---

## 9.3 TUNLY-028 — Header Manipulation

Support:

* Add header
* Remove header
* Replace header
* Host rewrite
* Forwarded headers
* X-Forwarded-* handling

Harus ada dokumentasi jelas mengenai trust model dari forwarded headers.

---

## 9.4 TUNLY-029 — Automatic Subdomain Provisioning

### Scope

* Wildcard DNS support
* Unique subdomain allocator
* Reuse subdomain berdasarkan tunnel ID
* Register tunnel
* Release tunnel
* Collision handling

Contoh:

```text
abc123.tunly.example.com
api.tunly.example.com
frontend.tunly.example.com
```

### Acceptance Criteria

* Tidak ada collision
* Subdomain tidak diberikan ke dua tunnel aktif
* Reconnect mempertahankan subdomain
* Tunnel removal me-release subdomain

---

## 9.5 TUNLY-030 — TCP Tunneling

### Tujuan

Mendukung raw TCP selain HTTP.

### Scope

* TCP port forwarding
* TCP stream forwarding
* Connection lifecycle
* Port mapping
* Authentication
* Per-tunnel limits

### Architecture

Pisahkan:

```text
HTTP Tunnel
TCP Tunnel
UDP Tunnel
```

dari common session/transport layer.

### Acceptance Criteria

* TCP service dapat diakses melalui tunnel
* Multiple TCP connections dapat berjalan
* Disconnect ditangani dengan benar
* Resource limits tetap berlaku

### Priority

TINGGI

---

## 9.6 TUNLY-031 — UDP Tunneling

### Scope

* UDP forwarding
* Datagram handling
* Session mapping
* Timeout
* Connection/resource limits

### Acceptance Criteria

* UDP packet dapat diteruskan
* Timeout bekerja
* Resource usage bounded
* Packet/session isolation benar

### Priority

MEDIUM

---

## 9.7 TUNLY-032 — Bandwidth Throttling

### Scope

* Per-tunnel bandwidth limit
* Global bandwidth limit
* Requests/sec
* Bytes/sec
* Backpressure
* Queue/reject policy

Contoh:

```text
tunnel: api
max_bandwidth: 10MB/s
```

---

# 10. Phase 5 — Scalability (v0.7.0+)

## 10.1 TUNLY-033 — Distributed Server

### Tujuan

Menjalankan beberapa server Tunly sebagai satu deployment.

### Scope

* Shared session state
* Shared tunnel registry
* Distributed token management
* Server node identity
* Load balancing
* Service discovery

### Possible Components

* Redis
* PostgreSQL
* External load balancer

Jangan membuat dependency distributed storage wajib untuk deployment single-node.

---

## 10.2 TUNLY-034 — High Availability

### Scope

* Multiple nodes
* Node health
* Failover
* Load balancing
* Recovery
* Session coordination

### Acceptance Criteria

* Satu node mati tidak menyebabkan seluruh control plane mati
* Node yang unhealthy tidak menerima traffic baru
* Recovery dapat diverifikasi melalui integration test

---

## 10.3 TUNLY-035 — Multi-Region

### Scope

* Multiple regions
* Region-aware assignment
* Latency-aware routing
* Regional failover

### Priority

LOW

Hanya dikerjakan jika architecture dan kebutuhan deployment memang memerlukannya.

---

# 11. Phase 6 — Advanced Management (v1.0.0+)

## 11.1 TUNLY-036 — Multi-Tenant

### Scope

* Tenant ID
* Tenant isolation
* Tenant-specific limits
* Tenant-specific tunnels
* Tenant-specific tokens

---

## 11.2 TUNLY-037 — RBAC

### Roles

```text
Admin
Operator
User
Read-only
```

### Permissions

* Create tunnel
* Delete tunnel
* View tunnel
* View requests
* Manage tokens
* Manage configuration
* Manage users

---

## 11.3 TUNLY-038 — Audit Logging

### Events

* Login/authentication
* Tunnel creation
* Tunnel deletion
* Token creation
* Token revoke
* Configuration change
* Admin action
* Permission changes

### Requirements

* Structured
* Searchable
* Timestamped
* Actor identified
* Resource identified

---

# 12. Documentation

Documentation bukan sekadar tambahan.

Untuk proyek self-hosted, documentation merupakan bagian dari product experience.

## 12.1 Installation

Dokumentasikan:

* Binary
* Cargo
* Docker
* Docker Compose
* Linux
* macOS
* Windows

---

## 12.2 Self-Hosting

Dokumentasikan:

* VPS
* Domain
* DNS
* Wildcard DNS
* TLS
* Reverse proxy
* Docker
* systemd
* Kubernetes
* Firewall

---

## 12.3 Reverse Proxy

Berikan contoh konfigurasi:

* Nginx
* Caddy
* Traefik

---

## 12.4 Migration

Buat:

```text
Migrating from ngrok
Migrating from frp
Migrating from other tunneling solutions
```

Jangan hanya membuat feature comparison.

Berikan contoh konfigurasi nyata.

---

## 12.5 Troubleshooting

Minimal:

* DNS failure
* TLS failure
* Authentication failure
* Server unreachable
* Local service unreachable
* WebSocket failure
* Reconnect failure
* IPv4/IPv6 failure
* Reverse proxy failure
* Port conflict
* Rate limit
* Timeout

---

## 12.6 API Documentation

Support:

* OpenAPI 3.x
* Swagger UI
* Request examples
* Response examples
* Authentication
* Error codes

Endpoint:

```text
/api/docs
```

---

# 13. Security Requirements

Semua release production harus mempertimbangkan:

## Authentication

* Secure token
* Token revocation
* Expiration
* Scope

## Transport

* TLS
* WSS
* Certificate validation

## Resource Protection

* Rate limit
* Connection limit
* Body size limit
* Timeout
* Bandwidth limit
* Memory limit

## Logging

Jangan log:

* Password
* API key
* Access token
* Cookie
* Private credential
* Full sensitive request body

## Request Replay

Request replay harus memperhatikan redaction.

---

# 14. Performance Requirements

Performance harus diukur, bukan diasumsikan.

Minimal benchmark:

```text
Throughput
Latency
Concurrent tunnels
Concurrent requests
Concurrent connections
Memory
CPU
Reconnect time
Large payload
Long-lived connection
WebSocket
TCP
UDP
```

Setiap optimisasi penting harus memiliki benchmark sebelum/sesudah.

Jangan melakukan optimisasi besar tanpa bukti bottleneck.

---

# 15. Error Handling Requirements

Error harus:

* jelas
* actionable
* tidak leak secret
* memiliki context
* memiliki error code jika cocok

Contoh buruk:

```text
connection failed
```

Contoh lebih baik:

```text
Failed to connect to tunly.example.com:443

Reason:
TLS handshake failed

Possible causes:
- Invalid certificate
- Wrong hostname
- TLS configuration mismatch

Run:
tunly doctor
```

---

# 16. CLI Experience

Target UX:

```bash
tunly connect 3000
tunly connect 3000 --name api

tunly up
tunly up api

tunly ls

tunly logs api

tunly status api

tunly doctor

tunly config validate

tunly down api
```

CLI tidak harus langsung memiliki semua command tersebut.

Tambahkan secara incremental berdasarkan architecture yang sudah ada.

---

# 17. Configuration Design

Configuration harus:

* predictable
* versioned
* validated
* documented
* backward-compatible sebisa mungkin

Gunakan:

```yaml
version: "1"
```

untuk memungkinkan migration pada masa depan.

Invalid configuration harus gagal lebih awal sebelum tunnel dimulai.

---

# 18. API Design

API harus:

* konsisten
* memiliki HTTP status yang benar
* memiliki error format yang konsisten
* terdokumentasi
* versionable jika diperlukan

Contoh error:

```json
{
  "error": {
    "code": "TOKEN_REVOKED",
    "message": "The authentication token has been revoked."
  }
}
```

---

# 19. Protocol Design

Tunnel protocol harus dipisahkan dari business logic jika memungkinkan.

Common layer:

```text
Session
Authentication
Identity
Heartbeat
Reconnect
Metadata
Metrics
```

Transport-specific layer:

```text
HTTP
TCP
UDP
```

Tujuannya agar penambahan TCP/UDP tidak memaksa rewrite seluruh architecture.

---

# 20. Release Strategy

## v0.4.0 — Core Reliability

Target:

* Multi-tunnel config
* Stable tunnel identity
* Automatic reconnect
* Heartbeat
* Rate limiting
* Resource limits
* Request history
* Request replay
* Testing
* Load testing

Dashboard:

**SUDAH ADA**

Lakukan enhancement hanya jika diperlukan.

---

## v0.5.0 — Production Ready

Target:

* Health checks
* Token management
* Observability
* Structured logging
* Prometheus metrics
* Performance optimization
* Production deployment
* Docker
* systemd
* TLS
* Basic self-hosting improvements

---

## v0.5.x — Developer Experience

Target:

* `tunly doctor`
* Bootstrap/setup helper
* Automatic TLS
* Installation documentation
* Self-hosting documentation
* Reverse proxy documentation
* Troubleshooting
* API documentation
* Migration guide

---

## v0.6.0 — Advanced Networking

Target:

* Host routing
* Path routing
* Request filtering
* Header manipulation
* Automatic subdomain
* TCP tunneling
* UDP tunneling
* Bandwidth throttling

---

## v0.7.0 — Scalability

Target:

* Distributed server
* Shared state
* Load balancing
* High availability
* Failover
* Optional multi-region

---

## v1.0.0 — Production Platform

Target:

* Multi-tenant
* RBAC
* Audit logging
* Advanced access control
* Stable API
* Stable protocol
* Production scalability

---

# 21. Implementation Priority

| ID                | Fitur                   |  Effort | Impact        | Priority |
| ----------------- | ----------------------- | ------: | ------------- | -------: |
| TUNLY-003         | Automatic Reconnect     |    2-3d | Sangat Tinggi |        1 |
| TUNLY-004         | Heartbeat               |    1-2d | Sangat Tinggi |        2 |
| TUNLY-002         | Stable Tunnel Identity  |    1-2d | Sangat Tinggi |        3 |
| TUNLY-001         | Multi-Tunnel Config     |    1-3d | Tinggi        |        4 |
| TUNLY-005         | Rate Limiting           |    2-3d | Tinggi        |        5 |
| TUNLY-008/009/010 | Testing                 | Ongoing | Sangat Tinggi |        6 |
| TUNLY-006         | Request History         |      2d | Tinggi        |        7 |
| TUNLY-007         | Request Replay          |    2-3d | Tinggi        |        8 |
| TUNLY-012         | Health Checks           |    1-2d | Tinggi        |        9 |
| TUNLY-013         | Token Management        |    2-3d | Tinggi        |       10 |
| TUNLY-014         | Observability           |    2-3d | Tinggi        |       11 |
| TUNLY-016         | Production Deployment   |    2-3d | Tinggi        |       12 |
| TUNLY-018         | `tunly doctor`          |    1-2d | Sangat Tinggi |       13 |
| TUNLY-019         | Automatic TLS           |    2-4d | Tinggi        |       14 |
| TUNLY-020         | IPv4/IPv6 Compatibility |    1-2d | Tinggi        |       15 |
| TUNLY-030         | TCP Tunneling           |    3-5d | Sangat Tinggi |       16 |
| TUNLY-026         | Routing                 |    2-3d | Tinggi        |       17 |
| TUNLY-029         | Auto Subdomain          |    1-2d | Tinggi        |       18 |
| TUNLY-031         | UDP Tunneling           |    3-5d | Medium        |       19 |
| TUNLY-032         | Bandwidth Throttling    |    2-3d | Medium        |       20 |
| TUNLY-033         | Distributed Server      |    4-7d | Tinggi        |       21 |
| TUNLY-034         | High Availability       |    4-7d | Tinggi        |       22 |
| TUNLY-036         | Multi-Tenant            |    3-5d | Medium        |       23 |
| TUNLY-037         | RBAC                    |    3-5d | Medium        |       24 |
| TUNLY-038         | Audit Logging           |    2-3d | Medium        |       25 |

---

# 22. Dependency Order

Jangan mengerjakan fitur secara acak.

Recommended order:

```text
Existing Tunnel Protocol
        |
        +---- Stable Identity
        |
        +---- Heartbeat
        |
        +---- Reconnect
        |
        +---- Multi-tunnel Config
        |
        +---- Rate/Resource Limits
        |
        +---- Request History
        |
        +---- Testing
        |
        v
Production Hardening
        |
        +---- Health
        +---- Token Management
        +---- Observability
        +---- Performance
        +---- Deployment
        |
        v
Self-hosting UX
        |
        +---- Doctor
        +---- TLS
        +---- DNS
        |
        v
Advanced Networking
        |
        +---- Routing
        +---- TCP
        +---- UDP
        +---- Filtering
        |
        v
Scalability
        |
        +---- Distributed
        +---- HA
        +---- Multi-region
        |
        v
Advanced Management
        |
        +---- Multi-tenant
        +---- RBAC
        +---- Audit
```

---

# 23. Acceptance Rules untuk AI Coding Agent

Sebelum menganggap sebuah task selesai:

1. Implementasikan hanya scope task.
2. Jangan melakukan rewrite architecture tanpa alasan kuat.
3. Jangan menghapus backward compatibility tanpa alasan.
4. Tambahkan test untuk behavior baru.
5. Tambahkan error handling.
6. Pastikan tidak ada secret yang masuk log.
7. Jalankan formatter.
8. Jalankan linter.
9. Jalankan unit test.
10. Jalankan integration test yang relevan.
11. Dokumentasikan configuration baru.
12. Dokumentasikan breaking change jika ada.
13. Update example configuration jika behavior berubah.
14. Jangan menambahkan dependency baru jika dependency existing masih cukup.
15. Jangan membuat feature yang belum ada di scope hanya karena "akan berguna".

---

# 24. Definition of Done

Sebuah feature dianggap DONE apabila:

### Code

* Implementasi selesai
* Tidak ada TODO kritis
* Error handling tersedia
* Tidak ada panic/crash pada normal failure path

### Tests

* Unit tests tersedia
* Integration tests tersedia bila relevan
* Failure case diuji

### Security

* Authentication/authorization diperiksa bila relevan
* Secret tidak masuk log
* Input divalidasi
* Resource usage bounded

### Documentation

* Configuration documented
* CLI usage documented
* API documented bila relevan
* Example tersedia bila relevan

### Compatibility

* Existing behavior tetap bekerja sebisa mungkin
* Migration path tersedia untuk breaking changes

---

# 25. Success Metrics

## Adoption

* GitHub stars bertumbuh
* GitHub contributors bertambah
* Pull requests komunitas
* Docker pulls meningkat
* crates.io downloads meningkat
* Deployment komunitas

## Technical

* Tunnel uptime
* Reconnect success rate
* Request success rate
* Average latency
* Concurrent tunnel capacity
* Concurrent connection capacity
* Throughput
* Memory usage
* CPU usage

## Reliability

Target utama:

* Client tidak crash ketika network disconnect
* Tunnel dapat reconnect otomatis
* Server tetap stabil di bawah load
* Resource usage tidak tumbuh tanpa batas

## Developer Experience

Target:

* Install mudah
* Setup mudah
* Config mudah
* Debug mudah
* Error message jelas
* Self-hosting documentation lengkap

---

# 26. Non-Goals

Tunly **tidak wajib**:

* Menjadi hosted SaaS
* Menyediakan billing
* Menyediakan marketplace
* Menentukan use case tertentu
* Meniru semua fitur ngrok secara identik
* Menambahkan enterprise feature sebelum dibutuhkan
* Menggunakan distributed database pada single-node deployment
* Membuat architecture kompleks tanpa kebutuhan nyata

---

# 27. Long-Term Vision

Tunly harus menjadi:

> **Open-source, self-hosted tunneling platform yang powerful, reliable, mudah di-debug, dan fleksibel.**

User dapat menjalankan satu server sederhana:

```text
Internet
    |
    v
Tunly Server
    |
    +---- Tunnel A
    |
    +---- Tunnel B
    |
    +---- Tunnel C
```

atau deployment besar:

```text
                Load Balancer
                     |
          +----------+----------+
          |                     |
      Tunly Node A          Tunly Node B
          |                     |
          +----------+----------+
                     |
             Shared State
```

Tetapi arsitektur sederhana harus tetap menjadi default.

---

# 28. Immediate Next Steps

Prioritas implementasi setelah roadmap ini diterapkan:

```text
1. Stable Tunnel Identity
2. Heartbeat
3. Automatic Reconnect
4. Multi-Tunnel Config
5. Rate + Resource Limits
6. Request History
7. Replay
8. Integration + Failure Testing
9. Health Checks
10. Token Management
11. Observability
12. Production Deployment
13. tunly doctor
14. Automatic TLS
15. IPv4/IPv6 compatibility
16. TCP
17. Routing
18. Auto Subdomain
19. UDP
20. Scalability
```

Dashboard tidak perlu diulang dari nol karena baseline-nya sudah tersedia.

Dashboard berikutnya cukup mengikuti development API dan observability baru.

---

# 29. Roadmap Philosophy

Roadmap Tunly bukan daftar fitur yang harus semuanya selesai secepat mungkin.

Prioritas utama:

```text
Make it work
      ↓
Make it reliable
      ↓
Make it secure
      ↓
Make it observable
      ↓
Make it easy to deploy
      ↓
Make it easy to use
      ↓
Make it powerful
      ↓
Make it scalable
```

Setiap release harus meningkatkan kualitas Tunly secara nyata, bukan sekadar menambah jumlah fitur.
