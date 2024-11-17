# Printers

Printers **is not a lib for printer drivers or cups**. Printers is a simple lib to call printers apis for unix *(cups)* and windows *(winspool)* systems.

Printer can provide a list of printers available on the system and perform document printing.

## Behavior

> Return a vector of available printers

```rust
printers::get_printers() -> Vec<Printer>
```

> Request print of a temp file after write they

```rust
printers::print(Printer, &[u8]) -> Job
printer.print(&[u8]) -> Job
```

> Request print of specific file from path

```rust
printers::print_file(Printer, &str) -> Job
printer.print_file(&str) -> Job
```

> Try get and return a single printer by your name

```rust
printers::get_printer_by_name(&str) -> Option<Printer>
```

> *NOTE*: get_printer_by_name is a simple utility, this functions just apply filters over call get_printers() result. They are improved on future to be more performatic 


## Example

```rust
use printers;

fn main() {


    // Vector of system printers
    let printers = printers::get_printers();

    // Print directly in all printers
    for printer in printers.clone() {

        println!("{:?}", printer);

        let status1 = printer.print("42".as_bytes(), Some("Everything"));
        println!("{:?}", status1);
        
        // Note: When you don't give the job_name
        // the file path will be that name by default
        let status2 = printer.print_file("/path/to/any.file", None);
        println!("{:?}", status2);

    }

    // Print directly by printer name
    printers::print("printer-a", "42".as_bytes(), Some("Everything"));
    printers::print_file("printer-b", "/path/to/any.file", Some("My Job"));

    // Try printer by name
    let test_printer = printers::get_printer_by_name("test");
    println!("{:?}", test_printer);

}
```

## System Requiriments

### Windows
For Windows printers will be use winspool apis to retrive printer and powershell to send a doc to printer

**Note**: For some complex reasons, the printing action stays doing using powershell. If you want collaborate to implement winspool for printing documents, your contribution will be greatly appreciated

### Unix
For Unix is necessary cups service installed
