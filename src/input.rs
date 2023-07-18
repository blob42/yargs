//DEBUG:
#![allow(dead_code)]

use std::fmt;
use regex::Regex;

const DEFAULT_SEP: &str = r"[\t]+";

type Column = Vec<String>;
type Columns = Vec<Column>;

/// split input text into columns based on separator character
/// returns a type representing a variable length array of strings (columns) ?
///
/// TODO:
///
///   error handling
///   accept &str and String
///
pub fn split_columns(text: &str, sep: &str) -> Result<Columns, fmt::Error>  {
    // read the first line stripping empty lines
    let lines: Vec<&str> = text.trim().lines().collect();
    eprintln!("lines: {:?}", lines);

    let re = Regex::new(sep).unwrap();

    // count number of columns 
    let n_col = match lines.first() {
        Some(line) => re.split(line).count(),
        None => return Err(std::fmt::Error)
    };

    // eprintln!("# columns: {n_col}");

    let mut columns = vec![Column::new(); n_col];

    for (i, line) in lines.iter().enumerate() {
        eprintln!("checking line {}", i);

        let new_n_col = re.split(line).count();

        if new_n_col != n_col {
            return Err(fmt::Error)
        }
        eprintln!("number of columns: {}", columns.len());

        for (c_idx, col) in re.split(line).enumerate() {
            columns[c_idx].push(col.to_string())
        }
    }

    eprintln!("{:?}", columns);

    Ok(columns)
}

#[test]
fn test_split_columns(){
    let coltext1 = r###"
file1.txt		title1
file2.pdf		title2
file3			title3
file with space	title 4
        "###;
    let columns = split_columns(coltext1, DEFAULT_SEP);

    // should have two columns
    assert_eq!(2, columns.clone().unwrap().len());

    assert_eq!(vec!["file1.txt",
                   "file2.pdf",
                    "file3",
                   "file with space"
            ], columns.unwrap()[0]);
}

// #[test]
fn test_re_split() {
    let text = "this is		two tabs";
    let re = Regex::new(r"[\t]+").unwrap();
    let fields: Vec<&str> = re.split(text).collect();
    eprintln!("{:?}", fields);
    assert!(false);
}
