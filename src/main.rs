use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file_res = File::open("hello.txt");

    let file = match file_res {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Error creating file: {:?}", err),
            },
            other_err => panic!("Error opening file: {}", other_err),
        },
    };
}
