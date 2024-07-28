use std::{fs::File, os, path::Path};

fn main() {
    if !is_uefi_available() {
        panic!("UEFI is not awailable on your platform.");
    }
}
