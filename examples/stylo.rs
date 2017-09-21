extern crate unsafe_unicorn;

use unsafe_unicorn::{Cloc, ClocVerbosity};

// what files account for the unsafety?
// pull out the top 5 files for each directory analyzed in main
fn analyze_files(dir: &str) {
    let mut cloc = Cloc::new();
    cloc.set_verbose(ClocVerbosity::File);
    cloc.analyze_dir(dir).unwrap();

    let top_cloc = cloc.top_unsafe(5);

    if top_cloc.len() > 0 {
        println!("{}", top_cloc)
    } else {
        println!("Nothing unsafe here!");
    }

}

fn main() {

    let mut cloc = Cloc::new();
    cloc.analyze_dir("/Users/ddh/mozilla/stylo").unwrap();
    println!("{}", cloc);

    for s in cloc.stats() {
        println!("Top unsafe files for {:?}", s.name());
        analyze_files(s.name().to_str().unwrap())
    }

}
