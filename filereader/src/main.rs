use std::{env, fs::File, io::{BufRead, BufReader}};


fn main() {
    let user_input: Vec<String> = env::args().collect();

    let file = File::open(&user_input[1]);
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("file not found: {}", e)
                },
                _ => {
                    panic!("failed to open the file: {}", e)
                }
            }
        }
    };

    println!("read the file successfully: {:?}", file);

    let reader = BufReader::new(file);
    for line in reader.lines(){
        match line {
            Ok(l) => println!("{}", l),
            Err(e) => panic!("failed to read: {}", e)
        }
    }
}
