#[cfg(windows)]
mod windows {
    use tracing::debug;

    pub fn add_to_exclusion(path: &str) -> Result<(), String> {
        todo!("Add code to add a path to antivirus exclusions");

        debug!("Adding {} to antivirus exclusions...", path);
        Ok(())
    }

    pub fn remove_from_exclusion(path: &str) -> Result<(), String> {
        todo!("Add code to remove a path from antivirus exclusions");

        debug!("Removing {} from antivirus exclusions...", path);
        Ok(())
    }
}

#[cfg(windows)]
pub use self::windows::*;
#[cfg(not(windows))]
pub fn require_admin_privileges() -> Result<(), String> {
    Err("This utility is only supported on Windows.".to_string())
}

pub fn add_to_exclusion(path: &str) -> Result<(), String> {
    #[cfg(windows)]
    return windows::add_to_exclusion(path);
    #[cfg(not(windows))]
    return Err("This utility is only supported on Windows.".to_string());
}

pub fn remove_from_exclusion(path: &str) -> Result<(), String> {
    #[cfg(windows)]
    return windows::remove_from_exclusion(path);
    #[cfg(not(windows))]
    return Err("This utility is only supported on Windows.".to_string());
}
