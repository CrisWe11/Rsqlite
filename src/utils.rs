pub fn read_var_int(buffer: &[u8]) -> (i64, usize) {
    let mut len = 0;
    let mut shift: usize = 0;
    let mut output: i64 = 0;
    for &byte in buffer {
        output <<= shift;
        output |= (byte & 0b01111111) as i64;
        shift += 7;
        len += 1;
        if len == 9 || byte & 0b10000000 == 0 {
            break;
        }
    }
    (output, len)
}

pub fn pn(page_size: usize) -> impl Fn(usize, usize) -> usize {
    move |current_page: usize, offset: usize| (current_page * page_size + offset)
}


#[macro_export]
macro_rules! rb {
    (u8, $arr:ident [ $idx:expr ]) => {
        $arr[$idx]
    };
    (u16, $arr:ident [ $idx:expr ]) => {
        u16::from_be_bytes([$arr[$idx], $arr[$idx + 1]])
    };
    (u32, $arr:ident [ $idx:expr ]) => {
        u32::from_be_bytes([$arr[$idx], $arr[$idx + 1], $arr[$idx + 2], $arr[$idx + 3]])
    };
    (u64, $arr:ident [ $idx:expr ]) => {
        u64::from_be_bytes([$arr[$idx], $arr[$idx + 1], $arr[$idx + 2], $arr[$idx + 3], $arr[$idx + 4], $arr[$idx + 5], $arr[$idx + 6], $arr[$idx + 7]])
    };
}
