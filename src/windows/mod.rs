use crate::{printer::{ JobState, PrintJob, Printer, PrinterState}, shared::interface::{JobGetters, PlatformPrinterGetters}};
use std::process::Command;

mod winspool;
mod util;

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
pub fn print(printer_system_name: &str, file_path: &str, job_name: Option<&str>) -> Result<bool, String> {
    let dir: std::path::PathBuf = env::temp_dir();
   
    let print = format!("-print-to {}", printer_system_name).to_owned();
    let shell_command = format!("{}SumatraPDF.exe {} -silent {}", dir.display(), print, file_path);

    //println!("{}", shell_command);
    let status = Command::new("powershell").args([shell_command]).spawn();

    return if status.is_ok() {
        Result::Ok(true)
    } else {
        Result::Err("failure to send document to printer".to_string())
    }

    //let mut print: String = "-print-to-default".to_owned();    
    //.arg("-print-settings").arg("paper=A4")

    /*let mut command  = Command::new("src\\windows\\lib\\SumatraPDF-3.5.2-64.exe");
    let status = command.arg("-silent")
    .arg("-print-to").arg(printer_system_name).arg(file_path).spawn();
 
    return if status.is_ok() {
        Result::Ok(true)
    } else {
        Result::Err("failure to send document to printer".to_string())
    }*/

    /*let result = &winspool::print_file(printer_system_name, file_path, job_name);
    return if result {
        Result::Ok(true)
    } else {
        Result::Err("failure on send document to printer".to_string())
    }*/

    // let result = lpr::add_job("123".as_bytes(), printer_system_name, job_name);
    /*let job_name = job_name.unwrap_or(file_path);
    let status = Command::new("powershell")
    .args(&[
        "-Command",
        &format!(
            "Start-Job -ScriptBlock {{ Get-Content '{}' | Out-Printer -Name '{}' }} -Name '{}' *> $null; Wait-Job -Name '{}' | Receive-Job *> $null",
            file_path, printer_system_name, job_name, job_name
        ),
    ])
    .spawn();

    return if status.is_ok() {
        Result::Ok(true)
    } else {
        Result::Err("failure to send document to printer".to_string())
    }*/
}

/**
 * Get print queue on windows systems using winspool
 */
pub fn print_queue(printer_system_name: &str, myjobs: i32, whichjobs: i32) -> Vec<PrintJob>  { 
    let queue_jobs= &winspool::enum_jobs(printer_system_name, myjobs, whichjobs);
    let mut jobs: Vec<PrintJob> = vec![];
    
    //println!("job -->  {:?}", queue_jobs);
    use crate::shared::interface::JobGetters;

    if queue_jobs.len() > 0 {
        for job in queue_jobs {
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
           
            jobs.push(Printer::from_job_getters(job, state));
        }
    }
    
    return jobs;
}


pub fn cancel_job(_printer_system_name: &str, _job_id: i32) -> bool {
    //return cups::cancel_job(printer_system_name, job_id);
    return false;
}

/*
pub fn get_job(_printer_system_name: &str, _job_id: i32) -> Vec<PrintJob> {
   let winspool_job = &winspool::get_job(_printer_system_name,&_job_id);
   let mut jobs: Vec<PrintJob> = vec![];
   for job in winspool_job {
     let job_state = job.get_state();
     let mut state = crate::printer::JobState::UNKNOWN;

     jobs.push(Printer::from_job_getters(job,state));
   }

   return jobs;
} */