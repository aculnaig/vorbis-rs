#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::null_mut;

    #[test]
    fn test_vorbis_encode_init() {

        let mut info: vorbis_info = vorbis_info {
            version: 0,
            channels: 0,
            rate: 0,
            bitrate_upper: 0,
            bitrate_nominal: 0,
            bitrate_lower: 0,
            bitrate_window: 0,
            codec_setup: null_mut(),
        };

        unsafe {
            vorbis_info_init(&mut info);
        }

        unsafe {
            match vorbis_encode_init(&mut info, 2, 44_100, 124_000, 62_000, 8_000) {
                0 => println!("vorbis_encode_init success"),
                _ => println!("vorbis_encode_init failed"),
            }
        }

        unsafe {
            vorbis_info_clear(&mut info);
        }
    }
}
