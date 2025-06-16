use std::collections::HashMap;
use std::rc::Rc;

pub type DictionaryIdx = usize;

#[derive(Default, Debug)]
pub struct Dictionary {
    str_to_idx: HashMap<Rc<str>, DictionaryIdx>,
    idx_to_str: Vec<Rc<str>>,
}

impl Dictionary {
    pub fn map_to_idx(&self, name: &str) -> Option<DictionaryIdx> {
        self.str_to_idx.get(name).cloned()
    }

    pub fn map_to_name(&self, idx: DictionaryIdx) -> &str {
        &self.idx_to_str[idx]
    }

    pub fn put(&mut self, name: &str) -> DictionaryIdx {
        if let Some(idx) = self.map_to_idx(name) {
            idx
        } else {
            let name: Rc<str> = Rc::from(name);
            let idx = self.idx_to_str.len();

            self.idx_to_str.push(name.clone());
            self.str_to_idx.insert(name, idx);

            idx
        }
    }

    pub fn len(&self) -> usize {
        self.idx_to_str.len()
    }
}
