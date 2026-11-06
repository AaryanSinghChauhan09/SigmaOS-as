//! Labels and symbol table.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    pub name: String,
    pub offset: usize,
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    labels: Vec<Label>,
}

impl SymbolTable {
    pub fn new() -> Self { Self::default() }
    pub fn define(&mut self, name: String, offset: usize) {
        self.labels.push(Label { name, offset });
    }
    pub fn resolve(&self, name: &str) -> Option<usize> {
        self.labels.iter().find(|l| l.name == name).map(|l| l.offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_table() {
        let mut st = SymbolTable::new();
        st.define("main".into(), 0x100);
        assert_eq!(st.resolve("main"), Some(0x100));
        assert_eq!(st.resolve("foo"), None);
    }
}
