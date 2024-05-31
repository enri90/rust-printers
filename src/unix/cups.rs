use libc::{c_char, c_int};
use std::{
    ffi::{CStr, CString},
    ptr,
};

use crate::shared::interface::{JobGetters, PlatformPrinterGetters};

/**
 * The CUPS destination struct (cups_job_s)
 * https://www.cups.org/doc/cupspm.html#cups_job_s
 */
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CupsJobS {
    pub id: i32,              /* The job ID */
    pub dest: *mut c_char,    /* Printer or class name */
    pub title: *mut c_char,   /* Title/job name */
    pub user: *mut c_char,    /* User that submitted the job */
    pub format: *mut c_char,  /* Document format */
    pub state: c_char,        /* Job state */
    pub size: c_int,          /* Size in kilobytes */
    pub priority: c_int,      /* Priority (1-100) */
    pub completed_time: i64,  /* Time the job was completed */
    pub creation_time: i64,   /* Time the job was created */
    pub processing_time: i64, /* Time the job was processed */
}

impl JobGetters for CupsJobS {
    fn get_id(&self) -> String {
        return self.id.to_string();
    }

    fn get_dest(&self) -> String {
        if self.dest.is_null() {
            return "".to_string();
        }

        let c_str = unsafe { CStr::from_ptr(self.dest.clone()) };
        return c_str.to_str().unwrap_or("").to_string();
    }

    fn get_title(&self) -> String {
        if self.title.is_null() {
            return "".to_string();
        }

        let c_str = unsafe { CStr::from_ptr(self.title.clone()) };
        return c_str.to_str().unwrap().to_string();
    }

    fn get_user(&self) -> String {
        if self.user.is_null() {
            return "".to_string();
        }

        let c_str = unsafe { CStr::from_ptr(self.user.clone()) };
        return c_str.to_str().unwrap().to_string();
    }

    fn get_format(&self) -> String {
        if self.format.is_null() {
            return "".to_string();
        }

        let c_str = unsafe { CStr::from_ptr(self.format.clone()) };
        return c_str.to_str().unwrap().to_string();
    }

    fn get_state(&self) -> String {
        return self.state.to_string();
    }

    fn get_size(&self) -> String {
        return self.size.to_string();
    }

    fn get_priority(&self) -> String {
        return self.priority.to_string();
    }

    fn get_completed_time(&self) -> String {
        return self.completed_time.to_string();
    }

    fn get_creation_time(&self) -> String {
        return self.creation_time.to_string();
    }

    fn get_processing_time(&self) -> String {
        return self.processing_time.to_string();
    }
}

/**
 * The CUPS option struct (cups_option_s)
 * https://www.cups.org/doc/cupspm.html#cups_option_s
 */
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CupsOptionT {
    pub name: *mut c_char,
    pub value: *mut c_char,
}

/**
 * The CUPS destination struct (cups_dest_s)
 * https://www.cups.org/doc/cupspm.html#cups_dest_s
 */
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CupsDestT {
    name: *mut c_char,
    instance: *mut c_char,
    is_default: c_int,
    num_options: c_int,
    options: *mut CupsOptionT,
}

impl CupsDestT {
    /**
     * Returns a string value of an key on cups options (If the key was not found return a empty string)
     */
    fn get_option_by_key(&self, key: &str) -> String {
        let mut value = "".to_string();

        for i in 1..self.num_options {
            let option_ptr = unsafe { self.options.offset(i as isize) };
            let option = unsafe { &*option_ptr };

            let name: &CStr = unsafe { CStr::from_ptr(option.name.clone()) };

            if name.to_string_lossy() == key {
                let value_srt = unsafe { CStr::from_ptr(option.value.clone()) };
                value = value_srt.to_string_lossy().to_string();
                break;
            }
        }

        return value;
    }
}

impl PlatformPrinterGetters for CupsDestT {
    /**
     * Returns the name of the destination
     */
    fn get_system_name(&self) -> String {
        if self.name.is_null() {
            return "".to_string();
        }

        let c_str = unsafe { CStr::from_ptr(self.name.clone()) };
        return c_str.to_str().unwrap().to_string();
    }

    /**
     * Returns default destination definition
     */
    fn get_is_default(&self) -> bool {
        return self.is_default == 1;
    }

    /**
     * Returns readable name of dest by "printer-info" option
     */
    fn get_name(&self) -> String {
        return self.get_option_by_key("printer-info");
    }

    /**
     * Returns readable name of the dest driver by "printer-make-and-model" option
     */
    fn get_marker_and_model(&self) -> String {
        return self.get_option_by_key("printer-make-and-model");
    }

    /**
     * Return if the destination is being shared with other computers
     */
    fn get_is_shared(&self) -> bool {
        return self.get_option_by_key("printer-is-shared") == "true";
    }

    /**
     * Return the drive version
     */
    fn get_uri(&self) -> String {
        return self.get_option_by_key("device-uri");
    }

    /**
     * Return the location option
     */
    fn get_location(&self) -> String {
        return self.get_option_by_key("printer-location");
    }

    /**
     * Return the state of the CUPS printer
     */
    fn get_state(&self) -> String {
        return self.get_option_by_key("printer-state");
    }
}

#[link(name = "cups")]
extern "C" {
    fn cupsGetDests(dests: *mut *mut CupsDestT) -> c_int;
    fn cupsPrintFile(
        printer_name: *const c_char,
        filename: *const c_char,
        title: *const c_char,
        options: i32,
    ) -> i32;
    fn cupsFreeDests(num_dests: c_int, dests: *const CupsDestT);
    fn cupsGetJobs(
        jobs: *mut *mut CupsJobS,
        name: *const c_char,
        myjobs: c_int,
        whichjobs: c_int,
    ) -> c_int;
    fn cupsFreeJobs(num_jobs: c_int, jobs: *const CupsJobS);
    fn cupsCancelJob(printer_name: *const c_char, job_id: c_int) -> c_int;
    fn cupsLastErrorString() -> *const c_char;
}
// http: *mut libc::c_void,

/**
 * Returns a vector of CupsDestT (cups_dest_s) struct with all available destinations
 * Using cupsGetDests
 */
pub fn get_dests() -> Vec<&'static CupsDestT> {
    let mut dests_ptr: *mut CupsDestT = ptr::null_mut();
    let dests_count = unsafe { cupsGetDests(&mut dests_ptr) };

    let mut dests: Vec<&CupsDestT> = Vec::new();
    for i in 0..dests_count {
        let dest_ptr = unsafe { dests_ptr.offset(i as isize) };
        let dest = unsafe { &*dest_ptr };

        // Not include printer with null names or duplex shared
        if !dest.name.is_null() && dest.get_option_by_key("printer-is-shared") != "" {
            dests.push(dest);
        }
    }

    return dests;
}

/**
 * Send an file to printer
 */
pub fn print_file(printer_name: &str, file_path: &str, job_name: Option<&str>) -> bool {
    unsafe {
        let printer_name = CString::new(printer_name).unwrap();
        let filename = CString::new(file_path).unwrap();
        let title = CString::new(job_name.unwrap_or(file_path)).unwrap();

        let result = cupsPrintFile(printer_name.as_ptr(), filename.as_ptr(), title.as_ptr(), 0);
        //println!("jobId: {}", result);
        return result != 0;
    }
}

/**
 * Free the allocated memory for dests
 */
pub fn free_dests(dests: &Vec<&CupsDestT>) {
    let ptr = dests.as_ptr();
    unsafe { cupsFreeDests(1 as i32, *ptr) };
}

/**
 * Ottiene la coda di stampa e restituisce una lista di lavori di stampa
 */
pub fn get_print_queue(
    printer_system_name: &str,
    myjobs: i32,
    whichjobs: i32,
) -> Vec<&'static CupsJobS> {
    let mut jobs_ptr: *mut CupsJobS = ptr::null_mut();
    let printer_name = CString::new(printer_system_name).unwrap();
    //println!("printer_name: {}", printer_name.to_str().unwrap());
    let number_of_jobs =
        unsafe { cupsGetJobs(&mut jobs_ptr, printer_name.as_ptr(), myjobs, whichjobs) };

    let mut jobs: Vec<&CupsJobS> = Vec::new();
    if number_of_jobs > 0 {
        for i in 0..number_of_jobs {
            let job_ptr = unsafe { jobs_ptr.offset(i as isize) };
            let job = unsafe { &*job_ptr };
            jobs.push(job);
        }
    }

    return jobs;
}

/**
 * Free the allocated memory for jobs
 */
pub fn free_jobs(jobs: &Vec<&CupsJobS>) {
    //println!("jobs len: {:?}", jobs.len());
    let ptr = jobs.as_ptr();
    unsafe { cupsFreeJobs(jobs.len() as i32, *ptr) };
}

pub fn cancel_job(printer_system_name: &str, job_id: i32) -> bool {
    let printer_name = CString::new(printer_system_name).unwrap();
    let result = unsafe { cupsCancelJob(printer_name.as_ptr(), job_id) };
    if result == 0 {
        println!("job_id: {}", job_id);
        println!("error: {:?}", get_last_error());
    }
    println!("result cancel_job: {}", result);
    return result != 0;
}

pub fn get_last_error() -> String {
    let error = unsafe { cupsLastErrorString() };
    let c_str = unsafe { CStr::from_ptr(error) };
    return c_str.to_str().unwrap().to_string();
}
