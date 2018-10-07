pub trait Compilable {
    fn compile(&self) -> String;

    fn compile_indented(&self, amount: usize) -> String {
        let indent = (0..(amount * 4)).map(|_| ' ').collect::<String>();
        let mut compiled = String::new();

        let sub_compiled = self.compile();

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
}

impl<T: Compilable> Compilable for Vec<T> {
    default fn compile(&self) -> String {
        let mut compiled = String::new();

        for (i, compilable) in self.iter().enumerate() {
            compiled.push_str(&compilable.compile());

            if i + 1 < self.len() {
                compiled.push_str("\r\n")
            }
        }

        compiled
    }
}