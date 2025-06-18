use std::io::{Cursor, Read};
use rodio::{Decoder, OutputStream, Sink};
use ureq;

fn play_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Get the audio data from the network
    let mut response = ureq::get(url).call()?.into_reader();

    // Read it all into a buffer
    let mut buffer = Vec::new();
    response.read_to_end(&mut buffer)?;

    // Wrap it in a Cursor so it can be Seeked
    let cursor = Cursor::new(buffer);

    // Set up Rodio
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    // Decode and play
    let source = Decoder::new(cursor)?;
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
