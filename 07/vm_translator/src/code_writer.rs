use crate::parser::Command;

// 表7-2 CodeWriterモジュール
pub struct CodeWriter {
}

impl CodeWriter {
    pub fn code(&self, command: Command) -> String {
        match command {
            Command::Push(segment, index) => {
                todo!()
            }
            _ => todo!()
        }
    }
}
