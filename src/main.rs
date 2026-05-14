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

    let mut planner: RealFftPlanner<f32> = RealFftPlanner::new();

    let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();
    let mut samples: Vec<f32> = samples[0..256].iter().map(|s| *s as f32).collect();
    //println!("original samples = {samples:?}");
    let fft = planner.plan_fft_forward(samples.len());
    let mut spectrum = fft.make_output_vec();

    println!("samples len: {}", samples.len());
    fft.process(&mut samples, &mut spectrum).unwrap();

    let norm_data: Vec<f32> = spectrum.iter().map(|a| a.norm()).collect();
    println!("after fft len: {}", norm_data.len());

    let ifft = planner.plan_fft_inverse(samples.len());
    let mut data_i = ifft.make_output_vec();

    let n = data_i.len() as f32;

    ifft.process(&mut spectrum, &mut data_i).unwrap();

    let data_r: Vec<f32> = data_i.iter().map(|s| *s / n).collect();
    println!("use ifft to return it: {}", data_r.len());

    // What the fuck why they are different so much!

    Ok(())
}
