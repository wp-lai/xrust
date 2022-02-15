fn main() {
    if let Err(e) = httpie::get_args().and_then(httpie::http_request) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
