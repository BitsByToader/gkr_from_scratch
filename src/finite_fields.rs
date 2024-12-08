use std::ops::{self};
use std::fmt;

/**
 * Describes an integer in a Z/pZ finite field.
 * Contains the value of the integer, bounded by P.
 */
 pub struct FFInt<const P: i64> {
    pub value: i64
}

impl<const P: i64> fmt::Debug for FFInt<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FFInt")
        .field("P", &P)
        .field("value", &self.value)
        .finish()
    }
}

impl<const P: i64> FFInt<P> {
    pub fn new(val: i64) -> FFInt<P> {
        FFInt::<P> {
            value: val % P
        }
    }

    // TODO: inverse.
    pub fn inverse(self) -> FFInt<P> {
        todo!("Implement inverse of FFInt.")
    }
}

impl<const P: i64> ops::Add<FFInt<P>> for FFInt<P> {
    type Output = FFInt<P>;

    fn add(self, rhs: FFInt<P>) -> Self::Output {
        FFInt::<P> {
            value: (self.value + rhs.value) % P
        }
    }
}

impl<const P: i64> ops::Mul<FFInt<P>> for FFInt<P> {
    type Output = FFInt<P>;

    fn mul(self, rhs: FFInt<P>) -> Self::Output {
        FFInt::<P> {
            value: (self.value * rhs.value) % P
        }
    }
}

impl<const P: i64> ops::Sub<FFInt<P>> for FFInt<P> {
    type Output = FFInt<P>;

    fn sub(self, rhs: FFInt<P>) -> Self::Output {
        FFInt::<P> {
            value: (self.value - rhs.value) % P
        }
    }
}

impl<const P: i64> ops::Neg for FFInt<P> {
    type Output = FFInt<P>;

    // TODO: Possibly broken???
    fn neg(self) -> Self::Output {
        FFInt::<P> {
            value: (-self.value) % P
        }
    }
}

impl<const P: i64> ops::Div<FFInt<P>> for FFInt<P> {
    type Output = FFInt<P>;

    fn div(self, rhs: FFInt<P>) -> Self::Output {
        self * rhs.inverse()
    }
}