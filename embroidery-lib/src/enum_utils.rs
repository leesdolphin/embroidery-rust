#[macro_export]
macro_rules! value_enum {
    {
        $(#[$enum_meta:meta])*
        $enum_vis:vis enum $enum_name:ident: $enum_type:ty {
            $(
                $(#[$variant_meta:meta])*
                $variant_vis:vis
                $variant:ident = $value:tt
            ),+  $(,)?
        }
    } => {
        $(#[$enum_meta])*
        $enum_vis enum $enum_name {
            $(
                $(#[$variant_meta])*
                $variant_vis $variant
            ),+
        }

        impl $enum_name {
            pub fn from_value(value: $enum_type) -> Option<Self> {
                match value {
                    $(
                        $value => Some(Self::$variant),
                    )+
                    _ => None
                }
            }
            pub fn from_value_unchecked(value: $enum_type) -> Self {
                Self::from_value(value).unwrap()
            }
            pub fn to_value(self) -> $enum_type {
                match self {
                    $(
                        Self::$variant => $value
                    ),+
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {

    mod integer {
        value_enum! {
            #[derive(Debug )]
            #[derive(PartialEq, Eq)]
            enum MyIntEnum: u8 {
                One = 1,
                Two = 2,
                Ten = 10,
            }
        }

        #[test]
        fn test_my_int_enum_from_value() {
            assert_eq!(MyIntEnum::from_value(1), Some(MyIntEnum::One));
            assert_eq!(MyIntEnum::from_value(2), Some(MyIntEnum::Two));
            assert_eq!(MyIntEnum::from_value(10), Some(MyIntEnum::Ten));
            assert_eq!(MyIntEnum::from_value(99), None);
        }

        #[test]
        fn test_my_int_enum_from_value_unchecked() {
            assert_eq!(MyIntEnum::from_value_unchecked(1), MyIntEnum::One);
            assert_eq!(MyIntEnum::from_value_unchecked(2), MyIntEnum::Two);
            assert_eq!(MyIntEnum::from_value_unchecked(10), MyIntEnum::Ten);
        }

        #[test]
        fn test_my_int_enum_to_value() {
            assert_eq!(MyIntEnum::One.to_value(), 1);
            assert_eq!(MyIntEnum::Two.to_value(), 2);
            assert_eq!(MyIntEnum::Ten.to_value(), 10);
        }
    }

    mod array {
        value_enum! {
            #[derive(Debug )]
            #[derive(PartialEq, Eq)]
            enum MyArrayEnum: [u8; 3] {
                One = [1, 1, 1],
                Two = [2, 2, 2],
                Ten = [10, 10, 10],
            }
        }

        #[test]
        fn test_my_int_enum_from_value() {
            assert_eq!(MyArrayEnum::from_value([1, 1, 1]), Some(MyArrayEnum::One));
            assert_eq!(MyArrayEnum::from_value([2, 2, 2]), Some(MyArrayEnum::Two));
            assert_eq!(MyArrayEnum::from_value([10, 10, 10]), Some(MyArrayEnum::Ten));
            assert_eq!(MyArrayEnum::from_value([99, 99, 99]), None);
        }

        #[test]
        fn test_my_int_enum_from_value_unchecked() {
            assert_eq!(MyArrayEnum::from_value_unchecked([1, 1, 1]), MyArrayEnum::One);
            assert_eq!(MyArrayEnum::from_value_unchecked([2, 2, 2]), MyArrayEnum::Two);
            assert_eq!(MyArrayEnum::from_value_unchecked([10, 10, 10]), MyArrayEnum::Ten);
        }

        #[test]
        fn test_my_int_enum_to_value() {
            assert_eq!(MyArrayEnum::One.to_value(), [1, 1, 1]);
            assert_eq!(MyArrayEnum::Two.to_value(), [2, 2, 2]);
            assert_eq!(MyArrayEnum::Ten.to_value(), [10, 10, 10]);
        }
    }
}
