use crate::{pattern::Pattern, PatternAttribute};
use std::collections::BTreeMap;
use std::iter::FromIterator;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PatternCollection {
    pub attributes: Vec<PatternAttribute>,
    pub patterns: BTreeMap<String, Pattern>,
}
impl PatternCollection {
    pub fn new() -> Self {
        Self {
            patterns: BTreeMap::new(),
            ..Self::default()
        }
    }
}

impl From<BTreeMap<String, Pattern>> for PatternCollection {
    fn from(patterns: BTreeMap<String, Pattern>) -> Self {
        Self {
            patterns,
            ..Self::default()
        }
    }
}

impl FromIterator<(String, Pattern)> for PatternCollection {
    fn from_iter<I: IntoIterator<Item = (String, Pattern)>>(iter: I) -> Self {
        Self {
            patterns: BTreeMap::from_iter(iter),
            ..Self::default()
        }
    }
}
