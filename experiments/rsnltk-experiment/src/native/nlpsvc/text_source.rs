
extern crate getopts;
use getopts::Options;
use std::env;
use std::process;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

struct AppConfig {
text_file: Option<String>,
}

impl AppConfig {
    fn new() -> AppConfig {
        AppConfig {
            text_file: None,
        }
    }
}

fn configure() -> AppConfig {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this message and exit");
    opts.optopt("f", "file", "match text from file", "NAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}",f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&args[0], &opts);
    }

    let mut cfg: AppConfig = AppConfig::new();
    cfg.text_file = matches.opt_str("f");

    cfg
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("\nUsage: {} [options]", program);
    print!("{}", opts.usage(&brief));
    println!("\nIf no file is given, input will be read from stdin.");
    process::exit(1);
}


struct TextSource {
    text: String,
}

impl TextSource {
    pub fn new(cfg: &AppConfig) -> TextSource {
        // Get the text to match against (from file or stdin)
        let mut txt = String::new();
        match cfg.text_file {
            None => {
                let stdin = io::stdin();
                stdin.lock().read_to_string(&mut txt).unwrap();
            },
            Some(ref fname) => {
                let fpath = Path::new(&fname);
                let mut f = File::open(fpath).expect("Could not open file");
                f.read_to_string(&mut txt).expect("Could not read file");
            }
        }

        TextSource { text: txt }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}
