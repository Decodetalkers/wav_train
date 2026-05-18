use std::sync::LazyLock;

use realfft::RealFftPlanner;

// DOCUMENT: https://en.wikipedia.org/wiki/Piano_key_frequencies
static STEP: LazyLock<f32> = LazyLock::new(|| 2_f32.powf(1. / 12.));

pub fn tone_up(data: &mut [f32], window: usize, tone: usize) -> Vec<f32> {
    let mut planner = RealFftPlanner::<f32>::new();
    let mut output = vec![];
    for data_clip in data.chunks_mut(window) {
        let data_up = tone_up_inner(&mut planner, data_clip, tone);
        output.extend(data_up);
    }
    output
}

fn tone_up_inner(planner: &mut RealFftPlanner<f32>, data: &mut [f32], tone: usize) -> Vec<f32> {
    let data_len = data.len();
    let fft = planner.plan_fft_forward(data_len);
    let mut spectrum = fft.make_output_vec();
    fft.process(data, &mut spectrum).unwrap();
    let mut spectrum_up = vec![realfft::num_complex::Complex { re: 0., im: 0. }; spectrum.len()];
    for (index, complex) in spectrum.iter().enumerate() {
        let shift_index = (index as f32 * (*STEP).powi(tone as i32)) as usize;
        if shift_index >= spectrum.len() {
            spectrum_up[index] += 0.1 * complex;
            continue;
        }
        spectrum_up[shift_index] += complex;
    }

    spectrum_up.last_mut().unwrap().im = 0.;
    let ifft = planner.plan_fft_inverse(data.len());
    let mut output = ifft.make_output_vec();
    ifft.process(&mut spectrum_up, &mut output).unwrap();
    output.iter_mut().for_each(|o| *o /= data_len as f32);

    output
}
pub fn tone_down(data: &mut [f32], window: usize, tone: usize) -> Vec<f32> {
    let mut planner = RealFftPlanner::<f32>::new();
    let mut output = vec![];
    for data_clip in data.chunks_mut(window) {
        let data_up = tone_down_inner(&mut planner, data_clip, tone);
        output.extend(data_up);
    }
    output
}

fn tone_down_inner(planner: &mut RealFftPlanner<f32>, data: &mut [f32], tone: usize) -> Vec<f32> {
    let data_len = data.len();
    let fft = planner.plan_fft_forward(data_len);
    let mut spectrum = fft.make_output_vec();
    fft.process(data, &mut spectrum).unwrap();
    let mut spectrum_down = vec![realfft::num_complex::Complex { re: 0., im: 0. }; spectrum.len()];
    for (index, complex) in spectrum.iter().enumerate() {
        let shift_index = (index as f32 / (*STEP).powi(tone as i32)) as usize;
        spectrum_down[shift_index] += complex;
    }

    spectrum_down[0].im = 0.;
    let ifft = planner.plan_fft_inverse(data.len());
    let mut output = ifft.make_output_vec();
    ifft.process(&mut spectrum_down, &mut output).unwrap();
    output.iter_mut().for_each(|o| *o /= data_len as f32);

    output
}
