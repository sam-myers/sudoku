use crate::board::Board;
use crate::error::ImportError;

use std::io::Read;

pub trait Importer {
    fn parse<R: Read>(&self, reader: &mut R) -> Result<Board, ImportError>;
}
