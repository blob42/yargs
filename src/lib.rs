//DEBUG:
#![allow(dead_code)]

use regex::Regex;
use std::ops::Deref;
use anyhow::{anyhow, Result};

pub mod input;

pub const DEFAULT_SEP_PATTERN: &str = r"[\t]+";

type Column = Vec<String>;
// type Columns = Vec<Column>;

#[derive(Clone, Debug)]
pub struct Columns(Vec<Column>);


/// split input text into columns based on separator character
/// returns a type representing a variable length array of strings (columns) ?
///
/// TODO:
///
///   error handling
///   accept &str and String
///

pub struct InputText<'a> {
    raw: &'a str,

    sep: String,
}

impl<'a> InputText<'a> {

    pub fn new(raw: &'a str, sep: &str) -> Self {
       InputText {
           raw: raw.into(),
           sep: sep.into()
       } 
    }

    pub fn n_cols(&self) -> Result<usize> {
        // read the first line stripping empty lines
        let lines: Vec<&str> = self.raw.trim().lines().collect();
        // eprintln!("lines: {:?}", lines);

        let re = Regex::new(&self.sep).unwrap();

        // count number of columns
        match lines.first() {
            Some(line) => Ok(re.split(line).count()),
            None => return Err(anyhow!("no lines left")),
        }
    }
}

/// Return the number of columns given input text and a separator
pub fn n_columns(text: &str, sep: &str) -> Result<usize> {
    // read the first line stripping empty lines
    let lines: Vec<&str> = text.trim().lines().collect();
    // eprintln!("lines: {:?}", lines);

    let re = Regex::new(sep).unwrap();

    // count number of columns
    match lines.first() {
        Some(line) => Ok(re.split(line).count()),
        None => return Err(anyhow!("no lines left")),
    }
}

pub fn split_columns(text: &str, sep: &str) -> Result<Columns> {
    let re = Regex::new(sep).unwrap();

    // eprintln!("# columns: {n_col}");
    let lines: Vec<&str> = text.trim().lines().collect();

    let n_col = n_columns(text, sep)?;
    let mut columns = vec![Column::new(); n_col];

    for (n, line) in lines.iter().enumerate() {
        // eprintln!("checking line {}", i);

        let new_n_col = re.split(line).count();

        if new_n_col != n_col {
            return Err(anyhow!(
                "unmatched column: expected {n_col} got {new_n_col} on line {}",
                n + 1
            ));
        }
        // eprintln!("number of columns: {}", columns.len());

        for (c_idx, col) in re.split(line).enumerate() {
            columns[c_idx].push(col.to_string())
        }
    }

    eprintln!("{:?}", columns);

    Ok(Columns(columns))
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

#[cfg(test)]
mod tests {
    use crate::{Columns, split_columns, DEFAULT_SEP_PATTERN};
    use crate::Regex;
    use std::error::Error;

    type TestResult = Result<(), Box<dyn Error>>;

    #[test]
    fn test_split_columns_default_sep() -> TestResult {
        let coltext1 = "
file1.txt\t\ttitle1
file2.pdf\t\ttitle2
file3\t\t\ttitle3
file with space \textra
        ";
        let columns = split_columns(coltext1, DEFAULT_SEP_PATTERN)?;

        // should have two columns
        assert_eq!(2, columns.clone().len());

        assert_eq!(
            vec!["file1.txt", "file2.pdf", "file3", "file with space "],
            columns[0]
        );
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_wrong_ncol_default_sep() {
        let coltext1 = "
file1.txt\t\ttitle1
file2.pdf\t\ttitle2
file3\t\t\ttitle3
file with space\ttitle 4\textra
        ";
        split_columns(coltext1, DEFAULT_SEP_PATTERN).unwrap();
    }

    // #[test]
    fn test_re_split() {
        let text = "this is		two tabs";
        let re = Regex::new(r"[\t]+").unwrap();
        let fields: Vec<&str> = re.split(text).collect();
        eprintln!("{:?}", fields);
        assert!(false);
    }

    #[test]
    fn test_columns_from_str() {
        let res: Columns = "first column\tsecond column\t\tthird column"
            .try_into()
            .unwrap();
        assert_eq!(res.len(), 3);
    }

    #[test]
    fn test_input_text(){
        // it =
    }
}
