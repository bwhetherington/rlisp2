extern crate rlisp_core;

use rlisp_core::{
    expression::Expression,
    intrinsics::functions::_import,
    prelude::*,
    util::{clear_color, print_err, set_green, set_red},
};
use std::io::{prelude::*, stdin, stdout};

fn main() {
    let mut ctx = Context::new();
    load(&mut ctx);

    // Load stdlib
    let res = _import(&[Expression::Str("rlisp-lib/loader.rl".into())], &mut ctx);
    if let Expression::Exception(ex) = res {
        print_err(&ex);
        return;
    }

    let mut line = String::new();
    loop {
        let prompt = ctx
            .get("PROMPT")
            .and_then(|p| match p {
                Expression::Str(s) => Some(s.to_string()),
                _ => None,
            })
            .unwrap_or_else(|| "rlisp> ".to_string());
        set_green();
        print!("{}", prompt);
        clear_color();
        stdout().flush().expect("failed to flush stdout");
        stdin().read_line(&mut line).expect("failed to read line");
        {
            let mut parser = Parser::new(line.chars());
            parser.parse_expr().map(|expr| {
                let result = expr.eval(&mut ctx);
                match result {
                    Expression::Exception(ex) => {
                        print_err(&ex);
                    }
                    ref res if !res.is_nil() => {
                        println!("{}", res);
                    }
                    _ => {}
                }
            });
        }
        line.clear();
    }
}
