use crate::{Error, Tag, metadata::vorbis, utilities::seek};
use std::{
	io::{ErrorKind, Read, Seek, Write, copy},
	slice,
};

pub fn get<R: Read + Seek>(source: &mut R) -> Result<Vec<Tag>, Error> {
	source.seek_relative(4)?;
	let mut metadata = Vec::new();
	loop {
		let Some((last, header)) = read_header(source)? else {
			break;
		};
		if header.block_type == 4 {
			let mut data = vec![0; header.size as usize];
			source.read_exact(&mut data)?;
			vorbis::get(&data, &mut metadata)?;
		} else {
			seek!(source, header.size)?;
		}
		if last {
			break;
		}
	}
	Ok(metadata)
}

pub fn delete<R: Read + Seek, W: Write>(source: &mut R, destination: &mut W) -> Result<(), Error> {
	copy(&mut source.take(4), destination)?;
	let mut previous: Option<(Header, Vec<u8>)> = None;
	loop {
		let Some((last, header)) = read_header(source)? else {
			return Err(Error::File);
		};
		match header.block_type {
			0 | 1 | 3 | 5 => {
				if let Some(previous) = previous {
					write_header(destination, false, previous.0)?;
					destination.write_all(&previous.1)?;
				}
				let mut data = vec![0; header.size as usize];
				source.read_exact(&mut data)?;
				previous = Some((header, data));
			}
			_ => {
				seek!(source, header.size)?;
			}
		}
		if last {
			if let Some(previous) = previous {
				write_header(destination, true, previous.0)?;
				destination.write_all(&previous.1)?;
			}
			copy(source, destination)?;
			break;
		}
	}
	Ok(())
}

struct Header {
	block_type: u8,
	size: u32,
}

fn read_header<R: Read>(source: &mut R) -> Result<Option<(bool, Header)>, Error> {
	let mut data = [0; 4];
	if let Err(error) = source.read_exact(slice::from_mut(&mut data[0])) {
		if error.kind() == ErrorKind::UnexpectedEof {
			return Ok(None);
		}
		return Err(Error::Io(error));
	};
	source.read_exact(&mut data[1..4])?;
	let mut size = data;
	size[0] = 0;
	let size = u32::from_be_bytes(size);
	Ok(Some((
		data[0] > 127,
		Header {
			block_type: data[0] & 0b01111111,
			size,
		},
	)))
}

fn write_header<W: Write>(writer: &mut W, last: bool, header: Header) -> Result<(), Error> {
	writer.write_all(&[(last as u8) << 7 + header.block_type])?;
	writer.write_all(&header.size.to_be_bytes()[1..4])?;
	Ok(())
}
