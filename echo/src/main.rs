use clap::{App, Arg};

fn main() {
    let matches = App::new("echo")
        .about("Rust version echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    let vals: Vec<&str> = matches.values_of("text").unwrap().collect();
    println!("{}", vals.join(" "));
}
