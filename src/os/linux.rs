use regex::Regex;
use std::fmt::{Display, Formatter};
use std::{error::Error, fmt::Debug, process::Command, str};

#[derive(Debug)]
pub struct BootInfo {
    pub boot_next: Option<u32>,
    pub boot_current: u32,
    pub timeout: Option<u32>,
    pub boot_order: Vec<u32>,
    pub boot_entries: Vec<BootEntry>,
}

impl BootInfo {
    fn new() -> BootInfo {
        BootInfo {
            boot_next: None,
            boot_current: 0,
            timeout: None,
            boot_order: vec![],
            boot_entries: vec![],
        }
    }
}

#[derive(Debug)]
pub struct BootEntry {
    pub boot_num: u32,
    pub boot_label: String,
}

impl Display for BootEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.boot_num, self.boot_label)
    }
}

pub fn get_boot_info_from_efibootmgr() -> Result<BootInfo, Box<dyn Error>> {
    let output = Command::new("efibootmgr").output()?;
    if !output.status.success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "efibootmgr command failed",
        )));
    }
    let stdout = str::from_utf8(&output.stdout)?.trim();
    get_boot_info_from_str(stdout)
}

pub fn get_boot_info_from_str(data: &str) -> Result<BootInfo, Box<dyn Error>> {
    let mut boot_info = BootInfo::new();

    let result = data.lines();

    for line in result {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        process_line(&mut boot_info, line)?;
    }

    Ok(boot_info)
}

fn process_line(boot_info: &mut BootInfo, line: &str) -> Result<(), Box<dyn Error>> {
    let parts = line.split_once(':');

    if let Some((name, value)) = parts {
        let (name, value) = (name.trim(), value.trim());

        match name {
            "BootNext" => boot_info.boot_next = Some(value.parse()?),

            "BootCurrent" => boot_info.boot_current = value.parse()?,

            "BootOrder" => {
                boot_info.boot_order = value
                    .split(',')
                    .map(|x| x.trim().parse().unwrap())
                    .collect();
            }

            // "Timeout" => efibootmgr_output.timeout = Some(value.parse()?),
            _ => println!("Unknown entry: {name} = {value}"),
        }
    } else {
        let regex = Regex::new(r"Boot(\d{4})\* (.+?)\t").unwrap();
        if let Some(captures) = regex.captures(line) {
            boot_info.boot_entries.push(BootEntry {
                boot_num: captures[1].parse()?,
                boot_label: captures[2].to_string(),
            });
        }
    }

    Ok(())
}
