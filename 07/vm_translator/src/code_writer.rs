use crate::parser::Command;
use crate::parser::MemorySegment;
use crate::parser::Operator;

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
    sp: StackPointer,
    label_generator: LabelGenrator,
}

impl CodeWriter {
    pub fn new() -> Self {
        Self {
            sp: StackPointer::new(),
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
                        a.append(&mut self.set_sp());
                        a
                    }
                    MemorySegment::Local => self.push_address_value("LCL", memory_access.index),
                    MemorySegment::Argument => self.push_address_value("ARG", memory_access.index),
                    MemorySegment::This => self.push_address_value("THIS", memory_access.index),
                    MemorySegment::That => self.push_address_value("THAT", memory_access.index),
                    MemorySegment::Pointer | MemorySegment::Temp => {
                        self.push_address_value(memory_access.segment.get_mapping(memory_access.index).as_str(), 0)
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
                        self.pop_to_address_value(memory_access.segment.get_mapping(memory_access.index).as_str(), 0)
                    }
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
                        ];
                        a.append(&mut self.push_d_value());
                        a.append(&mut self.set_sp());
                        a
                    }
                    Operator::Sub => {
                        let mut a = vec![
                            format!("@{}", self.sp.decrement()),
                            "D=M".into(),
                            format!("@{}", self.sp.decrement()),
                            "D=M-D".into(),
                        ];
                        a.append(&mut self.push_d_value());
                        a.append(&mut self.set_sp());
                        a
                    }
                    Operator::Neg => {
                        let mut a = vec![
                            format!("@{}", self.sp.decrement()),
                            "D=M".into(),
                            "D=-D".into(),
                        ];
                        a.append(&mut self.push_d_value());
                        a.append(&mut self.set_sp());
                        a
                    }
                    Operator::Eq => self.comparison_operation("JEQ"),
                    Operator::Gt => self.comparison_operation("JGT"),
                    Operator::Lt => self.comparison_operation("JLT"),
                    Operator::And => {
                        let mut a = vec![
                            format!("@{}", self.sp.decrement()),
                            "D=M".into(),
                            format!("@{}", self.sp.decrement()),
                            "D=D&M".into(),
                        ];
                        // 比較結果(Dの値)をスタックに戻す
                        a.append(&mut self.push_d_value());
                        // スタックポインタを更新する
                        a.append(&mut self.set_sp());
                        a
                    }
                    Operator::Or => {
                        let mut a = vec![
                            format!("@{}", self.sp.decrement()),
                            "D=M".into(),
                            format!("@{}", self.sp.decrement()),
                            "D=D|M".into(),
                        ];
                        // 比較結果(Dの値)をスタックに戻す
                        a.append(&mut self.push_d_value());
                        // スタックポインタを更新する
                        a.append(&mut self.set_sp());
                        a
                    }
                    Operator::Not => {
                        let mut a = vec![
                            format!("@{}", self.sp.decrement()),
                            "D=!M".into(),
                        ];
                        // 比較結果(Dの値)をスタックに戻す
                        a.append(&mut self.push_d_value());
                        // スタックポインタを更新する
                        a.append(&mut self.set_sp());
                        a
                    }
                }
            }
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

    fn comparison_operation(&mut self, jump: &str) -> Vec<String> {
        let label_true = self.label_generator.gen();
        let label_false = self.label_generator.gen();

        let mut a = vec![
            // 減算して比較する
            format!("@{}", self.sp.decrement()),
            "D=M".into(),
            format!("@{}", self.sp.decrement()),
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
        // スタックポインタを更新する
        a.append(&mut self.set_sp());
        a
    }

    fn push_d_value(&mut self) -> Vec<String> {
        vec![
            // 結果(Dの値)をスタックに戻す
            format!("@{}", self.sp.increment()),
            "M=D".into(),
        ]
    }

    fn set_memory_address_to_a(base_address: &str, index: u16) -> Vec<String> {
        vec![
            format!("@{}", index),
            "D=A".into(),
            format!("@{}", base_address),
            "A=D+A".into(),
        ]
    }

    fn push_address_value(&mut self, base_address: &str, index: u16) -> Vec<String> {
        let mut a = Self::set_memory_address_to_a(base_address, index);
        a.append(&mut vec![
            "D=M".into(),
        ]);
        a.append(&mut self.push_d_value());
        a.append(&mut self.set_sp());
        a
    }

    fn pop_to_address_value(&mut self, base_address: &str, index: u16) -> Vec<String> {
        let mut a = vec![
            format!("@{}", self.sp.decrement()),
            "D=M".into(),
        ];
        a.append(&mut Self::set_memory_address_to_a(base_address, index));
        a.append(&mut vec![
            "M=D".into(),
        ]);
        a
    }
}
