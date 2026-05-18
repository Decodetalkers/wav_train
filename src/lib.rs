use std::sync::LazyLock;

use realfft::RealFftPlanner;

// DOCUMENT: https://en.wikipedia.org/wiki/Piano_key_frequencies
static STEP_LOG2: LazyLock<f32> = LazyLock::new(|| 1. / 12.);

pub fn pitch_shift(data: &mut [f32], window: usize, shift: i32) -> Vec<f32> {
    let mut planner = RealFftPlanner::<f32>::new();
    let mut output = vec![];
    for data_clip in data.chunks_mut(window) {
        let data_up = shift_inner(&mut planner, data_clip, shift);
        output.extend(data_up);
    }
    output
}

fn shift_inner(planner: &mut RealFftPlanner<f32>, data: &mut [f32], shift: i32) -> Vec<f32> {
    let data_len = data.len();
    let fft = planner.plan_fft_forward(data_len);
    let mut spectrum = fft.make_output_vec();
    fft.process(data, &mut spectrum).unwrap();
    let mut spectrum_up = vec![realfft::num_complex::Complex { re: 0., im: 0. }; spectrum.len()];
    for (index, complex) in spectrum.iter().enumerate() {
        let shift_log2_index = (shift as f32) * *STEP_LOG2 + (index as f32).log2();
        let shift_index = 2_f32.powf(shift_log2_index) as usize;
        if shift_index >= spectrum.len() {
            spectrum_up[index] += 0.1 * complex;
            continue;
        }
        spectrum_up[shift_index] += complex;
    }

    spectrum_up.last_mut().unwrap().im = 0.;
    spectrum_up[0].im = 0.;
    let ifft = planner.plan_fft_inverse(data.len());
    let mut output = ifft.make_output_vec();
    ifft.process(&mut spectrum_up, &mut output).unwrap();
    output.iter_mut().for_each(|o| *o /= data_len as f32);

    output
}
