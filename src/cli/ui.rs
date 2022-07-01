use crate::cli::term;
use anyhow::Result;
use clap::{ArgMatches, Command};
use rust_embed::RustEmbed;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

#[derive(RustEmbed)]
#[folder = "app/.svelte-kit/output/server/"]
struct Asset;

pub fn subcommand() -> Command<'static> {
    Command::new("ui").about("run the ui")
}

pub fn run(_matches: &ArgMatches) -> Result<()> {
    term::display_success("starting webui");

    let files = Asset::iter();

    for file in files {
        println!("Got: {}", file.to_string());
    }

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.js")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let msg = format!("serving {}", filename);
    term::display_success(&msg);
    let index_html = Asset::get(filename).unwrap();
    let contents = std::str::from_utf8(index_html.data.as_ref()).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
