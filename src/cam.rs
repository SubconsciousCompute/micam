/// List out all camera sources that can provide outsie view
/// This may be a web cam, or HDMI port that support video input even though it doesn't have any
/// camera
pub fn get_cam_devices() -> Vec<String> {
    let paths = std::fs::read_dir("/dev");
    if paths.is_err() {
        tracing::warn!(
            "Devicess are not found on this machine, Maybe you're running in chroot environment"
        );
        return vec![];
    }
    let mut cam_devices = Vec::new();
    let paths = paths.unwrap();
    for path in paths {
        if let Ok(path) = path {
            let path_str = path.path().display().to_string();
            if path_str.contains("video") {
                cam_devices.push(path_str);
            }
        }
    }
    cam_devices
}

#[cfg(test)]
mod test {
    use super::get_cam_devices;
    #[test]
    fn cam_test() {
        let cams = get_cam_devices();
        for cam in cams {
            println!("{:?}", crate::fuser::get_pid_using_file(cam.as_str()));
        }
    }
}
