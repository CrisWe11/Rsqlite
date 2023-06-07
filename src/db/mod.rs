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
// enum BTreePageType {
//     InteriorIndexBTreePage,
//     InteriorTableBTreePage,
//     LeafIndexBTreePage,
//     LeafTableBTreePage,
// }
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
// struct BTreePageHeader {
//     page_type: BTreePageType,
//     freeblock_offset: Option<Offset>,
//     number_of_cells: u16,
//     start_of_the_content_area: Offset,
//     fragmented_bytes: u8,
//     right_most_pointer: Option<Offset>,
// }
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
    page_size: u16,
    file_format_write_version: u8,
    file_format_read_version: u8,
    reserved_space_size: u8,
    maximum_embedded_payload_fraction: u8,
    minimum_embedded_payload_fraction: u8,
    leaf_payload_fraction: u8,
    file_change_counter: u32,
    in_header_database_size: u32,
    first_freelist_trunk_page: u32,
    total_number_of_freelist_pages: u32,
    schema_cookie: u32,
    schema_format: u32,
    default_page_cache_size: u32,
    largest_root_btree_page: u32,
    db_text_encoding: u32,
    user_version: u32,
    incremental_vacuum_mode: u32,
    application_id: u32,
    reserved_space: &'b [u8],
    version_valid_for: u32,
    sqlite_version_number: u32,
}

impl<'b> From<&'b [u8; 100]> for DBHeader<'b> {
    fn from(value: &'b [u8; 100]) -> Self {
        DBHeader {
            buffer: value,
            page_size: rb!(u16, value[16]),
            file_format_write_version: value[18],
            file_format_read_version: value[19],
            reserved_space_size: value[20],
            maximum_embedded_payload_fraction: value[21],
            minimum_embedded_payload_fraction: value[22],
            leaf_payload_fraction: value[23],
            file_change_counter: rb!(u32, value[24]),
            in_header_database_size: rb!(u32, value[28]),
            first_freelist_trunk_page: rb!(u32, value[32]),
            total_number_of_freelist_pages: rb!(u32, value[36]),
            schema_cookie: rb!(u32, value[40]),
            schema_format: rb!(u32, value[44]),
            default_page_cache_size: rb!(u32, value[48]),
            largest_root_btree_page: rb!(u32, value[52]),
            db_text_encoding: rb!(u32, value[56]),
            user_version: rb!(u32, value[60]),
            incremental_vacuum_mode: rb!(u32, value[64]),
            application_id: rb!(u32, value[68]),
            reserved_space: &value[72..92],
            version_valid_for: rb!(u32, value[92]),
            sqlite_version_number: rb!(u32, value[96]),
        }
    }
}
impl<'b> Display for DBHeader<'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "database page size: {}",self.page_size)?;
        writeln!(f, "write format: {}",self.file_format_write_version)?;
        writeln!(f, "read format: {}",self.file_format_read_version)?;
        writeln!(f, "reserved bytes: {}",self.reserved_space_size)?;
        writeln!(f, "file change counter: {}",self.file_change_counter)?;
        writeln!(f, "database page count: {}",self.in_header_database_size)?;
        writeln!(f, "freelist page count: {}",self.total_number_of_freelist_pages)?;
        writeln!(f, "schema cookie: {}",self.schema_cookie)?;
        writeln!(f, "schema format: {}",self.schema_format)?;
        writeln!(f, "default cache size: {}",self.default_page_cache_size)?;
        writeln!(f, "autovacuum top root: {}",self.largest_root_btree_page)?;
        writeln!(f, "incremental vacuum: {}",self.incremental_vacuum_mode)?;
        writeln!(f, "text encoding: {}",self.db_text_encoding)?;
        writeln!(f, "user version: {}",self.user_version)?;
        writeln!(f, "application id: {}",self.application_id)?;
        Ok(())
    }
}
