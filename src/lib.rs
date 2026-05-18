use std::sync::LazyLock;

use realfft::RealFftPlanner;

// DOCUMENT: https://en.wikipedia.org/wiki/Piano_key_frequencies
static STEP_LOG2: LazyLock<f32> = LazyLock::new(|| 1. / 12.);

#[derive(Debug, Clone, Copy)]
pub struct PitchShiftPlan {
    window_size: usize,
    window_fn: WindowFn,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum WindowFn {
    Hann,
    #[default]
    None,
}

impl PitchShiftPlan {
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size,
            window_fn: WindowFn::None,
        }
    }
    pub fn with_windowfn(self, window_fn: WindowFn) -> Self {
        Self { window_fn, ..self }
    }

    pub fn shift(&self, samples: &mut [f32], shift: i32) -> Vec<f32> {
        let mut planner = RealFftPlanner::<f32>::new();
        let mut output = vec![];
        for data_clip in samples.chunks_mut(self.window_size) {
            let data_up = match self.window_fn {
                WindowFn::None => shift_inner(&mut planner, data_clip, shift),
                WindowFn::Hann => {
                    let mut window_data = hann_window(data_clip);
                    shift_inner(&mut planner, &mut window_data, shift)
                }
            };
            output.extend(data_up);
        }
        output
    }
}

fn hann_window(samples: &[f32]) -> Vec<f32> {
    let mut windowed_samples = Vec::with_capacity(samples.len());
    let samples_len = samples.len() as f32;
    for (i, sample) in samples.iter().enumerate() {
        let two_pi_i = 2.0 * std::f32::consts::PI * i as f32;
        let idontknowthename = (two_pi_i / samples_len).cos();
        let multiplier = 0.5 * (1.0 - idontknowthename);
        windowed_samples.push(sample * multiplier)
    }
    windowed_samples
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

    spectrum_up[spectrum.len() - 1].im = 0.;
    spectrum_up[0].im = 0.;
    let ifft = planner.plan_fft_inverse(data.len());
    let mut output = ifft.make_output_vec();
    ifft.process(&mut spectrum_up, &mut output).unwrap();
    output.iter_mut().for_each(|o| *o /= data_len as f32);

    output
}
