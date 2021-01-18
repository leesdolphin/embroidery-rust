use std::{f64, iter::Iterator};

use crate::stitch::{ColorGroup, Stitch};
use crate::transforms::{RemoveDuplicateStitches, SplitLongStitches};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum PatternAttribute {
    Arbitrary(String, String),
    Title(String),
    Author(String),
    Copyright(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    pub name: String,
    pub attributes: Vec<PatternAttribute>,
    pub color_groups: Vec<ColorGroup>,
}

impl Pattern {
    pub fn iter_stitches(self: &Self) -> impl Iterator<Item = &Stitch> {
        self.color_groups.iter().flat_map(ColorGroup::iter_stitches)
    }

    pub fn get_bounds(self: &Self) -> (f64, f64, f64, f64) {
        let mut min_x: f64 = f64::NAN;
        let mut min_y: f64 = f64::NAN;
        let mut max_x: f64 = f64::NAN;
        let mut max_y: f64 = f64::NAN;
        for stitch in self.iter_stitches() {
            min_x = min_x.min(stitch.x);
            min_y = min_y.min(stitch.y);
            max_x = max_x.max(stitch.x);
            max_y = max_y.max(stitch.y);
        }
        if min_x.is_nan() || min_y.is_nan() || max_x.is_nan() || max_y.is_nan() {
            (0., 0., 0., 0.)
        } else {
            (min_x, min_y, max_x, max_y)
        }
    }
}

impl RemoveDuplicateStitches for Pattern {
    fn remove_duplicate_stitches(self) -> Self {
        Pattern {
            color_groups: self
                .color_groups
                .into_iter()
                .map(|cg| cg.remove_duplicate_stitches())
                .collect(),
            ..self
        }
    }
}
impl SplitLongStitches for Pattern {
    fn split_stitches(self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        Pattern {
            color_groups: self
                .color_groups
                .into_iter()
                .map(|cg| cg.split_stitches(min_x, max_x, min_y, max_y))
                .collect(),
            ..self
        }
    }
}
