use crate::{
    printer::{Printer, PrinterState},
    printer_job::PrintJob,
    shared::interface::PlatformPrinterGetters,
};

use std::env;
use std::process::Command;

mod powershell;
mod winspool;

/**
 * Get printers on windows using winspool
 */
pub fn get_printers() -> Vec<Printer> {
    let available_printers = &winspool::enum_printers();
    let mut printers = Vec::<Printer>::new();

    for printer in available_printers {
        let mut state = crate::printer::PrinterState::UNKNOWN;
        let winspool_state = printer.get_state();

        if winspool_state == "0" {
            state = PrinterState::READY;
        }

        if winspool_state == "1" || winspool_state == "2" {
            state = PrinterState::PAUSED;
        }

        if winspool_state == "5" {
            state = PrinterState::PRINTING;
        }

        printers.push(Printer::from_platform_printer_getters(printer, state));
    }

    return printers;
}

/**
 * Print on windows systems using winspool
 */
pub fn print(
    printer_system_name: &str,
    file_path: &str,
    _job_name: Option<&str>,
) -> Result<bool, String> {
    let dir: std::path::PathBuf = env::temp_dir();

    let sumatra_pdf_path = format!("{}SumatraPDF.exe", dir.display());
    //let sumatra_pdf_path = "src\\windows\\lib\\SumatraPDF.exe";
    let status = Command::new(&sumatra_pdf_path)
        .arg("-silent")
        .arg("-print-to")
        .arg(printer_system_name)
        .arg(file_path)
        .spawn();

    return if status.is_ok() {
        Result::Ok(true)
    } else {
        Result::Err("failure to send document to printer".to_string())
    };
}

/**
 * Get print queue on windows systems using winspool
 */
pub fn print_queue(printer_system_name: &str, _myjobs: i32, _whichjobs: i32) -> Vec<PrintJob> {
    let queue_jobs = powershell::get_jobs(printer_system_name.to_string());
    queue_jobs
}

pub fn cancel_job(_printer_system_name: &str, _job_id: i32) -> bool {
    //return cups::cancel_job(printer_system_name, job_id);
    return false;
}
