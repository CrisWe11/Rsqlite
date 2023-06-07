use crate::utils::read_var_int;
use super::*;

#[test]
#[ignore]
fn test_read_varint() {
    assert_eq!(read_var_int(&[0b11000101, 0b01000011, 3]), (8899, 2))
}

#[test]
fn macro_test() {
    let arr:&[u8] = &[0,1,2,3,4,5,6,7,8,9,10];
    println!("{}", rb!(u64, arr[1]))
}



