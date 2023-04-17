//! A library to check who is accessing mic and camera on Linux based systems
//!

pub mod cam;
pub(crate) mod fuser;
pub mod mic;

#[cfg(test)]
mod test {
    #[test]
    fn test_video_device() {
        let cams = crate::cam::get_cam_devices();
        eprintln!("{cams:?}");
    }
}
