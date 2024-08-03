pub mod read;
pub mod write;
pub mod append;

pub use read::read_file;
pub use write::write_file;
pub use append::append_file;