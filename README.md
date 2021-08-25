# Printers

## What is and what is not
Printers is not a lib for print drivers or cups. Printers is a simple lib for running "native" printing commands from unix (lp/lpstat) and windows (lpr/wmic) systems.

Printer can provide a list of printers available on the system and perform document printing.

## Behavior
- Printer::get_printers() -> Vec<Printer>;
    -> Run lpstat or wmic -> Return a vector of available printers 

- Job::print(Printer, buffer) -> Job;
    -> Save buffer as temporary file -> Request print of the temp file with lp or lpr

## Example

```rs
use printers::Job;
use printers::Printer;

fn main() {

    // Vector of system printers
    let printers = Printer::get_printers();
    println!("{:?}", printers);

    // Print in all printers
    for printer in printers {
        let job = Job::print(&printer, "42".as_bytes());
        println!("{:?}", job);
    }

}
```