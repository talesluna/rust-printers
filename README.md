# [Printers](https://crates.io/crates/printers): A printing APIs implementation for unix *(cups)* and windows *(winspool)*.

Provides all system printers, create and manage print jobs.

![Build and tests](https://img.shields.io/github/actions/workflow/status/talesluna/rust-printers/ci.yml?label=build%20%26%20tests)
![Crates.io Version](https://img.shields.io/crates/v/printers)
![Crates.io License](https://img.shields.io/crates/l/printers)
![docs.rs](https://img.shields.io/docsrs/printers)
![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/printers)

## Documentation
See the references in [docs.rs](https://docs.rs/printers).

## 🛠️ Features

| Feature | Status |
| :--- | :---: |
| List available printers | ✅ |
| List printer jobs | ✅ |
| Manage printer jobs (pause, resume, cancel, restart) | ✅ |
| Print plain text | ✅ |
| Print PDF, images etc... (*1)| ✅ |
| Converters (Ghostscript) | ✅ |
| DOCx / XLS / PPTx converter | ⏳ |
| Converter pipeline (doc -> pdf -> ps) | ⏳ |

> *1 If necessary, you can raster the file using converters supported by the lib, such as Ghostscript. See the examples below.

## 👇 Examples

**Get all available printers**

```rust
let printers = get_printers();
// Vec<Printer>
``` 

**Create print job of an byte array**

```rust
let job_id = printer.print("42".as_bytes(), PrinterJobOptions::none());
// Result<u64, PrintersError>
```

**Create print job of an file**

```rust
let job_id = printer.print_file("my_file/example/path.pdf", PrinterJobOptions {
    name: Some("My print job"),
    raw_properties: &[
        ("copies", "2"),
        ("document-format", "RAW"),
    ],
    converter: Converter::Ghostscript(GhostscriptConverterOptions::ps2write()),
});
// Result<u64, PrintersError>
```

**Get a printer by name**

```rust
let my_printer = get_printer_by_name("my_printer");
// Option<Printer>
```

**Get the default printer**

```rust
let printer = get_default_printer();
// Option<Printer>
```

**Manage state of printer job**

```rust
// Pause
printer.pause_job(123);

// Resume
printer.resume_job(123);

// Restart
printer.restart_job(123);

// Cancel
printer.cancel_job(123)
```
