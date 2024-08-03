use std::env;

use clap::{Parser, Subcommand};

use reboot_to::os::{Linux, OperatingSystem, Windows};

#[derive(Debug, Parser)]
#[command(name = "reboot-to")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Displays boot info
    List,
    /// Changes the boot_next value.
    #[command(arg_required_else_help = true)]
    Change {
        /// Number of a boot entry.
        number: u32,
        /// Reboot after successful execution
        #[arg(short, long, action)]
        reboot: bool,
    },
}

fn main() {
    let system: Box<dyn OperatingSystem> = match env::consts::OS {
        "linux" => Box::new(Linux),
        "windows" => Box::new(Windows),
        &_ => panic!("Other OS not supported"),
    };

    let cli = Cli::parse();
    println!("{cli:?}");

    match cli.command {
        Commands::List { .. } => system
                .print_boot_info()
                .unwrap_or_else(|e| eprintln!("{e:?}")),

        Commands::Change { number, reboot } => {
            if let Err(e) = system.change_boot_next(number) {
                eprintln!("{e:?}");
                return;
            }
            println!("Successfully changed Boot Next to {number}");

            if reboot {
                println!("Rebooting system!");
                system.reboot();
            }
        },
    }
}
