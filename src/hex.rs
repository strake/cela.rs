use core::{convert::{TryFrom, TryInto}, ops::Not};

use self::Neighborhood::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Neighborhood { N0, N1, N2o, N2m, N2p, N3o, N3m, N3p, N4o, N4m, N4p, N5, N6 }

impl Neighborhood {
    #[inline]
    pub fn from_moore(x: u8) -> Self { ::hexagonalize(x).try_into().unwrap() }
}

impl TryFrom<u8> for Neighborhood {
    type Error = ();

    #[inline]
    fn try_from(x: u8) -> Result<Self, ()> { match ::moore::isotropicize(::hexagonalize(x)) {
        0x00 => Ok(N0),
        0x01 => Ok(N1),
        0x03 => Ok(N2o),
        0x05 => Ok(N2m),
        0x09 => Ok(N2p),
        0x07 => Ok(N3o),
        0x0B => Ok(N3m),
        0x15 => Ok(N3p),
        _ if 0 == x & 0xC0 => Self::try_from(!x & 0x3F).map(Self::not),
        _ => Err(())
    } }
}

impl Not for Neighborhood {
    type Output = Self;

    #[inline]
    fn not(self) -> Self { match self {
        N0 => N6,
        N1 => N5,
        N2o => N4o,
        N2m => N4m,
        N2p => N4p,
        N3o => N3o,
        N3m => N3m,
        N3p => N3p,
        N4o => N2o,
        N4m => N2m,
        N4p => N2p,
        N5 => N1,
        N6 => N0,
    } }
}

#[inline]
pub fn evolve<Rule: Fn(Neighborhood, bool) -> bool>(src: &::Grid, dst: &mut ::Grid, size: [usize; 2], rule: Rule) {
    ::evolve(src, dst, size, |::Neighborhood(xs, x)| rule(Neighborhood::from_moore(xs), x))
}
