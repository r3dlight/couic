use std::fs;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use caps::{CapSet, Capability, drop, has_cap};
use nix::sys::prctl;
use nix::unistd::{Group, User, chown};

pub const SEC_FILE_PERM: u32 = 0o600;
pub const SEC_DIR_PERM: u32 = 0o755;
pub const SEC_SOCKET_PERM: u32 = 0o770;

#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("User not found: {0}")]
    UserNotFound(String),
    #[error("Group not found: {0}")]
    GroupNotFound(String),
    #[error("Failed to change owner/group: {0}")]
    ChownFailed(String),
    #[error("Failed to set permissions: {0}")]
    PermissionFailed(String),
    #[error("Ownership or permissions incorrect: {0}")]
    OwnershipOrPermsIncorrect(String),
    #[error("Capability error: {0}")]
    CapabilityError(String),
    #[error("Privilege error: {0}")]
    PrivilegeError(String),
}

pub struct SecurityService;

impl SecurityService {
    pub fn set_owner_group_perms<P: AsRef<Path>>(
        path: P,
        username: &str,
        group_name: &str,
        mode: u32,
    ) -> Result<(), SecurityError> {
        let uid = match User::from_name(username) {
            Ok(Some(user)) => user.uid,
            Ok(None) => return Err(SecurityError::UserNotFound(username.to_string())),
            Err(e) => return Err(SecurityError::UserNotFound(format!("{username}: {e}"))),
        };
        let gid = match Group::from_name(group_name) {
            Ok(Some(group)) => group.gid,
            Ok(None) => return Err(SecurityError::GroupNotFound(group_name.to_string())),
            Err(e) => return Err(SecurityError::GroupNotFound(format!("{group_name}: {e}"))),
        };
        if let Err(e) = chown(path.as_ref(), Some(uid), Some(gid)) {
            return Err(SecurityError::ChownFailed(e.to_string()));
        }
        if let Err(e) = fs::set_permissions(&path, fs::Permissions::from_mode(mode)) {
            return Err(SecurityError::PermissionFailed(e.to_string()));
        }
        Ok(())
    }

    pub fn check_owner_group_perms<P: AsRef<Path>>(
        path: P,
        username: &str,
        group_name: &str,
        mode: u32,
    ) -> Result<(), SecurityError> {
        let uid = match User::from_name(username) {
            Ok(Some(user)) => user.uid,
            Ok(None) => return Err(SecurityError::UserNotFound(username.to_string())),
            Err(e) => return Err(SecurityError::UserNotFound(format!("{username}: {e}"))),
        };
        let gid = match Group::from_name(group_name) {
            Ok(Some(group)) => group.gid,
            Ok(None) => return Err(SecurityError::GroupNotFound(group_name.to_string())),
            Err(e) => return Err(SecurityError::GroupNotFound(format!("{group_name}: {e}"))),
        };
        let metadata = fs::metadata(&path)
            .map_err(|e| SecurityError::OwnershipOrPermsIncorrect(e.to_string()))?;
        let file_uid = metadata.uid();
        let file_gid = metadata.gid();
        let file_mode = metadata.permissions().mode() & 0o777;
        if file_uid != uid.as_raw() || file_gid != gid.as_raw() || file_mode != mode {
            return Err(SecurityError::OwnershipOrPermsIncorrect(format!(
                "File {} has uid={}, gid={}, mode={:o} (expected uid={}, gid={}, mode={:o})",
                path.as_ref().display(),
                file_uid,
                file_gid,
                file_mode,
                uid.as_raw(),
                gid.as_raw(),
                mode
            )));
        }
        Ok(())
    }

    pub fn check_required_capabilities() -> Result<(), SecurityError> {
        let required_caps = [Capability::CAP_NET_ADMIN, Capability::CAP_SYS_ADMIN];
        for &cap in &required_caps {
            match has_cap(None, CapSet::Effective, cap) {
                Ok(true) => {}
                Ok(false) => {
                    return Err(SecurityError::CapabilityError(format!(
                        "Missing capability to load ebpf xdp program: {cap:?}"
                    )));
                }
                Err(e) => {
                    return Err(SecurityError::CapabilityError(format!(
                        "Error checking capability {cap:?}: {e}"
                    )));
                }
            }
        }
        Ok(())
    }

    pub fn drop_all_caps_nonewprivs() -> Result<(), SecurityError> {
        let all_caps = caps::all();
        for cap in all_caps {
            for set in &[CapSet::Effective, CapSet::Permitted, CapSet::Inheritable] {
                if let Err(e) = drop(None, *set, cap) {
                    return Err(SecurityError::CapabilityError(format!(
                        "Failed to drop capability {cap:?} from {set:?}: {e}"
                    )));
                }
            }
        }
        if let Err(e) = prctl::set_no_new_privs() {
            return Err(SecurityError::PrivilegeError(format!(
                "Failed to set no_new_privs: {e}"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nix::unistd::{Group, User, getgid, getuid};
    use tempfile::{NamedTempFile, TempDir};

    // Helper to get current user/group names
    fn get_current_user_group() -> (String, String) {
        let uid = getuid();
        let gid = getgid();

        let username = User::from_uid(uid)
            .unwrap()
            .map(|u| u.name)
            .unwrap_or_else(|| "root".to_string());

        let groupname = Group::from_gid(gid)
            .unwrap()
            .map(|g| g.name)
            .unwrap_or_else(|| "root".to_string());

        (username, groupname)
    }

    #[test]
    fn test_set_owner_group_perms_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let (username, groupname) = get_current_user_group();

        // Test setting permissions to SEC_FILE_PERM
        let result = SecurityService::set_owner_group_perms(
            temp_file.path(),
            &username,
            &groupname,
            SEC_FILE_PERM,
        );

        assert!(result.is_ok());

        // Verify the permissions were actually set
        let metadata = fs::metadata(temp_file.path()).unwrap();
        let mode = metadata.permissions().mode() & 0o777;
        assert_eq!(mode, SEC_FILE_PERM);
    }

    #[test]
    fn test_set_owner_group_perms_nonexistent_user() {
        let temp_file = NamedTempFile::new().unwrap();
        let (_, groupname) = get_current_user_group();

        let result = SecurityService::set_owner_group_perms(
            temp_file.path(),
            "nonexistent_user_12345",
            &groupname,
            SEC_FILE_PERM,
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            SecurityError::UserNotFound(user) => {
                assert!(user.contains("nonexistent_user_12345"));
            }
            _ => panic!("Expected UserNotFound error"),
        }
    }

    #[test]
    fn test_set_owner_group_perms_nonexistent_group() {
        let temp_file = NamedTempFile::new().unwrap();
        let (username, _) = get_current_user_group();

        let result = SecurityService::set_owner_group_perms(
            temp_file.path(),
            &username,
            "nonexistent_group_12345",
            SEC_FILE_PERM,
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            SecurityError::GroupNotFound(group) => {
                assert!(group.contains("nonexistent_group_12345"));
            }
            _ => panic!("Expected GroupNotFound error"),
        }
    }

    #[test]
    fn test_set_owner_group_perms_nonexistent_file() {
        let (username, groupname) = get_current_user_group();

        let result = SecurityService::set_owner_group_perms(
            "/nonexistent/path/to/file",
            &username,
            &groupname,
            SEC_FILE_PERM,
        );

        assert!(result.is_err());
        // Could be ChownFailed or PermissionFailed depending on which operation fails first
        assert!(matches!(
            result.unwrap_err(),
            SecurityError::ChownFailed(_) | SecurityError::PermissionFailed(_)
        ));
    }

    #[test]
    fn test_check_owner_group_perms_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let (username, groupname) = get_current_user_group();

        // First set the permissions
        SecurityService::set_owner_group_perms(temp_file.path(), &username, &groupname, 0o644)
            .unwrap();

        // Then check them
        let result = SecurityService::check_owner_group_perms(
            temp_file.path(),
            &username,
            &groupname,
            0o644,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_check_owner_group_perms_wrong_permissions() {
        let temp_file = NamedTempFile::new().unwrap();
        let (username, groupname) = get_current_user_group();

        // Set permissions to SEC_FILE_PERM
        SecurityService::set_owner_group_perms(
            temp_file.path(),
            &username,
            &groupname,
            SEC_FILE_PERM,
        )
        .unwrap();

        // Check for different permissions (0o644)
        let result = SecurityService::check_owner_group_perms(
            temp_file.path(),
            &username,
            &groupname,
            0o644,
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            SecurityError::OwnershipOrPermsIncorrect(msg) => {
                assert!(msg.contains("mode=600"));
                assert!(msg.contains("expected"));
                assert!(msg.contains("mode=644"));
            }
            _ => panic!("Expected OwnershipOrPermsIncorrect error"),
        }
    }

    #[test]
    fn test_check_owner_group_perms_nonexistent_file() {
        let (username, groupname) = get_current_user_group();

        let result = SecurityService::check_owner_group_perms(
            "/nonexistent/path/to/file",
            &username,
            &groupname,
            SEC_FILE_PERM,
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            SecurityError::OwnershipOrPermsIncorrect(_) => {}
            _ => panic!("Expected OwnershipOrPermsIncorrect error"),
        }
    }

    #[test]
    fn test_check_owner_group_perms_nonexistent_user() {
        let temp_file = NamedTempFile::new().unwrap();
        let (_, groupname) = get_current_user_group();

        let result = SecurityService::check_owner_group_perms(
            temp_file.path(),
            "nonexistent_user_12345",
            &groupname,
            SEC_FILE_PERM,
        );

        assert!(result.is_err());
        match result.unwrap_err() {
            SecurityError::UserNotFound(_) => {}
            _ => panic!("Expected UserNotFound error"),
        }
    }

    #[test]
    fn test_security_workflow() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test_file.txt");
        let (username, groupname) = get_current_user_group();

        // Create a test file
        fs::write(&test_file, "test content").unwrap();

        // Set owner, group, and permissions
        SecurityService::set_owner_group_perms(&test_file, &username, &groupname, SEC_FILE_PERM)
            .unwrap();

        // Verify they were set correctly
        SecurityService::check_owner_group_perms(&test_file, &username, &groupname, SEC_FILE_PERM)
            .unwrap();

        // Change permissions and verify the check fails
        fs::set_permissions(&test_file, fs::Permissions::from_mode(0o644)).unwrap();

        let result = SecurityService::check_owner_group_perms(
            &test_file,
            &username,
            &groupname,
            SEC_FILE_PERM,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_various_permission_modes() {
        let (username, groupname) = get_current_user_group();

        let modes = [0o600, 0o644, 0o755, 0o777, 0o400, 0o444];

        for &mode in &modes {
            let temp_file = NamedTempFile::new().unwrap();

            // Set the permissions
            SecurityService::set_owner_group_perms(temp_file.path(), &username, &groupname, mode)
                .unwrap();

            // Verify they were set correctly
            let metadata = fs::metadata(temp_file.path()).unwrap();
            let actual_mode = metadata.permissions().mode() & 0o777;
            assert_eq!(actual_mode, mode, "Mode mismatch for {:o}", mode);

            // Verify check_owner_group_perms works
            SecurityService::check_owner_group_perms(temp_file.path(), &username, &groupname, mode)
                .unwrap();
        }
    }
}
