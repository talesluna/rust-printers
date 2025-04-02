#[derive(Debug, Default, Clone)]
pub struct GetPrintersOptions {
    // Set to true to exclude printers that is shared and can print both sides of a sheet of paper
    pub exclude_shared_duplex_printer: bool,
}
