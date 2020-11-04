use std::ops::{
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign,
    Div, DivAssign,
};
use std::cmp::{
    PartialEq, PartialOrd,
    Ordering,
};

const MODULUS: u64 = 1000000007;

#[derive(Clone, Copy, Debug)]
pub struct modint { val: u64 }

impl modint {
    pub const fn new(x: u64) -> modint { modint { val: x % MODULUS } }

    pub const fn value(self) -> u64 { self.val }

    pub fn mod_pow(&mut self, mut n: u64) -> modint {
        if self.val == 0 { return modint::new(0); }
        let mut res = 1u64;
        while n > 0 {
            if n % 2 == 1 { res = res * self.val % MODULUS; }
            self.val = self.val * self.val % MODULUS;
            n /= 2;
        }
        modint::new(res)
    }

    pub fn mod_inv(&mut self) -> modint { self.mod_pow(MODULUS-2) }
}

impl AddAssign for modint {
    fn add_assign(&mut self, rhs: Self) {
        self.val += rhs.val;
        if self.val >= MODULUS { self.val -= MODULUS; }
    }
}

impl Add for modint {
    type Output = modint;
    fn add(mut self, rhs: modint) -> modint {
        self += rhs;
        self
    }
}

impl SubAssign for modint {
    fn sub_assign(&mut self, rhs: Self) {
        if self.val < rhs.val { self.val += MODULUS; }
        self.val -= rhs.val;
    }
}

impl Sub for modint {
    type Output = modint;
    fn sub(mut self, rhs: Self) -> modint {
        self -= rhs;
        self
    }
}

impl MulAssign for modint {
    fn mul_assign(&mut self, rhs: Self) { self.val = self.val * rhs.val % MODULUS; }
}

impl Mul for modint {
    type Output = modint;
    fn mul(mut self, rhs: Self) -> modint {
        self *= rhs;
        self
    }
}

impl DivAssign for modint {
    fn div_assign(&mut self, mut rhs: modint) {
        assert_ne!(rhs.val, 0);
        *self *= rhs.mod_inv();
    }
}

impl Div for modint {
    type Output = modint;
    fn div(mut self, rhs: Self) -> modint {
        assert_ne!(rhs.val, 0);
        self /= rhs;
        self
    }
}

impl PartialEq for modint {
    fn eq(&self, rhs: &Self) -> bool { self.val == rhs.val }
}

impl PartialOrd for modint {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        if self.val == rhs.val { Some(Ordering::Equal) }
        else if self.val > rhs.val { Some(Ordering::Greater) }
        else if self.val < rhs.val { Some(Ordering::Less) }
        else { None }
    }
}

#[test]
fn add_test_case() {
    assert_eq!(modint::new(2), modint::new(1) + modint::new(1));
    assert_eq!(modint::new(1), modint::new(1000000007) + modint::new(1));
    assert_eq!(modint::new(1000000007 + 20), modint::new(1000000007 + 5) + modint::new(15))
}

#[test]
fn sub_test_case() {
    assert_eq!(modint::new(1), modint::new(1) - modint::new(1000000007));
    assert_eq!(modint::new(0), modint::new(10) - modint::new(10));
    assert_eq!(modint::new(10), modint::new(100) - modint::new(90));
}

#[test]
fn mul_test_case() {
    assert_eq!(modint::new(1), modint::new(1) * modint::new(1000000008));
    assert_eq!(modint::new(2).mod_pow(1000000007-1), modint::new(1));
    assert_eq!(modint::new(1000000007+1000000000).mod_pow(1000000007-1), modint::new(1));
    assert_eq!(modint::new(123456789), modint::new(12345678900000) / modint::new(100000))
}

#[test]
fn div_test_case() {
    assert_eq!(modint::new(1000000006), modint::new(1000000006).mod_inv());
    assert_eq!(modint::new(1), modint::new(1).mod_inv());
}

#[test]
fn cmp_test_case() {
    assert!(modint::new(1000000007 + 100) > modint::new(1000000007));
    assert_eq!(modint::new(1000000007), modint::new(1000000007));
}
