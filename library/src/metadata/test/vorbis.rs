use crate::{Tag, metadata::vorbis::get};

macro_rules! ok {
	($data:expr) => {
		let mut metadata = Vec::new();
        assert!(get($data, &mut metadata).is_ok());
        assert!(metadata.is_empty());
	};
	($data:expr, $($name:expr => $value:expr),*) => {
		let mut a = Vec::new();
		let mut b = Vec::new();
        $(b.push(Tag { name: $name.to_string(), value: $value.to_string() });)*
        assert!(get($data, &mut a).is_ok());
		assert_eq!(a, b);
	};
}

macro_rules! error {
	($data:expr) => {
		let mut metadata = Vec::new();
		assert!(get($data, &mut metadata).is_err());
	};
}

#[test]
fn no_tags() {
	let data = &[
		0x0D, 0x00, 0x00, 0x00, b'L', b'a', b'v', b'f', b'6', b'0', b'.', b'1', b'2', b'.', b'1',
		b'0', b'0', 0x00, 0x00, 0x00, 0x00,
	];
	ok!(data);
}

#[test]
fn one_tag() {
	let data = &[
		0x0D, 0x00, 0x00, 0x00, b'L', b'a', b'v', b'f', b'6', b'0', b'.', b'1', b'2', b'.', b'1',
		b'0', b'0', 0x01, 0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00, b'T', b'I', b'T', b'L', b'E',
		b'=', b'H', b'i', b'g', b'h', b'e', b'r', b' ', b'G', b'r', b'o', b'u', b'n', b'd',
	];
	ok!(data, "TITLE" => "Higher Ground");
}

#[test]
fn two_tags() {
	let data = &[
		0x0D, 0x00, 0x00, 0x00, b'L', b'a', b'v', b'f', b'6', b'0', b'.', b'1', b'2', b'.', b'1',
		b'0', b'0', 0x02, 0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00, b'T', b'I', b'T', b'L', b'E',
		b'=', b'H', b'i', b'g', b'h', b'e', b'r', b' ', b'G', b'r', b'o', b'u', b'n', b'd', 0x14,
		0x00, 0x00, 0x00, b'A', b'R', b'T', b'I', b'S', b'T', b'=', b'S', b't', b'e', b'v', b'i',
		b'e', b' ', b'W', b'o', b'n', b'd', b'e', b'r',
	];
	ok!(data, "TITLE" => "Higher Ground", "ARTIST" => "Stevie Wonder");
}

#[test]
fn no_value() {
	let data = &[
		0x0D, 0x00, 0x00, 0x00, b'L', b'a', b'v', b'f', b'6', b'0', b'.', b'1', b'2', b'.', b'1',
		b'0', b'0', 0x01, 0x00, 0x00, 0x00, 0x0D, 0x00, 0x00, 0x00, b'H', b'i', b'g', b'h', b'e',
		b'r', b' ', b'G', b'r', b'o', b'u', b'n', b'd',
	];
	ok!(data, "Higher Ground" => "");
}

#[test]
fn missing_data_1() {
	error!(&[]);
}

#[test]
fn missing_data_2() {
	error!(&[0x0D, 0x00, 0x00, 0x00]);
}

#[test]
fn missing_data_3() {
	error!(&[
		0x0D, 0x00, 0x00, 0x00, b'L', b'a', b'v', b'f', b'6', b'0', b'.', b'1', b'2', b'.', b'1',
		b'0', b'0',
	]);
}

#[test]
fn missing_data_4() {
	error!(&[
		0x0D, 0x00, 0x00, 0x00, b'L', b'a', b'v', b'f', b'6', b'0', b'.', b'1', b'2', b'.', b'1',
		b'0', b'0', 0x01, 0x00, 0x00, 0x00,
	]);
}

#[test]
fn missing_data_5() {
	error!(&[
		0x0D, 0x00, 0x00, 0x00, b'L', b'a', b'v', b'f', b'6', b'0', b'.', b'1', b'2', b'.', b'1',
		b'0', b'0', 0x01, 0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00,
	]);
}
