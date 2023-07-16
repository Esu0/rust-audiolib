use ndarray::Array;
use std::fs::File;
use wav::{BitDepth, Header};

fn main() {
    let mut input = File::open("files/aozora.wav").expect("file open error");
    let (header, data) = wav::read(&mut input).expect("wav open error");

    let dat = Array::from(data.try_into_sixteen().expect("into sixteen error"));
    let mut left: Vec<i16> = if header.channel_count == 2 {
        dat.iter().copied().step_by(2).collect()
    } else {
        dat.to_vec()
    };

    left.iter_mut().for_each(|e| *e = e.saturating_mul(5));
    let mut output = File::create("aozora2.wav").expect("file open error");
    let new_head = Header::new(header.audio_format, 1, header.sampling_rate, 16);
    let bit_depth = BitDepth::Sixteen(left);
    wav::write(new_head, &bit_depth, &mut output).expect("wav_write_error");
}
