use bin::slice::Bits;
use core::ops::*;

use Neighborhood;

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
