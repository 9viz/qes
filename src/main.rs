use qes;
use std::io::{stdout, Write};

fn main() {
    // get roots
    let roots = qes::calculate_root();

    // closure to print string
    let print = |string: &str| {
        print!("{}", string);
        // flush
        stdout().flush().unwrap();
    };

    print(&"the roots are ");

    print(&roots.0.to_string()[..]);
    if ! roots.2 { print(&"i"); }

    print(&" ");

    print(&roots.1.to_string()[..]);
    if ! roots.2 { print(&"i"); }

    println!();
}
