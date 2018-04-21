use self::Neighborhood::*;

#[inline]
fn reflect(x: u8) -> u8 { x.rotate_left(1).reverse_bits() }

#[inline]
pub(crate) fn isotropicize(x: u8) -> u8 {
    (0..4).map(|k| ::core::cmp::min(x.rotate_left(k << 1), reflect(x).rotate_left(k << 1))).min().unwrap_or(x)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Neighborhood {
    N0, N1c, N1e, N2c, N2e, N2k, N2a, N2i, N2n,
    N3c, N3e, N3k, N3a, N3i, N3n, N3y, N3q, N3j, N3r,
    N4c, N4e, N4k, N4a, N4i, N4n, N4y, N4q, N4j, N4r, N4t, N4w, N4z,
    N5c, N5e, N5k, N5a, N5i, N5n, N5y, N5q, N5j, N5r,
    N6c, N6e, N6k, N6a, N6i, N6n, N7c, N7e, N8,
}

impl From<u8> for Neighborhood {
    #[inline]
    fn from(x: u8) -> Self { match isotropicize(x) {
        0x00 => N0,
        0x01 => N1c,
        0x02 => N1e,
        0x05 => N2c,
        0x0A => N2e,
        0x09 => N2k,
        0x03 => N2a,
        0x22 => N2i,
        0x11 => N2n,
        0x15 => N3c,
        0x2A => N3e,
        0x29 => N3k,
        0x0E => N3a,
        0x07 => N3i,
        0x0D => N3n,
        0x25 => N3y,
        0x13 => N3q,
        0x0B => N3j,
        0x23 => N3r,
        0x55 => N4c,
        0xAA => N4e,
        0x2D => N4k,
        0x0F => N4a,
        0x36 => N4i,
        0x17 => N4n,
        0x35 => N4y,
        0x39 => N4q,
        0x2B => N4j,
        0x2E => N4r,
        0x27 => N4t,
        0x1B => N4w,
        0x33 => N4z,
        x => !Self::from(!x)
    } }
}

impl ::core::ops::Not for Neighborhood {
    type Output = Self;

    #[inline]
    fn not(self) -> Self { match self {
        N0 => N8,
        N1c => N7c,
        N1e => N7e,
        N2c => N6c,
        N2e => N6e,
        N2k => N6k,
        N2a => N6a,
        N2i => N6i,
        N2n => N6n,
        N3c => N5c,
        N3e => N5e,
        N3k => N5k,
        N3a => N5a,
        N3i => N5i,
        N3n => N5n,
        N3y => N5y,
        N3q => N5q,
        N3j => N5j,
        N3r => N5r,
        N4c => N4e,
        N4e => N4c,
        N4k => N4k,
        N4a => N4a,
        N4i => N4t,
        N4n => N4r,
        N4y => N4j,
        N4q => N4w,
        N4j => N4y,
        N4r => N4n,
        N4t => N4i,
        N4w => N4q,
        N4z => N4z,
        N5c => N3c,
        N5e => N3e,
        N5k => N3k,
        N5a => N3a,
        N5i => N3i,
        N5n => N3n,
        N5y => N3y,
        N5q => N3q,
        N5j => N3j,
        N5r => N3r,
        N6c => N2c,
        N6e => N2e,
        N6k => N2k,
        N6a => N2a,
        N6i => N2i,
        N6n => N2n,
        N7c => N1c,
        N7e => N1e,
        N8 => N0,
    } }
}

#[inline]
pub fn evolve<Rule: Fn(Neighborhood, bool) -> bool>(src: &::Grid, dst: &mut ::Grid, size: [usize; 2], rule: Rule) {
    ::evolve(src, dst, size, |::Neighborhood(xs, x)| rule(Neighborhood::from(xs), x))
}
