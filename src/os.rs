use std::{error::Error, path::Path, process::Command};

use linux::EfiBootMgr;

pub mod linux;

pub enum OperatingSystemType {
    Linux,
    Windows,
}

pub trait OperatingSystem {
    fn get_required_binaries(&self) -> Vec<&str>;
    fn is_uefi_available(&self) -> bool;
    fn can_access_boot_entries(&self) -> bool;
    fn get_boot_entries(&self) -> Result<(), Box<dyn Error>>;
    fn change_boot_entry(&self) -> Result<(), Box<dyn Error>>;
}

pub struct Linux;

impl OperatingSystem for Linux {
    fn get_required_binaries(&self) -> Vec<&str> {
        vec!["efibootmgr"]
    }

    fn is_uefi_available(&self) -> bool {
        Path::new("/sys/firmware/efi").exists()
    }

    fn can_access_boot_entries(&self) -> bool {
        // !todo: Find a better way to check if binaries exist
        self.is_uefi_available() && Command::new("efibootmgr").spawn().is_ok()
    }

    fn get_boot_entries(&self) -> Result<(), Box<dyn Error>> {
        let boot_info = EfiBootMgr::get_boot_info()?;
        println!("{:?}", boot_info);

        Ok(())
    }

    fn change_boot_entry(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

pub struct Windows;

impl OperatingSystem for Windows {
    fn get_required_binaries(&self) -> Vec<&str> {
        todo!()
    }

    fn is_uefi_available(&self) -> bool {
        todo!()
    }

    fn can_access_boot_entries(&self) -> bool {
        todo!()
    }

    fn get_boot_entries(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn change_boot_entry(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
