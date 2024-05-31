use crate::printer_job::{JobState, PrintJob};
use crate::shared::interface::{JobGetters, PlatformPrinterGetters};

/**
 * Enum of the Printer state
 */
#[derive(Debug, Clone)]
pub enum PrinterState {
    /**
     * The printer is able to receive jobs (also idle)
     */
    READY,

    /**
     * The printer is not accepting jobs (also stopped)
     */
    PAUSED,

    /**
     * The printer is now printing an document (also processing)
     */
    PRINTING,

    /**
     * All other status like error, resources, manual intervention, etc...
     */
    UNKNOWN,
}

/**
 * Printer is a struct to representation the system printer
 * They has an ID composed by your system_name and has printing method to print directly
 */
pub struct Printer {
    /**
     * Visual reference of system printer name
     */
    pub name: String,

    /**
     * Name of Printer exactly as on system
     */
    pub system_name: String,

    /**
     * Name of the Printer driver
     */
    pub driver_name: String,

    /**
     * Uri of Print (default is empty string)
     */
    pub uri: String,

    /**
     * Location definition of printer (default is empty string)
     */
    pub location: String,

    /**
     * Definition if the printer is the default printer
     */
    pub is_default: bool,

    /**
     * Definition if the printer is shared
     */
    pub is_shared: bool,

    /**
     * The state of the printer
     */
    pub state: PrinterState,
}

impl std::fmt::Debug for Printer {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "Printer {{
                \r  name: {:?},
                \r  state: {:?},
                \r  system_name: {:?},
                \r  is_default: {:?},
                \r  uri: {:?},
                \r  is_shared: {:?},
                \r  location: {:?},
                \r  driver_name: {:?}
            \r}}",
            self.name,
            self.state,
            self.system_name,
            self.is_default,
            self.uri,
            self.is_shared,
            self.location,
            self.driver_name
        )
    }
}

impl Clone for Printer {
    fn clone(&self) -> Printer {
        return Printer {
            name: self.name.clone(),
            state: self.state.clone(),
            uri: self.uri.clone(),
            location: self.location.clone(),
            is_default: self.is_default.clone(),
            system_name: self.system_name.clone(),
            driver_name: self.driver_name.clone(),
            is_shared: self.is_shared.clone(),
        };
    }
}

impl Printer {
    pub fn from_platform_printer_getters(
        platform_printer: &dyn PlatformPrinterGetters,
        state: PrinterState,
    ) -> Printer {
        let printer = Printer {
            name: platform_printer.get_name(),
            system_name: platform_printer.get_system_name(),
            driver_name: platform_printer.get_marker_and_model(),
            location: platform_printer.get_location(),
            state,
            uri: platform_printer.get_uri(),
            is_default: platform_printer.get_is_default(),
            is_shared: platform_printer.get_is_shared(),
        };

        return printer;
    }

    /**
     * Print bytes with self printer instance
     */
    pub fn print(&self, buffer: &[u8], job_name: Option<&str>) -> Result<bool, String> {
        return crate::print(&self.system_name, buffer, job_name);
    }

    /**
     * Print specific file with self printer instance
     */
    pub fn print_file(&self, file_path: &str, job_name: Option<&str>) -> Result<bool, String> {
        return crate::print_file(&self.system_name, file_path, job_name);
    }

    /**
     * Return all jobs in print queue
     */
    pub fn from_job_getters(select_job: &dyn JobGetters, state: JobState) -> PrintJob {
        let job = PrintJob {
            id: select_job.get_id(),
            dest: select_job.get_dest(),
            title: select_job.get_title(),
            user: select_job.get_user(),
            format: select_job.get_format(),
            state,
            size: select_job.get_size(),
            priority: select_job.get_priority(),
            creation_time: select_job.get_creation_time(),
            completed_time: select_job.get_completed_time(),
            processing_time: select_job.get_processing_time(),
        };
        return job;
    }

    /**
     *  Return all jobs in print queue
     */
    pub fn print_queue(&self, myjobs: i32, whichjobs: i32) -> Vec<PrintJob> {
        return crate::print_queue(&self.system_name, myjobs, whichjobs);
    }

    /**
     *   Cancel a job in print queue
     */
    pub fn cancel_job(&self, job_id: i32) -> bool {
        return crate::cancel_job(&self.system_name, job_id);
    }

    /**
     * Return the last error message
     */
    pub fn get_last_error() -> String {
        return crate::get_last_error();
    }
}
