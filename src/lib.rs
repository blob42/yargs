//DEBUG:
#![allow(dead_code)]

use anyhow::Result;
use std::ops::Deref;
use parse::split_columns;

pub mod stdin;
pub mod parse;

pub const DEFAULT_SEP_PATTERN: &str = r"[\t]+";

type Column = Vec<String>;

#[derive(Clone, Debug)]
pub struct Columns(Vec<Column>);



impl Deref for Columns {
    type Target = Vec<Vec<String>>;

    fn deref(&self) -> &Vec<Vec<String>> {
        &self.0
    }

}

// build Columns from &str
impl TryFrom<&str> for Columns {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        split_columns(value, DEFAULT_SEP_PATTERN)
    }
}

