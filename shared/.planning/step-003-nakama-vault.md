# Step 003: Build nakama-vault (Credential Storage)

## Objective
Implement the secure credential vault — OS keychain with encrypted file fallback.

## Tasks
- CredentialVault trait definition (store, retrieve, delete, list_keys, rotate)
- OS Keychain backend via keyring-rs (macOS Keychain, Linux libsecret, Windows DPAPI)
- Encrypted file fallback: AES-256-GCM encryption, Argon2id key derivation from master password
- Environment variable fallback (NAKAMA_<TOOL>_<KEY> pattern, with warnings)
- SecretString wrapper: auto-zeroing on drop, [REDACTED] Display/Debug
- Credential naming convention enforcement (nakama-<tool> / <provider>-<type>)
- Token rotation support (store new, delete old, log rotation event)
- `nakama auth` CLI: add, remove, list (keys only), rotate
- Unit tests: store/retrieve/delete round-trip, memory zeroing verification, permission checks

## Acceptance Criteria
- Credentials stored in OS keychain on supported platforms
- Falls back to encrypted file when keychain unavailable
- SecretString never appears in logs or debug output
- `nakama auth add --service anthropic --key <key>` stores securely
- `nakama auth list` shows key names only, never values
- All credential access logged to audit (when audit is available)

## Dependencies
- Step 002 (nakama-core)
