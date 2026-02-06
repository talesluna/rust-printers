pub mod job {
    use printers::{common::base::job::PrinterJobOptions, get_default_printer, get_printers};
    use std::time::SystemTime;

    #[test]
    fn test_get_job_history() {
        for printer in get_printers() {
            for job in printer.get_job_history() {
                assert!(job.id > 0);
                assert!(job.created_at > SystemTime::UNIX_EPOCH);
            }
        }
    }

    #[test]
    fn test_get_active_jobs() {
        for printer in get_printers() {
            for job in printer.get_active_jobs() {
                assert!(job.id > 0);
                assert!(job.created_at > SystemTime::UNIX_EPOCH);
                assert_eq!(job.printer_name, printer.system_name);
            }
        }
    }

    #[test]
    fn test_manage_job() {
        let printer = if let Some(printer) = get_default_printer() {
            printer
        } else {
            panic!("Default printer must be available")
        };

        let job_id = if let Ok(job_id) = printer.print(b"test", PrinterJobOptions::none()) {
            job_id
        } else {
            panic!("Cannot create test job")
        };

        let result = printer
            .pause_job(job_id)
            .map(|_| printer.resume_job(job_id))
            .map(|_| printer.restart_job(job_id))
            .map(|_| printer.cancel_job(job_id));

        assert!(result.is_ok());
    }
}
