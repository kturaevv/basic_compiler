use crate::lexer::Token;

#[derive(Default)]
pub struct Emitter {
    full_path: String,
    header: String,
    code: String,
}

impl Emitter {
    pub fn new(full_path: &str) -> Emitter {
        Emitter {
            full_path: full_path.to_string(),
            ..Default::default()
        }
    }

    pub fn process(&mut self, ast: Vec<Token>) {
        self.emit_header("#include <stdio.h>");
        self.emit_header("int main(void){");

        ast.iter().for_each(|token| println!("{:?}", token));

        self.emit_line("return 0;");
        self.emit_line("}");
    }

    pub fn emit(&mut self, code: &str) {
        self.code.push_str(code);
    }

    pub fn emit_line(&mut self, code: &str) {
        self.emit(code);
        self.code.push('\n');
    }

    pub fn emit_header(&mut self, header: &str) {
        self.header.push_str(header);
        self.header.push('\n');
    }

    pub fn write_to_file(&self) {
        let content = [self.header.as_bytes(), self.code.as_bytes()].concat();
        std::fs::write(self.full_path.as_str(), content).expect("Couldnt write to file");
    }
}
