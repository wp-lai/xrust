use clap::{arg, App};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let matches = App::new("cat")
        .about("Rust version cat")
        .arg(arg!(<file> ...))
        .arg(arg!(number: -n "Number the output lines"))
        .get_matches();

    let files = matches.values_of("file").unwrap();
    let number_lines: bool = matches.is_present("number");

    for filename in files {
        match File::open(filename) {
            Ok(file) => {
                for (num, line_result) in BufReader::new(file).lines().enumerate() {
                    let line = line_result.unwrap();
                    if number_lines {
                        println!("{:>6}\t{}", num + 1, line)
                    } else {
                        println!("{}", line)
                    }
                }
            }
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
        }
    }
}
