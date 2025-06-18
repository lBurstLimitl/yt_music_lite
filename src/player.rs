use rodio::{Decoder, OutputStream, Sink};
use std::io::BufReader;

pub fn play_from_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;

    let resp = ureq::get(url).call().into_reader();
    let source = Decoder::new(BufReader::new(resp))?;
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}