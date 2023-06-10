use std::fs::File;
use std::io::{self, Read};
fn main() {
    fn read_usr_from_file() -> Result<String, io::Error> {
        let usr_file_res = File::open("hello.txt");

        let mut usr_file = match usr_file_res {
            Ok(file) => file,
            Err(err) => return Err(err),
        };

        let mut usr = String::new();
        match usr_file.read_to_string(&mut usr) {
            Ok(_) => Ok(usr),
            Err(err) => Err(err),
        }
    }
}
