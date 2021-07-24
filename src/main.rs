extern crate ureq;
extern crate select;
extern crate colored;
extern crate textwrap;
extern crate getopts;

mod article;
mod homepage;
mod phoronix_cli;
mod phoronix_gui;

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let mut opts = getopts::Options::new();
    opts.optflag("c", "console", "Print on console");
    opts.optflag("n", "no-color", "Print without colors");
    opts.optflag("h", "help", "Show usage");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}",f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("c") {
        let mut colored = true;
        if matches.opt_present("n") {
            colored = false;
        }
        phoronix_cli::print(colored)
    } else {
        phoronix_gui::phoronix_gui()
    }
}