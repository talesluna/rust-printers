# Printers

Printers **is not a lib for printer drivers or cups**. Printers is a simple lib for running "native" printing commands in unix *(lp/lpstat)* and windows *(powershell utilities)* systems.

Printer can provide a list of printers available on the system and perform document printing.

## Behavior

```rust
printers::get_printers() -> Vec<Printer>
```
> Return a vector of available printers

```rust
printers::print(Printer, &[u8]) -> Job
printer.print(&[u8]) -> Job
```
> Request print of a temp file after write they

```rust
printers::print_file(Printer, &str) -> Job
printer.print_file(&str) -> Job
```
> Request print of specific file from path

```rust
printers::get_printer_by_id(&str) -> Option<Printer>
```
> Try get and return a single printer by your ID

```rust
printers::get_printer_by_name(&str) -> Option<Printer>
```
> Try get and return a single printer by your name

> *NOTE*: get_printer_by_id and get_printer_by_name yet is a simple utility, this functions just apply filters over call get_printers() result. They are improved on future to be more performatic 


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

    // Try get printer by uuid
    let test_printer = printers::get_printer_by_name("4be0643f-1d98-573b-97cd-ca98a65347dd");
    println!("{:?}", test_printer);

    // Try printer by name
    let test_printer = printers::get_printer_by_name("test");
    println!("{:?}", test_printer);

}
```

## System Requiriments

### Windows / LPD
For Windows is necessary powershell installed

### Unix / Cups
For Unix is necessary cups service running to perform lp command to print
