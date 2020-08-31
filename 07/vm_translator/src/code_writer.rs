use crate::parser::Command;
use crate::parser::MemorySegment;
use crate::parser::Operator;

struct LabelGenrator {
    n: u16,
}

impl LabelGenrator {
    fn new() -> Self {
        Self {
            n: 0,
        }
    }

    fn gen(&mut self) -> String {
        let n = self.n;
        self.n += 1;
        format!("LABEL{}", n)
    }
}

// 表7-2 CodeWriterモジュール
pub struct CodeWriter {
    label_generator: LabelGenrator,
}

impl CodeWriter {
    pub fn new() -> Self {
        Self {
            label_generator: LabelGenrator::new(),
        }
    }

    pub fn code(&mut self, command: Command) -> Vec<String> {
        match command {
            Command::Push(memory_access) => {
                match memory_access.segment {
                    MemorySegment::Constant => {
                        let mut a = vec![
                            format!("@{}", memory_access.index),
                            "D=A".into(),
                        ];
                        a.append(&mut self.push_d_value());
                        a
                    }
                    MemorySegment::Local => self.push_address_value("LCL", memory_access.index),
                    MemorySegment::Argument => self.push_address_value("ARG", memory_access.index),
                    MemorySegment::This => self.push_address_value("THIS", memory_access.index),
                    MemorySegment::That => self.push_address_value("THAT", memory_access.index),
                    MemorySegment::Pointer | MemorySegment::Temp => {
                        self.push_static_address_value(memory_access.get_static_address())
                    }
                }
            }
            Command::Pop(memory_access) => {
                match memory_access.segment {
                    MemorySegment::Constant => unreachable!(),
                    MemorySegment::Local => self.pop_to_address_value("LCL", memory_access.index),
                    MemorySegment::Argument => self.pop_to_address_value("ARG", memory_access.index),
                    MemorySegment::This => self.pop_to_address_value("THIS", memory_access.index),
                    MemorySegment::That => self.pop_to_address_value("THAT", memory_access.index),
                    MemorySegment::Pointer | MemorySegment::Temp => {
                        self.pop_to_static_address_value(memory_access.get_static_address())
                    }
                }
            }
            Command::Arithmetic(operator) => {
                match operator {
                    Operator::Add => {
                        let mut a = Self::pop_for_binary_operator();
                        a.append(&mut vec!["D=D+M".into()]);
                        a.append(&mut self.push_d_value());
                        a
                    }
                    Operator::Sub => {
                        let mut a = Self::pop_for_binary_operator();
                        a.append(&mut vec!["D=M-D".into()]);
                        a.append(&mut self.push_d_value());
                        a
                    }
                    Operator::Neg => {
                        let mut a = vec![
                            "@SP".into(),
                            "AM=M-1".into(),
                            "D=M".into(),
                            "D=-D".into(),
                        ];
                        a.append(&mut self.push_d_value());
                        a
                    }
                    Operator::Eq => self.comparison_operation("JEQ"),
                    Operator::Gt => self.comparison_operation("JGT"),
                    Operator::Lt => self.comparison_operation("JLT"),
                    Operator::And => {
                        let mut a = Self::pop_for_binary_operator();
                        a.append(&mut vec!["D=D&M".into()]);
                        // 比較結果(Dの値)をスタックに戻す
                        a.append(&mut self.push_d_value());
                        a
                    }
                    Operator::Or => {
                        let mut a = Self::pop_for_binary_operator();
                        a.append(&mut vec!["D=D|M".into()]);
                        // 比較結果(Dの値)をスタックに戻す
                        a.append(&mut self.push_d_value());
                        a
                    }
                    Operator::Not => {
                        let mut a = vec![
                            "@SP".into(),
                            "AM=M-1".into(),
                            "D=!M".into(),
                        ];
                        // 比較結果(Dの値)をスタックに戻す
                        a.append(&mut self.push_d_value());
                        a
                    }
                }
            }
        }
    }

    fn comparison_operation(&mut self, jump: &str) -> Vec<String> {
        let label_true = self.label_generator.gen();
        let label_false = self.label_generator.gen();

        let mut a = vec![
            // 減算して比較する
            "@SP".into(),
            "AM=M-1".into(),
            "D=M".into(),
            "@SP".into(),
            "AM=M-1".into(),
            "D=M-D".into(),

            // 判定(trueならJUMP)
            format!("@{}", label_true),
            format!("D;{}", jump),

            // 判定(falseならDにゼロをセットしてJUMP)
            "D=0".into(),
            format!("@{}", label_false),
            "0;JMP".into(),

            // trueのJUMP先
            // Dにtrue(-1)をセットする
            format!("({})", label_true),
            "D=-1".into(),

            // falseのJUMP先
            // JUMP前に予めDにfalse(0)がセットされている
            format!("({})", label_false),

        ];
        // 比較結果(Dの値)をスタックに戻す
        a.append(&mut self.push_d_value());
        a
    }

    fn push_d_value(&mut self) -> Vec<String> {
        vec![
            // 結果(Dの値)をスタックに戻す
            "@SP".into(),
            "A=M".into(),
            "M=D".into(),
            "@SP".into(),
            "M=M+1".into(),
        ]
    }

    fn pop_for_binary_operator() -> Vec<String> {
        // D -> y
        // M -> x
        vec![
            "@SP".into(),
            "AM=M-1".into(),
            "D=M".into(),
            "@SP".into(),
            "AM=M-1".into(),
        ]
    }

    fn set_memory_address_to_a(base_address: &str, index: u16) -> Vec<String> {
        let mut a = vec![
            format!("@{}", base_address),
            "A=M".into(),
        ];
        for _i in 0..index {
            a.append(&mut vec!["A=A+1".into()]);
        }
        a
    }

    fn push_address_value(&mut self, base_address: &str, index: u16) -> Vec<String> {
        let mut a = Self::set_memory_address_to_a(base_address, index);
        a.append(&mut vec![
            "D=M".into(),
        ]);
        a.append(&mut self.push_d_value());
        a
    }

    fn push_static_address_value(&mut self, static_address: u16) -> Vec<String> {
        let mut a = vec![
            format!("@{}", static_address),
            "D=M".into(),
        ];
        a.append(&mut self.push_d_value());
        a
    }

    fn pop_to_address_value(&mut self, base_address: &str, index: u16) -> Vec<String> {
        let mut a = vec![
            "@SP".into(),
            "AM=M-1".into(),
            "D=M".into(),
        ];
        a.append(&mut Self::set_memory_address_to_a(base_address, index));
        a.append(&mut vec![
            "M=D".into(),
        ]);
        a
    }

    fn pop_to_static_address_value(&mut self, static_address: u16) -> Vec<String> {
        let mut a = vec![
            "@SP".into(),
            "AM=M-1".into(),
            "D=M".into(),
        ];
        a.append(&mut vec![
            format!("@{}", static_address),
            "M=D".into(),
        ]);
        a
    }
}
