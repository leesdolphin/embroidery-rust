use embroidery_lib::format::PatternWriter;

use crate::header::PatternType;

pub struct HusVipPatternWriter {
    mode: PatternType,
}

impl HusVipPatternWriter {
    pub fn hus() -> Self {
        Self { mode: PatternType::Hus }
    }
    pub fn vip() -> Self {
        Self { mode: PatternType::Vip }
    }
}

impl PatternWriter for HusVipPatternWriter {
    fn write_pattern(
        &self,
        _: &embroidery_lib::Pattern,
        _: &mut dyn std::io::Write,
    ) -> std::result::Result<(), embroidery_lib::errors::write::Error> {
        let _magic = self.mode.magic_bytes();
        if self.mode == PatternType::Hus {
            todo!()
        } else {
            todo!()
        }
    }
}
