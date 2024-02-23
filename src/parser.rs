/// An entry point which every macro and scan function should return. 
/// An entry point should be able to convert to a node in the DAG.
struct EntryPoint{
    name: String,
    depends_on: Vec<String>,
    generates: Vec<String>,
}

fn scan_textfile(filename: &str) -> Vec<EntryPoint> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(filename).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");
    contents
}