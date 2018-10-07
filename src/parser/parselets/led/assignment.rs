use ast::Expr;
use lexer::{Op, Token};
use parser::parselets::Led;
use parser::Parser;
use ast::exprs::{Assignment, BAnd, BOr, Div, LShift, Mod, Mul, RShift, Sub, Sum, Xor};

pub struct AssignmentParselet;

impl Led for AssignmentParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        let mut rhs = parser.parse_expr(Some(self.get_precedence() - 1))?;

        match token {
            Token::Op(Op::Assign) => (),

            Token::Op(Op::AddAssign) => rhs = Sum::new(lhs.clone(), rhs).into(),

            Token::Op(Op::SubAssign) => rhs = Sub::new(lhs.clone(), rhs).into(),

            Token::Op(Op::MulAssign) => rhs = Mul::new(lhs.clone(), rhs).into(),

            Token::Op(Op::DivAssign) => rhs = Div::new(lhs.clone(), rhs).into(),

            Token::Op(Op::ModAssign) => rhs = Mod::new(lhs.clone(), rhs).into(),

            Token::Op(Op::BAndAssign) => rhs = BAnd::new(lhs.clone(), rhs).into(),

            Token::Op(Op::BOrAssign) => rhs = BOr::new(lhs.clone(), rhs).into(),

            Token::Op(Op::XorAssign) => rhs = Xor::new(lhs.clone(), rhs).into(),

            Token::Op(Op::LShiftAssign) => rhs = LShift::new(lhs.clone(), rhs).into(),

            Token::Op(Op::RShiftAssign) => rhs = RShift::new(lhs.clone(), rhs).into(),

            _ => unreachable!()
        }

        Ok(Assignment::new(lhs, rhs).into())
    }

    fn get_precedence(&self) -> i8 {
        0
    }
}