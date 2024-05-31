//! Get printers and print files or bytes on unix and windows
//!
//! Printers **is not a lib for printer drivers or cups**. Printers is a simple lib to call printers apis for unix *(cups)* and windows *(winspool)* systems.
//! Printer can provide a list of printers available on the system and perform document printing.
//!
//! ```rust
//! use printers;
//!
//! let printers = printers::get_printers();
//!
//! for printer in printers {
//!     let job1 = printer.print("42".as_bytes(), Some("Everything"));
//!     let job2 = printer.print_file("/path/to/any.file", None);
//!
//!     println!("{:?}", printer);
//!     println!("{:?}", job1);
//!     println!("{:?}", job2);
//! }
//! ```
//!
//!
use std::env;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

/// Printer and Job control
pub mod printer;
pub mod printer_job;
pub mod shared;

use printer_job::PrintJob;

#[cfg(target_family = "unix")]
mod unix;

#[cfg(target_family = "windows")]
mod windows;

/**
 * Print bytes on specific printer
 */
pub fn print(printer_name: &str, buffer: &[u8], job_name: Option<&str>) -> Result<bool, String> {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    let tmp_file_path = env::temp_dir().join(time.to_string());

    let mut tmp_file = File::create(&tmp_file_path).unwrap();
    let save = tmp_file.write(buffer);

    if save.is_err() {
        let error = save.err().unwrap();
        return Err(error.to_string());
    }

    return print_file(printer_name, tmp_file_path.to_str().unwrap(), job_name);
}

/**
 * Print specific file on a specific printer
 */
pub fn print_file(
    printer_name: &str,
    file_path: &str,
    job_name: Option<&str>,
) -> Result<bool, String> {
    #[cfg(target_family = "unix")]
    return unix::print(printer_name, file_path, job_name);

    #[cfg(target_family = "windows")]
    return windows::print(printer_name, file_path, job_name);
}

/**
 * Return all available printers on system
 */
pub fn get_printers() -> Vec<printer::Printer> {
    #[cfg(target_family = "windows")]
    return windows::get_printers();

    #[cfg(target_family = "unix")]
    return unix::get_printers();

    #[cfg(target_family = "wasm")]
    panic!("Unsupported Platform");
}

/**
 * If you known the printer Name you can try get the printer directly from they
 */
pub fn get_printer_by_name(name: &str) -> Option<printer::Printer> {
    let printers = get_printers();

    let opt = printers.iter().find(|&printer| {
        return printer.clone().name.eq(name) || printer.clone().system_name.eq(name);
    });

    return opt.cloned();
}

pub fn print_queue(printer_system_name: &str, myjobs: i32, whichjobs: i32) -> Vec<PrintJob> {
    #[cfg(target_family = "unix")]
    return unix::print_queue(printer_system_name, myjobs, whichjobs);

    #[cfg(target_family = "windows")]
    return windows::print_queue(printer_system_name, myjobs, whichjobs);
}

pub fn cancel_job(printer_system_name: &str, job_id: i32) -> bool {
    #[cfg(target_family = "unix")]
    return unix::cancel_job(printer_system_name, job_id);

    #[cfg(target_family = "windows")]
    return windows::cancel_job(printer_system_name, job_id);
}

pub fn get_last_error() -> String {
    #[cfg(target_family = "unix")]
    return unix::get_last_error();

    #[cfg(target_family = "windows")]
    return "".to_string();
    // return windows::get_last_error();
}
