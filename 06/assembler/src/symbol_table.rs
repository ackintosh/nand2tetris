use std::collections::HashMap;

// 6.3.4 SymbolTableモジュール
pub struct SymbolTable {
    // ラベル, アドレス
    inner: HashMap<String, u16>,
    next_ram_address: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
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
