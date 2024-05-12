use std::str;
use crate::printer::{ JobState, PrintJob, Printer, PrinterState};


mod cups;

/**
 * Get printers on unix systems using CUPS
 */
pub fn get_printers() -> Vec<Printer> {

    let cups_dests = &cups::get_dests();
    let mut printers: Vec<Printer> = vec![];

    use crate::shared::interface::PlatformPrinterGetters;

    for dest in cups_dests {        

        let mut state = crate::printer::PrinterState::UNKNOWN;
        let cups_state = dest.get_state();

        if cups_state == "3" {
            state = PrinterState::READY;
        }
        
        if cups_state == "4" {
            state = PrinterState::PRINTING;
        }

        if cups_state == "5" {
            state = PrinterState::PAUSED;
        }

        printers.push(Printer::from_platform_printer_getters(dest.clone(), state));
    }

    cups::free_dests(cups_dests);
    return printers;
}

/**
 * Print on unix systems using CUPS
 */
pub fn print(printer_system_name: &str, file_path: &str, job_name: Option<&str>) -> Result<bool, String> {
    let result = cups::print_file(printer_system_name, file_path, job_name);
    return if result {
        Result::Ok(true)
    } else {
        Result::Err("failure on send document to printer".to_string())
    }
}

/**
 * Get print queue on unix systems using CUPS
 */
pub fn print_queue(printer_system_name: &str, myjobs: i32, whichjobs: i32) -> Vec<PrintJob>  { 
    let cups_jobs= &cups::get_print_queue(printer_system_name, myjobs, whichjobs);
    let mut jobs: Vec<PrintJob> = vec![];

    if cups_jobs.len() > 0 {

        use crate::shared::interface::JobGetters;

        for job in cups_jobs {
            let mut state = crate::printer::JobState::UNKNOWN;
            let cups_state = job.get_state();

            if cups_state == "3" {
                state = JobState::PENDING;
            }
            
            if cups_state == "4" {
                state = JobState::HELD;
            }

            if cups_state == "5" {
                state = JobState::PROCESSING;
            }

            if cups_state == "6" {
                state = JobState::STOPPED;
            }

            if cups_state == "7" {
                state = JobState::CANCELED;
            }

            if cups_state == "8" {
                state = JobState::ABORTED;
            }

            if cups_state == "9" {
                state = JobState::COMPLETED;
            }

            jobs.push(Printer::from_job_getters(job.clone(), state));
        }

        cups::free_jobs(cups_jobs);
    }

    return jobs;
}

pub fn cancel_job(printer_system_name: &str, job_id: i32) -> bool {
    return cups::cancel_job(printer_system_name, job_id);
}

pub fn get_last_error() -> String {
    return cups::get_last_error();
}