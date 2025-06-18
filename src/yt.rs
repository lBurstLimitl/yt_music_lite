use std::process::Command;

pub fn get_audio_url(query: &str) -> Option<String> {
    let output = Command::new("yt-dlp")
        .args(["-f", "bestaudio", "-g", query])
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}