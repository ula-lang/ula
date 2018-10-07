mod compilable;
pub use self::compilable::Compilable;

use lexer::Lexer;
use parser::Parser;

pub fn compile_str(input: &str) -> Result<String, Vec<String>> {
    match Lexer::lex(input) {
        Ok(tokens) => {
            let tree = Parser::new(tokens).parse()?;

            Ok(tree.compile())
        }

        Err(error) => Err(vec![error])
    }
}