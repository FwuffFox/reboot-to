

use std::{path::Path, process::Command};

use efibootmgr::EfiBootMgr;

use super::OperatingSystem;

pub mod efibootmgr;

struct Linux;


impl OperatingSystem for Linux {
    fn get_required_binaries() -> Vec<String> {
        vec!["efibootmgr".to_string()]
    }

    fn is_uefi_available() -> bool {
        Path::new("/sys/firmware/efi").exists()
    }

    fn can_access_boot_entries() -> bool {
        // !todo: Find a better way to check if binaries exist
        Linux::is_uefi_available()
            && Command::new("efibootmgr")
                .spawn()
                .is_ok()
    }

    fn get_boot_entries() {
        
    }

    fn change_boot_entry() -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}