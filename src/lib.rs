pub mod cli;
mod process;

pub use process::process_csv;
pub use process::process_genpass;
pub use process::{process_decode, process_encode};
