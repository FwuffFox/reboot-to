use std::error::Error;
pub mod linux;

trait OperatingSystem {
    fn get_required_binaries() -> Vec<String>;
    fn is_uefi_available() -> bool;
    fn can_access_boot_entries() -> bool;
    fn get_boot_entries();
    fn change_boot_entry() -> Result<(), Box<dyn Error>>;
}

