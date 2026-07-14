use numeta::{Error, Metadata};
use std::{
	borrow::Cow,
	env::current_dir,
	ffi::{OsStr, OsString},
	fs::{File, OpenOptions},
	io::{BufReader, BufWriter},
	path::{Path, PathBuf},
};

mod options;
mod temporary;
use options::Options;
use temporary::Temporary;

fn main() -> Result<(), Error> {
	let Ok(options) = Options::parse() else {
		println!("Usage:");
		println!("  numeta <file>");
		println!("  numeta -d [-o <file>|-r] <file>\n");
		println!("Options:");
		println!("  -d   Delete metadata");
		println!("  -o   Write the results to a file");
		println!("  -r   Write the results to the input file");
		return Ok(());
	};
	let source = File::open(&options.source)?;
	let mut source = BufReader::new(source);
	let extension = options.source.extension().and_then(OsStr::to_str);
	let Some(metadata) = Metadata::guess(&mut source, extension)? else {
		eprintln!("File format not supported");
		return Ok(());
	};
	if options.delete {
		let directory = match directory(&options.destination) {
			Some(path) => Cow::Borrowed(path),
			None => Cow::Owned(current_dir()?),
		};
		let temporary = Temporary::unique(directory)?;
		metadata.delete(&mut source, &mut BufWriter::new(&temporary.writer))?;
		let destination = options
			.destination
			.unwrap_or_else(|| create_from_template(&options.source));
		temporary.persist(destination)?;
	} else {
		for tag in metadata.get(&mut source)? {
			println!("{}", tag);
		}
	}
	Ok(())
}

fn directory(path: &Option<PathBuf>) -> Option<&Path> {
	path.as_deref()
		.and_then(Path::parent)
		.filter(|path| !path.as_os_str().is_empty())
}

fn create_from_template<P: AsRef<Path>>(template: P) -> PathBuf {
	let template = template.as_ref();
	let mut number = 1;
	loop {
		let mut name = OsString::from(template.file_stem().unwrap());
		if number > 1 {
			name.push("-");
			name.push(number.to_string());
		}
		if let Some(extension) = template.extension() {
			name.push(".");
			name.push(extension);
		}
		if OpenOptions::new()
			.write(true)
			.create_new(true)
			.open(&name)
			.is_ok()
		{
			return PathBuf::from(name);
		}
		number += 1;
	}
}

#[test]
fn test_directory_1() {
	assert_eq!(
		directory(&Some("/data/in.png".into())),
		Some(Path::new("/data"))
	);
}

#[test]
fn test_directory_2() {
	assert_eq!(
		directory(&Some("data/in.png".into())),
		Some(Path::new("data"))
	);
}

#[test]
fn test_directory_3() {
	assert_eq!(directory(&Some("/in.png".into())), Some(Path::new("/")));
}

#[test]
fn test_directory_4() {
	assert_eq!(directory(&Some("in.png".into())), None);
}

#[test]
fn test_directory_5() {
	assert_eq!(directory(&None), None);
}
