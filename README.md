A crate for analyzing the usage of unsafe code in Rust, based on [cloc-rust](https://github.com/avadacatavra/cloc-rust).

Currently, this is based on a textual analysis of code. In the future, this could be expanded to use the AST for further analysis.

For more information on unsafe code:
- [Meet Safe and Unsafe](https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html)
- [Rust book](https://doc.rust-lang.org/book/second-edition/ch19-01-unsafe-rust.html)
- [Rust by Example](https://rustbyexample.com/unsafe.html)



## Example

```rust
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

    // Change the verbosity to be file based and then get stats file by file for resources dir
    cloc.set_verbose(ClocVerbosity::File);
    cloc.analyze_dir("./resources").unwrap();
    println!("{}", cloc)

}
```

More examples are available in `examples/`


## TODO
- [ ] make PR for [tokei](https://github.com/Aaronepower/tokei/tree/master/src)
- [ ] add dependency analysis
- [ ] expand tests
- [ ] add docs
- [ ] clean up code
- [ ] look for c-like array iteration?
