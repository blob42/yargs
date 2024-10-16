use super::{Columns, Column};
use anyhow::{anyhow, Result};
use regex::Regex;

/// split input text into columns based on separator character
/// returns a type representing a variable length array of strings (columns) ?
///
/// TODO:
///
///   error handling
///   accept &str and String
///

#[derive(Debug)]
pub struct InputText<'a> {
    pub raw: &'a str,

    pub sep: String,
}

impl<'a> InputText<'a> {

    pub fn new(raw: &'a impl AsRef<str>, sep: &str) -> Self {
       InputText {
           raw: raw.as_ref(),
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
            None => Ok(0)
        }
    }

    pub fn len(self) -> usize {
        self.raw.len()
    }

    pub fn is_empty(self) -> bool {
        self.raw.is_empty()
    }

    pub fn split(&self) -> Result<Columns> {
        split_columns(self.raw, &self.sep)
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
        None => Err(anyhow!("no lines left")),
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

    // eprintln!("{:?}", columns);

    Ok(Columns(columns))
}

// TODO!: use \s+ as default separator and create new tests for \t
#[cfg(test)]
mod tests {
    use crate::DEFAULT_SEP_PATTERN;

    use super::*;
    use std::{error::Error, fmt};


    type TestResult = Result<(), Box<dyn Error>>;

    #[test]
fn test_split_columns_tab() -> TestResult {
        let coltext1 = "
file1.txt\t\ttitle1
file2.pdf\t\ttitle2
file3\t\t\ttitle3
file with space \textra
        ";
        let columns = split_columns(coltext1, "[\t]+")?;

        // should have two columns
        assert_eq!(2, columns.clone().len());

        assert_eq!(
            vec!["file1.txt", "file2.pdf", "file3", "file with space "],
            columns[0]
        );
        Ok(())
    }

    #[test]
    fn test_split_columns_default_sep() -> TestResult {
        let coltext1 = "
file1.txt title1
file2.pdf   title2
file3       title3
some_file title4
        ";
        let columns = split_columns(coltext1, DEFAULT_SEP_PATTERN)?;

        // should have two columns
        assert_eq!(2, columns.clone().len());

        assert_eq!(
            vec!["file1.txt", "file2.pdf", "file3", "some_file"],
            columns[0]
        );
        assert_eq!(
            vec!["title1", "title2", "title3", "title4"],
            columns[1]
        );
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_wrong_ncol_default_sep() {
        let coltext1 = "
file1.txt  title1
file2.pdf   title2
file3   title3
extra space field
        ";
        split_columns(coltext1, DEFAULT_SEP_PATTERN).unwrap();
    }

    // #[test]
    // fn test_re_split() {
    //     let text = "this is		two tabs";
    //     let re = Regex::new(r"[\t]+").unwrap();
    //     let fields: Vec<&str> = re.split(text).collect();
    //     eprintln!("{:?}", fields);
    //     assert!(false);
    // }

    #[test]
    fn test_columns_from_tab() {
        let res = split_columns("first column\tsecond column\t\tthird column", "[\t]+").unwrap();
        assert_eq!(res.len(), 3);
    }

    #[test]
    fn into_columns() -> TestResult {
        let t: Vec<Vec<String>> = vec![vec!["1".to_string(), "2".to_string()]];
        let _cols: Columns = t.try_into()?;
        Ok(())
    }

    #[test]
    fn test_columns_from_default_sep() {
        let res: Columns = "first second third"
            .try_into()
            .unwrap();
        assert_eq!(res.len(), 3);
    }

    #[test]
    fn test_input_text(){
        // it =
    }
}
