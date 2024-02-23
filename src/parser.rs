fn scan_textfile(filename: &str) -> String {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");
    contents
}