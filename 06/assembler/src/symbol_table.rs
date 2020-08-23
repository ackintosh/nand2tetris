use std::collections::HashMap;

// 6.3.4 SymbolTableモジュール
pub struct SymbolTable {
    // ラベル, アドレス
    inner: HashMap<String, u16>
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn add(&mut self, symbol: &str, address: u16) {
        self.inner.insert(symbol.to_string(), address);
    }
}
