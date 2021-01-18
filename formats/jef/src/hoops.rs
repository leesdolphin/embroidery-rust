const HOOP_110X110: u32 = 0;
const HOOP_50X50: u32 = 1;
const HOOP_140X200: u32 = 2;
const HOOP_126X110: u32 = 3;
const HOOP_200X200: u32 = 4;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum JefHoop {
    Hoop110x110,
    Hoop50x50,
    Hoop140x200,
    Hoop126x110,
    Hoop200x200,
    Other(u32),
}

impl JefHoop {
    pub fn from_byte(hoop_code: u32) -> Self {
        match hoop_code {
            HOOP_50X50 => JefHoop::Hoop50x50,
            HOOP_110X110 => JefHoop::Hoop110x110,
            HOOP_126X110 => JefHoop::Hoop140x200,
            HOOP_140X200 => JefHoop::Hoop126x110,
            HOOP_200X200 => JefHoop::Hoop200x200,
            other => JefHoop::Other(other),
        }
    }
    #[allow(dead_code)]
    pub fn to_bytes(&self) -> u32 {
        match self {
            JefHoop::Hoop50x50 => HOOP_50X50,
            JefHoop::Hoop110x110 => HOOP_110X110,
            JefHoop::Hoop126x110 => HOOP_126X110,
            JefHoop::Hoop140x200 => HOOP_140X200,
            JefHoop::Hoop200x200 => HOOP_200X200,
            JefHoop::Other(code) => *code,
        }
    }
    #[allow(dead_code)]
    pub fn hoop_size(self) -> Option<(f64, f64)> {
        match self {
            JefHoop::Hoop50x50 => Some((50.0, 50.0)),
            JefHoop::Hoop110x110 => Some((110.0, 110.0)),
            JefHoop::Hoop126x110 => Some((126.0, 110.0)),
            JefHoop::Hoop140x200 => Some((140.0, 200.0)),
            JefHoop::Hoop200x200 => Some((200.0, 200.0)),
            _ => None,
        }
    }
}
