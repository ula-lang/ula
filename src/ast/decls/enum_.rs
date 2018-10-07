use std::fmt;

use ast::{Expr, Node};
use compilation::Compilable;

pub struct EnumDecl {
    flags: (bool,),
    ident: String,
    fields: Vec<(String, Option<Expr>)>
}

impl EnumDecl {
    pub fn new<SI, SF, EF>(flags: (bool,), ident: SI, fields: Vec<(SF, Option<EF>)>) -> Self where SI: ToString, SF: ToString, EF: Into<Expr> {
        Self {
            flags,
            ident: ident.to_string(),
            fields: fields.into_iter().map(|(i, m_e)| (i.to_string(), m_e.map(|e| e.into()))).collect()
        }
    }

    fn gen_value(&self, field_ident: &str) -> String {
        format!(r#""|_:{}.{}:_|""#, self.ident, field_ident)
    }
}

impl Compilable for EnumDecl {
    fn compile(&self) -> String {
        let mut compiled = String::new();

        compiled.push_str(&format!("--[[ Begin enum: {:?} ]]--\r\n", self.ident));

        // Enum is local
        if self.flags.0 {
            compiled.push_str("local ");
        }

        compiled.push_str(&format!("{} = {{", self.ident));

        for (i, &(ref ident, ref def)) in self.fields.iter().enumerate() {
            compiled.push_str(&format!("\r\n    {} = ", ident));

            match def {
                &Some(ref def) => {
                    compiled.push_str(&def.compile());
                }

                &None => {
                    compiled.push_str(&self.gen_value(ident));
                }
            }

            if i + 1 < self.fields.len() {
                compiled.push(',');
            }

            compiled.push_str("\r\n");
        }

        compiled.push('}');

        compiled.push_str(&format!("\r\n--[[ End enum: {:?} ]]--\r\n", self.ident));

        compiled
    }
}

impl Into<Node> for EnumDecl {
    fn into(self) -> Node {
        Node::EnumDecl(self)
    }
}

impl fmt::Debug for EnumDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Enum(")?;

        for (i, &(ref ident, ref def)) in self.fields.iter().enumerate() {
            write!(f, "{}", ident)?;

            match def {
                &Some(ref expr) => {
                    write!(f, " = {:?}", expr)?;
                }

                &None => ()
            }

            if i + 1 < self.fields.len() {
                write!(f, ", ")?;
            }
        }

        write!(f, ")")
    }
}