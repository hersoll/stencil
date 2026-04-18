use super::Number;

impl Number {
    pub fn pow(&self, exponent: Number) -> Number {
        match (self, exponent) {
            // Integer ^ Integer
            (Number::Integer(base), Number::Integer(exp)) => {
                if exp >= 0 {
                    Number::Integer(base.pow(exp as u32))
                } else {
                    // e.g. 2^-3 = 1/8
                    Number::Fraction(1, base.pow((-exp) as u32))
                }
            }

            // Fraction ^ Integer
            (Number::Fraction(num, denom), Number::Integer(exp)) => {
                if exp >= 0 {
                    let e = exp as u32;
                    Number::Fraction(num.pow(e), denom.pow(e))
                } else {
                    // (a/b)^-n = b^n / a^n
                    let e = (-exp) as u32;
                    Number::Fraction(denom.pow(e), num.pow(e))
                }
            }

            // Integer ^ Fraction — may produce an irrational (e.g. 2^(1/2) = √2)
            (Number::Integer(base), Number::Fraction(num, denom)) => {
                // Only handle the common nth-root case: base^(1/n)
                if num == 1 {
                    let result = (*base as f64).powf(1.0 / denom as f64);
                    let display = format!("{}^(1/{denom})", base);
                    // Leak is acceptable here since these are computed once; alternatively
                    // keep a lookup table for common cases.
                    Number::Irrational(result, Box::leak(display.into_boxed_str()))
                } else {
                    Number::from((*base as f64).powf(num as f64 / denom as f64))
                }
            }

            // Irrational ^ Integer — keep the symbolic label alive
            (Number::Irrational(val, label), Number::Integer(exp)) => {
                let result = val.powi(exp);
                let display = format!("({label})^{exp}");
                Number::Irrational(result, Box::leak(display.into_boxed_str()))
            }

            // Everything else: fall back to f64
            (l_val, r_val) => Number::from(l_val.value().powf(r_val.value())),
        }
    }
}

impl std::ops::Neg for Number {
    type Output = Number;
    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(val) => Self::Integer(-val),
            Self::Decimal(val) => Self::Decimal(-val),
            Self::Fraction(num, denom) => Self::Fraction(-num, denom),
            Self::Irrational(val, s) => Self::Irrational(-val, s),
        }
    }
}

impl std::ops::Add<&Number> for Number {
    type Output = Number;
    fn add(self, rhs: &Number) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(l), Number::Integer(r)) => Number::Integer(l + r),
            (Number::Integer(l), Number::Fraction(num, denom)) => {
                Number::Fraction(num + l * denom, *denom)
            }
            (Number::Fraction(num, denom), Number::Integer(r)) => {
                Number::Fraction(num + r * denom, denom)
            }
            (Number::Fraction(l_num, l_denom), Number::Fraction(r_num, r_denom)) => {
                Number::Fraction(l_num * r_denom + r_num * l_denom, l_denom * r_denom)
            }
            (l_val, r_val) => Number::from(l_val.value() + r_val.value()),
        }
    }
}

impl std::ops::Add<i32> for Number {
    type Output = Number;
    fn add(self, rhs: i32) -> Self::Output {
        self + Number::Integer(rhs)
    }
}

impl std::ops::Add<Number> for i32 {
    type Output = Number;
    fn add(self, rhs: Number) -> Self::Output {
        rhs + Number::Integer(self)
    }
}

impl std::ops::Add<&Number> for &Number {
    type Output = Number;
    fn add(self, rhs: &Number) -> Self::Output {
        *self + rhs
    }
}

impl std::ops::Add<Number> for Number {
    type Output = Number;
    fn add(self, rhs: Number) -> Self::Output {
        &self + &rhs
    }
}

impl std::ops::AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<&Number> for Number {
    type Output = Number;
    fn sub(self, rhs: &Number) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(l), Number::Integer(r)) => Number::Integer(l - r),
            (Number::Integer(l), Number::Fraction(num, denom)) => {
                Number::Fraction(l * denom - num, *denom)
            }
            (Number::Fraction(num, denom), Number::Integer(r)) => {
                Number::Fraction(num - r * denom, denom)
            }
            (Number::Fraction(l_num, l_denom), Number::Fraction(r_num, r_denom)) => {
                Number::Fraction(l_num * r_denom - r_num * l_denom, l_denom * r_denom)
            }
            (l_val, r_val) => Number::from(l_val.value() - r_val.value()),
        }
    }
}

impl std::ops::Sub<&Number> for &Number {
    type Output = Number;
    fn sub(self, rhs: &Number) -> Self::Output {
        *self - rhs
    }
}

impl std::ops::Sub<Number> for Number {
    type Output = Number;
    fn sub(self, rhs: Number) -> Self::Output {
        &self - &rhs
    }
}

impl std::ops::Sub<i32> for Number {
    type Output = Number;
    fn sub(self, rhs: i32) -> Self::Output {
        self - Number::Integer(rhs)
    }
}

impl std::ops::Sub<Number> for i32 {
    type Output = Number;
    fn sub(self, rhs: Number) -> Self::Output {
        Number::Integer(self) - rhs
    }
}

impl std::ops::SubAssign for Number {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<&Number> for Number {
    type Output = Number;
    fn mul(self, rhs: &Number) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(l), Number::Integer(r)) => Number::Integer(l * r),
            (Number::Integer(l), Number::Fraction(num, denom)) => Number::Fraction(l * num, *denom),
            (Number::Fraction(num, denom), Number::Integer(r)) => Number::Fraction(num * r, denom),
            (Number::Fraction(l_num, l_denom), Number::Fraction(r_num, r_denom)) => {
                Number::Fraction(l_num * r_num, l_denom * r_denom)
            }
            (l_val, r_val) => Number::from(l_val.value() * r_val.value()),
        }
    }
}

impl std::ops::Mul<&Number> for &Number {
    type Output = Number;
    fn mul(self, rhs: &Number) -> Self::Output {
        *self * rhs
    }
}

impl std::ops::Mul<Number> for Number {
    type Output = Number;
    fn mul(self, rhs: Number) -> Self::Output {
        &self * &rhs
    }
}

impl std::ops::Mul<i32> for Number {
    type Output = Number;
    fn mul(self, rhs: i32) -> Self::Output {
        self * Number::Integer(rhs)
    }
}

impl std::ops::Mul<Number> for i32 {
    type Output = Number;
    fn mul(self, rhs: Number) -> Self::Output {
        Number::Integer(self) * rhs
    }
}

impl std::ops::Mul<f64> for &Number {
    type Output = Number;
    fn mul(self, rhs: f64) -> Self::Output {
        self * &rhs.into()
    }
}

impl std::ops::Mul<f64> for Number {
    type Output = Number;
    fn mul(self, rhs: f64) -> Self::Output {
        self * Number::from(rhs)
    }
}

impl std::ops::MulAssign<Number> for Number {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::MulAssign<i32> for Number {
    fn mul_assign(&mut self, rhs: i32) {
        *self = *self * rhs;
    }
}

impl std::ops::Div<Number> for Number {
    type Output = Number;
    fn div(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(l), Number::Integer(r)) => Number::Fraction(l, r).simplify(),
            (Number::Integer(l), Number::Fraction(num, denom)) => Number::Fraction(l * denom, num),
            (Number::Fraction(num, denom), Number::Integer(r)) => Number::Fraction(num, denom * r),
            (Number::Fraction(l_num, l_denom), Number::Fraction(r_num, r_denom)) => {
                Number::Fraction(l_num * r_denom, l_denom * r_num)
            }
            (l_val, r_val) => Number::from(l_val.value() / r_val.value()),
        }
    }
}

impl std::ops::Div<&Number> for &Number {
    type Output = Number;
    fn div(self, rhs: &Number) -> Self::Output {
        *self / *rhs
    }
}

impl std::ops::Div<&Number> for Number {
    type Output = Number;
    fn div(self, rhs: &Number) -> Self::Output {
        self / *rhs
    }
}

impl std::ops::Div<i32> for Number {
    type Output = Number;
    fn div(self, rhs: i32) -> Self::Output {
        self / Number::Integer(rhs)
    }
}

impl std::ops::Div<Number> for i32 {
    type Output = Number;
    fn div(self, rhs: Number) -> Self::Output {
        Number::Integer(self) / rhs
    }
}

impl std::ops::DivAssign for Number {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::DivAssign<i32> for Number {
    fn div_assign(&mut self, rhs: i32) {
        *self = *self / rhs;
    }
}

impl std::ops::Rem<Number> for Number {
    type Output = Self;
    fn rem(self, rhs: Number) -> Self::Output {
        match (self, rhs) {
            (Number::Integer(l), Number::Integer(r)) => Number::Integer(l % r),
            (_, _) => {
                tracing::error!(
                    "Number % Number is currently only implemented in the case where both are Integers."
                );
                Number::Integer(1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PI;

    #[test]
    fn addition() {
        let integer: Number = 3.into();
        let decimal: Number = 1.2.into();
        let decimal_2: Number = 1.8.into();
        let fraction: Number = (3, 4).into();

        assert_eq!((integer + integer).to_string(), "6");
        assert_eq!((integer + decimal).to_string(), "num(\"4.2\")");
        assert_eq!((integer + fraction).to_string(), "15/4");
        assert_eq!((decimal + fraction).to_string(), "num(\"1.95\")");
        assert_eq!((PI + integer).to_string(), "num(\"6.142\")");
        assert_eq!((decimal + decimal_2).to_string(), "3");
    }

    #[test]
    fn subtraction() {
        let integer: Number = 3.into();
        let decimal: Number = 1.2.into();
        let fraction: Number = (3, 4).into();

        assert_eq!((integer - integer).to_string(), "0");
        assert_eq!((integer - decimal).to_string(), "num(\"1.8\")");
        assert_eq!((integer - fraction).to_string(), "9/4");
        assert_eq!((decimal - fraction).to_string(), "num(\"0.45\")");
        assert_eq!((PI - integer).to_string(), "num(\"0.142\")");
    }

    #[test]
    fn multiplication() {
        let integer: Number = 3.into();
        let decimal: Number = 1.2.into();
        let fraction: Number = (3, 4).into();

        assert_eq!((integer * integer).to_string(), "9");
        assert_eq!((integer * decimal).to_string(), "num(\"3.6\")");
        assert_eq!((integer * fraction).to_string(), "9/4");
        assert_eq!((decimal * fraction).to_string(), "num(\"0.9\")");
        assert_eq!((PI * integer).to_string(), "num(\"9.425\")");
    }

    #[test]
    fn division() {
        let integer: Number = 3.into();
        let decimal: Number = 1.2.into();
        let fraction: Number = (3, 4).into();

        assert_eq!((integer / integer).to_string(), "1");
        assert_eq!((integer / decimal).to_string(), "num(\"2.5\")");
        assert_eq!((integer / fraction).simplify().to_string(), "4");
        assert_eq!((decimal / fraction).to_string(), "num(\"1.6\")");
        assert_eq!((PI / integer).to_string(), "num(\"1.047\")");
    }
}
