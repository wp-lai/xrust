use clap::{arg, App};

fn main() {
    let matches = App::new("echo")
        .about("Rust version echo")
        .arg(arg!(<text> ...))
        .get_matches();

    let vals: Vec<&str> = matches.values_of("text").unwrap().collect();
    println!("{}", vals.join(" "));
}
