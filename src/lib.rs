#![no_std]

#![feature(reverse_bits)]
#![feature(slice_patterns)]
#![feature(test)]

extern crate bin;
extern crate endian;

use bin::slice::Bits;
use core::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Neighborhood(pub u8, pub bool);

pub struct Grid(Bits<::endian::Lil>);

#[inline]
fn linearize<A: Copy + Add<Output = A> + Mul<Output = A> + Rem<Output = A>>(w: A, h: A, x: A, y: A) -> A { modulo(x, w) + modulo(y, h) * w }

#[inline]
fn modulo<A: Copy + Add<Output = A> + Rem<Output = A>>(a: A, n: A) -> A { (a + n) % n }

impl Grid {
    #[inline]
    pub fn get_nbhd(&self, [w, h]: [usize; 2], [x, y]: [usize; 2]) -> Neighborhood {
        Neighborhood((self.0.get(linearize(w, h, x+1, y+1) as _).unwrap() as u8) << 0 |
                     (self.0.get(linearize(w, h, x+0, y+1) as _).unwrap() as u8) << 1 |
                     (self.0.get(linearize(w, h, x-1, y+1) as _).unwrap() as u8) << 2 |
                     (self.0.get(linearize(w, h, x-1, y+0) as _).unwrap() as u8) << 3 |
                     (self.0.get(linearize(w, h, x-1, y-1) as _).unwrap() as u8) << 4 |
                     (self.0.get(linearize(w, h, x+0, y-1) as _).unwrap() as u8) << 5 |
                     (self.0.get(linearize(w, h, x+1, y-1) as _).unwrap() as u8) << 6 |
                     (self.0.get(linearize(w, h, x+1, y+0) as _).unwrap() as u8) << 7,
                     self.0.get(linearize(w, h, x, y) as _).unwrap())
    }

    #[inline]
    pub fn set(&mut self, [w, h]: [usize; 2], [x, y]: [usize; 2], b: bool) {
        self.0.modify(linearize(w, h, x, y), |_| b);
    }
}

#[inline]
pub fn evolve<Rule: Fn(Neighborhood) -> bool>(src: &Grid, dst: &mut Grid, size: [usize; 2], rule: Rule) {
    for y in 0..size[1] { for x in 0..size[0] {
        dst.set(size, [x, y], rule(src.get_nbhd(size, [x, y])))
    } }
}

#[inline]
pub fn hexagonalize(x: u8) -> u8 { (x >> 1 & 7) << 0 | (x >> 5 & 7) << 3 }

pub mod hex;
pub mod moore;

#[inline]
pub fn conway_life(Neighborhood(xs, x): Neighborhood) -> bool { 3 == xs.count_ones() | x as u32 }

#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate test;

    use self::rand::Rng;
    use self::test::Bencher;

    use super::*;

    #[test]
    fn conway_life_blinker() {
        use core::mem;
        let src_raw_grid: [u8; 8] = [2, 2, 2, 0, 0, 0, 0, 0];
        let mut dst_raw_grid = [0u8; 8];
        evolve(unsafe { mem::transmute(&src_raw_grid[..]) }, unsafe { mem::transmute(&mut dst_raw_grid[..]) }, [8; 2], conway_life);
        assert_eq!([0, 7, 0, 0, 0, 0, 0, 0], dst_raw_grid);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        use core::mem;
        let mut raw_grids: [[u8; 512]; 2] = unsafe { mem::uninitialized() };
        rand::thread_rng().fill_bytes(&mut raw_grids[0][..]);
        let mut which = false;
        b.iter(|| {
            evolve(unsafe { mem::transmute(&raw_grids[which as usize][..]) },
                   unsafe { mem::transmute(&mut raw_grids[!which as usize][..]) },
                   [64; 2], conway_life);
            which ^= true;
            raw_grids[which as usize]
        });
    }
}
