use crate::{
	Error, Tag,
	utilities::bytes::{Bytes, Le},
};

pub fn get(data: &[u8], metadata: &mut Vec<Tag>) -> Result<(), Error> {
	if data.len() < 4 {
		return Err(Error::Metadata);
	}
	let size = Le::u32(data) as usize;
	if data.len() < 8 + size {
		return Err(Error::Metadata);
	}
	let tags = Le::u32(&data[4 + size..]) as usize;
	let mut data = &data[8 + size..];
	for _ in 0..tags {
		if data.len() < 4 {
			return Err(Error::Metadata);
		}
		let size = Le::u32(data) as usize;
		if data.len() < 4 + size {
			return Err(Error::Metadata);
		}
		let position = data
			.iter()
			.position(|&character| character == b'=')
			.unwrap_or(data.len());
		let name = String::from_utf8_lossy(&data[4..position]).to_string();
		let value = String::from_utf8_lossy(&data[position + 1..4 + size]).to_string();
		metadata.push(Tag { name, value });
		data = &data[size + 4..];
	}
	Ok(())
}
