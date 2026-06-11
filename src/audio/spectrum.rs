use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use rustfft::{num_complex::Complex, FftPlanner};

pub const NUM_BANDS: usize = 64;
const FFT_SIZE: usize = 2048;

pub struct SpectrumAnalyzer {
    planner: FftPlanner<f32>,
    sample_buffer: Arc<Mutex<VecDeque<f32>>>,
}

impl SpectrumAnalyzer {
    pub fn new(sample_buffer: Arc<Mutex<VecDeque<f32>>>) -> Self {
        SpectrumAnalyzer {
            planner: FftPlanner::new(),
            sample_buffer,
        }
    }

    /// Retorna amplitudes normalizadas [0.0, 1.0] para cada uma das NUM_BANDS bandas.
    pub fn compute(&mut self) -> [f32; NUM_BANDS] {
        let samples: Vec<f32> = {
            let buf = self.sample_buffer.lock().unwrap();
            if buf.len() < FFT_SIZE {
                return [0.0; NUM_BANDS];
            }
            buf.iter().rev().take(FFT_SIZE).cloned().collect()
        };

        let fft = self.planner.plan_fft_forward(FFT_SIZE);

        // Janela de Hann para reduzir leakage espectral
        let mut input: Vec<Complex<f32>> = samples
            .iter()
            .enumerate()
            .map(|(i, &s)| {
                let window = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (FFT_SIZE - 1) as f32).cos());
                Complex { re: s * window, im: 0.0 }
            })
            .collect();

        fft.process(&mut input);

        // Apenas metade positiva do espectro
        let half = FFT_SIZE / 2;
        let magnitudes: Vec<f32> = input[..half]
            .iter()
            .map(|c| (c.re * c.re + c.im * c.im).sqrt() / FFT_SIZE as f32)
            .collect();

        // Agregar em NUM_BANDS bandas em escala logarítmica
        let mut bands = [0.0f32; NUM_BANDS];
        let log_min = (1f32).log2();
        let log_max = (half as f32).log2();

        for (i, band) in bands.iter_mut().enumerate() {
            let lo = log_min + (log_max - log_min) * i as f32 / NUM_BANDS as f32;
            let hi = log_min + (log_max - log_min) * (i + 1) as f32 / NUM_BANDS as f32;

            let idx_lo = (2f32.powf(lo) as usize).clamp(0, half - 1);
            let idx_hi = (2f32.powf(hi) as usize).clamp(idx_lo + 1, half);

            let sum: f32 = magnitudes[idx_lo..idx_hi].iter().sum();
            let count = (idx_hi - idx_lo).max(1) as f32;
            *band = sum / count;
        }

        // Normalizar para [0.0, 1.0] com escala dB
        let max_val = bands.iter().cloned().fold(0.0f32, f32::max).max(1e-10);
        for b in bands.iter_mut() {
            *b = (*b / max_val).clamp(0.0, 1.0);
        }

        bands
    }
}
