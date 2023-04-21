use crate::fuser::{fusers, fusers_is_open, pid_name};

/// List out all microphone sources that can provide sound input
/// This can be integrated mic, or HDMI input etc.
pub fn get_mic_devices() -> Vec<String> {
    let paths = std::fs::read_dir("/dev/snd");
    if paths.is_err() {
        tracing::warn!(
            "Devicess are not found on this machine, Maybe you're running in chroot environment"
        );
        return vec![];
    }
    let mut mic_devices = Vec::new();
    let paths = paths.unwrap();
    for path in paths.flatten() {
        let path_str = path.path().display().to_string();
        if path_str.contains("pcm") || path_str.contains("hw") {
            mic_devices.push(path_str);
        }
    }
    // Add other mic devices, Which may be present.
    mic_devices.push("/dev/audio".to_string());
    mic_devices.push("/dev/dsp".to_string());
    mic_devices
}

/// Accumulates all PIDs that are using all microphone available on the system
pub fn pid_using_mic() -> Vec<i32> {
    let mut pids = std::collections::HashSet::new();
    for cam in get_mic_devices() {
        let pid = fusers(cam.as_str());
        pids.extend(pid.iter());
    }
    pids.into_iter().collect()
}

/// Accumulates names of all processes using the microphone
pub fn proc_using_mic() -> Vec<String> {
    let pids = pid_using_mic();
    let mut processes = Vec::with_capacity(pids.len());
    for pid in pids {
        processes.push(pid_name(pid).unwrap_or(format!("Unknow PID {}", pid)));
    }
    processes
}

pub fn is_mic_open() -> bool {
    for mic in get_mic_devices() {
        if fusers_is_open(mic.as_str()) {
            return true;
        }
    }
    false
}

#[test]
fn test_mic_usage() {
    let procs = proc_using_mic();
    println!("{procs:#?}");
}
