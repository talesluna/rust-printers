mod printer {
    use printers::*;

    #[test]
    fn test_get_printers() {
        for printer in get_printers() {
            assert!(!printer.name.is_empty());
            assert!(!printer.system_name.is_empty());
        }
    }

    #[test]
    fn test_get_default_printer() {
        let printer = get_default_printer();
        if let Some(printer) = printer {
            assert!(!printer.name.is_empty());
            assert!(!printer.system_name.is_empty());
            assert!(printer.is_default);
        }
    }

    #[test]
    fn test_get_printer_by_name() {
        if let Some(printer) = get_printer_by_name("SamplePrinter") {
            assert!(!printer.name.is_empty());
            assert!(!printer.system_name.is_empty());
        }
    }

    #[test]
    fn test_print() {
        for printer in get_printers() {
            let result = printer.print(
                "test".as_bytes(),
                common::base::job::PrinterJobOptions {
                    converter: common::converters::Converter::None,
                    name: None,
                    raw_properties: &[("copies", "1")],
                },
            );

            if let Ok(job_id) = result {
                assert!(job_id > 0);
            }
        }
    }

    #[test]
    fn test_print_file() {
        for printer in get_printers() {
            let result = printer.print_file(
                "/not/valid/path",
                common::base::job::PrinterJobOptions::none(),
            );

            assert!(result.is_err());
        }
    }
}
