#![allow(non_snake_case)]
use crate::printer_job::{JobState, PrintJob};
use serde::Deserialize;
use serde_json::Value;
use std::os::windows::process::CommandExt;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct PowerShellPrintJob {
    DocumentName: String,
    Id: u32,
    //TotalPages: u32,
    //Position: u32,
    Size: u64,
    SubmittedTime: Option<String>,
    UserName: String,
    //PagesPrinted: u32,
    JobTime: Value, // Allow mixed types by using Value
    //ComputerName: String,
    Datatype: String,
    PrinterName: String,
    Priority: u32,
    JobStatus: u32,
}

impl From<PowerShellPrintJob> for PrintJob {
    fn from(ps_job: PowerShellPrintJob) -> Self {
        let state = match ps_job.JobStatus {
            0 => JobState::HELD,
            19 => JobState::HELD,
            8 => JobState::PROCESSING,
            6 => JobState::STOPPED,
            4 | 256 => JobState::CANCELED,
            2 | 18 => JobState::ABORTED,
            512 | 4096 => JobState::COMPLETED,
            _ => JobState::UNKNOWN,
        };

        let creation_time = match ps_job.SubmittedTime {
            Some(submitted_time) => {
                let timestamp = submitted_time
                    .trim_start_matches("/Date(")
                    .trim_end_matches(")/")
                    .parse::<i64>()
                    .unwrap_or_default(); // Handle parse error gracefully
                timestamp.to_string()
            }
            None => "".to_string(),
        };

        let processing_time = match ps_job.JobTime {
            Value::Number(time) => time.to_string(),
            _ => "".to_string(),
        };

        PrintJob {
            id: ps_job.Id.to_string(),
            dest: ps_job.PrinterName,
            title: ps_job.DocumentName,
            user: ps_job.UserName,
            format: ps_job.Datatype,
            state,
            size: ps_job.Size.to_string(),
            priority: ps_job.Priority.to_string(),
            completed_time: "".to_string(), // These fields are not available from PowerShell data
            creation_time,
            processing_time,
        }
    }
}

/**
 * Get printer job on windows using powershell
 */
pub fn get_jobs(printername: String) -> Vec<PrintJob> {
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-WindowStyle", "Hidden",
            "-Command",
            &format!(
                "Get-PrintJob -PrinterName \"{}\"  | Select-Object DocumentName,Id,TotalPages,Position,Size,SubmmitedTime,UserName,PagesPrinted,JobTime,ComputerName,Datatype,PrinterName,Priority,SubmittedTime,JobStatus | ConvertTo-Json",
                printername
            )
        ])
        .creation_flags(0x08000000) // This flag hides the console window
        .output();
    //println!("job -->  {:?}", output);

    match output {
        Ok(output) => {
            let stdout = String::from_utf8(output.stdout).unwrap_or_default();
            //println!("job -->  {:?}", stdout);

            if stdout.trim().is_empty() {
                println!("PowerShell command returned an empty output.");
                return vec![];
            }
            // Try deserializing as an array first
            let ps_jobs: Result<Vec<PowerShellPrintJob>, _> = serde_json::from_str(&stdout);
            if let Ok(ps_jobs) = ps_jobs {
                return ps_jobs.into_iter().map(PrintJob::from).collect();
            }
            // If deserializing as an array fails, try deserializing as a single object
            let ps_job: Result<PowerShellPrintJob, _> = serde_json::from_str(&stdout);
            if let Ok(ps_job) = ps_job {
                return vec![PrintJob::from(ps_job)];
            }
            // Log the error if both attempts fail
            println!("Error parsing JSON: invalid format");
            vec![]
        }
        Err(e) => {
            println!("Error executing PowerShell command: {}", e);
            vec![]
        }
    }
}
