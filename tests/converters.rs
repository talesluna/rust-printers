mod converters {
    use printers::common::{base::job::PrinterJobOptions, converters::{Converter, Converters}};

    fn pdf_buffer() -> &'static [u8] {
        b"%PDF-1.1
        1 0 obj
        <<>>
        endobj
        xref
        0 2
        0000000000 65535 f 
        0000000010 00000 n 
        trailer
        << /Size 2 /Root 1 0 R >>
        startxref
        38
        %%EOF"
    }

    #[test]
    fn test_ghostscript() {
        let result = Converters::ghostscript().convert(pdf_buffer(), &PrinterJobOptions::default());

        #[cfg(target_family = "unix")]
        assert!(result.is_ok());

        #[cfg(target_family = "windows")]
        assert!(result.is_err());
    }
}
