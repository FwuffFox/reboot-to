use std::{error::Error, io::{self, Stdout}, process::Command};

pub struct EfiBootMgr;

pub struct EfiBootMgrOutput {
    boot_current: u32,
    boot_order: Vec<u32>,
    boot_next: u32,
    boot_entry: Vec<EfiBootEntry>,
}

pub struct EfiBootEntry {
    boot_num: u32,
    boot_label: String,
}

impl EfiBootMgr {
    fn get_boot_entries() -> Result<EfiBootMgrOutput, Box<dyn Error>> {
        let output = Command::new("efibootmgr").output()?;
        let mut result = 
                output
                .stdout
                .split(|&x| x == b'\n');

        return Ok(EfiBootMgrOutput {
            boot_current: String::from_utf8(result.next().unwrap()
                                .split(|&x| x == b' ')
                                .nth(1).unwrap().to_vec())?.parse()?,

            boot_order: todo!(),
            boot_next: todo!(),
            boot_entry: todo!(),
                                
        });
    }
}