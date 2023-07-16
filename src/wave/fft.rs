use ndarray::Array1;
use num_complex::Complex64;

/// dat.len() must be 2^n
pub fn fft_inplace(dat: &mut [Complex64], n: u8) {
    bit_rev(dat, n);
    for i in 0..n {
        let ch_size = 2 << i;
        let w0 = Complex64::from_polar(1., 2. * std::f64::consts::PI / ch_size as f64);
        for slc in dat.chunks_exact_mut(ch_size) {
            let (slc1, slc2) = slc.split_at_mut(slc.len() / 2);
            let mut w = Complex64::from(1.);
            for (a, b) in slc1.iter_mut().zip(slc2) {
                *b *= w;
                let tmp = *a;
                *a += *b;
                *b = tmp - *b;
                w *= w0;
            }
        }
    }
}

/// dat.len() must be 2^n
pub fn bit_rev(dat: &mut [Complex64], n: u8) {
    for i in 1..(dat.len() - 1) {
        let j = i.reverse_bits() >> (usize::BITS - n as u32);
        if i > j {
            dat.swap(i, j);
        }
    }
}

pub fn fft<T>(dat: impl Iterator<Item = T>, n: u8) -> Array1<Complex64>
where
    T: num_traits::Float,
{
    let mut buf: Vec<Complex64> = dat
        .map(|e| Complex64::from(e.to_f64().unwrap_or_else(|| 0.)))
        .collect();
    fft_inplace(&mut buf, n);
    buf.into()
}
