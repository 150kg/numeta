use crate::{Error, Tag, xml::parse_name};
use quick_xml::{
	Reader,
	events::{BytesStart, Event},
};

pub fn get(data: &[u8], metadata: &mut Vec<Tag>) -> Result<(), Error> {
	let data = String::from_utf8_lossy(data);
	let mut reader = Reader::from_str(&data);
	loop {
		match reader.read_event()? {
			Event::Start(start) => parse_start(&mut reader, start, true, metadata)?,
			Event::Eof => break,
			_ => {}
		}
	}
	Ok(())
}

fn parse_start(
	reader: &mut Reader<&[u8]>,
	start: BytesStart,
	keep: bool,
	metadata: &mut Vec<Tag>,
) -> Result<(), Error> {
	let (name, _) = parse_name(start.name().as_ref());
	let keep = keep && !matches!(name.as_str(), "History" | "Manifest" | "Thumbnails");
	loop {
		match reader.read_event()? {
			Event::End(end) if end.name() == start.name() => break,
			Event::End(_) | Event::Eof => return Err(Error::Metadata),
			Event::Start(start) => parse_start(reader, start, keep, metadata)?,
			Event::Text(text) if keep && !text.trim_ascii().is_empty() => {
				metadata.push(Tag {
					name: name.clone(),
					value: String::from_utf8_lossy(&text).to_string(),
				});
			}
			_ => {}
		}
	}
	Ok(())
}
