use super::{data, metadata};
use crate::metadata::{
	Tag,
	pdf::{delete, get},
};
use std::io::Cursor;

fn reference_1(version: f32) -> [u8; 329] {
	let mut data = *b"%PDF-1.0
%\xBB\xAD\xC0\xDE
1 0 obj
<</Type/Catalog/Pages 2 0 R>>
endobj
2 0 obj
<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>
endobj
3 0 obj
<</Type/Page/Parent 2 0 R/Resources<<>>>>
endobj
xref
0 4
0000000000 65535 f \n\
0000000015 00000 n \n\
0000000060 00000 n \n\
0000000133 00000 n \n\
trailer
<</Root 1 0 R/Size 4>>
startxref
190
%%EOF";
	let version = format!("{version:.1}");
	data[5..5 + version.len()].copy_from_slice(version.as_bytes());
	data
}

fn reference_2(version: f32) -> [u8; 334] {
	let mut data = *b"%PDF-1.5
%\xBB\xAD\xC0\xDE
1 0 obj
<</Type/Catalog/Pages 2 0 R>>
endobj
2 0 obj
<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>
endobj
3 0 obj
<</Type/Page/Parent 2 0 R/Resources<<>>>>
endobj
4 0 obj
<</Type/XRef/Root 1 0 R/Size 5/W[1 4 2]/Index[1 4]/Length 28>>stream
\x01\x00\x00\x00\x0F\x00\x00\x01\x00\x00\x00\x3C\x00\x00\x01\x00\x00\x00\x85\x00\x00\x01\x00\x00\x00\xBE\x00\x00
endstream \n\
endobj

startxref
190
%%EOF";
	let version = format!("{version:.1}");
	data[5..5 + version.len()].copy_from_slice(version.as_bytes());
	data
}

const BASIC: &[u8] = b"%PDF-1.0
1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj
2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj
3 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>>>endobj
xref
0 4
0000000000 65535 f \n\
0000000009 00000 n \n\
0000000052 00000 n \n\
0000000123 00000 n \n\
trailer<</Root 1 0 R/Size 4>>
startxref
178
%%EOF";

const FREE_OBJECT: &[u8] = b"%PDF-1.0
1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj
2 0 obj<</Type/Pages/Count 1/Kids[4 0 R]/MediaBox[0 0 595 842]>>endobj
3 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>>>endobj
4 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>>>endobj
xref
0 5
0000000003 65535 f \n\
0000000009 00000 n \n\
0000000052 00000 n \n\
0000000000 00001 f \n\
0000000178 00000 n \n\
trailer<</Root 1 0 R/Size 5>>
startxref
233
%%EOF";

const UNREFERENCED_OBJECT: &[u8] = b"%PDF-1.0
1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj
2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj
3 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>>>endobj
4 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>>>endobj
xref
0 5
0000000000 65535 f \n\
0000000009 00000 n \n\
0000000052 00000 n \n\
0000000123 00000 n \n\
0000000178 00000 n \n\
trailer<</Root 1 0 R/Size 5>>
startxref
233
%%EOF";

const UNKNOWN_DATA: &[u8] = b"%PDF-1.0
1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj
2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj
3 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>>>endobj
xref
0 4
0000000000 65535 f \n\
0000000009 00000 n \n\
0000000052 00000 n \n\
0000000123 00000 n \n\
trailer<</Root 1 0 R/Size 4>>????
startxref
178
%%EOF";

const COMMENTS: &[u8] = b"%PDF-1.7
% Just a regular comment
1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj %catalog
2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj% pages
3 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>>>endobj%page \n\
xref
0 4
0000000000 65535 f \n\
0000000034 00000 n \n\
0000000086 00000 n \n\
0000000164 00000 n \n\
trailer<</Root 1 0 R/Size 4>>
startxref
225
%%EOF";

const BINARY_COMMENT: &[u8] = b"%PDF-1.7
%\x80\x80\x80\x80
1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj
2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj
3 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>>>endobj
xref
0 4
0000000000 65535 f \n\
0000000015 00000 n \n\
0000000058 00000 n \n\
0000000129 00000 n \n\
trailer<</Root 1 0 R/Size 4>>
startxref
184
%%EOF";

const BINARY_COMMENT_ASCII: &[u8] = b"%PDF-1.7
% \x80\x80\x80\x80
1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj
2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj
3 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>>>endobj
xref
0 4
0000000000 65535 f \n\
0000000016 00000 n \n\
0000000059 00000 n \n\
0000000130 00000 n \n\
trailer<</Root 1 0 R/Size 4>>
startxref
185
%%EOF";

const CROSS_REFERENCE_STREAM: &[u8] = b"%PDF-1.5
1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj
2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj
3 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>>>endobj
4 0 obj<</Type/XRef/Index[1 3]/W[1 1 1]/Root 1 0 R/Size 4/Length 9>>stream
\x01\x09\x00\x01\x34\x00\x01\x7B\x00endstream
endobj
startxref
178
%%EOF";

const OBJECT_STREAM: &[u8] = b"%PDF-1.5
1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj
2 0 obj<</Type/Pages/Count 1/Kids[4 0 R]/MediaBox[0 0 595 842]>>endobj
3 0 obj<</Type/ObjStm/First 3/N 1/Length 44>>stream
4 0<</Type/Page/Parent 2 0 R/Resources<<>>>>endstream
endobj
5 0 obj<</Type/XRef/Index[1 4]/W[1 1 1]/Root 1 0 R/Size 5/Length 12>>stream
\x01\x09\x00\x01\x34\x00\x01\x7B\x00\x02\x03\x03endstream
endobj
startxref
236
%%EOF";

const CHECKSUM: &[u8] = b"%PDF-1.0
1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj
2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj
3 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>>>endobj
xref
0 4
0000000000 65535 f \n\
0000000009 00000 n \n\
0000000052 00000 n \n\
0000000123 00000 n \n\
trailer<</Root 1 0 R/Size 4/DocChecksum/B224C4CB2C001E84531B12645C5A81CA>>
startxref
178
%%EOF";

const ID: &[u8] = b"%PDF-1.0
1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj
2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj
3 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>>>endobj
xref
0 4
0000000000 65535 f \n\
0000000009 00000 n \n\
0000000052 00000 n \n\
0000000123 00000 n \n\
trailer<</Root 1 0 R/Size 4/ID[<5D3463C1B9E9768F6EE333BE501D23E2><5D3463C1B9E9768F6EE333BE501D23E2>]>>
startxref
178
%%EOF";

const INFO: &[u8] = b"%PDF-1.0
1 0 obj<</Creator(LibreOffice 20.0)>>endobj
2 0 obj<</Type/Catalog/Pages 3 0 R>>endobj
3 0 obj<</Type/Pages/Count 1/Kids[4 0 R]/MediaBox[0 0 595 842]>>endobj
4 0 obj<</Type/Page/Parent 3 0 R/Resources<<>>>>endobj
xref
0 5
0000000000 65535 f \n\
0000000009 00000 n \n\
0000000053 00000 n \n\
0000000096 00000 n \n\
0000000167 00000 n \n\
trailer<</Root 2 0 R/Size 5/Info 1 0 R>>
startxref
222
%%EOF";

const INFO_DEPRECATED: &[u8] = b"%PDF-2.0
1 0 obj<</Creator(LibreOffice 20.0)>>endobj
2 0 obj<</Type/Catalog/Pages 3 0 R>>endobj
3 0 obj<</Type/Pages/Count 1/Kids[4 0 R]/MediaBox[0 0 595 842]>>endobj
4 0 obj<</Type/Page/Parent 3 0 R/Resources<<>>>>endobj
xref
0 5
0000000000 65535 f \n\
0000000009 00000 n \n\
0000000053 00000 n \n\
0000000096 00000 n \n\
0000000167 00000 n \n\
trailer<</Root 2 0 R/Size 5/Info 1 0 R>>
startxref
222
%%EOF";

const SIGNATURE: &[u8] = b"%PDF-1.0
1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj
2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj
3 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>>>endobj
4 0 obj<</Type/Sig/Filter/Adobe.PPKLite/SubFilter/adbe.pkcs7.detached/Contents<E1A9DE5DC7F97CC18CADE55D04EA0B3DD52AC4F0>>>endobj
xref
0 5
0000000000 65535 f \n\
0000000009 00000 n \n\
0000000052 00000 n \n\
0000000123 00000 n \n\
0000000178 00000 n \n\
trailer<</Root 1 0 R/Size 5>>
startxref
307
%%EOF";

const XMP: &[u8] = b"%PDF-1.4
1 0 obj<</Type/Catalog/Pages 2 0 R/Metadata 4 0 R>>endobj
2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj
3 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>>>endobj
4 0 obj<</Type/Metadata/Subtype/XML/Length 258>>stream
<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<xmpmeta xmlns:x=\"adobe:ns:meta/\">
<rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">
<rdf:Description rdf:about=\"\">
<pdf:Producer>LibreOffice 20.0</pdf:Producer>
</rdf:Description>
</rdf:RDF>
</xmpmeta>
endstream
endobj
xref
0 5
0000000000 65535 f \n\
0000000009 00000 n \n\
0000000067 00000 n \n\
0000000138 00000 n \n\
0000000193 00000 n \n\
trailer<</Root 1 0 R/Size 5>>
startxref
523
%%EOF";

const OBJECT_METADATA: &[u8] = b"%PDF-1.4
1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj
2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj
3 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>/Metadata 4 0 R>>endobj
4 0 obj<</Type/Metadata/Subtype/XML/Length 258>>stream
<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<xmpmeta xmlns:x=\"adobe:ns:meta/\">
<rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">
<rdf:Description rdf:about=\"\">
<pdf:Producer>LibreOffice 20.0</pdf:Producer>
</rdf:Description>
</rdf:RDF>
</xmpmeta>
endstream
endobj
xref
0 5
0000000000 65535 f \n\
0000000009 00000 n \n\
0000000052 00000 n \n\
0000000123 00000 n \n\
0000000179 00000 n \n\
trailer<</Root 1 0 R/Size 5>>
startxref
523
%%EOF";

const DETACHED_METADATA: &[u8] = b"%PDF-1.4
1 0 obj<</Type/Catalog/Pages 2 0 R>>endobj
2 0 obj<</Type/Pages/Count 1/Kids[3 0 R]/MediaBox[0 0 595 842]>>endobj
3 0 obj<</Type/Page/Parent 2 0 R/Resources<<>>>>endobj
4 0 obj<</Type/Metadata/Subtype/XML/Length 258>>stream
<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<xmpmeta xmlns:x=\"adobe:ns:meta/\">
<rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">
<rdf:Description rdf:about=\"\">
<pdf:Producer>LibreOffice 20.0</pdf:Producer>
</rdf:Description>
</rdf:RDF>
</xmpmeta>
endstream
endobj
xref
0 5
0000000000 65535 f \n\
0000000009 00000 n \n\
0000000052 00000 n \n\
0000000123 00000 n \n\
0000000164 00000 n \n\
trailer<</Root 1 0 R/Size 5>>
startxref
508
%%EOF";

#[test]
fn get_basic() {
	metadata!(BASIC);
}

#[test]
fn delete_basic() {
	data!(BASIC, reference_1(1.0));
}

#[test]
fn get_free_object() {
	metadata!(FREE_OBJECT);
}

#[test]
fn delete_free_object() {
	data!(FREE_OBJECT, reference_1(1.0));
}

#[test]
fn get_unreferenced_object() {
	metadata!(UNREFERENCED_OBJECT);
}

#[test]
fn delete_unreferenced_object() {
	data!(UNREFERENCED_OBJECT, reference_1(1.0));
}

#[test]
fn get_unknown_data() {
	metadata!(UNKNOWN_DATA);
}

#[test]
fn delete_unknown_data() {
	data!(UNKNOWN_DATA, reference_1(1.0));
}

#[test]
fn get_comments() {
	metadata!(COMMENTS);
}

#[test]
fn delete_comments() {
	data!(COMMENTS, reference_1(1.7));
}

#[test]
fn get_binary_comment() {
	metadata!(BINARY_COMMENT);
}

#[test]
fn delete_binary_comment() {
	let mut reference = reference_1(1.7);
	reference[10..14].copy_from_slice(&[0x80, 0x80, 0x80, 0x80]);
	data!(BINARY_COMMENT, reference);
}

#[test]
fn get_binary_comment_ascii() {
	metadata!(BINARY_COMMENT_ASCII);
}

#[test]
fn delete_binary_comment_ascii() {
	data!(BINARY_COMMENT_ASCII, reference_1(1.7));
}

#[test]
fn get_cross_reference_stream() {
	metadata!(CROSS_REFERENCE_STREAM);
}

#[test]
fn delete_cross_reference_stream() {
	data!(CROSS_REFERENCE_STREAM, reference_2(1.5));
}

#[test]
fn get_object_stream() {
	metadata!(OBJECT_STREAM);
}

#[test]
fn delete_object_stream() {
	data!(OBJECT_STREAM, reference_2(1.5));
}

#[test]
fn get_checksum() {
	metadata!(CHECKSUM, "DocChecksum" => "B224C4CB2C001E84531B12645C5A81CA");
}

#[test]
fn delete_checksum() {
	data!(CHECKSUM, reference_1(1.0));
}

#[test]
fn get_id() {
	metadata!(ID);
}

#[test]
fn delete_id() {
	data!(INFO, reference_1(1.0));
}

#[test]
fn get_info() {
	metadata!(INFO, "Creator" => "LibreOffice 20.0");
}

#[test]
fn delete_info() {
	data!(INFO, reference_1(1.0));
}

#[test]
fn get_info_deprecated() {
	metadata!(INFO_DEPRECATED, "Creator" => "LibreOffice 20.0");
}

#[test]
fn delete_info_deprecated() {
	data!(INFO_DEPRECATED, reference_1(2.0));
}

#[test]
fn get_signature() {
	metadata!(SIGNATURE);
}

#[test]
fn delete_signature() {
	data!(SIGNATURE, reference_1(1.0));
}

#[test]
fn get_xmp() {
	metadata!(XMP, "Producer" => "LibreOffice 20.0");
}

#[test]
fn delete_xmp() {
	data!(XMP, reference_1(1.4));
}

#[test]
fn get_object_metadata() {
	metadata!(OBJECT_METADATA);
}

#[test]
fn delete_object_metadata() {
	data!(OBJECT_METADATA, reference_1(1.4));
}

#[test]
fn get_detached_metadata() {
	metadata!(DETACHED_METADATA);
}

#[test]
fn delete_detached_metadata() {
	data!(DETACHED_METADATA, reference_1(1.4));
}
