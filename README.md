# micam
Query which PID had opened the video device of the linux system.

## Usage
```rust
let pids: Vec<i32> = crate::cam::pid_using_camera();
let names: Vec<String> = crate::cam::proc_using_camera();
```

## Woking
This program considers your linux system lists camera device as `/dev/videoX` where X is some number `[0-inf)` respecting the FS limitations.

## Note 
This program accumulate the PID of processes that had opened the file related to camera device.
