use crate::parser::Command;
use crate::parser::MemorySegment;
use crate::parser::Operator;

// 表7-2 CodeWriterモジュール
pub struct CodeWriter {
    sp: StackPointer,
}

struct StackPointer {
    address: u16,
}

impl StackPointer {
    pub fn new() -> Self {
        Self {
            address: 256,
        }
    }

    pub fn increment(&mut self) -> u16 {
        let a = self.address;
        self.address += 1;
        if self.address > 2047 {
            panic!("Out of stack memory space")
        }

        return a;
    }

    pub fn decrement(&mut self) -> u16 {
        self.address -= 1;
        if self.address < 256 {
            panic!("Out of stack memory space")
        }

        return self.address;
    }

    pub fn current(&self) -> u16 {
        self.address
    }
}

impl CodeWriter {
    pub fn new() -> Self {
        Self {
            sp: StackPointer::new(),
        }
    }

    pub fn code(&mut self, command: Command) -> Vec<String> {
        match command {
            Command::Push(segment, index) => {
                match segment {
                    MemorySegment::Constant => {
                        let mut a = vec![
                            format!("@{}", index),
                            "D=A".into(),
                            format!("@{}", self.sp.increment()),
                            "M=D".into(),
                        ];
                        a.append(&mut self.set_sp());
                        a
                    }
                    _ => todo!()
                }
            }
            Command::Arithmetic(operator) => {
                match operator {
                    Operator::Add => {
                        let mut a = vec![
                            format!("@{}", self.sp.decrement()),
                            "D=M".into(),
                            format!("@{}", self.sp.decrement()),
                            "D=D+M".into(),
                            format!("@{}", self.sp.increment()),
                            "M=D".into(),
                        ];
                        a.append(&mut self.set_sp());
                        a
                    }
                    _ => todo!(),
                }
            }
            _ => todo!()
        }
    }

    // スタックポインタを更新するアセンブリコードを返す
    fn set_sp(&self) -> Vec<String> {
        vec![
            format!("@{}", self.sp.current()),
            "D=A".into(),
            "@SP".into(),
            "M=D".into(),
        ]
    }
}
