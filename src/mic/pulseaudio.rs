//! Module to interact with pulseaudio

use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct AppInfo {
    application_name: String,
    native_protocol_peer: String,
    native_protocol_version: String,
    application_id: String,
    application_icon_name: String,
    application_process_id: i32,
    application_process_user: String,
    application_process_host: String,
    application_process_binary: String,
    application_language: String,
    window_x11_display: String,
}

// impl AppInfo {
//     pub fn parse_from_str(client_info: &str) -> Self {
//         let mut application_name = "".to_string().to_string();
//         let mut native_protocol_peer = "".to_string().to_string();
//         let mut native_protocol_version = "".to_string().to_string();
//         let mut application_id = "".to_string().to_string();
//         let mut application_icon_name = "".to_string().to_string();
//         let mut application_process_id = 0;
//         let mut application_process_user = "".to_string().to_string();
//         let mut application_process_host = "".to_string().to_string();
//         let mut application_process_binary = "".to_string().to_string();
//         let mut application_language = "".to_string().to_string();
//         let mut window_x11_display = "".to_string().to_string();
//
//         let reader = BufReader::new(client_info.as_bytes());
//         for line in reader.lines().flatten() {
//             let mut parts = line.splitn(2, '=');
//             let key = parts
//                 .next()
//                 .ok_or_else(|| "Failed to parse key")
//                 .unwrap()
//                 .trim();
//             println!("{key:#?}");
//             let value.to_string() = parts
//                 .next()
//                 .ok_or_else(|| "Failed to parse value.to_string()")
//                 .unwrap()
//                 .trim();
//             match key {
//                 "application.name" => application_name = value.to_string().to_string(),
//                 "native-protocol.peer" => native_protocol_peer = value.to_string().to_string(),
//                 "native-protocol.version" => native_protocol_version = value.to_string().to_string(),
//                 "application.id" => application_id = value.to_string().to_string(),
//                 "application.icon_name" => application_icon_name = value.to_string().to_string(),
//                 "application.process.id" => {
//                     application_process_id = value.to_string()
//                         .to_string()
//                         .parse()
//                         .map_err(|_| "Failed to parse application.process.id")
//                         .unwrap()
//                 }
//                 "application.process.user" => application_process_user = value.to_string().to_string(),
//                 "application.process.host" => application_process_host = value.to_string().to_string(),
//                 "application.process.binary" => application_process_binary = value.to_string().to_string(),
//                 "application.language" => application_language = value.to_string().to_string(),
//                 "window.x11.display" => window_x11_display = value.to_string().to_string(),
//                 _ => {
//                     // Ignore unknown keys
//                 }
//             }
//         }
//
//         AppInfo {
//             application_name,
//             native_protocol_peer,
//             native_protocol_version,
//             application_id,
//             application_icon_name,
//             application_process_id,
//             application_process_user,
//             application_process_host,
//             application_process_binary,
//             application_language,
//             window_x11_display,
//         }
//     }
// }

fn remove_first_and_last_char(s: &str) -> &str {
    if s.len() > 2 {
        &s[1..s.len() - 1] // remove first and last character using slice
    } else {
        "" // return empty string if the input string has less than 2 characters
    }
}

impl AppInfo {
    pub fn parse_from_str(client_info: &str) -> Result<Self, String> {
        let mut application_name = "".to_string();
        let mut native_protocol_peer = "".to_string();
        let mut native_protocol_version = "".to_string();
        let mut application_id = "".to_string();
        let mut application_icon_name = "".to_string();
        let mut application_process_id = 0;
        let mut application_process_user = "".to_string();
        let mut application_process_host = "".to_string();
        let mut application_process_binary = "".to_string();
        let mut application_language = "".to_string();
        let mut window_x11_display = "".to_string();

        let reader = BufReader::new(client_info.as_bytes());
        for line in reader.lines().skip(3).flatten() {
            let mut parts = line.splitn(2, '=');
            let key = parts.next().ok_or_else(|| "Failed to parse key")?.trim();
            let value = remove_first_and_last_char(
                parts.next().ok_or_else(|| "Failed to parse value")?.trim(),
            );

            println!("{key} {value}");

            match key {
                "application.name" => application_name = value.to_string(),
                "native-protocol.peer" => native_protocol_peer = value.to_string(),
                "native-protocol.version" => native_protocol_version = value.to_string(),
                "application.id" => application_id = value.to_string(),
                "application.icon_name" => application_icon_name = value.to_string(),
                "application.process.id" => {
                    application_process_id = value
                        .to_string()
                        .parse()
                        .map_err(|_| "Failed to parse application.process.id")?
                }
                "application.process.user" => application_process_user = value.to_string(),
                "application.process.host" => application_process_host = value.to_string(),
                "application.process.binary" => application_process_binary = value.to_string(),
                "application.language" => application_language = value.to_string(),
                "window.x11.display" => window_x11_display = value.to_string(),
                _ => {
                    // Ignore unknown keys
                }
            }
        }

        Ok(AppInfo {
            application_name,
            native_protocol_peer,
            native_protocol_version,
            application_id,
            application_icon_name,
            application_process_id,
            application_process_user,
            application_process_host,
            application_process_binary,
            application_language,
            window_x11_display,
        })
    }
}
#[test]
fn test_client_parser() {
    let client_info = r#"Driver: protocol-native.c
	Owner Module: 11
	Properties:
		application.name = "Plasma PA"
		native-protocol.peer = "UNIX socket client"
		native-protocol.version = "35"
		application.id = "org.kde.plasma-pa"
		application.icon_name = "audio-card"
		application.process.id = "43919"
		application.process.user = "tan"
		application.process.host = "arch"
		application.process.binary = "plasmashell"
		application.language = "C"
		window.x11.display = ":0"
		application.process.machine_id = "0242d735af52457ea911955030aff251"
		application.process.session_id = "2""#;
    let appinfo = AppInfo::parse_from_str(client_info);
    println!("{appinfo:#?}");
}
fn check_paudio_utils() -> bool {
    let op = std::process::Command::new("pactl")
        .arg("--version")
        .output();
    op.is_ok() && op.unwrap().status.success()
}
