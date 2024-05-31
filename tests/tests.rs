#[cfg(test)]
mod tests {

    use printers;
    use printers::printer::Printer;
    use printers::printer_job::PrintJob;

    use std::thread::sleep;
    use std::time::Duration;

    const PRINTER_NAME: &str = "NPI858230";
    const FILE_PATH: &str = "C:\\Users\\Kaos\\Desktop\\rust-printers\\ordine_42131.pdf";

    #[cfg(target_family = "windows")]
    mod windows_tests {

        use std::env;
        use std::fs;
        use std::io::Write;

        #[test]
        pub fn main() {
            setup_sumatra_pdf();
            assert!(
                check_sumatra_pdf_installed(),
                "Installazione di SumatraPDF non riuscita"
            );
        }

        fn setup_sumatra_pdf() {
            if !check_sumatra_pdf_installed() {
                let sm = include_bytes!("lib/SumatraPDF.exe");
                let dir = env::temp_dir();
                //println!("{:?}", dir);
                let result = create_file(&dir, sm);
                if let Err(err) = result {
                    panic!("Errore durante la creazione del file: {}", err);
                }
            }
        }

        /**
         * Create sm.exe to temp
         */
        fn create_file(dir: &std::path::Path, bin: &[u8]) -> std::io::Result<()> {
            let file_path = dir.join("SumatraPDF.exe");
            let mut file = fs::File::create(file_path)?;
            file.write_all(bin)?;
            file.sync_all()?;
            Ok(())
        }

        /**
         * Check if SumatraPDF.exe exists in the temp directory
         */
        fn check_sumatra_pdf_installed() -> bool {
            let file_path = env::temp_dir().join("SumatraPDF.exe");
            fs::metadata(&file_path).is_ok()
        }
    }

    #[test]
    pub fn test_get_printes() {
        let _printers: Vec<_> = printers::get_printers();
        // Verifica se la lista non è vuota
        assert!(
            !_printers.is_empty(),
            "La funzione get_printers ha restituito una lista vuota"
        );

        if !_printers.is_empty() {
            // Verifica se il primo elemento è un Printer
            let first_printer = &_printers[0];
            assert!(
                matches!(first_printer, Printer { .. }),
                "Il primo elemento della lista non è un Printer"
            );
        }
    }

    #[test]
    pub fn test_printer_pdf() {
        let status = printers::print_file(PRINTER_NAME, FILE_PATH, Some("test.pdf"));
        assert_eq!(status, Ok(true));
        if status == Ok(true) {
            sleep(Duration::from_secs(6));
            test_queue_status();
        }
    }

    #[test]
    pub fn test_queue_status() {
        let _printers: Vec<_> = printers::print_queue(PRINTER_NAME, 1, 1);
        //println!("{:?}", _printers);

        // Verifica se la lista non è vuota
        assert!(
            !_printers.is_empty(),
            "La funzione print_queue ha restituito una lista vuota"
        );

        if !_printers.is_empty() {
            // Verifica se il primo elemento è un Printer
            let first_printer = &_printers[0];
            assert!(
                matches!(first_printer, PrintJob { .. }),
                "Il primo elemento della lista non è un Printer"
            );
        }
    }
}
