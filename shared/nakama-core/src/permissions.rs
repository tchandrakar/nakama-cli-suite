use crate::error::NakamaResult;
use std::path::Path;

/// Set directory permissions to 0700 (owner only).
#[cfg(unix)]
pub fn set_dir_permissions(path: &Path) -> NakamaResult<()> {
    use std::os::unix::fs::PermissionsExt;
    let perms = std::fs::Permissions::from_mode(0o700);
    std::fs::set_permissions(path, perms)?;
    Ok(())
}

/// Set file permissions to 0600 (owner read/write only).
#[cfg(unix)]
pub fn set_file_permissions(path: &Path) -> NakamaResult<()> {
    use std::os::unix::fs::PermissionsExt;
    let perms = std::fs::Permissions::from_mode(0o600);
    std::fs::set_permissions(path, perms)?;
    Ok(())
}

/// Check if a file has secure permissions (not world/group readable).
#[cfg(unix)]
pub fn check_secure_permissions(path: &Path) -> NakamaResult<bool> {
    use std::os::unix::fs::PermissionsExt;
    let metadata = std::fs::metadata(path)?;
    let mode = metadata.permissions().mode();
    // Check that group and other have no permissions
    Ok(mode & 0o077 == 0)
}

#[cfg(not(unix))]
pub fn set_dir_permissions(_path: &Path) -> NakamaResult<()> {
    // Windows doesn't use Unix permissions — rely on DPAPI for secrets
    Ok(())
}

#[cfg(not(unix))]
pub fn set_file_permissions(_path: &Path) -> NakamaResult<()> {
    Ok(())
}

#[cfg(not(unix))]
pub fn check_secure_permissions(_path: &Path) -> NakamaResult<bool> {
    Ok(true)
}
