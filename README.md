# [Printers](https://crates.io/crates/printers): A printing APIs implementation for unix *(cups)* and windows *(winspool)*.

Provides all system printers, create and get print jobs.

![Crates.io Version](https://img.shields.io/crates/v/printers)
![Crates.io License](https://img.shields.io/crates/l/printers)
![docs.rs](https://img.shields.io/docsrs/printers)
![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/printers)

## Documentation
See the references in [docs.rs](https://docs.rs/printers).

## Features

|  Target |    API   | List printers | List jobs | Print bytes and text files | Print PDF,images, etc... |
|:-------:|:--------:|:-------------:|:---------:|:-----------------------:|:------------------------:|
| Unix    | cups     |       ✅       |     ✅     |            ✅            |             ✅          |
| Windows | winspool |       ✅       |     ✅     |            ✅            |             🤔**        |

> ** On Windows this lib use RAW datatype to process printing. Expected output depends of printer firmware.

## Examples

**Get all available printers**

```rust
let printers = get_printers();
// Vec<Printer>
``` 

**Create print job of an byte array**

```rust
let job_id = printer.print("42".as_bytes(), PrinterJobOptions::none());
// Result<u64, &'static str>
```

**Create print job of an file**

```rust
let job_id = printer.print_file("my_file/example/path.txt", PrinterJobOptions {
    name: Some("My print job"),
    raw_properties: &[
        ("copies", "2"),
        ("others", "prop"),
    ],
});
// Result<u64, &'static str>
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

**Simple compilation**

```rust
use printers::{get_printer_by_name, get_default_printer, get_printers};

fn main() {

    // Iterate all available printers
    for printer in get_printers() {
        println!("{:?}", printer);
    }

    // Get a printer by the name
    let my_printer = get_printer_by_name("my_printer");
    if my_printer.is_some() {
        let job_id = my_printer.unwrap().print_file("notes.txt", PrinterJobOptions::none());
        // Err("...") or Ok(())
    }

    // Use the default printer
    let default_printer = get_default_printer();
    if default_printer.is_some() {
        let job_id = default_printer.unwrap().print("dlrow olleh".as_bytes(), PrinterJobOptions {
            name: None,
            // options are currently UNIX-only. see https://www.cups.org/doc/options.html
            raw_properties: &[
                ("document-format", "application/vnd.cups-raw"),
                ("copies", "2"),
            ],
        });
        // Err("...") or Ok(())
    }
}
```
