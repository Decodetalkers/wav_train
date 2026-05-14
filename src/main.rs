use realfft::RealFftPlanner;

fn main() -> hound::Result<()> {
    let mut reader = hound::WavReader::open("./misc/voice.wav")?;
    let spec = reader.spec();
    println!(
        "rate = {}, channels = {}, format = {:?}",
        spec.sample_rate, spec.channels, spec.sample_format
    );

    let mut planner: RealFftPlanner<f32> = RealFftPlanner::new();

    let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();
    let mut samples: Vec<f32> = samples[0..256].iter().map(|s| *s as f32).collect();
    //println!("original samples = {samples:?}");
    let fft = planner.plan_fft_forward(samples.len());
    let mut spectrum = fft.make_output_vec();

    fft.process(&mut samples, &mut spectrum).unwrap();

    println!("after fft: {spectrum:?}");

    let ifft = planner.plan_fft_inverse(samples.len());
    let mut data_i = ifft.make_output_vec();

    let n = data_i.len() as f32;

    ifft.process(&mut spectrum, &mut data_i).unwrap();

    let _data_r: Vec<f32> = data_i.iter().map(|s| *s / n).collect();
    //println!("use ifft to return it: {data_r:?}");

    // What the fuck why they are different so much!

    Ok(())
}
