// // use difference::{Changeset, Difference};

// use embroidery_lib::format::PatternReader;
// use embroidery_lib::prelude::*;
// use embroidery_lib::transforms::RemoveDuplicateStitches;

// use embroidery_fmt_vp4::Vp4PatternReader;

// use std::collections::BTreeMap;
// use std::io::Cursor;

// #[test]
// fn test_hus_file_load() {
//     let data: &[u8] = include_bytes!("test_data/premier+/bird.vp4");
//     let loader = Vp4PatternReader {};

//     assert!(loader.is_loadable(&mut Cursor::new(data)).unwrap());
//     let pattern = loader.read_pattern(&mut Cursor::new(data)).unwrap();

//     let cgs = pattern.color_groups;
//     assert_eq!(cgs.len(), 1);
//     assert_eq!(
//         cgs[0].thread,
//         Some(Thread {
//             color: Color {
//                 red: 0,
//                 green: 0,
//                 blue: 127
//             },
//             name: "Dark Blue".to_string(),
//             code: "HUS:13".to_string(),
//             manufacturer: None,
//             attributes: BTreeMap::new(),
//         })
//     );

//     let sgs = &cgs[0].stitch_groups;
//     assert_eq!(sgs.len(), 2);
//     let sg = &sgs[0];
//     assert_eq!(sg.cut, true);
//     assert_eq!(sg.trim, true);
//     assert_eq!(sg.stitches[0], Stitch::new(-88.9, -12.7));
//     assert_eq!(sg.stitches.last(), Some(&Stitch::new(111.4, -21.1)));
// }

// // #[test]
// // fn test_star_hus_file_load() {
// //     let data: &[u8] = include_bytes!("test_data/Star.hus");
// //     let loader = HusVipPatternReader {};
// //
// //     // assert!(loader.is_loadable(&mut Cursor::new(data)).unwrap());
// //     let pattern = loader.read_pattern(&mut Cursor::new(data)).unwrap();
// //
// //     println!("{:?}", pattern);
// //
// //     let cgs = pattern.color_groups.clone();
// //     assert_eq!(cgs.len(), 2);
// //     assert_eq!(
// //         cgs[0].thread,
// //         Some(Thread {
// //             color: Color { red: 255, green: 0, blue: 0 }, name: "Red".to_string(), code:
// // "HUS:03".to_string(), manufacturer: None,             attributes: BTreeMap::new(),
// //         })
// //     );
// //
// //     let sgs = &cgs[0].stitch_groups;
// //     let sg = &sgs[0];
// //     assert_eq!(sg.cut, true);
// //     assert_eq!(sg.trim, true);
// //     assert_eq!(sg.stitches[0], Stitch::new(-88.9, -12.7));
// //     assert_eq!(sg.stitches.last(), Some(&Stitch::new(111.4, -21.1)));
// //
// //     assert_eq!(sgs.len(), 2);
// // }
// // #[test]
// // fn test_star_vip_file_load() {
// //     let loader = HusVipPatternReader {};
// //     let vip_data: &[u8] = include_bytes!("test_data/Star.vip");
// //     let vip_pattern = loader.read_pattern(&mut
// // Cursor::new(vip_data)).unwrap().remove_duplicate_stitches(); }
