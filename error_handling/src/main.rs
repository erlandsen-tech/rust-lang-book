use std::fs::File;
fn main() {
    //panic!("crash and burn!");
    let vec = vec![1,2,3];
    let f = File::open("test.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("something horrible happened while opening the file"),
    };
}
