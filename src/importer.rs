use crate::board::Board;
use crate::error_import::*;

use std::io::BufRead;

pub trait Importer {
    fn parse<R: BufRead>(reader: &mut R) -> ImportErrorResult<Board>;
}
