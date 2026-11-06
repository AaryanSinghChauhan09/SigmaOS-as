//! Object file model: sections, relocations, code/data emission.

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub code: Vec<u8>,
    pub data: Vec<u8>,
}

impl Section {
    pub fn new(name: String) -> Self {
        Self { name, code: Vec::new(), data: Vec::new() }
    }
}

#[derive(Debug, Clone)]
pub struct Relocation {
    pub offset: usize,
    pub symbol: String,
    pub kind: &'static str,
}

#[derive(Debug, Default)]
pub struct Object {
    pub sections: Vec<Section>,
    pub relocations: Vec<Relocation>,
}

impl Object {
    pub fn new() -> Self { Self::default() }
    pub fn add_section(&mut self, s: Section) { self.sections.push(s); }
    pub fn add_relocation(&mut self, r: Relocation) { self.relocations.push(r); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_sections() {
        let mut o = Object::new();
        o.add_section(Section::new(".text".into()));
        assert_eq!(o.sections.len(), 1);
    }
}
