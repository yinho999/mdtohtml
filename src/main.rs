use std::{fs, io::Write};

use clap::Parser;
use maud::html;
use pulldown_cmark::{html, Event, OffsetIter, Options, Parser as pr};

/// simple converter from md to html
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Set the input md file
    #[clap(short, long, value_parser)]
    src: String,

    /// target html file
    #[clap(short, long, value_parser)]
    target: Option<String>,

    /// Print Event
    #[clap(short, long, value_parser)]
    event: Option<bool>,

    /// link for css file
    #[clap(short, long, value_parser)]
    css: Option<String>,
}

fn print_event(iter: &Vec<Event>) {
    for i in iter {
        println!("{:?}", i);
    }
}

fn create_html_str(s: &str, css: &Option<String>) -> String {
    html!(
        (maud::DOCTYPE)
        html{
            head{
                meta charset="UTF-8";
                meta http-equiv="X-UA-Compatible" content="IE=edge";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title{
                   "Md Converter"
                }
                @if let Some(s) = css{
                    link rel="stylesheet" href=(s);
                }
            }
            body{
                (maud::PreEscaped(s))
            }
    })
    .into_string()
}

fn main() {
    let args = Args::parse();

    let input_path = args.src.clone();

    // read the md file
    let input_file = fs::read_to_string(input_path).expect("Cannot locate the file");

    // Parse the md to the parser
    let ps = pr::new(&input_file);

    // create buffer
    let mut html_buffer = String::new();

    html::push_html(&mut html_buffer, ps.clone().into_iter());

    println!("{:?}", html_buffer);

    // Showing Events
    if let Some(true) = args.event {
        let events = &ps.into_iter().collect();
        print_event(events);
    }

    // Target file
    if let Some(target_file) = args.target {
        let html = create_html_str(&html_buffer, &args.css);
        let mut file = fs::File::create(&target_file).expect("Error occur creating files");
        let result = file.write_all(&html.into_bytes());
        match result {
            Ok(()) => println!("data write successfully"),
            Err(e) => println!("Cant write data, please do again, {}", e),
        }
    }
}
