use std::{error::Error, io, path::Path, process::Command};
use nix::libc::reboot;
use nix::unistd::Uid;

pub mod linux;

pub enum OperatingSystemType {
    Linux,
    Windows,
}

pub trait OperatingSystem {
    fn get_required_binaries(&self) -> Vec<&str>;
    fn is_uefi_available(&self) -> bool;
    fn can_access_boot_entries(&self) -> bool;
    fn print_boot_info(&self) -> Result<(), Box<dyn Error>>;
    fn change_boot_next(&self, num: u32) -> Result<(), Box<dyn Error>>;

    fn reboot(&self);
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

    fn print_boot_info(&self) -> Result<(), Box<dyn Error>> {
        let boot_info = linux::get_boot_info_from_efibootmgr()?;

        let current = boot_info
            .boot_entries
            .iter()
            .find(|&x| x.boot_num == boot_info.boot_current)
            .unwrap();

        println!("Current => {current}");

        for ele in boot_info.boot_entries {
            println!("{ele}");
        }

        Ok(())
    }

    fn change_boot_next(&self, num: u32) -> Result<(), Box<dyn Error>> {
        if !Uid::effective().is_root() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "Changing boot_entry requires sudo!",
            )));
        }

        let output = Command::new("efibootmgr")
            .arg("--bootnext")
            .arg(num.to_string())
            .output()?;

        if !output.status.success() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                std::str::from_utf8(&output.stderr)?.trim(),
            )));
        }

        Ok(())
    }

    fn reboot(&self) {
        Command::new("reboot")
            .spawn().unwrap()
            .wait().expect("Failure to reboot.");
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

    fn print_boot_info(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn change_boot_next(&self, num: u32) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn reboot(&self) {
        todo!()
    }
}
