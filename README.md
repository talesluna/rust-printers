# Printers

Printers **is not a lib for printer drivers or cups**. Printers is a simple lib for running "native" printing commands in unix *(lp/lpstat)* and windows *(lpr/wmic)* systems.

Printer can provide a list of printers available on the system and perform document printing.

## Behavior

```rs
printers::get_printers() -> Vec<Printer>
```
> Return a vector of available printers

```rs
printers::print(Printer, &[u8]) -> Job
printer.print(&[u8]) -> Job
```
> Request print of a temp file after write they

```rs
printers::print_file(Printer, &str) -> Job
printer.print_file(&str) -> Job
```
> Request print of specific file from path

## Example

```rs
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

}
```

## System Requiriments

### Windows / LPD
For Windows is necessary turn on LPR and LPD running on localhost to perform lpr command to print

### Unix / Cups
For Unix is necessary cups service running to perform lp command to print
