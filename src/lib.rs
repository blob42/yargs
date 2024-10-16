//DEBUG:
#![allow(dead_code)]

use anyhow::Result;
use std::fmt;
use std::{fmt::{Display, Formatter}, ops::{Deref, DerefMut}};
use parse::split_columns;

pub mod stdin;
pub mod parse;

// pub const DEFAULT_SEP_PATTERN: &str = r"[\t]+";
// default separator is one or more consecutive space characters
pub const DEFAULT_SEP_PATTERN: &str = r"[\s]+";

//FIX: fix tests for tab separator pattern and default separator pattern
//
//NOTE: learn how to setup a global var that will hold the current separator pattern


// pub mod patterns {
//     use super::DEFAULT_SEP_PATTERN;
//     use std::sync::Arc;
//
//     pub static CUR_SEP_PATTERN: Arc<&str> = Arc::new("lol");
// }

pub type Column = Vec<String>;

#[derive(Clone, Debug)]
pub struct Columns(Vec<Column>);


impl Display for Columns {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        let mut res = String::new();

        let nrows = self[0].len();
        let mut i = 0;
        while i < nrows {
            for col in self.deref() {
                assert!(i < col.len(), "column number mismatch");
                res.push_str(&format!("{} ", col[i]))
            }
            // remove last space
            res.pop();
            if i < nrows-1 {
                res.push('\n');
            }
            i+=1;
        }
        write!(f, "{}", res)
    }
}


impl DerefMut for Columns {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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


impl TryFrom<Vec<Vec<String>>> for Columns {
    type Error = anyhow::Error;
    
     fn try_from(value: Vec<Vec<String>>) -> Result<Self> {
       Ok(Columns(value))
    }
}
