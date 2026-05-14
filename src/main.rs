use realfft::num_traits::ToPrimitive;
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
        .map(|s| s.unwrap().to_f32().unwrap())
        .collect();

    let tone_up = tone_down(&mut samples, spec.sample_rate, 560, 0);

    let mut writer = hound::WavWriter::create("./output.wav", spec)?;
    for data in tone_up {
        writer.write_sample(data.to_i16().unwrap())?;
    }
    writer.finalize()?;
    // What the fuck why they are different so much!

    Ok(())
}
