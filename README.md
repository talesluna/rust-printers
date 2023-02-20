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

        let job1 = printer.print("42".as_bytes());
        let job2 = printer.print_file("/path/to/any.file");

        println!("{:?}", printer);
        println!("{:?}", job1);
        println!("{:?}", job2);
    }

    // Print with aux lib function (legacy)
    printers::print(&printers[0], "42".as_bytes());
    printers::print_file(&printers[1], "/path/to/any.file");

    // Try printer by name
    let test_printer = printers::get_printer_by_name("test");
    println!("{:?}", test_printer);

}
```

## System Requiriments

### Windows
For Windows printers will be use winspool apis

### Unix
For Unix is necessary cups service installed
