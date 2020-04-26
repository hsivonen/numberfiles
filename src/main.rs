// SPDX: Apache-2.0 WITH LLVM-exception

use std::path::Path;

fn main() -> std::io::Result<()> {
	let mut args = std::env::args_os();
	if args.next().is_none() {
		eprintln!("Program name missing.");
		std::process::exit(-1);
	}
	let dir = if let Some(dir) = args.next() {
		dir
	} else {
		eprintln!("Directory missing.");
		std::process::exit(-1);		
	};
	let start = if let Some(start_str) = args.next() {
		start_str.to_str().unwrap().parse::<usize>().unwrap()
	} else {
		eprintln!("Starting number missing.");
		std::process::exit(-1);		
	};
	let dir_path = Path::new(&dir);
	let mut vec = Vec::new();

	for entry in std::fs::read_dir(dir_path)? {
		let entry = entry?;
		let path = entry.path();
		if let Some(ext) = path.extension() {
			if ext == "JPG" {
				if let Some(file_name) = path.file_name() {
					vec.push(file_name.to_owned());
				}
			}
		}
	}

	vec.sort();

	for (i, file_name) in vec.into_iter().enumerate() {
		let src = dir_path.join(file_name);
		let dst = dir_path.join(format!("{:04}.jpg", i + start));
		std::fs::rename(src, dst)?;
	}
	Ok(())
}
