use std::sync::LazyLock;

use realfft::RealFftPlanner;

// DOCUMENT: https://en.wikipedia.org/wiki/Piano_key_frequencies
static STEP: LazyLock<f32> = LazyLock::new(|| 2_f32.powf(1. / 12.));

fn tone_up_one_all(data: &mut [f32], rate: u32, window: usize) -> Vec<f32> {
    let mut planner = RealFftPlanner::<f32>::new();
    let mut output = vec![];
    for data_clip in data.chunks_mut(window) {
        let freq = rate as f32 / data_clip.len() as f32;
        let data_up = tone_up_one(&mut planner, data_clip, freq);
        output.extend(data_up);
    }
    output
}

fn tone_up_one(planner: &mut RealFftPlanner<f32>, data: &mut [f32], freq_step: f32) -> Vec<f32> {
    let data_len = data.len();
    let fft = planner.plan_fft_forward(data_len);
    let mut spectrum = fft.make_output_vec();
    fft.process(data, &mut spectrum).unwrap();
    let mut max_freq = 0.;
    let mut am_max = 0.;
    for (i, complex) in spectrum.iter().enumerate() {
        let am = complex.norm();
        if am > am_max {
            am_max = am;
            max_freq = (i as f32) * freq_step;
        }
    }
    let mut shift = ((max_freq / *STEP) / freq_step) as u32;
    while shift > 0 {
        spectrum.insert(0, realfft::num_complex::Complex { re: 0., im: 0. });
        spectrum.pop();
        shift -= 1;
    }
    spectrum.last_mut().unwrap().im = 0.;
    let ifft = planner.plan_fft_inverse(data.len());
    let mut output = ifft.make_output_vec();
    ifft.process(&mut spectrum, &mut output).unwrap();

    output
}

fn main() -> hound::Result<()> {
    let mut reader = hound::WavReader::open("./misc/voice.wav")?;
    let spec = reader.spec();
    println!(
        "rate = {}, channels = {}, format = {:?}",
        spec.sample_rate, spec.channels, spec.sample_format
    );

    let mut samples: Vec<f32> = reader.samples::<i16>().map(|s| s.unwrap() as f32).collect();

    let tone_up = tone_up_one_all(&mut samples, spec.sample_rate, 240);

    let mut writer = hound::WavWriter::create("./output.wav", spec)?;
    for data in tone_up {
        writer.write_sample(data as i16)?;
    }
    writer.finalize()?;
    // What the fuck why they are different so much!

    Ok(())
}
