use std::ops::{self};
use std::fmt;

/**
 * Describes an integer in a Z/pZ finite field.
 * Contains the value of the integer, bounded by P.
 */
#[derive(Clone, Copy)]
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
            value: val.rem_euclid(P)
        }
    }

    fn egcd(a: i64, b: i64 ) -> (i64, i64, i64) {
        let mut a = a;
        let mut b = b;
        
        if a.abs() > b.abs() {
            return Self::egcd(b, a);
        }

        if b.abs() == 0 {
            return (1, 0, a);
        }

        let mut x1 = 0;
        let mut x2 = 1;
        let mut y1 = 1;
        let mut y2 = 0;

        while b.abs() > 0 {
            let (q, r) = (a/b, a.rem_euclid(b));
            let x = x2 - q*x1;
            let y = y2 - q*y1;
            (a, b, x2, x1, y2, y1) = (b, r, x1, x, y1, y);
        }

        return (x2, y2, a);
    }

    pub fn inverse(&self) -> FFInt<P> {
        let (x, _, _) = Self::egcd(self.value, P);
        FFInt::<P>::new(x)
    }

    pub fn divmod(a: FFInt<P>, b: FFInt<P>) -> (FFInt<P>, FFInt<P>) {
        let (q, r) = (a.value/b.value, a.value.rem_euclid(b.value));
        (FFInt::<P>::new(q), FFInt::<P>::new(r))
    }
}

impl<const P: i64> ops::Add<FFInt<P>> for FFInt<P> {
    type Output = FFInt<P>;

    fn add(self, rhs: FFInt<P>) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl<const P: i64> ops::Mul<FFInt<P>> for FFInt<P> {
    type Output = FFInt<P>;

    fn mul(self, rhs: FFInt<P>) -> Self::Output {
        Self::new(self.value * rhs.value)
    }
}

impl<const P: i64> ops::Sub<FFInt<P>> for FFInt<P> {
    type Output = FFInt<P>;

    fn sub(self, rhs: FFInt<P>) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

impl<const P: i64> ops::Neg for FFInt<P> {
    type Output = FFInt<P>;

    fn neg(self) -> Self::Output {
        Self::new(-self.value)
    }
}

impl<const P: i64> ops::Div<FFInt<P>> for FFInt<P> {
    type Output = FFInt<P>;

    fn div(self, rhs: FFInt<P>) -> Self::Output {
        self * rhs.inverse()
    }
}

impl<const P: i64> ops::Rem<FFInt<P>> for FFInt<P> {
    type Output = FFInt<P>;

    fn rem(self, rhs: FFInt<P>) -> Self::Output {
        Self::new(self.value % rhs.value)
    }
}