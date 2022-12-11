
type Column = Vec<String>;
type Columns = Vec<Column>;

// split input text into columns based on separator character
// returns a type representing a variable length array of strings (columns) ?
pub fn split_columns(text: &str, sep: char) -> Option<Columns>  {

    Some(Columns::new())
}

#[test]
fn test_split_columns(){
    let coltext1 = r###"
file1.txt       title1
file2.pdf       title2
file3           title3
        "###;
    let columns = split_columns(coltext1, '\t');
    println!("columns:\n{:?}", columns);
}
