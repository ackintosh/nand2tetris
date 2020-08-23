use std::collections::HashMap;

// 6.3.4 SymbolTableモジュール
pub struct SymbolTable {
    // ラベル, アドレス
    inner: HashMap<String, u16>,
    next_ram_address: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut inner: HashMap<String, u16> = HashMap::new();

        // 定義済みシンボル
        inner.insert("SP".to_string(), 0);
        inner.insert("LCL".to_string(), 1);
        inner.insert("ARG".to_string(), 2);
        inner.insert("THIS".to_string(), 3);
        inner.insert("THAT".to_string(), 4);
        inner.insert("R0".to_string(), 0);
        inner.insert("R1".to_string(), 1);
        inner.insert("R2".to_string(), 2);
        inner.insert("R3".to_string(), 3);
        inner.insert("R4".to_string(), 4);
        inner.insert("R5".to_string(), 5);
        inner.insert("R6".to_string(), 6);
        inner.insert("R7".to_string(), 7);
        inner.insert("R8".to_string(), 8);
        inner.insert("R9".to_string(), 9);
        inner.insert("R10".to_string(), 10);
        inner.insert("R11".to_string(), 11);
        inner.insert("R12".to_string(), 12);
        inner.insert("R13".to_string(), 13);
        inner.insert("R14".to_string(), 14);
        inner.insert("R15".to_string(), 15);
        inner.insert("SCREEN".to_string(), 16384);
        inner.insert("KBD".to_string(), 24576);

        Self {
            inner,
            next_ram_address: 16,
        }
    }

    pub fn add(&mut self, symbol: &str, address: u16) {
        self.inner.insert(symbol.to_string(), address);
    }

    pub fn new_symbol(&mut self, symbol: &str) -> u16 {
        assert!(!self.inner.contains_key(symbol));

        let address = self.next_ram_address;
        self.inner.insert(symbol.to_string(), address);

        self.next_ram_address += 1;
        return address;
    }

    pub fn address(&self, symbol: &str) -> Option<&u16> {
        self.inner.get(symbol)
    }
}
