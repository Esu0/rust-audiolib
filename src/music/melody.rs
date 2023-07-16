#![allow(dead_code)]
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug)]
pub struct Code(u128);
#[derive(Clone, Debug, Default)]
pub struct Melody {
    codes: Vec<Code>,
}

#[derive(Clone, Debug)]
pub struct FreqList(u128);

impl Code {
    const A0: Code = Code(0x0010000);
    const B0: Code = Code(0x0040000);
    const C0: Code = Code(0x0100000);
    const D0: Code = Code(0x0200000);
    const E0: Code = Code(0x0800000);
    const F0: Code = Code(0x2000000);
    const G0: Code = Code(0x8000000);
    const A1: Code = Code(0x0010000000);
    const B1: Code = Code(0x0040000000);
    const C1: Code = Code(0x0100000000);
    const D1: Code = Code(0x0200000000);
    const E1: Code = Code(0x0800000000);
    const F1: Code = Code(0x2000000000);
    const G1: Code = Code(0x8000000000);
    const A2: Code = Code(0x0010000000000);
    const B2: Code = Code(0x0040000000000);
    const C2: Code = Code(0x0100000000000);
    const D2: Code = Code(0x0200000000000);
    const E2: Code = Code(0x0800000000000);
    const F2: Code = Code(0x2000000000000);
    const G2: Code = Code(0x8000000000000);
    const A3: Code = Code(0x0010000000000000);
    const B3: Code = Code(0x0040000000000000);
    const C3: Code = Code(0x0100000000000000);
    const D3: Code = Code(0x0200000000000000);
    const E3: Code = Code(0x0800000000000000);
    const F3: Code = Code(0x2000000000000000);
    const G3: Code = Code(0x8000000000000000);
    const A4: Code = Code(0x0010000000000000000);
    const B4: Code = Code(0x0040000000000000000);
    const C4: Code = Code(0x0100000000000000000);
    const D4: Code = Code(0x0200000000000000000);
    const E4: Code = Code(0x0800000000000000000);
    const F4: Code = Code(0x2000000000000000000);
    const G4: Code = Code(0x8000000000000000000);
    const A5: Code = Code(0x0010000000000000000000);
    const B5: Code = Code(0x0040000000000000000000);
    const C5: Code = Code(0x0100000000000000000000);
    const D5: Code = Code(0x0200000000000000000000);
    const E5: Code = Code(0x0800000000000000000000);
    const F5: Code = Code(0x2000000000000000000000);
    const G5: Code = Code(0x8000000000000000000000);
    const A6: Code = Code(0x0010000000000000000000000);
    const B6: Code = Code(0x0040000000000000000000000);
    const C6: Code = Code(0x0100000000000000000000000);
    const D6: Code = Code(0x0200000000000000000000000);
    const E6: Code = Code(0x0800000000000000000000000);
    const F6: Code = Code(0x2000000000000000000000000);
    const G6: Code = Code(0x8000000000000000000000000);
    const A7: Code = Code(0x0010000000000000000000000000);
    const B7: Code = Code(0x0040000000000000000000000000);
    const C7: Code = Code(0x0100000000000000000000000000);
    const D7: Code = Code(0x0200000000000000000000000000);
    const E7: Code = Code(0x0800000000000000000000000000);
    const F7: Code = Code(0x2000000000000000000000000000);
    const G7: Code = Code(0x8000000000000000000000000000);
    const A8: Code = Code(0x0010000000000000000000000000000);
    const B8: Code = Code(0x0040000000000000000000000000000);
    const C8: Code = Code(0x0100000000000000000000000000000);
    const D8: Code = Code(0x0200000000000000000000000000000);
    const E8: Code = Code(0x0800000000000000000000000000000);
    const F8: Code = Code(0x2000000000000000000000000000000);
    const G8: Code = Code(0x8000000000000000000000000000000);
    const A: Code = Code::A4;
    const B: Code = Code::B4;
    const C: Code = Code::C4;
    const D: Code = Code::D4;
    const E: Code = Code::E4;
    const F: Code = Code::F4;
    const G: Code = Code::G4;

    pub const fn sharp(self) -> Self {
        Self(self.0 << 1)
    }

    pub const fn flat(self) -> Self {
        Self(self.0 >> 1)
    }

    pub const fn double_sharp(self) -> Self {
        Self(self.0 << 2)
    }

    pub const fn double_flat(self) -> Self {
        Self(self.0 >> 2)
    }

    pub const fn frequency(self) -> FreqList {
        FreqList(self.0)
    }
}

impl FreqList {
    fn next_freq(&mut self) -> Option<f64> {
        let zeros = self.0.leading_zeros();
        if zeros >= 128 {
            None
        } else {
            self.0 &= std::u128::MAX >> (zeros + 1);
            Some(440. * ((63. - zeros as f64) / 12.).exp2())
        }
    }
}

impl Iterator for FreqList {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_freq()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.count_ones() as _
    }
}

impl Add for Code {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl AddAssign for Code {
    fn add_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
}
