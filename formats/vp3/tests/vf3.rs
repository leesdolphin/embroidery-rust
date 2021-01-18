use embroidery_lib::format::CollectionFormat;

use embroidery_fmt_vp3::Vf3CollectionFormat;

#[test]
fn test_send_vf3_file_read() {
    let mut data: &[u8] = include_bytes!("test_data/vf3/Send.vf3");
    let loader = Vf3CollectionFormat::default().reader().unwrap();
    loader.read_pattern(&mut data).unwrap();
}
#[test]
fn test_times_new_roman_r_fsb_50_vf3() {
    let mut data: &[u8] = include_bytes!("test_data/vf3/Times New Roman_R_FSB_50.vf3");
    let loader = Vf3CollectionFormat::default().reader().unwrap();
    loader.read_pattern(&mut data).unwrap();
}
#[test]
fn test_microsoft_sans_serif_r_a_c1_50_vf3() {
    let mut data: &[u8] = include_bytes!("test_data/vf3/Microsoft Sans Serif_R_A_C1_50.vf3");
    let loader = Vf3CollectionFormat::default().reader().unwrap();
    loader.read_pattern(&mut data).unwrap();
}
#[test]
fn test_microsoft_sans_serif_r_fsb_h1_50_vf3() {
    let mut data: &[u8] = include_bytes!("test_data/vf3/Microsoft Sans Serif_R_FSB_H1_50.vf3");
    let loader = Vf3CollectionFormat::default().reader().unwrap();
    loader.read_pattern(&mut data).unwrap();
}
#[test]
fn test_microsoft_sans_serif_r_s_c1_10_vf3() {
    let mut data: &[u8] = include_bytes!("test_data/vf3/Microsoft Sans Serif_R_S_C1_10.vf3");
    let loader = Vf3CollectionFormat::default().reader().unwrap();
    loader.read_pattern(&mut data).unwrap();
}
#[test]
fn test_microsoft_sans_serif_r_s_c2_10_vf3() {
    let mut data: &[u8] = include_bytes!("test_data/vf3/Microsoft Sans Serif_R_S_C2_10.vf3");
    let loader = Vf3CollectionFormat::default().reader().unwrap();
    loader.read_pattern(&mut data).unwrap();
}
#[test]
fn test_microsoft_sans_serif_r_s_h1_10_vf3() {
    let mut data: &[u8] = include_bytes!("test_data/vf3/Microsoft Sans Serif_R_S_H1_10.vf3");
    let loader = Vf3CollectionFormat::default().reader().unwrap();
    loader.read_pattern(&mut data).unwrap();
}
#[test]
fn test_microsoft_sans_serif_r_s_w1_10_vf3() {
    let mut data: &[u8] = include_bytes!("test_data/vf3/Microsoft Sans Serif_R_S_W1_10.vf3");
    let loader = Vf3CollectionFormat::default().reader().unwrap();
    loader.read_pattern(&mut data).unwrap();
}
#[test]
fn test_microsoft_sans_serif_r_s_w2_10_vf3() {
    let mut data: &[u8] = include_bytes!("test_data/vf3/Microsoft Sans Serif_R_S_W2_10.vf3");
    let loader = Vf3CollectionFormat::default().reader().unwrap();
    loader.read_pattern(&mut data).unwrap();
}
#[test]
fn test_microsoft_sans_serif_r_s_w3_10_vf3() {
    let mut data: &[u8] = include_bytes!("test_data/vf3/Microsoft Sans Serif_R_S_W3_10.vf3");
    let loader = Vf3CollectionFormat::default().reader().unwrap();
    loader.read_pattern(&mut data).unwrap();
}
#[test]
fn test_microsoft_sans_serif_r_s_w4_10_vf3() {
    let mut data: &[u8] = include_bytes!("test_data/vf3/Microsoft Sans Serif_R_S_W4_10.vf3");
    let loader = Vf3CollectionFormat::default().reader().unwrap();
    loader.read_pattern(&mut data).unwrap();
}
#[test]
fn test_webdings_r_s_w3_50_vf3() {
    let mut data: &[u8] = include_bytes!("test_data/vf3/Webdings_R_S_W3_50.vf3");
    let loader = Vf3CollectionFormat::default().reader().unwrap();
    loader.read_pattern(&mut data).unwrap();
}

mod comic_sans {
    use super::*;

    #[test]
    fn test_comic_sans_ms_b_s_w1_10_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_B_S_W1_10.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }
    #[test]
    fn test_comic_sans_ms_bi_s_w1_10_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_BI_S_W1_10.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }
    #[test]
    fn test_comic_sans_ms_i_s_w1_10_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_I_S_W1_10.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }
    #[test]
    fn test_comic_sans_ms_r_o_w1_25_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_R_O_W1_25.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }
    #[test]
    fn test_comic_sans_ms_r_s_w1_10_d2_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_R_S_W1_10_D2.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }
    #[test]
    fn test_comic_sans_ms_r_s_w1_10_ew_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_R_S_W1_10_EW.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }
    #[test]
    fn test_comic_sans_ms_r_s_w1_10_p1_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_R_S_W1_10_P1.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }
    #[test]
    fn test_comic_sans_ms_r_s_w1_10_zz_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_R_S_W1_10_ZZ.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }
    #[test]
    fn test_comic_sans_ms_r_s_w1_10_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_R_S_W1_10.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }
    #[test]
    fn test_comic_sans_ms_r_s_w1_11_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_R_S_W1_11.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }
    #[test]
    fn test_comic_sans_ms_r_s_w1_12_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_R_S_W1_12.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }
    #[test]
    fn test_comic_sans_ms_r_s_w1_13_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_R_S_W1_13.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }
    #[test]
    fn test_comic_sans_ms_r_s_w1_14_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_R_S_W1_14.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }

    #[test]
    fn test_comic_sans_ms_r_sb_w1_50_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_R_SB_W1_50.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }

    #[test]
    fn test_comic_sans_ms_r_fsb_w1_50_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_R_FSB_W1_50.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }

    #[test]
    fn test_comic_sans_ms_r_a_w1_50_vf3() {
        let mut data: &[u8] = include_bytes!("test_data/vf3/Comic Sans MS_R_A_W1_50.vf3");
        let loader = Vf3CollectionFormat::default().reader().unwrap();
        loader.read_pattern(&mut data).unwrap();
    }
}
