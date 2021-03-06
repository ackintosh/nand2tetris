use crate::parser::{Command, Call};
use crate::parser::MemorySegment;
use crate::parser::Operator;

struct LabelGenerator {
    n: u16,
    prefix: String,
}

impl LabelGenerator {
    fn new(prefix: String) -> Self {
        Self {
            n: 0,
            prefix,
        }
    }

    fn gen(&mut self) -> String {
        let n = self.n;
        self.n += 1;
        format!("LABEL{}{}", self.prefix, n)
    }
}

// 表7-2 CodeWriterモジュール
pub struct CodeWriter {
    label_generator: LabelGenerator,
    variable_symbol_prefix: String,
}

impl CodeWriter {
    pub fn new(prefix: String) -> Self {
        Self {
            label_generator: LabelGenerator::new(prefix.clone()),
            variable_symbol_prefix: prefix,
        }
    }

    pub fn bootstrap_code() -> Vec<String> {
        let mut a = vec![
            "@256".into(),
            "D=A".into(),
            "@SP".into(),
            "M=D".into(),
        ];
        let mut cw = Self::new("bootstrap".into());
        a.extend(cw.code(Command::Call(Call::new("Sys.init".into(), 0))));
        a
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
                    MemorySegment::Static => {
                        let mut a = vec![
                            format!("@{}.{}", self.variable_symbol_prefix, memory_access.index),
                            "D=M".into(),
                        ];
                        a.append(&mut self.push_d_value());
                        a
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
                    MemorySegment::Static => {
                        vec![
                            "@SP".into(),
                            "AM=M-1".into(),
                            "D=M".into(),
                            format!("@{}.{}", self.variable_symbol_prefix, memory_access.index),
                            "M=D".into(),
                        ]
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
            Command::Label(label) => {
                vec![
                    format!("({})", label),
                ]
            }
            Command::IfGoto(label) => {
                vec![
                    "@SP".into(),
                    "AM=M-1".into(),
                    "D=M".into(),
                    format!("@{}", label),
                    "D;JNE".into(),
                ]
            }
            Command::Goto(label) => {
                vec![
                    format!("@{}", label),
                    "0;JMP".into(),
                ]
            }
            Command::Function(function) => {
                let mut a = vec![
                    format!("({})", function.name),
                    "D=0".into(),
                ];
                for _ in 0..function.num_local_variables {
                    a.append(&mut self.push_d_value());
                }
                a
            }
            Command::Call(call) => {
                let return_label = self.label_generator.gen();
                let mut a = vec![];
                // push return-address
                a.extend(vec![
                    format!("@{}", return_label),
                    "D=A".into(),
                ]);
                a.extend(self.push_d_value());
                // push LCL
                a.append(&mut self.push_address_value("LCL", 0));
                // push ARG
                a.append(&mut self.push_address_value("ARG", 0));
                // push THIS
                a.append(&mut self.push_address_value("THIS", 0));
                // push THAT
                a.append(&mut self.push_address_value("THAT", 0));
                // ARG = SP-n-5
                a.append(&mut vec![
                    "@SP".into(),
                    "D=M".into(),
                    format!("@{}", call.num_arguments),
                    "D=D-A".into(),
                    "@5".into(),
                    "D=D-A".into(),
                    "@ARG".into(),
                    "M=D".into(),
                ]);
                // LCL = SP
                a.append(&mut vec![
                    "@SP".into(),
                    "D=M".into(),
                    "@LCL".into(),
                    "M=D".into(),
                ]);
                // goto f
                a.append(&mut vec![
                    format!("@{}", call.function_name),
                    "0;JMP".into(),
                ]);
                // (return-address)
                a.append(&mut vec![
                    format!("({})", return_label),
                ]);
                a
            }
            Command::Return => {
                let mut a = vec![
                    // FRAME = LCL
                    "@LCL".into(),
                    "D=M".into(),
                    "@R13".into(),
                    "M=D".into(),
                    // RET = *(FRAME-5)
                    "@R13".into(),
                    "D=M".into(),
                    "@5".into(),
                    "D=D-A".into(),
                    "A=D".into(),
                    "D=M".into(),
                    "@R14".into(),
                    "M=D".into(),
                ];
                // *ARG = pop()
                a.append(&mut self.pop_to_address_value("ARG", 0));
                // SP = ARG+1
                a.append(&mut vec![
                    "@ARG".into(),
                    "D=M+1".into(),
                    "@SP".into(),
                    "M=D".into(),
                ]);
                // THAT = *(FRAME-1)
                a.append(&mut vec![
                    "@R13".into(),
                    "A=M-1".into(),
                    "D=M".into(),
                    "@THAT".into(),
                    "M=D".into(),
                ]);
                // THIS = *(FRAME-2)
                a.append(&mut vec![
                    "@R13".into(),
                    "D=M".into(),
                    "@2".into(),
                    "A=D-A".into(),
                    "D=M".into(),
                    "@THIS".into(),
                    "M=D".into(),
                ]);
                // ARG = *(FRAME-3)
                a.append(&mut vec![
                    "@R13".into(),
                    "D=M".into(),
                    "@3".into(),
                    "A=D-A".into(),
                    "D=M".into(),
                    "@ARG".into(),
                    "M=D".into(),
                ]);
                // LCL = *(FRAME-4)
                a.append(&mut vec![
                    "@R13".into(),
                    "D=M".into(),
                    "@4".into(),
                    "A=D-A".into(),
                    "D=M".into(),
                    "@LCL".into(),
                    "M=D".into(),
                ]);
                // goto RET
                a.append(&mut vec![
                    "@R14".into(),
                    "A=M".into(),
                    "0;JMP".into(),
                ]);
                a
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
