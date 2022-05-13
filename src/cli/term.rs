use crossterm::style::Stylize;
use crossterm::{
    execute,
    style::{self, Color, PrintStyledContent},
};
use std::io::{self, Write};

pub fn display_error(err_msg: &str) {
    let mut stdout = io::stderr();
    let message = style::style(format!("Error: {}\n", err_msg)).with(Color::Red);
    let _ = execute!(stdout, PrintStyledContent(message),);
    let _ = stdout.flush();
}

pub fn display_success(msg: &str) {
    let mut stdout = io::stdout();
    let message = style::style(format!("{}\n", msg)).with(Color::Green);
    let _ = execute!(stdout, PrintStyledContent(message),);
    let _ = stdout.flush();
}
