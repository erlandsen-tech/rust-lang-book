use std::io::ErrorKind;
use std::fs::File;
fn main() {
    //panic!("crash and burn!");
    let vec = vec![1,2,3];
    let f = File::open("test.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("something horrible happened while opening the file \n
                             {:?}", error),
    };

    let g = File::open("does_not_exist.txt");
    //nested error
    let g = match g {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("but_this_will.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Unable to create file \n
                                 {:?}", e),
            },
            other_error => panic!("Problem opening the file {:?}", other_error),
        },
    };

}
