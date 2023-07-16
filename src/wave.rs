#![allow(dead_code)]
pub mod fft;

use ndarray::{s, Array, Array1, Array2};
use num_complex::Complex64;
const SAMP_FREQ: u16 = 44100;
// sampling frequency = 44100 Hz
// T_0 = 1 / 44100 s

pub struct Wave {
    inner: Array1<f64>,
}

pub struct Spectrogram {
    inner: Array2<Complex64>,
}

#[derive(Clone, Copy, Debug)]
/// 窓関数の種類
pub enum Window {
    /// 矩形窓
    Rectangular,
    /// ハン窓(ハニング窓)
    Hanning,
    /// ハミング窓
    Hamming,
    /// 一般化ハミング窓  
    /// パラメータは0.5以上1以下とします。
    GeneralizedHamming(f64),
    /// フラットトップ窓
    FlatTop,
    /// ブラックマン窓
    Blackman,
    /// サイン窓
    Sine,
    /// カイザー窓  
    /// パラメータには、1.5, 2, 3などが使われます。
    Kaiser(f64),
    /// ガウス窓  
    /// 確率密度関数を使います。  
    /// パラメータは標準偏差です。
    Gaussian(f64),
    /// バートレット窓(三角窓)
    Bartlett,
    /// バートレット - ハン窓
    BartlettHann,
    /// パルザン窓
    Parzen,
}

impl Window {
    pub fn to_wave(self, l: usize) -> Wave {
        let m = 2. * std::f64::consts::PI / (l - 1) as f64;
        match self {
            Self::Hanning => Wave {
                inner: (0..l).map(|n| 0.5 - 0.5 * (m * n as f64).cos()).collect(),
            },
            Self::Blackman => Wave {
                inner: (0..l)
                    .map(|n| {
                        let theta = m * n as f64;
                        0.42 - 0.5 * theta.cos() + 0.08 * (theta * 2.0).cos()
                    })
                    .collect(),
            },
            _ => Wave {
                inner: (0..l).map(|_| 1.0).collect(),
            },
        }
    }
}
impl Wave {
    pub fn sine(freq: f64, l: usize) -> Self {
        let m = freq * 2. * std::f64::consts::PI / SAMP_FREQ as f64;
        Self {
            inner: Array::from_shape_fn(l, |i| (m * i as f64).sin()),
        }
    }

    pub fn spectogram(&self, frame_n: u8, frame_t: usize) -> Spectrogram {
        let frame_len = 1 << frame_n;
        let w = Window::Blackman.to_wave(frame_len);
        let l = self.inner.len();
        let mut i = 0;
        let mut i2 = frame_len;
        let mut sp = Array::from_shape_fn((0, frame_len), |(_, _)| (0.).into());
        while i2 <= l {
            let w = fft::fft(
                self.inner
                    .slice(s![i..i2])
                    .iter()
                    .zip(w.inner.iter())
                    .map(|(x, w)| *x * *w),
                frame_n,
            );
            sp.push_row(w.view()).unwrap();
            i += frame_t;
            i2 += frame_t;
        }
        Spectrogram { inner: sp }
    }
}
