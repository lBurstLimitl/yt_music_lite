mod player;
mod yt;

use std::io::{stdout, Write};
use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};
use std::io;

fn main() -> io::Result<()> {
    execute!(stdout(), EnterAlternateScreen)?;
    println!("YouTube Music Lite – minimal RAM usage");
    print!("Search or paste YouTube URL: ");
    stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let query = input.trim();

    if let Some(url) = yt::get_audio_url(query) {
        println!("\nStreaming from: {}", url);
        if let Err(e) = player::play_from_url(&url) {
            eprintln!("Error during playback: {}", e);
        }
    } else {
        eprintln!("❌ Failed to retrieve audio stream.");
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}