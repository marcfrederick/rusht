use crate::parse::Ast;

fn interpret(ast: Ast) {
    match ast {
        Ast::Atom(_) => {}
        Ast::List(_) => {}
    }
}
