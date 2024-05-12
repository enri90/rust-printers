#![allow(non_snake_case)]
use crate::shared::interface::JobGetters;

use libc::{c_int, c_uint, c_ulong, c_void, wchar_t};


#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct JobInfo2w {
    pub JobId: c_uint,
    pub pPrinterName: *mut wchar_t,
    pub pMachineName: *mut wchar_t,
    pub pUserName: *mut wchar_t,
    pub pDocument: *mut wchar_t,
    pub pNotifyName: *mut wchar_t,
    pub pDatatype: *mut wchar_t,
    pub pPrintProcessor: *mut wchar_t,
    pub pParameters: *mut wchar_t,
    pub pDriverName: *mut wchar_t,
    pub pDevMode: *mut c_void,
    pub pStatus: *mut wchar_t,
    pub pSecurityDescriptor: *mut c_void,
    pub Status: c_uint,
    pub Priority: c_uint,
    pub Position: c_uint,
    pub StartTime: c_uint,
    pub UntilTime: c_uint,
    pub TotalPages: c_uint,
    pub Size: c_uint,
    pub Submitted: c_uint,
    pub Time: c_uint,
    pub PagesPrinted: c_uint,
}

impl JobInfo2w {
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



impl JobGetters for JobInfo2w {
    fn get_id(&self) -> String {
        return self.JobId.to_string();
    }
    fn get_dest(&self) -> String {
        return self.get_wchar_t_value(self.pPrinterName);
    }

    fn get_title(&self) -> String {
        return self.get_wchar_t_value(self.pDocument);
    }

    fn get_user(&self) -> String {
        return self.get_wchar_t_value(self.pUserName);
    }

    fn get_format(&self) -> String {
        return self.get_wchar_t_value(self.pDatatype);
    }

    fn get_state(&self) -> String {
        return self.Status.to_string();
    }

    fn get_size(&self) -> String {
        return self.Size.to_string();
    }

    fn get_priority(&self) -> String {
        return self.Priority.to_string();
    }

    fn get_creation_time(&self) -> String {
        return self.Submitted.to_string();
    }

    fn get_completed_time(&self) -> String {
        return self.Time.to_string();
    }

    fn get_processing_time(&self) -> String {
        return self.StartTime.to_string();
    }
}