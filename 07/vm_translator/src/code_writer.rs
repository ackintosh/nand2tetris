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
                    Operator::Sub => {
                        let mut a = vec![
                            format!("@{}", self.sp.decrement()),
                            "D=M".into(),
                            format!("@{}", self.sp.decrement()),
                            "D=D-M".into(),
                            format!("@{}", self.sp.increment()),
                            "M=D".into(),
                        ];
                        a.append(&mut self.set_sp());
                        a
                    }
                    Operator::Neg => {
                        vec![
                            format!("@{}", self.sp.decrement()),
                            "D=M".into(),
                            "D=-D".into(),
                            format!("@{}", self.sp.increment()),
                            "M=D".into(),
                        ]
                    }
                    Operator::Eq => {
                        let label_true = self.label_generator.gen();
                        let label_false = self.label_generator.gen();

                        let mut a = vec![
                            // 減算して比較する
                            format!("@{}", self.sp.decrement()),
                            "D=M".into(),
                            format!("@{}", self.sp.decrement()),
                            "D=D-M".into(),

                            // 判定(trueならJUMP)
                            format!("@{}", label_true),
                            "D;JEQ".into(),

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

                            // 比較結果(Dの値)をスタックに戻す
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
