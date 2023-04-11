//! A basic functionality replacing the use of `fuser` command on linux
use std::fs;

/// Get the PIDs of all the process that have openend the file.
pub(crate) fn fusers(file_path: &str) -> Vec<i32> {
    let mut pids = Vec::new();
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries {
            if let Ok(ref entry) = entry {
                let meta = entry.metadata();
                if meta.is_ok() && meta.unwrap().is_dir() {
                    if let Ok(ref pid) = entry.file_name().into_string().unwrap().parse::<i32>() {
                        if let Ok(fds) = fs::read_dir(format!("/proc/{}/fd", pid)) {
                            // Using named for loop so that it can be breaked at once if we find fd
                            // realted to file_path.
                            // This loop lists out all the fds opened by a PID
                            'fids_per_pid: for fd in fds.flatten() {
                                if let Ok(ref opened_file) = fs::read_link(fd.path()) {
                                    if let Some(open_file_name) = opened_file.to_str() {
                                        if open_file_name == file_path {
                                            pids.push(*pid);
                                            break 'fids_per_pid;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    pids
}

#[test]
fn test_pid_check() {
    let pid = fusers("/dev/video0");
    eprintln!("{pid:?}");
}
