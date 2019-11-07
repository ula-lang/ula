use crate::lexer::Lexer;
use crate::parser::Parser;

pub use self::compilable::Compilable;
pub use self::scope::Scope;

mod compilable;
mod scope;

pub fn compile_str(input: &str, scope: Option<Scope>) -> Result<String, Vec<String>> {
    let scope = scope.unwrap_or_default();

    match Lexer::lex(input) {
        Ok(tokens) => {
            let tree = Parser::new(tokens).parse()?;

            let code = tree.compile(&scope);

            Ok(code)
        }

        Err(error) => Err(vec![error])
    }
}
