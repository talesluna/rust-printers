mod converters {
    use printers::common::converters::{Converter, GhostscriptConverterOptions};

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
        let result =
            Converter::Ghostscript(GhostscriptConverterOptions::png16m()).convert(pdf_buffer());

        #[cfg(target_family = "unix")]
        assert!(result.is_ok());

        #[cfg(target_family = "windows")]
        assert!(result.is_err());
    }

    #[test]
    fn test_none() {
        let result = Converter::None.convert(pdf_buffer());
        assert!(result.is_ok());

        if let Ok(converted) = result {
            assert_eq!(converted, pdf_buffer());
        }
    }
}
