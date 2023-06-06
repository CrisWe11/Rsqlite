use crate::utils::read_var_int;
use super::*;

#[test]
fn test_read_varint() {
    assert_eq!(read_var_int(&[0b11000101, 0b01000011, 3]), (8899, 2))
}
