#![allow(non_snake_case)]
use crate::shared::interface::PlatformPrinterGetters;

use libc::{c_int, c_uint, c_ulong, c_void, wchar_t};
use std::path::Path;
use std::{ ptr, slice };

use super::util::jobinfo::JobInfo2w;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct DocInfo1w {
    pub pDocName: *mut wchar_t,
    pub pOutputFile: *mut wchar_t,
    pub pDatatype: *mut wchar_t,
}

#[link(name = "winspool")]
extern "system" {
    fn EnumPrintersW(
        Flags: c_ulong,
        Name: *const wchar_t,
        Level: c_uint,
        pPrinterEnum: *mut c_void,
        cbBuf: c_ulong,
        pcbNeeded: *mut c_ulong,
        pcReturned: *mut c_ulong,
    ) -> c_int;

    fn GetDefaultPrinterW(pszBuffer: *mut wchar_t, pcchBuffer: *mut c_ulong) -> c_int;

    fn WritePrinter(hPrinter: *mut c_void, pBuf: *mut c_void, cbBuf: c_uint, pcWritten: *mut c_uint) -> c_int;

    fn AddJobW(
        hPrinter: *mut c_void,
        Level: c_int,
        pData: *mut c_void,
        cbBuf: c_uint,
        pcbNeeded: *mut c_uint,
    ) -> c_int;

    fn GetJobW(
        hPrinter: *mut c_void,
        JobId: c_uint,
        Level: c_int,
        pJob: *mut c_void,
        cbBuf: c_ulong,
        pcbNeeded: *mut c_ulong,
    ) -> c_int;

    /*fn SetJobW(
        hPrinter: *mut c_void,
        JobId: c_uint,
        Level: c_int,
        pJob: *mut c_void,
        Command: c_uint,
    ) -> c_int;*/

    fn EnumJobsW(
        hPrinter: *mut c_void,
        FirstJob: c_ulong,
        NoJobs: c_ulong,
        Level: c_int,
        pJob: *mut c_void,
        cbBuf: c_ulong,
        pcbNeeded: *mut c_ulong,
        pcReturned: *mut c_ulong,
    ) -> c_int;

    //fn RemoveJobW(hPrinter: *mut c_void, JobId: c_uint) -> c_int;

    fn GetPrinterW(hPrinter: *mut c_void, Level: c_uint, pPrinter: *mut c_void, cbBuf: c_uint, pcbNeeded: *mut c_uint) -> c_int;
    //fn ScheduleJob(hPrinter: *mut c_void, JobId: c_uint) -> c_int;
    fn OpenPrinterW(pPrinterName: *const wchar_t, phPrinter: *mut *mut c_void, pDefault: *const c_void) -> i32;
    fn StartDocPrinterW(hPrinter: *mut c_void, Level: c_uint, pDocInfo: *mut c_void) -> c_int;
    fn StartPagePrinter(hPrinter: *mut c_void) -> c_int;
    fn EndPagePrinter(hPrinter: *mut c_void) -> c_int;
    fn EndDocPrinter(hPrinter: *mut c_void) -> c_int;
    fn ClosePrinter(hPrinter: *mut c_void) -> c_int;
    fn GetLastError() -> c_int;
    //fn GetModuleHandleW(lpModuleName: *const wchar_t) -> *mut c_void;
    //fn GetProcAddress(hModule: *mut c_void, lpProcName: *const wchar_t) -> *mut c_void;
    //fn DrvDocumentEvent(hPrinter: *mut c_void,JobId: c_uint,Event: c_uint,lParam: c_ulong) -> c_int;
}

extern "system" {
    fn GetModuleHandleW(lpModuleName: *const wchar_t) -> *mut c_void;
    fn GetProcAddress(hModule: *mut c_void, lpProcName: *const u16) -> *mut c_void;
    fn drv_document_event(hPrinter: *mut c_void, Event: u32, dwData: *mut c_void) -> i32;
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct PrinterInfo2w {
    pub pServerName: *mut wchar_t,
    pub pPrinterName: *mut wchar_t,
    pub pShareName: *mut wchar_t,
    pub pPortName: *mut wchar_t,
    pub pDriverName: *mut wchar_t,
    pub pComment: *mut wchar_t,
    pub pLocation: *mut wchar_t,
    pub pDevMode: *mut c_void,
    pub pSepFile: *mut wchar_t,
    pub pPrintProcessor: *mut wchar_t,
    pub pDatatype: *mut wchar_t,
    pub pParameters: *mut wchar_t,
    pub pSecurityDescriptor: *mut c_void,
    pub Attributes: c_ulong,
    pub Priority: c_ulong,
    pub DefaultPriority: c_ulong,
    pub StartTime: c_ulong,
    pub UntilTime: c_ulong,
    pub Status: c_ulong,
    pub cJobs: c_ulong,
    pub AveragePPM: c_ulong,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct PrinterDefaults {
    pDatatype: *mut wchar_t,
    pDevMode: *mut c_void,
    DesiredAccess: c_ulong,
}

impl PrinterInfo2w {
    /**
     * Returns a string of wchar_t pointer
     */
    fn get_wchar_t_value(&self, s: *const wchar_t) -> String {
        if s.is_null() {
            return "".to_string();
        }

        let mut vec: Vec<u16> = Vec::new();
        let mut i = 0;
        unsafe {
            while *s.offset(i) != 0 {
                vec.push(*s.offset(i) as u16);
                i += 1;
            }
        }
        return String::from_utf16_lossy(&vec);
    }
}

impl PlatformPrinterGetters for PrinterInfo2w {
    /**
     * Returns the readable name of print
     */
    fn get_name(&self) -> String {
        return self.get_wchar_t_value(self.pPrinterName);
    }

    /**
     * Returns default printer definition
     */
    fn get_is_default(&self) -> bool {
        return unsafe { *self.pPrinterName == *self::get_default_printer() };
    }

    /**
     * Returns the name of printer on system (also name)
     */
    fn get_system_name(&self) -> String {
        return self.get_wchar_t_value(self.pPrinterName);
    }

    /**
     * Returns readable name of the printer driver
     */
    fn get_marker_and_model(&self) -> String {
        return self.get_wchar_t_value(self.pDriverName);
    }

    /**
     * Return if the printer is being shared with other computers
     */
    fn get_is_shared(&self) -> bool {
        return (self.Attributes & 0x00000008) == 8;
    }

    /**
     * Return the printer uri
     */
    fn get_uri(&self) -> String {
        return "".to_string();
    }

    /**
     * Return the location of the printer
     */
    fn get_location(&self) -> String {
        return self.get_wchar_t_value(self.pLocation);
    }

    /**
     * Return the state of the Winspool printer
     */
    fn get_state(&self) -> String {
        return self.Status.to_string();
    }
}

/**
 * Returns the default system printer
 */
fn get_default_printer() -> *const wchar_t {
    let mut name_size: c_ulong = 0;
    unsafe {
        GetDefaultPrinterW(ptr::null_mut(), &mut name_size);
        let mut buffer: Vec<wchar_t> = vec![0; name_size as usize];
        GetDefaultPrinterW(buffer.as_mut_ptr(), &mut name_size);
        return buffer.as_ptr();
    }
}

/**
 * Returns the system printers list
 */
pub fn enum_printers() -> Vec<PrinterInfo2w> {
    let mut tries = 0;
    let mut bytes_needed: c_ulong = 0;
    let mut count_printers: c_ulong = 0;

    let mut buffer: Vec<PrinterInfo2w> = Vec::with_capacity(bytes_needed as usize);

    loop {
        if tries > 2 {
            break;
        }

        tries += 1;
        let buffer_ptr = buffer.as_mut_ptr();

        let result = unsafe {
            EnumPrintersW(
                0x00000002 | 0x00000004,
                ptr::null_mut(),
                2,
                buffer_ptr as *mut c_void,
                bytes_needed,
                &mut bytes_needed,
                &mut count_printers,
            )
        };

        if result != 0 {
            let sliced = unsafe { slice::from_raw_parts(buffer_ptr, count_printers as usize) };
            for info in sliced {
                if !info.pPrinterName.is_null() {
                    buffer.push(info.clone());
                }
            }
            break;
        }

        buffer.reserve(bytes_needed as usize - buffer.capacity() + 1000);
    }

    return buffer;
}


/**
 *  Return the job list of the printer
 */
pub fn enum_jobs(printer_name: &str, first_job: &i32, no_jobs: &i32) -> Vec<JobInfo2w> {
    let mut tries = 0;
    let mut bytes_needed: c_ulong = 0;
    let mut count_jobs: c_ulong = 0;
    let mut buffer: Vec<JobInfo2w> = Vec::with_capacity(bytes_needed as usize);
    let mut h_printer: *mut std::ffi::c_void = ptr::null_mut();

    let printer_name_wide: Vec<u16> = printer_name.encode_utf16().chain(Some(0)).collect();

    unsafe {

        if OpenPrinterW(
            printer_name_wide.as_ptr(),
            &mut h_printer,
            ptr::null_mut(),
        ) == 0
        {
            println!("Errore nell'apertura della stampante");
            //return false;
        } 
    }

    let first_job_value = if *first_job != 0 { *first_job as c_ulong } else { 0 };
    let no_jobs_value = if *no_jobs != 0 { *no_jobs as c_ulong } else { 0xFFFFFFFF };

    loop {
        if tries > 2 {
            break;
        }

        tries += 1;
        let buffer_ptr = buffer.as_mut_ptr();

        let result = unsafe {
            EnumJobsW(
                h_printer,
                0,
                0xFFFFFFFF,
                2,
                buffer_ptr as *mut c_void,
                bytes_needed,
                &mut bytes_needed,
                &mut count_jobs,
            )
        };

        if result != 0 {
            let sliced = unsafe { slice::from_raw_parts(buffer_ptr, count_jobs as usize) };
            for info in sliced {
                if !info.pPrinterName.is_null() {
                    buffer.push(info.clone());
                }
            }
            break;
        }

        buffer.reserve(bytes_needed as usize - buffer.capacity() + 1000);
    }
    return buffer;
    
}

pub fn get_job(printer_name: &str, _job_id:&i32 ) ->  Vec<JobInfo2w> {
    let mut tries = 0;
    let mut bytes_needed: c_ulong = 0;
    let mut buffer: Vec<JobInfo2w> = Vec::with_capacity(bytes_needed as usize);

    let mut h_printer: *mut std::ffi::c_void = ptr::null_mut();

    let printer_name_wide: Vec<u16> = printer_name.encode_utf16().chain(Some(0)).collect();

    unsafe {
        if OpenPrinterW(
            printer_name_wide.as_ptr(),
            &mut h_printer,
            ptr::null_mut(),
        ) == 0
        {
            println!("Errore nell'apertura della stampante");
        } 
    }

    loop {
        if tries > 2 {
            break;
        }

        tries += 1;

        let buffer_ptr = buffer.as_mut_ptr();

        let result = unsafe { 
            GetJobW(
                h_printer,
                27,
                2,
                buffer_ptr as *mut c_void,
                bytes_needed,   
                &mut bytes_needed
            )
        };

        if result != 0 {

            let job_info_ptr = buffer.as_ptr() as *const JobInfo2w;
            let job_info = unsafe { *job_info_ptr };
        
            buffer.push(job_info.clone());
            break;
        }

        buffer.reserve(bytes_needed as usize - buffer.capacity() + 1000);
    }

    // Chiusura della stampante
    unsafe { ClosePrinter(h_printer); }

    buffer
}

/**
 *  Print on windows systems using winspool
 */
pub fn print_file(printer_name: &str, file_path: &str, job_name: Option<&str>) -> bool {
    unsafe {
        let printer_name_wide: Vec<u16> = printer_name.encode_utf16().chain(Some(0)).collect();

        // Apertura della stampante
        let mut h_printer: *mut std::ffi::c_void = ptr::null_mut();
       
        if OpenPrinterW(
            printer_name_wide.as_ptr(),
            &mut h_printer,
            ptr::null_mut(),
        ) == 0
        {
            println!("Errore nell'apertura della stampante: {}", get_last_error_message());
            return false;
        } 

        // Caricamento del modulo del driver della stampante
        let driver_module = GetModuleHandleW(ptr::null_mut());
        if driver_module.is_null() {
            println!("Impossibile caricare il modulo del driver della stampante: {}", get_last_error_message());
            ClosePrinter(h_printer);
            return false;
        }
        println!("driver_module: {:?}", driver_module);


        // Ottenimento del puntatore alla funzione di stampa del driver
        let print_proc_name = "DrvDocumentEvent";
        let print_proc_ptr = GetProcAddress(driver_module, print_proc_name.as_ptr() as *mut wchar_t);
        if print_proc_ptr.is_null() {
            println!("Impossibile ottenere il puntatore alla funzione di stampa del driver: {}", get_last_error_message());
            ClosePrinter(h_printer);
            return false;
        }
        let print_proc: extern "system" fn(*mut c_void, u32, *mut c_void) -> i32 = mem::transmute(print_proc_ptr);
        

        let path = Path::new(file_path);
        let doc_name_wide = path.file_name().map(|os_str| os_str.to_string_lossy().encode_utf16().chain(Some(0)).collect()).unwrap_or_else(|| vec![0]);
        let data_type_wide: Vec<u16> = "RAW".to_string().encode_utf16().chain(Some(0)).collect();

        let doc_info = DocInfo1w {
            pDocName: doc_name_wide.as_ptr() as *mut wchar_t,
            pOutputFile: ptr::null_mut() as *mut wchar_t,
            pDatatype: data_type_wide.as_ptr() as *mut wchar_t,//ptr::null_mut() as *mut wchar_t,
        };
        let doc_info_ptr = Box::into_raw(Box::new(doc_info));

        // Avvio del documento di stampaptr::null_mut()
        let job_id = StartDocPrinterW(h_printer, 1, doc_info_ptr as *mut _ );  //as *const _ as _

        if job_id == 0 {
            println!("Errore nell'avvio del documento di stampa: {}", get_last_error_message());
            ClosePrinter(h_printer);
            return false;
        }
        println!("job_id: {:?}", job_id);
     
        // Avvio di una pagina
        if StartPagePrinter(h_printer) == 0 {
            println!("Errore nell'avvio della pagina: {}", get_last_error_message());
            EndDocPrinter(h_printer);
            ClosePrinter(h_printer);
            return false;
        }

        // Lettura del contenuto del file da stampare
        let file_content = std::fs::read(file_path);
        if let Err(err) = file_content {
            println!("Errore nella lettura del file: {}", err);
            EndPagePrinter(h_printer);
            EndDocPrinter(h_printer);
            ClosePrinter(h_printer);
            return false;
        }
        let content = file_content.unwrap();

        // Chiamata alla funzione di stampa del driver
        let result = print_proc(h_printer, 0x1004, content.as_ptr() as *mut _);
        if result == 0 {
            println!("Errore durante la stampa del documento: {}", get_last_error_message());
            EndPagePrinter(h_printer);
            EndDocPrinter(h_printer);
            ClosePrinter(h_printer);
            return false;
        }
        /*// Chiamata alla funzione di stampa del driver
        let result = drv_document_event(h_printer, job_id, content.as_ptr() as *mut c_void);
        if result == 0 {
            println!("Errore durante la stampa del documento");
            EndPagePrinter(h_printer);
            EndDocPrinter(h_printer);
            ClosePrinter(h_printer);
            return false;
        }*/
        // Scrittura del contenuto sulla stampante
       /*let mut bytes_written: u32 = 0;
        if WritePrinter(
            h_printer,
            content.as_ptr() as *mut std::ffi::c_void,
            content.len() as u32,
            &mut bytes_written,
        ) == 0
        {
            println!("Errore nella scrittura sulla stampante: {}", get_last_error_message());
            EndPagePrinter(h_printer);
            EndDocPrinter(h_printer);
            ClosePrinter(h_printer);
            return false;
        } */ 
        /*if let Ok(content) = file_content {
            // Scrittura del contenuto sulla stampante
            let mut bytes_written: u32 = 0;
            if WritePrinter(
                h_printer,
                content.as_ptr() as *mut c_void, //as *mut std::ffi::c_void,
                content.len() as u32,
                &mut bytes_written,
            ) == 0
            {
                println!("Errore nella scrittura sulla stampante");
                EndPagePrinter(h_printer);
                EndDocPrinter(h_printer);
                ClosePrinter(h_printer);
                return false;
            }
        } else {
            println!("Errore nella lettura del file");
            EndPagePrinter(h_printer);
            EndDocPrinter(h_printer);
            ClosePrinter(h_printer);
            return false;
        }*/

        // Fine della pagina e del documento di stampa
        if EndPagePrinter(h_printer) == 0 {
            println!("Errore nella fine della pagina: {}", get_last_error_message());
            EndDocPrinter(h_printer);
            ClosePrinter(h_printer);
            return false;
        }

        if EndDocPrinter(h_printer) == 0 {
            println!("Errore nella fine del documento di stampa: {}", get_last_error_message());
            ClosePrinter(h_printer);
            return false;
        }

        // Chiusura della stampante
        if ClosePrinter(h_printer) == 0 {
            println!("Errore nella chiusura della stampante: {}", get_last_error_message());
            return false;
        }
     
    }

    true
}

// Funzione per ottenere il messaggio di errore
fn get_last_error_message() -> String {
    unsafe {
        let err_code = GetLastError();
    
        //println!("errr: {:?}", errr);
        let error_message =format!("Errore sconosciuto: {}", err_code);

        error_message
    }
}