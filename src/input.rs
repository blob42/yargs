use std::io;

type Column = Vec<String>;
type Columns = Vec<Column>;

// split input text into columns based on separator character
// returns a type representing a variable length array of strings (columns) ?
pub fn split_columns(text: &str, sep: char) -> Result<Columns, io::Error>  {
    // read the first line stripping empty lines
    let lines: Vec<&str> = text.trim().lines().collect();

    // count number of columns
    let n_col = lines.iter().next().unwrap();

    eprintln!("{:?}", lines);

    // detect number of columns
    Ok(Columns::new())
}

#[test]
fn test_split_columns(){
    let coltext1 = r###"
file1.txt       title1
file2.pdf       title2
file3           title3
        "###;
    let columns = split_columns(coltext1, '\t');

    // should have two columns
    assert_eq!(columns.unwrap().len(), 2);


    // println!("columns:\n{:?}", columns);
}
