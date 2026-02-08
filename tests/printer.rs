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
        let printer = if let Some(printer) = get_default_printer() {
            printer
        } else {
            panic!("Default printer must be available")
        };

        let result = printer.print(
            b"test_print",
            common::base::job::PrinterJobOptions::default(),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_print_file() {
        for printer in get_printers() {
            let result = printer.print_file(
                "/not/valid/path",
                common::base::job::PrinterJobOptions::default(),
            );

            assert!(result.is_err());
        }
    }
}
