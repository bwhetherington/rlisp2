use clap::{App, Arg, ArgMatches};
use rlisp_core::{
    prelude::*,
    intrinsics::functions::_import,
    util::print_err
};
use repl::run_repl;

fn create_app<'a>() -> ArgMatches<'a> {
    App::new("RLisp")
        .version("1.0")
        .author("Benjamin Hetherington <b.w.hetherington@gmail.com>")
        .about("A simple Lisp interpreter made in Rust, loosely based on Scheme")
        .arg(Arg::with_name("lib-loc")
            .short("l")
            .long("lib-loc")
            .value_name("LIB_LOC")
            .help("Sets the location to load the standard library from")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to interpret")
            .required(false)
            .index(1))
        .arg(Arg::with_name("interactive")
            .short("i")
            .long("interactive")
            .takes_value(false)
            .help("Determines whether or not to start an interactive REPL session after loading the specified input")
            .required(false))
        .get_matches()
}

pub fn run() {
    let matches = create_app();

    let lib_loc = matches.value_of("lib-loc").unwrap_or_else(|| "rlisp-lib/loader.rl");

    let mut ctx = init_context();
    let res = _import(&[Expression::Str(lib_loc.into())], &mut ctx);
    if let Expression::Exception(ex) = res {
        print_err(&ex);
        return;
    }

    match matches.value_of("INPUT") {
        Some(input) => {
            // Load input file
            let res = _import(&[Expression::Str(input.into())], &mut ctx);
            if let Expression::Exception(ex) = res {
                print_err(&ex);
                return;
            }

            if matches.is_present("interactive") {
                run_repl(&mut ctx);
            }
        }
        None => {
            run_repl(&mut ctx);
        }
    }
}