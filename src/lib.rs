
use std::fmt;
use std::path::Path;
use std::io;
use std::fs;
use std::result::Result::Err;
use std::error::Error;

type ClocResult = Result<ClocStats, String>;

pub struct ClocStats {
    num_unsafe: i64,
    unsafe_fns: i64,
    total_fns: i64,
    blank: i64,
    comment: i64,
    files: i64,
    code: i64,
    panics: i64,
}

impl ClocStats {
    pub fn new() -> ClocStats {
        ClocStats {
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

    /// Gets stats for a single file
    pub fn from_file(file: &Path) -> ClocResult {
        if !file.is_file() {
            return Err(
                "{} was not a file. Did you mean to use from_directory?".to_owned(),
            );
        }
        let mut c = ClocStats::new();
        c.cloc_file();
        Ok(c)
    }

    /// Aggregates stats for an entire directory
    pub fn from_directory(dir: &Path) -> ClocResult {
        let mut c = ClocStats::new();
        if !dir.is_dir() {
            return Err("Not a directory. Did you mean to use from_file?".to_owned());
        }

        //for entry in fs::read_dir(dir)?.map_err(|e| e.description()) {
        //    println!("{:?}", entry);
        //}
        Ok(c)

    }

    fn cloc_file(&mut self) {}
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
    #[test]
    fn it_works() {
        let c = ClocStats::from_file(&Path::new("utils.rs"));
    }
}
