#![no_std]

#![feature(reverse_bits)]
#![feature(slice_patterns)]
#![feature(test)]

extern crate bin;
extern crate endian;

use core::mem;
use core::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Neighborhood(pub u8, pub bool);

mod grid;
pub use grid::*;

#[inline]
pub fn evolve<Rule: Fn(Neighborhood) -> bool>(src: &Grid, dst: &mut Grid, size: [usize; 2], rule: Rule) {
    assert!(mem::size_of_val(dst).checked_shl(3).unwrap() >=
            usize::checked_mul(size[0], size[1]).unwrap());
    assert!(mem::size_of_val(src).checked_shl(3).unwrap() >=
            usize::checked_mul(size[0], size[1]).unwrap());
    unsafe { evolve_unchecked(src, dst, size, rule) };
}

#[inline]
pub unsafe fn evolve_unchecked<Rule: Fn(Neighborhood) -> bool>(src: &Grid, dst: &mut Grid, size: [usize; 2], rule: Rule) {
    for y in 0..size[1] { for x in 0..size[0] {
        dst.set_unchecked(size, [x, y], rule(src.get_nbhd_unchecked(size, [x, y])))
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
        let src_raw_grid: [u8; 8] = [2, 2, 2, 0, 0, 0, 0, 0];
        let mut dst_raw_grid = [0u8; 8];
        evolve((&src_raw_grid[..]).into(), (&mut dst_raw_grid[..]).into(), [8, 8], conway_life);
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
