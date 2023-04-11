use std::fs;

pub fn get_pid_using_file(file_path: &str) -> Option<i32> {
    // Read the list of processes in /proc
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries {
            if let Ok(entry_name) = entry.unwrap().file_name().into_string() {
                if let Ok(pid) = entry_name.parse::<i32>() {
                    // Read the file descriptors of the process
                    println!("{}", "here before");
                    if let Ok(fd_entries) = fs::read_dir(format!("/proc/{}/fd", pid)) {
                        println!("{}", "here after");
                        for fd_entry in fd_entries.flatten() {
                            if let Ok(link_target) = fs::read_link(fd_entry.path()) {
                                if let Some(link_target_str) = link_target.to_str() {
                                    // Check if the link target matches the file path
                                    if link_target_str == file_path {
                                        return Some(pid);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

#[test]
fn test_pid_check() {
    let _ = get_pid_using_file("/dev/video0");
}
