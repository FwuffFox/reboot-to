use std::env;

use reboot_to::os::{Linux, OperatingSystem, Windows};

fn main() {
    println!("{}", env::consts::OS);
    let system: Box<dyn OperatingSystem> = if env::consts::OS == "linux" {
        Box::new(Linux)
    } else {
        Box::new(Windows)
    };

    system
        .get_boot_entries()
        .unwrap_or_else(|e| eprintln!("{e:?}"));
}
