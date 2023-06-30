use std::fs::read;
use std::io::Cursor;
use crate::utils::read_varint;


struct TableBTree<'b> {
    root: TableBTreeNode<'b>,
    depth: usize,
}

struct TablePayload<'b> {
    rowid: i64,
    col_types: Vec<ColType>,
    buffer: &'b mut [u8],
}

impl<'b> TablePayload<'b> {
    fn new(buffer: &'b [u8], pager_config: &crate::db::PagerConfig) -> Self {
        let mut buffer = Cursor::new(buffer);
        let payload_length = read_varint(&mut buffer).unwrap();
        let rowid = read_varint(&mut buffer).unwrap();

        let X = pager_config.page_size - pager_config.reserved_space_size as u16 - 35;
        if payload_length > X as i64 {} else {}
        todo!()
    }
}

impl<'b> TablePayload<'b> {}

enum ColType {
    NULL,
    I8,
    I32,
    I48,
    I64,
    F64,
    ZERO,
    ONE,
    BLOB,
    STRING,
}

enum TableBTreeNode<'b> {
    Interior {
        children: Vec<Box<TableBTreeNode<'b>>>,
    },
    Leaf {
        payload: Vec<TablePayload<'b>>,
    },
}
