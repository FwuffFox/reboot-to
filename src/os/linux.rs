use std::{error::Error, fmt::Debug, process::Command};

pub struct EfiBootMgr;

#[derive(Debug)]
pub struct EfiBootMgrOutput {
    boot_next: Option<u32>,
    boot_current: u32,
    timeout: Option<u32>,
    boot_order: Vec<u32>,
    boot_entry: Vec<EfiBootEntry>,
}

impl EfiBootMgrOutput {
    fn new() -> EfiBootMgrOutput {
        EfiBootMgrOutput {
            boot_next: None,
            boot_current: 0,
            timeout: None,
            boot_order: vec![],
            boot_entry: vec![],
        }
    }
}

#[derive(Debug)]
pub struct EfiBootEntry {
    boot_num: u32,
    boot_label: String,
}

impl EfiBootMgr {
    pub fn get_boot_info() -> Result<EfiBootMgrOutput, Box<dyn Error>> {
        let mut efibootmgr_output = EfiBootMgrOutput::new();

        let output = Command::new("efibootmgr").output()?;
        let result = output.stdout.split(|&x| x == b'\n');

        for line in result {
            Self::process_line(&mut efibootmgr_output, line)?;
        }

        Ok(efibootmgr_output)
    }

    fn process_line(
        efibootmgr_output: &mut EfiBootMgrOutput,
        line: &[u8],
    ) -> Result<(), Box<dyn Error>> {
        let parts = line.split_once(|&x| x == b':');
        match parts {
            Some(x) => {
                let (name, value) = (
                    String::from_utf8(x.0.to_vec())?,
                    String::from_utf8(x.1.to_vec())?,
                );

                let (name, value) = (name.trim(), value.trim());

                match name {
                    "BootNext" => efibootmgr_output.boot_next = Some(value.parse()?),

                    "BootCurrent" => efibootmgr_output.boot_current = value.parse()?,

                    "BootOrder" => {
                        efibootmgr_output.boot_order = value
                            .split(|x| x == ',')
                            .map(|x| x.trim().parse().unwrap())
                            .collect()
                    },

                    "Timeout" => efibootmgr_output.timeout = Some(value.parse()?),

                    _ => println!("Unknown entry: {name} {value}"),
                }
            }
            None => println!("No boot enries support yet"),
        }
        Ok(())
    }
}
