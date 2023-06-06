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
