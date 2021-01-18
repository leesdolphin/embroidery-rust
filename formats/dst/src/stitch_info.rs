macro_rules! expand_from_delta {
    ($x:ident, $y:ident,y, +, $val:tt) => {
        $y += $val;
    };
    ($x:ident, $y:ident,y, -, $val:tt) => {
        $y -= $val;
    };
    ($x:ident, $y:ident,x, +, $val:tt) => {
        $x += $val;
    };
    ($x:ident, $y:ident,x, -, $val:tt) => {
        $x -= $val;
    };
}

macro_rules! expand_to_condition {
    ($x:ident, $y:ident,y, $sign:tt, $val:tt: $block:block) => {
        if (0 $sign $y) >= (($val - 1) / 2 + 1) {
            $block;
            $y -= 0 $sign $val;
        }
    };
    ($x:ident, $y:ident,x, $sign:tt, $val:tt: $block:block) => {
        if (0 $sign $x) >= (($val - 1) / 2 + 1) {
            $block;
            $x -= 0 $sign $val;
        }
    };
}

macro_rules! stitch_definitions {
    ($($bits:tt -> ($var: ident $sign:tt $val:tt)),+) => {
        #[inline]
        fn from_int(val: u32) -> (i8, i8) {
            let mut x: i8 = 0;
            let mut y: i8 = 0;
            $(
                if val & $bits == $bits {
                    expand_from_delta!(x, y, $var, $sign, $val);
                }
            )+
            return (x, y)
        }

        #[inline]
        fn to_int(_x: i8, _y: i8) -> Option<u32> {
            if _x < -121 || _x > 121 || _y < -121 || _y > 121 {
                return None;
            }
            let mut x = _x;
            let mut y = _y;
            let mut bytes: u32 = 0x00_00_03;
            $(
                expand_to_condition!(x, y, $var, $sign, $val: {
                    bytes |= $bits;
                });
            )+
            if x != 0 || y != 0 {
                None
            } else {
                Some(bytes)
            }
        }
    };
}

stitch_definitions!(
    0x_00_00_20 -> (y + 81),
    0x_00_00_10 -> (y - 81),
    0x_00_00_08 -> (x - 81),
    0x_00_00_04 -> (x + 81),

    0x_00_20_00 -> (y + 27),
    0x_00_10_00 -> (y - 27),
    0x_00_08_00 -> (x - 27),
    0x_00_04_00 -> (x + 27),

    0x_20_00_00 -> (y + 9),
    0x_10_00_00 -> (y - 9),
    0x_08_00_00 -> (x - 9),
    0x_04_00_00 -> (x + 9),

    0x_00_80_00 -> (y + 3),
    0x_00_40_00 -> (y - 3),
    0x_00_02_00 -> (x - 3),
    0x_00_01_00 -> (x + 3),

    0x_80_00_00 -> (y + 1),
    0x_40_00_00 -> (y - 1),
    0x_02_00_00 -> (x - 1),
    0x_01_00_00 -> (x + 1)
);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StitchType {
    Regular,
    Jump,
    Stop,
    JumpStop,
}

impl StitchType {
    pub fn from_bytes(bytes: [u8; 3]) -> Self {
        match bytes[2] & 0b1100_0000 {
            0b0100_0000 => StitchType::Stop,
            0b1000_0000 => StitchType::Jump,
            0b1100_0000 => StitchType::JumpStop,
            _ => StitchType::Regular,
        }
    }
    pub fn with_stop(self) -> Self {
        match self {
            StitchType::Stop => StitchType::Stop,
            StitchType::Jump => StitchType::JumpStop,
            StitchType::JumpStop => StitchType::JumpStop,
            StitchType::Regular => StitchType::Stop,
        }
    }

    #[allow(dead_code)]
    pub fn with_jump(self) -> Self {
        match self {
            StitchType::Stop => StitchType::JumpStop,
            StitchType::Jump => StitchType::Jump,
            StitchType::JumpStop => StitchType::JumpStop,
            StitchType::Regular => StitchType::Jump,
        }
    }

    pub fn is_jump(self) -> bool {
        match self {
            StitchType::Stop => false,
            StitchType::Jump => true,
            StitchType::JumpStop => true,
            StitchType::Regular => false,
        }
    }
    pub fn is_stop(self) -> bool {
        match self {
            StitchType::Stop => true,
            StitchType::Jump => false,
            StitchType::JumpStop => true,
            StitchType::Regular => false,
        }
    }
    pub fn is_regular(self) -> bool {
        !self.is_jump() && !self.is_stop()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StitchInformation {
    Move(i8, i8, StitchType),
    End,
}

impl StitchInformation {
    pub fn from_bytes(bytes: [u8; 3]) -> Self {
        let val: u32 = u32::from(bytes[0]);
        let val: u32 = u32::from(bytes[1]) | (val << 8);
        let val: u32 = u32::from(bytes[2]) | (val << 8);
        let (x, y) = from_int(val);
        if val & 0x_00_00_F0 == 0x_00_00_F0 {
            StitchInformation::End
        } else {
            StitchInformation::Move(x, y, StitchType::from_bytes(bytes))
        }
    }
    pub fn to_bytes(self: Self) -> Option<[u8; 3]> {
        match self {
            StitchInformation::Move(x, y, stitch_type) => {
                let val = to_int(x, y)?;
                let option_bits = match stitch_type {
                    StitchType::Jump => 0x80,
                    StitchType::Stop => 0x40,
                    StitchType::JumpStop => 0xC0,
                    StitchType::Regular => 0x00,
                };
                Some([
                    ((val >> 16) & 0xFF) as u8,
                    ((val >> 8) & 0xFF) as u8,
                    option_bits | (val & 0xFF) as u8,
                ])
            },
            StitchInformation::End => Some([0x00, 0x00, 0xF3]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_gen_from_int() {
        assert_eq!(from_int(0x00_00_00), (0, 0));
        assert_eq!(from_int(0x00_00_20), (0, 81));
        assert_eq!(from_int(0x00_00_18), (-81, -81));
    }

    #[test]
    fn test_macro_gen_to_int() {
        assert_eq!(to_int(0, 0), Some(0x00_00_03));
        assert_eq!(to_int(127, 127), None);
        assert_eq!(to_int(127, 5), None);
        assert_eq!(to_int(-121, -121), Some(0x5A_5A_1B));
        assert_eq!(to_int(121, -121), Some(0x55_55_17));
        assert_eq!(to_int(-12, 64), Some(0xA8_12_23));
        assert_eq!(to_int(8, -8), Some(0x96_00_03));
    }

    #[test]
    fn test_macro_any_byte_sequence_parses() {
        for i in 0..0x1_00_00_00 {
            let (x, y) = from_int(i);
            assert!(
                x >= -121 && x <= 121 && y >= -121 && y <= 121,
                "{:06X} -> ({}, {})",
                i,
                x,
                y,
            )
        }
    }

    #[test]
    fn test_macro_gen_valids_roundtrip() {
        for x in -121..122 {
            for y in -121..122 {
                let test_val = to_int(x, y).unwrap();
                let (test_x, test_y) = from_int(test_val);
                let test2_val = to_int(test_x, test_y).unwrap();
                assert_eq!(
                    (test_x, test_y),
                    (x, y),
                    "({}, {}) -> {:06X} -> ({}, {}) -> {:06X}",
                    x,
                    y,
                    test_val,
                    test_x,
                    test_y,
                    test2_val
                );
                assert_eq!(
                    test_val, test2_val,
                    "({}, {}) -> {:06X} -> ({}, {}) -> {:06X}",
                    x, y, test_val, test_x, test_y, test2_val
                );
            }
        }
    }

    #[test]
    fn test_stitch_information_from_bytes() {
        assert_eq!(
            StitchInformation::from_bytes([0x00, 0x00, 0x83]),
            StitchInformation::Move(0, 0, StitchType::Jump),
        );
        assert_eq!(
            StitchInformation::from_bytes([0x00, 0x00, 0x43]),
            StitchInformation::Move(0, 0, StitchType::Stop),
        );
        assert_eq!(
            StitchInformation::from_bytes([0x00, 0x00, 0xC3]),
            StitchInformation::Move(0, 0, StitchType::JumpStop),
        );
        assert_eq!(
            StitchInformation::from_bytes([0x00, 0x00, 0xF3]),
            StitchInformation::End,
        );
    }
}
