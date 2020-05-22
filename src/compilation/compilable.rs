use super::Scope;

pub trait Compilable {
    fn compile(&self, scope: &Scope) -> String;

    fn compile_indented(&self, scope: &Scope, amount: usize) -> String {
        let indent = (0..(amount * 4)).map(|_| ' ').collect::<String>();
        let mut compiled = String::new();

        let sub_compiled = self.compile(scope);

        let lines: Vec<(usize, &str)> = sub_compiled.lines().enumerate().collect();

        for &(i, line) in &lines {
            compiled.push_str(&indent);
            compiled.push_str(line);

            if i + 1 <  lines.len() {
                compiled.push_str("\r\n");
            }
        }

        compiled
    }

    fn is_pure(&self) -> bool {
        false
    }
}

impl<T> Compilable for Vec<T> where T: Compilable {
    default fn compile(&self, scope: &Scope) -> String {
        let mut compiled = String::new();

        for (i, compilable) in self.iter().enumerate() {
            compiled.push_str(&compilable.compile(scope));

            if i + 1 < self.len() {
                compiled.push_str("\r\n")
            }
        }

        compiled
    }

    default fn is_pure(&self) -> bool {
        self.iter().all(|c| c.is_pure())
    }
}
