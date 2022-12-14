use std::fs;
use std::path;
use clap::Parser;
use reqwest;

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    /// file to load data from
    #[clap(value_parser, default_value_t = String::from("input.txt"))]
    filename: String,
    /// force download
    #[clap(short, long, action , default_value_t = false)]
    force: bool,
}

pub fn get_input(year: i64, day: i64) -> String {

    let args = Args::parse();

    if args.filename == "input.txt" {
        let path = path::Path::new(&args.filename);
        if args.force || !path.exists() {
            let input = retrieve_input_from_server(year, day);
            fs::write(args.filename, &input).expect("Couldn't write file.");
            return input;
        }
    }

    get_input_from_file(&args.filename)
}

pub fn get_input_from_file(filename: &str) -> String {
    println!("Reading from {}", filename);
    let contents = fs::read_to_string(filename).expect("Couldn't read file");
    contents
}

pub fn retrieve_input_from_server(year: i64, day: i64) -> String {

    println!("Reading from AOC");

    let url = format!("https://adventofcode.com/{y}/day/{d}/input",
                      y=year, d=day);

    let session = fs::read_to_string("session_cookie.secret")
        .expect("Couldn't read secret");
    let session_cookie = format!("session={}", session.trim());

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(url)
        .header(reqwest::header::COOKIE, session_cookie)
        .send();

    let body = response.unwrap().text().unwrap();

    body
}
