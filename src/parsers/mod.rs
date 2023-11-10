mod csv_file;
mod debug_file;
mod diagnostic;
mod loc_file;
mod metainfo;
pub type Loc = loc_file::Loc;
pub type Diagnostic = diagnostic::Diagnostic;
pub type MetaInfo = metainfo::MetaInfo;
