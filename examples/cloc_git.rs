extern crate unsafe_unicorn;
extern crate git2;

use git2::Repository;
use std::fs;
use unsafe_unicorn::Cloc;

fn git_example(url: &str) {
    // clone into a temporary repository
    Repository::clone(url, "./cloc-git-tmp").unwrap();

    let mut c = Cloc::new();
    c.analyze_dir("./cloc-git-tmp").unwrap();
    println!("{}", c);
    fs::remove_dir_all("./cloc-git-tmp").unwrap();
}

fn main() {
    // let servo_url = "https://github.com/servo/servo.git";
    // let rust_url = "https://github.com/rust-lang/rust";
    let wr_url = "https://github.com/servo/webrender";

    git_example(wr_url);
}
