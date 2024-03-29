extern crate clap;
use clap::{App, Arg};

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::string::String;

// This macro is stolen and prevents println! from panicing when pipes are broken
// https://www.reddit.com/r/rust/comments/60erzc/hey_rustaceans_got_an_easy_question_ask_here/df8swpz/
// println!() calls print!() internally, so that's the only macro we have to override
macro_rules! print (
    // The extra scope is necessary so we don't leak imports
    ($($arg:tt)*) => ({
        // The `write!()` macro is written so it can use `std::io::Write`
        // or `std::fmt::Write`, this import sets which to use
        use std::io::{self, Write};
        if let Err(e) = write!(io::stdout(), $($arg)*) {
            // Optionally write the error to stderr
            ::std::process::exit(0);
        }
    })
);


fn main() -> io::Result<()> {
    //TODO: Add a start end specifier
    let matches = App::new("transpose")
        .version("0.1.0")
        .author("Seth Stadick <sstadick@gmail.com>")
        .about("Transpose a tabular file")
        .arg(
            Arg::with_name("INPUT")
                .help("The input file to transpose. - if STDIN")
                .required(true)
                .index(1)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("seperator")
                .short("s")
                .long("sep")
                .required(false)
                .takes_value(true)
                .default_value("\t")
                .help("Seperator character."),
        )
        .get_matches();

    let seperator = matches.value_of("seperator").unwrap();

    match matches.value_of("INPUT") {
        Some("-") => {
            let stdin = io::stdin();
            let f = stdin.lock();
            parse_file(f, seperator).unwrap();
        }
        Some(file_path) => {
            let f = File::open(file_path)?;
            let f = BufReader::new(f);
            parse_file(f, seperator).unwrap();
        }
        _ => {}
    }

    Ok(())
}

fn parse_file<T>(handle: T, sep: &str) -> io::Result<()>
where
    T: BufRead,
{
    let mut max_row = 0;
    let mut table: Vec<Vec<&str>> = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    for line in handle.lines() {
        // Doing it this way to later allow start and stop line specs
        let mut line = line.unwrap();
        line.truncate(line.trim_end().len());
        lines.push(line);
    }
    let max_col = lines.len();
    for line in lines.iter() {
        let vals: Vec<&str> = line.split(sep).collect();
        if vals.len() > max_row {
            max_row = vals.len()
        }
        table.push(vals);
    }

    for i in 0..max_row {
        let new_row = (0..max_col)
            .map(|j| if i < table[j].len() { table[j][i] } else { "" })
            .collect::<Vec<&str>>()
            .join(sep);
        println!("{}", new_row);
    }
    Ok(())
}