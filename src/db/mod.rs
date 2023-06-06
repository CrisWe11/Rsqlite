use nom::Err::Error;
use std::fs::File;

struct FreelistTrunkPage {}

// struct DBFile {
//     file: File,
//     header: DBHeader,
// }
struct Cell {}

enum Page<'b> {
    BTreePage {
        db_header: Option<DBHeader<'b>>,
        b_tree_page_header: BTreePageHeader,
        cells: Vec<Cell>,
    },
}

enum BTreePageType {
    InteriorIndexBTreePage,
    InteriorTableBTreePage,
    LeafIndexBTreePage,
    LeafTableBTreePage,
}

impl TryFrom<u8> for BTreePageType {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(BTreePageType::InteriorIndexBTreePage),
            5 => Ok(BTreePageType::InteriorTableBTreePage),
            10 => Ok(BTreePageType::LeafIndexBTreePage),
            13 => Ok(BTreePageType::LeafTableBTreePage),
            _ => Err(&format!(
                "Invalid byte: {} for BTreePageType conversion",
                value
            )),
        }
    }
}

enum Offset {
    StartOfFile(u32),
    StartOfPage(u16),
}

struct BTreePageHeader {
    page_type: BTreePageType,
    freeblock_offset: Option<Offset>,
    number_of_cells: u16,
    start_of_the_content_area: Offset,
    fragmented_bytes: u8,
    right_most_pointer: Option<Offset>,
}

impl TryFrom<&[u8]> for BTreePageHeader {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let page_type: BTreePageType = BTreePageType::try_from(value[0])?;
        let mut header_len = 0;
        let mut right_most_pointer: Option<Offset> = None;
        match page_type {
            BTreePageType::InteriorIndexBTreePage | BTreePageType::InteriorTableBTreePage => {
                header_len = 12;
                right_most_pointer = Some(Offset::StartOfFile(u32::from_be_bytes(
                    (value[8..12]).try_into().expect(""),
                )))
            }
            BTreePageType::LeafIndexBTreePage | BTreePageType::LeafTableBTreePage => header_len = 8,
        }
        if value.len() < header_len {
            return Err("BTreePageHeader parse error: too short");
        }
        let freeblock_offset = u16::from_be_bytes([value[1], value[2]]);
        let freeblock_offset = if freeblock_offset == 0 {
            None
        } else {
            Some(Offset::StartOfPage(freeblock_offset))
        };
        todo!()
    }
}

struct DBHeader<'b> {
    buffer: &'b [u8; 100],
    page_size: u16,
    file_format_write_version: u8,
    file_format_read_version: u8,
    reserved_space_size: u8,
    maximum_embedded_payload_fraction: u8,
    minimum_embedded_payload_fraction: u8,
    leaf_payload_fraction: u8,
    file_change_counter: u32,
    in_header_database_size: u32,
    first_freelist_trunk_page: FreelistTrunkPage,
    total_number_of_freelist_pages: u32,
    schema_cookie: u32,
    schema_format: u32,
    default_page_cache_size: u32,
    largest_root_btree_page: u32,
    db_text_encoding: u32,
    user_version: u32,
    incremental_vacuum_mode: u32,
    application_id: u32,
    reserved_space: &'b [u8; 20],
    version_valid_for: u32,
    sqlite_version_number: u32,
}

impl<'b> DBHeader<'b> {
    pub fn new(buffer: &'b [u8; 100]) -> DBHeader<'b> {
        DBHeader { buffer }
    }
}
