use anyhow::Result;
use std::io::{BufReader, self, Read};


// this will read and validate input from stdin
// TODO: make as iterator, avoid loading all stdin to memroy
pub fn read_stdin() -> Result<Box<String>> {
    let mut r = BufReader::new(io::stdin());
    let mut buf = Box::<String>::default();
    r.read_to_string(&mut buf)?;
    Ok(buf)
}
//
//NOTE: need deeper understanding of difference with prev implementation
// pub fn read_stdin() -> Result<Box<dyn BufRead>> {
//     Ok(Box::new(BufReader::new(io::stdin())))
// }




#[cfg(test)]
mod tests {
    // #[test]
    // fn read_stdin() {
    //     panic!()
    // }
}
