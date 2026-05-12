fn main() -> hound::Result<()> {
    let reader = hound::WavReader::open("./misc/voice.wav")?;
    let spec = reader.spec();
    println!("rate = {}, channels = {}", spec.sample_rate, spec.channels);
    Ok(())
}
