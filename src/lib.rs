use std::sync::LazyLock;

use realfft::RealFftPlanner;

// DOCUMENT: https://en.wikipedia.org/wiki/Piano_key_frequencies
static STEP: LazyLock<f32> = LazyLock::new(|| 2_f32.powf(1. / 12.));

pub fn tone_up(data: &mut [f32], rate: u32, window: usize, tone: usize) -> Vec<f32> {
    let mut planner = RealFftPlanner::<f32>::new();
    let mut output = vec![];
    for data_clip in data.chunks_mut(window) {
        let freq = (rate as f32 / 2.) / data_clip.len() as f32;
        let data_up = tone_up_inner(&mut planner, data_clip, freq, tone);
        output.extend(data_up);
    }
    output
}

fn tone_up_inner(
    planner: &mut RealFftPlanner<f32>,
    data: &mut [f32],
    freq_step: f32,
    tone: usize,
) -> Vec<f32> {
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
    let mut shift = ((max_freq / *STEP) / freq_step) as u32 * tone as u32;
    while shift > 0 {
        spectrum.insert(0, realfft::num_complex::Complex { re: 0., im: 0. });
        spectrum.pop();
        shift -= 1;
    }

    spectrum.last_mut().unwrap().im = 0.;
    let ifft = planner.plan_fft_inverse(data.len());
    let mut output = ifft.make_output_vec();
    ifft.process(&mut spectrum, &mut output).unwrap();
    output.iter_mut().for_each(|o| *o /= data_len as f32);

    output
}
pub fn tone_down(data: &mut [f32], rate: u32, window: usize, tone: usize) -> Vec<f32> {
    let mut planner = RealFftPlanner::<f32>::new();
    let mut output = vec![];
    for data_clip in data.chunks_mut(window) {
        let freq = (rate as f32 / 2.) / data_clip.len() as f32;
        let data_up = tone_down_inner(&mut planner, data_clip, freq, tone);
        output.extend(data_up);
    }
    output
}

fn tone_down_inner(
    planner: &mut RealFftPlanner<f32>,
    data: &mut [f32],
    freq_step: f32,
    tone: usize,
) -> Vec<f32> {
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
    let mut shift = ((max_freq / *STEP) / freq_step) as u32 * tone as u32;
    while shift > 0 {
        spectrum.push(realfft::num_complex::Complex { re: 0., im: 0. });
        spectrum.remove(0);
        shift -= 1;
    }

    spectrum[0].im = 0.;
    let ifft = planner.plan_fft_inverse(data.len());
    let mut output = ifft.make_output_vec();
    ifft.process(&mut spectrum, &mut output).unwrap();
    output.iter_mut().for_each(|o| *o /= data_len as f32);

    output
}
