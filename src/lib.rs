//DEBUG:
#![allow(dead_code)]

use regex::Regex;
use std::result;
use std::ops::Index;
use std::slice::SliceIndex;
use crate::parsing::DEFAULT_SEP_PATTERN;


pub mod parsing {

    pub const DEFAULT_SEP_PATTERN: &str = r"[\t]+";

}


type Column = Vec<String>;
// type Columns = Vec<Column>;

#[derive(Clone, Debug)]
pub struct Columns(Vec<Column>);

type Result<T> = result::Result<T, String>;


/// split input text into columns based on separator character
/// returns a type representing a variable length array of strings (columns) ?
///
/// TODO:
///
///   error handling
///   accept &str and String
///
pub fn split_columns(text: &str, sep: &str) -> Result<Columns>  {
    // read the first line stripping empty lines
    let lines: Vec<&str> = text.trim().lines().collect();
    // eprintln!("lines: {:?}", lines);

    let re = Regex::new(sep).unwrap();

    // count number of columns 
    let n_col = match lines.first() {
        Some(line) => re.split(line).count(),
        None => return Err(format!("no lines left"))
    };

    // eprintln!("# columns: {n_col}");

    let mut columns = vec![Column::new(); n_col];

    for (n, line) in lines.iter().enumerate() {
        // eprintln!("checking line {}", i);

        let new_n_col = re.split(line).count();

        if new_n_col != n_col {
            return Err(
                format!("unmatched column: expected {n_col} got {new_n_col} on line {}", n+1)
                )
        }
        // eprintln!("number of columns: {}", columns.len());

        for (c_idx, col) in re.split(line).enumerate() {
            columns[c_idx].push(col.to_string())
        }
    }

    eprintln!("{:?}", columns);

    Ok(Columns(columns))
}

impl Columns {

    //NOTE: is there a way to auto implement what's implemented in the wrapped type self.0 ?
    fn len(&self) -> usize {
        self.0.len()
    }
}

// build Columns from &str
impl TryFrom<&str> for Columns {
    type Error = String;

    fn try_from(value: &str) -> Result<Self> {
        split_columns(value, DEFAULT_SEP_PATTERN)
    }
}

// impl Index to allow indexing in our wrapped Vector
impl <I> Index<I> for Columns
where
    I: SliceIndex<[Column]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.0.index(index)
    }
}

#[test]
fn test_split_columns_default_sep(){
    let coltext1 = "
file1.txt\t\ttitle1
file2.pdf\t\ttitle2
file3\t\t\ttitle3
file with space \textra
        ";
    let columns = split_columns(coltext1, DEFAULT_SEP_PATTERN);

    // should have two columns
    assert_eq!(2, columns.clone().unwrap().len());

    assert_eq!(vec!["file1.txt",
                    "file2.pdf",
                    "file3",
                    "file with space "
            ], columns.unwrap()[0]);
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
    let res: Columns = "first column\tsecond column\t\tthird column".try_into().unwrap();
    assert_eq!(res.len(), 3);
}
