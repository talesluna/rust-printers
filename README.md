# Printers

Printers is a simple lib to call printers apis for unix *(cups)* and windows *(winspool)* systems.

Printer can provide a list of printers available on system and send print jobs to theirs

## Behavior

> Return a vector of all available printers

```rust
let printers = get_printers(); // -> Vec<Printer>
```

> Create print job of an byte array

```rust
let data = "42".as_bytes();
printer.print(data); // -> Result<(), &'static str>
```

> Create print job of an file

```rust
let path = "my_file/example/path.txt";
printer.print_file(path); // -> Result<(), &'static str>
```

> Find printer by the name

```rust
let my_printer = get_printer_by_name("my_printer"); // -> Option<Printer>
```

> Get the default printer

```rust
let printer = get_default_printer(); // -> Option<Printer>
```

## Example

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
        my_printer.unwrap().print_file("notes.txt", None);
        // Err("cupsPrintFile failed")
    }

    // Use the default printer
    let default_printer = get_default_printer();
    if default_printer.is_some() {
        default_printer.unwrap().print("dlrow olleh".as_bytes(), Some("My Job"));
        // Ok(())
    }

}

```

## System Requiriments

### Windows
For Windows printers will be use winspool apis to retrive printers and create jobs with RAW datatypes

**Note**: For some reasons, printing for complex files like PDF, DOCx and others can`t works as well in many printers. If you want collaborate to implement winspool for printing documents, your contribution will be greatly appreciated

### Unix
For Unix is necessary cups service installed
