use wav_train::*;

fn main() -> hound::Result<()> {
    let mut reader = hound::WavReader::open("./misc/voice.wav")?;
    let spec = reader.spec();
    println!(
        "rate = {}, channels = {}, format = {:?}",
        spec.sample_rate, spec.channels, spec.sample_format
    );

    let mut samples: Vec<f32> = reader.samples::<i16>().map(|s| s.unwrap() as f32).collect();

    let tone_up = pitch_shift(&mut samples, 1024, -1);

    let mut writer = hound::WavWriter::create("./output.wav", spec)?;
    for data in tone_up {
        writer.write_sample(data as i16)?;
    }
    writer.finalize()?;

    Ok(())
}
