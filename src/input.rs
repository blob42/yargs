//DEBUG:
#![allow(dead_code)]

use std::fmt;

type Column = Vec<String>;
type Columns = Vec<Column>;

/// split input text into columns based on separator character
/// returns a type representing a variable length array of strings (columns) ?
/// TODO:
///
///   accept &str and String
///   error handling
pub fn split_columns(text: &str, sep: char) -> Result<Columns, fmt::Error>  {
    // read the first line stripping empty lines
    let lines: Vec<&str> = text.trim().lines().collect();
    eprintln!("lines: {:?}", lines);

    // count number of columns 
    let n_col = match lines.first() {
        Some(line) => line.split(sep).count(),
        None => return Err(std::fmt::Error)
    };
    // eprintln!("first line: {:?}", lines.first().unwrap());
    // eprintln!("# columns: {n_col}");

    let mut columns = vec![Column::new(); n_col];
    for (_l_idx, line) in lines.iter().enumerate() {

        let new_n_col = line.split(sep).count();

        // HACK: I should handle repeating separators with a glob or regex library
        // TIP: usek 
        if new_n_col != n_col {
            return Err(std::fmt::Error)
        }
        eprintln!("number of columns: {}", columns.len());
        for (c_idx, col) in line.split(sep).enumerate() {
            columns[c_idx].push(col.to_string())
        }
    }

    eprintln!("{:?}", columns);

    // let n_col = *lines.first().unwrap();

    // detect number of columns
    Ok(Columns::new())
}

#[test]
fn test_split_columns(){
    let coltext1 = r###"
file1.txt		title1
file2.pdf		title2
file3			title3
        "###;
    let columns = split_columns(coltext1, '\t');

    // should have two columns
    assert_eq!(2, columns.unwrap().len());

    // println!("columns:\n{:?}", columns);
}
