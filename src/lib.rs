#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fmt;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::fs::{self,File};
use std::result::Result::Err;
use regex::RegexSet;

type ClocResult = Result<ClocStats, String>;

//TODO dependency analysis
//TODO maybe use pretty table?
//TODO expand unit tests

lazy_static!{
  static ref REGEXES: RegexSet = RegexSet::new(&[
    r"```",                     // block comment
    r"^//|^\s/\*|^\s\*|^\s\*/", //comment
    r"\s*fn\s+[a-zA-Z_]*",      //function
    r"\s*unsafe impl.*for.*",   //unsafe impl
    r"\s*unsafe\s*\{.*\}",      //unsafe one liner
    r".*unsafe\s*\{",           //more unsafe
    r"panic",                   //panic
  ]).unwrap();
}

/// exclude tests etc from analysis
lazy_static!{
    static ref EXCLUDE: Vec<&'static str> = vec!(
        ".git",
        "tests",
        "examples",
        "benches"
        );
}

/// Determine how to summarize and display statistics
///     File: show unsafe info file by file
///     Crate: (default) show info 'crate' by 'crate'
///     TopLevel: combine all subdirectory stats into one toplevel output
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ClocVerbosity {
    File,
    Crate,
    TopLevel,
}

// cloc should be the struct that you actually interact with
// so you set the verbosity and call it on a path, then it figures out how to split all of the data up
#[derive(Debug)]
pub struct Cloc {
    verbose: ClocVerbosity,
    stats: Vec<ClocStats>,
}

impl Cloc {
    pub fn new() -> Cloc {
        Cloc {
            verbose: ClocVerbosity::Crate,
            stats: vec!(),
        }
    }

    pub fn stats(&self) -> &Vec<ClocStats> {
        &self.stats
    }

    pub fn set_verbose(&mut self, level: ClocVerbosity) {
        self.verbose = level;
    }

    pub fn add_stats(&mut self, stats: ClocStats) {
        self.stats.push(stats);
    }

    pub fn clear_stats(&mut self) {
        self.stats.clear()
    }

    pub fn len(&self) -> usize {
        self.stats.len()
    }

    pub fn analyze_dir(&mut self, dir: &str) -> Result<(), io::Error> {

        let mut c = ClocStats::new(PathBuf::from(dir));
        let mut subdirs = vec!();
        subdirs.push((dir.to_owned(), fs::read_dir(&Path::new(dir))?));

        while !subdirs.is_empty(){
            let (dir_name, paths) = subdirs.pop().unwrap();

            // when you switch subdirectories, check to see if you need a new CLocStats
            if PathBuf::from(&dir_name).join("Cargo.toml").exists() && self.verbose == ClocVerbosity::Crate {
                    if !(c.is_empty()) {
                        self.add_stats(c.clone());
                    }
                    c = ClocStats::new(PathBuf::from(dir_name));
                }

            for p in paths {
                let p = p.unwrap();
                
                if p.file_type().unwrap().is_dir(){
                    if !(EXCLUDE.contains(&p.path().file_name().unwrap().to_str().unwrap())) {
                        let ppath = p.path();
                        let subdir_name = ppath.to_str().unwrap();
                        subdirs.push((subdir_name.to_owned(), fs::read_dir(subdir_name).unwrap()));
                    }
                } else {
                    if p.path().extension().unwrap_or_default() == "rs" {
                        match self.verbose {
                            ClocVerbosity::File => {
                                let path = p.path();
                                let c = ClocStats::from_file(path.to_str().unwrap()).unwrap();
                                self.add_stats(c);
                            },
                            _ => c.cloc_file(&mut File::open(p.path()).expect("Couldn't open file")),
                        
                        };

                    }
                }
            }

        }
        if !(c.is_empty()) {
                self.add_stats(c.clone());
        }
        Ok(())
    }

    pub fn sort_stats(&mut self) {
        self.stats.sort_by(|a, b| {
            b.unsafe_ratio().partial_cmp(&a.unsafe_ratio()).unwrap()
        });
    }

    // returns a Cloc object to make output better
    pub fn top_unsafe(&mut self, num: usize) -> Cloc {
        let mut c = Cloc::new();
        c.set_verbose(self.verbose);

        self.sort_stats();
        for s in self.stats.iter() {
            if c.len() == num {
                break;
            }
            if s.num_unsafe > 0 {
                c.add_stats(s.clone());
            }
        }
        c
    }
}

impl fmt::Display for Cloc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let header = ["\t", "#files", "blank", "comment", "code", "unsafe", "%unsafe", 
                      "#fns", "#unsafe fns", "%unsafe fns", "#panics"];
        for h in header.iter() {
            write!(f, "{}\t", h)?;
        }
        write!(f, "\n")?;
        for s in &self.stats {
            write!(f, "{}\t", s.name().file_name().unwrap().to_str().unwrap())?;
            for val in s.summarize(){
                match val {
                    SummaryType::Ratio(x) => write!(f, "{:.*}\t", 2, x)?,
                    SummaryType::Int(x) => write!(f, "{}\t", x)?,
                };
            }
            write!(f, "\n")?;

        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClocStats {
    name: PathBuf,
    pub num_unsafe: usize,
    unsafe_fns: usize,
    total_fns: usize,
    blank: usize,
    comment: usize,
    files: usize,
    code: usize,
    panics: usize,
}

// helper type to store all summary values in a vec
#[derive(Debug, PartialEq)]
pub enum SummaryType {
    Ratio(f64),
    Int(usize)
}

impl ClocStats {
    pub fn new(dir_name: PathBuf) -> ClocStats {
        ClocStats {
            name: dir_name.to_owned(),
            num_unsafe: 0,
            unsafe_fns: 0,
            total_fns: 0,
            blank: 0,
            comment: 0,
            files: 0,
            code: 0,
            panics: 0,
        }

    }

    pub fn name(&self) -> &PathBuf {
        &self.name
    }

    pub fn count_fns(&self) -> usize {
        self.total_fns
    }

    pub fn count_unsafe_fns(&self) -> usize {
        self.unsafe_fns
    }

    pub fn to_vec(&self) -> Vec<usize> {
        vec!(self.files, self.blank, self.comment, self.code, 
             self.num_unsafe, self.total_fns, self.unsafe_fns, self.panics)
    }

    // Consider empty if there haven't been any functions
    pub fn is_empty(&self) -> bool {
        !(self.total_fns > 0)
    }

    pub fn summarize(&self) -> Vec<SummaryType> {
        let mut unsafe_ratio = self.num_unsafe as f64 / self.code as f64 * 100.0;
        let mut fn_ratio = self.unsafe_fns as f64 / self.total_fns as f64 * 100.0;
        if unsafe_ratio.is_nan() {
            unsafe_ratio = 0.0;
        }
        if fn_ratio.is_nan() {
            fn_ratio = 0.0;
        }
        vec!(
            SummaryType::Int(self.files), 
            SummaryType::Int(self.blank),
            SummaryType::Int(self.comment),
            SummaryType::Int(self.code),
            SummaryType::Int(self.num_unsafe),
            SummaryType::Ratio(unsafe_ratio),
            SummaryType::Int(self.total_fns),
            SummaryType::Int(self.unsafe_fns),
            SummaryType::Ratio(fn_ratio), 
            SummaryType::Int(self.panics))
    }

    /// Gets stats for a single file
    pub fn from_file(filename: &str) -> ClocResult {
        let file_path = Path::new(filename);
        if file_path.extension().unwrap().to_str().unwrap() != "rs" {
            return Err("Not a rust file".to_owned());
        } 
        let mut f = File::open(filename).expect("Couldn't open file");

        let mut c = ClocStats::new(PathBuf::from(filename));
        c.cloc_file(&mut f);
        Ok(c)
    }

    /// Aggregates stats for an entire directory
    pub fn from_directory(dir: &str) -> ClocResult {
        let mut c = ClocStats::new(PathBuf::from(dir));
        let mut subdirs = vec!();
        subdirs.push(fs::read_dir(&Path::new(dir)).unwrap());

        while !subdirs.is_empty(){
            let paths = subdirs.pop();
            for p in paths.unwrap() {
                let p = p.unwrap();
                if p.file_type().unwrap().is_dir(){
                    if p.path().to_str().unwrap().contains(".git") {continue}
                    //TODO ignore git
                    subdirs.push(fs::read_dir(p.path()).unwrap());
                } else {
                    if p.path().extension().unwrap_or_default() == "rs" {
                        c.cloc_file(&mut File::open(p.path()).expect("Couldn't open file"));
                    }
                }
            }
        }

        Ok(c)

    }

    // TODO compare performance with BufReader -- isolate read for benchmarking
    // TODO will if/else work better than continue?
    fn cloc_file(&mut self, f: &mut File) {
        self.files += 1;
        let mut contents = String::new();

        // track brackets for unsafe blocks, fns etc
        let mut bracket_count = 0;
        // track comment flag
        let mut comment_flag = false; // handles ```...```
        let mut block_flag = false; //not totally sure if i need 2 flags? might be able to reuse

        f.read_to_string(&mut contents).expect(
            "something went wrong reading the file",
        );

        // TODO could probably split into methods if i store the flag/count in the struct
        for line in contents.lines() {
            let contains = REGEXES.matches(line);
            // skip content lines
            if contains.matched(0) {
                self.comment += 1;
                comment_flag = !comment_flag;
                continue;
            }
            if contains.matched(1) {
                self.comment += 1;
                continue;
            }
            //skip blank lines
            if line.len() == 0 {
                self.blank += 1;
                continue;
            }
            self.code += 1;
            if block_flag {
                if line.contains("{") {
                    bracket_count += 1;
                }
                if line.contains("}") {
                    bracket_count -= 1;
                }
                if bracket_count == 0 {
                    block_flag = false;
                } else {
                    self.num_unsafe += 1
                }
            }
            if contains.matched(3) {
                self.num_unsafe += 1;   //TODO is this always a 1 liner
            }
            if contains.matched(2) {
                self.total_fns += 1;
                if line.contains("unsafe") {
                    block_flag = true;
                    bracket_count += 1;
                    self.unsafe_fns += 1;
                }
            } else if contains.matched(4) {
                self.num_unsafe += 1;
            } else if contains.matched(5) {
                block_flag = true;
                bracket_count += 1;
            }
            if contains.matched(6) {
                self.panics += 1;
            }
        }

    }

    /// Compute ratio of unsafe code to total code
    pub fn unsafe_ratio(&self) -> f64 {
        match self.code {
            0 => 0.0,
            _ => self.num_unsafe as f64 / self.code as f64
        }
    }
}

impl fmt::Display for ClocStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}, {}, {}, {}, {}",
            self.num_unsafe,
            self.unsafe_fns,
            self.total_fns,
            self.blank,
            self.comment,
            self.files,
            self.code,
            self.panics
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c = ClocStats::from_file("./resources/test.rs").unwrap();
        assert_eq!(c.to_vec(), vec!(1, 5, 5, 25, 9, 3, 1, 0) );
    }
}
