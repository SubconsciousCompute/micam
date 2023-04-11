use crate::fuser::fusers;

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

/// Accumulates all PIDs that are using all camera vaialable on the system
pub fn pid_using_camera() -> Vec<i32> {
    let mut pids = Vec::new();
    for cam in get_cam_devices() {
        let pid = fusers(cam.as_str());
        pids.extend(pid.iter());
    }
    pids
}

#[test]
fn test_get_cam_devices() {
    let cams = get_cam_devices();
    // Test will only pass if your device has atleast 1 camera input.
    assert!(cams.len() > 0);
}

#[test]
fn test_pid_using_camera() {
    let _cam = std::fs::File::open("/dev/video0");
    let pids = pid_using_camera();
    assert!(pids.len() > 0);
}
