use std::fmt::{Display, Formatter};
use crate::rb;
//
// struct FreelistTrunkPage {}
//
// // struct DBFile {
// //     file: File,
// //     header: DBHeader,
// // }
// struct Cell {}
//
// enum Page<'b> {
//     BTreePage {
//         db_header: Option<DBHeader<'b>>,
//         b_tree_page_header: BTreePageHeader,
//         cells: Vec<Cell>,
//     },
// }
//
//
// impl TryFrom<u8> for BTreePageType {
//     type Error = &'static str;
//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         match value {
//             2 => Ok(BTreePageType::InteriorIndexBTreePage),
//             5 => Ok(BTreePageType::InteriorTableBTreePage),
//             10 => Ok(BTreePageType::LeafIndexBTreePage),
//             13 => Ok(BTreePageType::LeafTableBTreePage),
//             _ => Err(&format!(
//                 "Invalid byte: {} for BTreePageType conversion",
//                 value
//             )),
//         }
//     }
// }
//
// enum Offset {
//     StartOfFile(u32),
//     StartOfPage(u16),
// }
//
//
// impl TryFrom<&[u8]> for BTreePageHeader {
//     type Error = &'static str;
//
//     fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
//         let page_type: BTreePageType = BTreePageType::try_from(value[0])?;
//         let mut header_len = 0;
//         let mut right_most_pointer: Option<Offset> = None;
//         match page_type {
//             BTreePageType::InteriorIndexBTreePage | BTreePageType::InteriorTableBTreePage => {
//                 header_len = 12;
//                 right_most_pointer = Some(Offset::StartOfFile(u32::from_be_bytes(
//                     (value[8..12]).try_into().expect(""),
//                 )))
//             }
//             BTreePageType::LeafIndexBTreePage | BTreePageType::LeafTableBTreePage => header_len = 8,
//         }
//         if value.len() < header_len {
//             return Err("BTreePageHeader parse error: too short");
//         }
//         let freeblock_offset = u16::from_be_bytes([value[1], value[2]]);
//         let freeblock_offset = if freeblock_offset == 0 {
//             None
//         } else {
//             Some(Offset::StartOfPage(freeblock_offset))
//         };
//         todo!()
//     }
// }
//
pub struct DBHeader<'b> {
    buffer: &'b [u8; 100],
    page_size: &'b [u8;2],
    file_format_write_version: &'b u8,
    file_format_read_version: &'b u8,
    reserved_space_size: &'b u8,
    maximum_embedded_payload_fraction: &'b u8,
    minimum_embedded_payload_fraction: &'b u8,
    leaf_payload_fraction: &'b u8,
    file_change_counter: &'b [u8;4],
    in_header_database_size: &'b [u8;4],
    first_freelist_trunk_page: &'b [u8;4],
    total_number_of_freelist_pages: &'b [u8;4],
    schema_cookie: &'b [u8;4],
    schema_format: &'b [u8;4],
    default_page_cache_size: &'b [u8;4],
    largest_root_btree_page: &'b [u8;4],
    db_text_encoding: &'b [u8;4],
    user_version: &'b [u8;4],
    incremental_vacuum_mode: &'b [u8;4],
    application_id: &'b [u8;4],
    reserved_space: &'b [u8;20],
    version_valid_for: &'b [u8;4],
    sqlite_version_number: &'b [u8;4],
}

impl<'b> From<&'b [u8; 100]> for DBHeader<'b> {
    fn from(value: &'b [u8; 100]) -> Self {
        DBHeader {
            buffer: value,
            page_size: value[16..18].try_into().unwrap(),
            file_format_write_version: &value[18],
            file_format_read_version: &value[19],
            reserved_space_size: &value[20],
            maximum_embedded_payload_fraction: &value[21],
            minimum_embedded_payload_fraction: &value[22],
            leaf_payload_fraction: &value[23],
            file_change_counter: value[24..28].try_into().unwrap(),
            in_header_database_size: value[28..32].try_into().unwrap(),
            first_freelist_trunk_page: value[32..36].try_into().unwrap(),
            total_number_of_freelist_pages: value[36..40].try_into().unwrap(),
            schema_cookie: value[40..44].try_into().unwrap(),
            schema_format: value[44..48].try_into().unwrap(),
            default_page_cache_size: value[48..52].try_into().unwrap(),
            largest_root_btree_page: value[52..56].try_into().unwrap(),
            db_text_encoding: value[56..60].try_into().unwrap(),
            user_version: value[60..64].try_into().unwrap(),
            incremental_vacuum_mode: value[64..68].try_into().unwrap(),
            application_id: value[68..72].try_into().unwrap(),
            reserved_space: value[72..92].try_into().unwrap(),
            version_valid_for: value[92..96].try_into().unwrap(),
            sqlite_version_number: value[96..100].try_into().unwrap(),
        }
    }
}
impl<'b> Display for DBHeader<'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "database page size:    {}",u16::from_be_bytes(*self.page_size))?;
        writeln!(f, "write format:          {}",self.file_format_write_version)?;
        writeln!(f, "read format:           {}",self.file_format_read_version)?;
        writeln!(f, "reserved bytes:        {}",self.reserved_space_size)?;
        writeln!(f, "file change counter:   {}",u32::from_be_bytes(*self.file_change_counter))?;
        writeln!(f, "database page count:   {}",u32::from_be_bytes(*self.in_header_database_size))?;
        writeln!(f, "freelist page count:   {}",u32::from_be_bytes(*self.total_number_of_freelist_pages))?;
        writeln!(f, "schema cookie:         {}",u32::from_be_bytes(*self.schema_cookie))?;
        writeln!(f, "schema format:         {}",u32::from_be_bytes(*self.schema_format))?;
        writeln!(f, "default cache size:    {}",u32::from_be_bytes(*self.default_page_cache_size))?;
        writeln!(f, "autovacuum top root:   {}",u32::from_be_bytes(*self.largest_root_btree_page))?;
        writeln!(f, "incremental vacuum:    {}",u32::from_be_bytes(*self.incremental_vacuum_mode))?;
        writeln!(f, "text encoding:         {}",u32::from_be_bytes(*self.db_text_encoding))?;
        writeln!(f, "user version:          {}",u32::from_be_bytes(*self.user_version))?;
        writeln!(f, "application id:        {}",u32::from_be_bytes(*self.application_id))?;
        writeln!(f, "software version:      {}",u32::from_be_bytes(*self.sqlite_version_number))?;
        Ok(())
    }
}

enum BTreePageType {
    InteriorIndexBTreePage,
    InteriorTableBTreePage,
    LeafIndexBTreePage,
    LeafTableBTreePage,
}

// struct BTreePageHeader {
//     page_type: BTreePageType,
//     freeblock_offset: Option<Offset>,
//     number_of_cells: u16,
//     start_of_the_content_area: Offset,
//     fragmented_bytes: u8,
//     right_most_pointer: Option<Offset>,
// }
