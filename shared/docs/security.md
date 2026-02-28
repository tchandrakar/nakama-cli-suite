# Shared Security Architecture — Nakama CLI Suite

> SaaS-grade security for a CLI toolkit. Every credential encrypted, every action audited, every boundary enforced.

## Guiding Principles

1. **Zero plaintext secrets** — No credential ever touches disk in plaintext
2. **Least privilege** — Each tool requests only the scopes it needs
3. **Defense in depth** — Multiple layers: OS keychain → encrypted config → memory zeroing
4. **Fail secure** — On any auth failure, deny access and log the attempt
5. **Auditable** — Every credential access is logged to the audit trail

---

## 1. Credential Vault (`nakama-vault`)

The shared credential management system. All 11 tools use this — none implement their own secret storage.

### Storage Hierarchy

```
┌─────────────────────────────────────────────────────────┐
│                   Credential Vault                       │
│                                                          │
│  Priority 1: OS Keychain (most secure)                   │
│  ┌─────────────────────────────────────────────────┐     │
│  │ macOS: Keychain Services (Security.framework)   │     │
│  │ Linux: libsecret (GNOME Keyring / KDE Wallet)   │     │
│  │ Windows: Windows Credential Manager (DPAPI)     │     │
│  └─────────────────────────────────────────────────┘     │
│                                                          │
│  Priority 2: Encrypted File Store (fallback)             │
│  ┌─────────────────────────────────────────────────┐     │
│  │ Location: ~/.nakama/vault/                       │     │
│  │ Encryption: AES-256-GCM                          │     │
│  │ Key derivation: Argon2id (from master password)  │     │
│  │ File permissions: 0600 (owner read/write only)   │     │
│  └─────────────────────────────────────────────────┘     │
│                                                          │
│  Priority 3: Environment Variables (CI/CD only)          │
│  ┌─────────────────────────────────────────────────┐     │
│  │ NAKAMA_<TOOL>_<KEY> pattern                      │     │
│  │ Only used when OS keychain unavailable           │     │
│  │ Warning logged when env vars are used            │     │
│  └─────────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────┘
```

### Vault API

```rust
pub trait CredentialVault {
    /// Store a credential securely
    fn store(service: &str, key: &str, value: SecretString) -> Result<()>;

    /// Retrieve a credential — returns SecretString (auto-zeroed on drop)
    fn retrieve(service: &str, key: &str) -> Result<SecretString>;

    /// Delete a credential
    fn delete(service: &str, key: &str) -> Result<()>;

    /// List stored credential keys (not values) for a service
    fn list_keys(service: &str) -> Result<Vec<String>>;

    /// Rotate a credential (store new, delete old, log rotation)
    fn rotate(service: &str, key: &str, new_value: SecretString) -> Result<()>;
}
```

### Credential Naming Convention

```
Service: "nakama-<tool>"
Key:     "<provider>-<credential-type>"

Examples:
  nakama-zangetsu / anthropic-api-key
  nakama-shinigami / github-token
  nakama-itachi / atlassian-api-token
  nakama-kami / google-oauth-refresh-token
  nakama-gate / env-staging-bearer-token
```

### Memory Safety

- All secrets use `SecretString` (from the `secrecy` crate) — auto-zeroed on drop
- Secrets are never logged, never included in error messages, never serialized to JSON
- Debug/Display traits on secret types print `[REDACTED]`
- Secrets are never passed as command-line arguments (visible in `ps`)

---

## 2. Authentication Flows

### API Key Authentication (simplest)
Used by: Zangetsu, Shinigami, Sharingan, Senku, Mugen, Tensai (for LLM providers)

```
1. User runs: nakama auth add --service anthropic --key <api-key>
2. Key stored in OS keychain under "nakama-shared / anthropic-api-key"
3. Tool retrieves key at runtime via CredentialVault::retrieve()
4. Key sent in HTTP headers (Authorization: Bearer / x-api-key)
5. Key never written to disk, config files, or logs
```

### OAuth 2.0 + PKCE (for platform integrations)
Used by: Kami (Google), Itachi (Atlassian), Byakugan (GitHub/GitLab/Bitbucket)

```
┌──────┐     ┌───────────┐     ┌────────────────┐
│ User │────→│ nakama    │────→│ OAuth Provider │
│      │     │ auth flow │     │ (browser)      │
└──────┘     └─────┬─────┘     └────────┬───────┘
                   │                     │
                   │  localhost callback  │
                   │◄────────────────────┘
                   │
                   ▼
            ┌─────────────┐
            │ Token Store  │
            │ (keychain)   │
            │              │
            │ access_token │  ← short-lived, auto-refreshed
            │ refresh_token│  ← long-lived, encrypted
            │ expiry       │  ← tracked for proactive refresh
            └─────────────┘
```

- PKCE (Proof Key for Code Exchange) required for all OAuth flows — prevents auth code interception
- Refresh tokens stored encrypted in OS keychain
- Access tokens refreshed proactively (before expiry, not after failure)
- Token refresh failures trigger re-authentication prompt
- All OAuth state parameters use cryptographic random values

### Personal Access Token (PAT)
Used by: Byakugan (GitHub/GitLab/Bitbucket), Itachi (Atlassian Data Center)

```
1. User runs: nakama auth add --service github --token <PAT>
2. Token stored in OS keychain
3. Scopes validated on storage (warn if excessive permissions)
```

---

## 3. Network Security

### TLS Configuration
- **Minimum TLS version:** 1.2 (prefer 1.3)
- **Certificate validation:** Always on (never skip in production)
- **Certificate pinning:** Optional for critical APIs (LLM providers)
- **Custom CA support:** For corporate proxies and self-hosted instances

### HTTP Client Hardening
```rust
pub fn secure_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .min_tls_version(tls::Version::TLS_1_2)
        .https_only(true)                    // no plaintext HTTP
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .redirect(Policy::limited(5))        // prevent redirect loops
        .user_agent("nakama-cli/<version>")
        .build()
}
```

### Request Sanitization
- Strip credentials from error messages and logs
- Sanitize URLs before logging (redact query params with tokens)
- Never log request/response bodies containing secrets

---

## 4. Input Validation & Injection Prevention

### Command Injection (Zangetsu)
Zangetsu translates NL to shell commands — highest risk surface:
- All generated commands pass through an allowlist of safe executables
- Dangerous patterns blocked: `; && || | \` $() `` rm -rf` etc. at the raw level
- Commands executed via `Command::new()` with explicit arg arrays — never `sh -c`
- User confirmation required for any command with side effects

### Prompt Injection
All tools that send user input to LLMs:
- System prompts are hardcoded, never constructed from user input
- User input is clearly delimited in prompts with XML tags
- LLM outputs are treated as untrusted — never executed without validation
- Tool-use responses from LLMs are validated against expected schemas

### SQL Injection
Tools using SQLite (history, cache, audit):
- All queries use parameterized statements — never string interpolation
- ORM/query builder layer enforces parameterization

---

## 5. File System Security

### Permissions
```
~/.nakama/                    drwx------  (700)
~/.nakama/vault/              drwx------  (700)
~/.nakama/vault/*.enc         -rw-------  (600)
~/.nakama/config.toml         -rw-------  (600)
~/.nakama/audit/              drwx------  (700)
~/.nakama/audit/*.log         -rw-------  (600)
```

### Config File Safety
- Config files never contain secrets (secrets are in vault)
- Config files validated on load (reject unknown keys, type-check values)
- File permissions checked on startup — warn if too open

---

## 6. SaaS-Grade Standards Compliance

| Standard | How We Comply |
|----------|---------------|
| **OWASP Top 10** | Input validation, parameterized queries, secure auth, no secrets in logs |
| **SOC 2 Type II** | Audit logging, access controls, encryption at rest and in transit |
| **GDPR** | No PII in logs, credential deletion support, data portability |
| **CIS Benchmarks** | File permissions, TLS enforcement, least privilege |

### Security Checklist Per Tool
Every tool must pass before release:
- [ ] All secrets stored via CredentialVault (never plaintext)
- [ ] All HTTP via HTTPS with TLS 1.2+
- [ ] All user input validated before processing
- [ ] All LLM inputs/outputs sanitized
- [ ] Audit trail covers all sensitive operations
- [ ] No secrets in logs, errors, or debug output
- [ ] File permissions set correctly
- [ ] Dependencies audited (`cargo audit`)
- [ ] SAST scan passed (clippy + custom lints)

---

## 7. Dependency Security

- **`cargo audit`** runs in CI — blocks releases with known vulnerabilities
- **`cargo deny`** enforces license policy and blocks banned crates
- **Lockfile committed** — reproducible builds
- **Minimal dependencies** — each tool only includes what it uses
- **Supply chain** — crates verified via crates.io checksums
