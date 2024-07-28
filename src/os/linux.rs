use regex::Regex;
use std::{error::Error, fmt::Debug, process::Command, str};

pub struct EfiBootMgr;

#[derive(Debug)]
pub struct EfiBootMgrOutput {
    boot_next: Option<u32>,
    boot_current: u32,
    timeout: Option<u32>,
    boot_order: Vec<u32>,
    boot_entries: Vec<EfiBootEntry>,
}

impl EfiBootMgrOutput {
    fn new() -> EfiBootMgrOutput {
        EfiBootMgrOutput {
            boot_next: None,
            boot_current: 0,
            timeout: None,
            boot_order: vec![],
            boot_entries: vec![],
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
        if !output.status.success() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "efibootmgr command failed",
            )));
        }
        let stdout = str::from_utf8(&output.stdout)?.trim();
        let result = stdout.lines();

        for line in result {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            Self::process_line(&mut efibootmgr_output, line)?;
        }

        Ok(efibootmgr_output)
    }

    fn process_line(
        efibootmgr_output: &mut EfiBootMgrOutput,
        line: &str,
    ) -> Result<(), Box<dyn Error>> {
        let parts = line.split_once(':');
        match parts {
            Some((name, value)) => {
                let (name, value) = (name.trim(), value.trim());

                match name {
                    "BootNext" => efibootmgr_output.boot_next = Some(value.parse()?),

                    "BootCurrent" => efibootmgr_output.boot_current = value.parse()?,

                    "BootOrder" => {
                        efibootmgr_output.boot_order = value
                            .split(|x| x == ',')
                            .map(|x| x.trim().parse().unwrap())
                            .collect()
                    }

                    "Timeout" => efibootmgr_output.timeout = Some(value.parse()?),

                    _ => println!("Unknown entry: {} = {}", name, value),
                }
            }
            None => {
                let regex = Regex::new(r"Boot(\d{4})\* (.+?)\t").unwrap();
                if let Some(captures) = regex.captures(line) {
                    println!("Regex matched: {:?}", captures);
                    efibootmgr_output.boot_entries.push(EfiBootEntry {
                        boot_num: captures[1].parse()?,
                        boot_label: captures[2].to_string(),
                    });
                } else {
                    println!("No match found for line: {}", line);
                }
            }
        }
        Ok(())
    }
}
