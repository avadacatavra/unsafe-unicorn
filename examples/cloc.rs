
use std::fmt;
use std::path::Path;
use std::io::self;
use std::fs::{self};

pub struct ClocStats {
	num_unsafe: i64,
	unsafe_fns: i64,
	total_fns: i64,
	blank: i64,
	comment: i64,
	files: i64,
	code: i64,
	panics: i64,
};

impl ClocStats {
	pub fn new() -> ClocStats {
		ClocStats{
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
	pub fn from_file(file: Path) -> io::Result<ClocStats> {
		if !file.is_file() {
			return io::Err("{} was not a file. Did you mean to use from_directory?", file)
		}
		let mut c = ClocStats::new();
		c.cloc_file()
	}

	/// Aggregates stats for an entire directory
	pub fn from_directory(dir: Path) -> ClocStats {
		let mut c = ClocStats::new();
		if !dir.is_dir() {
			return io::Err("{} was not a directory. Did you mean to use from_file?", dir)
		}

		for entry in fs::read_dir(dir)? {
			
		}

	}

	fn cloc_file(&mut self) {

	}
}

impl fmt::Display for ClocStats {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}, {}, {}, {}, {}, {}, {}, {}",
			self.num_unsafe,
			self.unsafe_fns,
			self.total_fns,
			self.blank,
			self.comment,
			self.files,
			self.code,
			self.panics)
	}

}

