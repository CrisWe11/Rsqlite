use std::io::Cursor;
use crate::utils::read_varint;
use super::*;

#[test]
fn test_read_varint() {
    let buffer = [0b11000101u8, 0b11000011u8];
    let b = &buffer[..];
    let mut c = Cursor::new(b);
    assert_eq!(read_varint(&mut c).unwrap(), 8899)
}
