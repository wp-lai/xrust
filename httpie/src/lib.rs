use clap::{arg, App, AppSettings};
use colored::Colorize;
use std::collections::HashMap;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

#[derive(Debug)]
pub enum Request {
    Get(Get),
    Post(Post),
}

#[derive(Debug)]
pub struct Get {
    url: String,
}

#[derive(Debug)]
pub struct Post {
    url: String,
    body: Vec<String>,
}

pub fn get_args() -> Result<Request, Box<dyn std::error::Error>> {
    let matches = App::new("httpie")
        .about("Rust version of httpie")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(App::new("GET").arg(arg!(<URL>)))
        .subcommand(App::new("POST").arg(arg!(<URL>)).arg(arg!([body]...)))
        .get_matches();

    let request = match matches.subcommand() {
        Some(("GET", sub_matches)) => Request::Get(Get {
            url: sub_matches.value_of("URL").unwrap().into(),
        }),
        Some(("POST", sub_matches)) => Request::Post(Post {
            url: sub_matches.value_of("URL").unwrap().into(),
            body: sub_matches
                .values_of("body")
                .unwrap()
                .map(|s| s.to_string())
                .collect(),
        }),
        _ => unreachable!(),
    };

    Ok(request)
}

fn body_to_map(body: Vec<String>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for kv in body {
        let mut split = kv.split('=');
        let err = format!("Failed to parse {}", kv);
        let k = split.next().ok_or(&err).unwrap().to_string();
        let v = split.next().ok_or(&err).unwrap().to_string();
        map.insert(k, v);
    }
    map
}

pub fn http_request(req: Request) -> Result<(), Box<dyn std::error::Error>> {
    match req {
        Request::Get(get) => http_get_requst(get.url),
        Request::Post(post) => http_post_request(post.url, post.body),
    }
    Ok(())
}

fn http_get_requst(url: String) {
    let res = reqwest::blocking::get(url).unwrap();
    print_resp(res);
}

fn http_post_request(url: String, body: Vec<String>) {
    let map = body_to_map(body);

    let client = reqwest::blocking::Client::new();
    let res = client.post(url).json(&map).send().unwrap();
    print_resp(res);
}

fn print_resp(res: reqwest::blocking::Response) {
    println!("{:?}, {}", res.version(), res.status().to_string().cyan());
    for (name, value) in res.headers() {
        println!("{}: {:?}", name.to_string().cyan(), value);
    }
    println!("");

    let content_type = res
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let body = res.text().unwrap();

    if content_type.contains("text/html") {
        print_syntect(&body, "html");
    } else if content_type.contains("application/json") {
        print_syntect(&body, "json");
    }
}

fn print_syntect(s: &str, ext: &str) {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}
