extern crate unsafe_unicorn;

use unsafe_unicorn::{ClocStats, Cloc, ClocVerbosity};


fn main() {

    // Get the stats for a single file
    let c = ClocStats::from_file("./resources/test.rs").unwrap();
    println!("{}", c);

    // Get the stats for the resources directory
    let mut cloc = Cloc::new();
    cloc.analyze_dir("./resources").unwrap();
    println!("{}", cloc);

    // Add the stats for the source directory
    // FIXME it's counting the unsafe regexes I think
    cloc.analyze_dir("./src").unwrap();
    println!("{}", cloc);

    // Change the verbosity to be file based and then get stats file by file for resources dir
    cloc.set_verbose(ClocVerbosity::File);
    cloc.analyze_dir("./resources").unwrap();
    println!("{}", cloc)

}
