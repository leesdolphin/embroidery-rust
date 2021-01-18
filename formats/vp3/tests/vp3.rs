use embroidery_lib::format::PatternFormat;

use embroidery_fmt_vp3::Vp3PatternFormat;

#[test]
fn test_t160_vp3_file_read() {
    // Taken from http://en.embgallery.com/sites/default/files/freedesign/T160.zip
    let mut data: &[u8] = include_bytes!("test_data/vp3/embgallery/T160.vp3");
    let loader = Vp3PatternFormat::default().reader().unwrap();
    loader.read_pattern(&mut data).unwrap();
}

#[test]
fn test_t42_1_vp3_file_read() {
    // Taken from http://en.embgallery.com/sites/default/files/freedesign/T42-1.zip
    let mut data: &[u8] = include_bytes!("test_data/vp3/embgallery/T42-1.vp3");
    let loader = Vp3PatternFormat::default().reader().unwrap();
    loader.read_pattern(&mut data).unwrap();
}
