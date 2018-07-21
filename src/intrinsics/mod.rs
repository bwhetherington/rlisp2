use context::Context;
use expression::Expression;

mod functions;
mod macros;

fn define_intrinsic(ctx: &mut Context, ident: impl ToString, f: fn(&[Expression]) -> Expression) {
    ctx.insert(ident.to_string(), Expression::Intrinsic(f));
}

fn define_macro(
    ctx: &mut Context,
    ident: impl ToString,
    f: fn(&Expression, &mut Context) -> Expression,
) {
    ctx.insert(ident.to_string(), Expression::Macro(f));
}

pub fn load_macros(ctx: &mut Context) {
    define_macro(ctx, "define", macros::_define);
    define_macro(ctx, "lambda", macros::_lambda);
    define_macro(ctx, "env", macros::_env);
    define_macro(ctx, "if", macros::_if);
    define_macro(ctx, "cond", macros::_cond);
}

pub fn load_functions(ctx: &mut Context) {
    // Mathematical functions
    define_intrinsic(ctx, "+", functions::_add);
    define_intrinsic(ctx, "-", functions::_sub);
    define_intrinsic(ctx, "*", functions::_mul);
    define_intrinsic(ctx, "/", functions::_div);
    define_intrinsic(ctx, "%", functions::_rem);
    define_intrinsic(ctx, "eq?", functions::_eq);
    define_intrinsic(ctx, "=", functions::_eq);
    define_intrinsic(ctx, ">", functions::_gt);
    define_intrinsic(ctx, ">=", functions::_gte);
    define_intrinsic(ctx, "<", functions::_lt);
    define_intrinsic(ctx, "<=", functions::_lte);

    // Lists
    define_intrinsic(ctx, "cons", functions::_cons);
    define_intrinsic(ctx, ":", functions::_cons);
    define_intrinsic(ctx, "head", functions::_head);
    define_intrinsic(ctx, "tail", functions::_tail);

    define_intrinsic(ctx, "exit", functions::_exit);
    define_intrinsic(ctx, "begin", functions::_begin);
    define_intrinsic(ctx, "println", functions::_println);

    define_intrinsic(ctx, "++", functions::_append);
    define_intrinsic(ctx, "append", functions::_append);
    define_intrinsic(ctx, "empty?", functions::_empty);
}