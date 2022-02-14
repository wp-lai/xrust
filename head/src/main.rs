use clap::{arg, App};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let matches = App::new("head")
        .about("Rust version head")
        .arg(arg!(-c --bytes [bytes] "Print bytes of each of the specified files."))
        .arg(
            arg!(-n --lines [count] "Print count lines of each of the specified files.")
                .default_value("10"),
        )
        .arg(arg!(<file>...))
        .get_matches();

    let files: Vec<&str> = matches.values_of("file").unwrap().collect();
    let num_files = files.len();

    let lines: usize = matches.value_of("lines").unwrap().parse().unwrap();

    for filename in files {
        if num_files > 1 {
            println!("==> {} <==", filename);
        }

        if let Some(bytes) = matches.value_of("bytes") {
            let bytes: usize = bytes.parse().unwrap();
            let f = File::open(filename).unwrap();
            let mut buffer = vec![0; bytes];
            let mut handle = f.take(bytes as u64);
            let bytes_read = handle.read(&mut buffer).unwrap();
            print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
        } else {
            let f = File::open(filename).unwrap();
            let lines_iter = BufReader::new(f).lines().take(lines).map(|l| l.unwrap());
            for line in lines_iter {
                println!("{}", line);
            }
        }
    }
}
