use wav_train::*;

fn main() -> hound::Result<()> {
    let mut reader = hound::WavReader::open("./misc/voice.wav")?;
    let spec = reader.spec();
    println!(
        "rate = {}, channels = {}, format = {:?}",
        spec.sample_rate, spec.channels, spec.sample_format
    );

    let mut samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.unwrap() as f32 / (i16::MAX as f32))
        .collect();

    let tone_up = tone_down(&mut samples, spec.sample_rate, 560, 1);

    let mut writer = hound::WavWriter::create(
        "./output.wav",
        hound::WavSpec {
            sample_format: hound::SampleFormat::Float,
            bits_per_sample: 32,
            channels: 2,
            ..spec
        },
    )?;
    for data in tone_up {
        writer.write_sample(data)?;
        writer.write_sample(data)?;
    }
    writer.finalize()?;
    // What the fuck why they are different so much!

    Ok(())
}
