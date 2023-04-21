use crate::fuser::{fusers, fusers_is_open, pid_name};

/// List out all camera sources that can provide outside view
/// This may be a web cam, or HDMI port that support video input even though it doesn't have any
/// camera
pub fn get_cam_devices() -> Vec<String> {
    if let Ok(paths) = std::fs::read_dir("/dev") {
        let mut cam_devices = Vec::new();
        for path in paths.flatten() {
            let path_str = path.path().display().to_string();
            if path_str.contains("video") {
                cam_devices.push(path_str);
            }
        }
        return cam_devices;
    }

    tracing::warn!(
        "Devices are not found on this machine, Maybe you're running in chroot environment"
    );

    vec![]
}

/// Accumulates all PIDs that are using all camera available on the system
pub fn pid_using_camera() -> Vec<i32> {
    let mut pids = Vec::new();
    for cam in get_cam_devices() {
        let pid = fusers(cam.as_str());
        pids.extend(pid.iter());
    }
    pids
}

/// Accumulates names of all processes using the camera
pub fn proc_using_camera() -> Vec<String> {
    let pids = pid_using_camera();
    let mut processes = Vec::with_capacity(pids.len());
    for pid in pids {
        processes.push(pid_name(pid).unwrap_or(format!("Unknow PID {}", pid)));
    }
    processes
}

pub fn is_cam_open() -> bool {
    for cam in get_cam_devices() {
        if fusers_is_open(cam.as_str()) {
            return true;
        }
    }
    false
}

#[test]
fn test_cam_usage() {
    let state = is_cam_open();
    println!("Samera state {}", state);
    let procs = proc_using_camera();
    println!("{procs:#?}");
}
