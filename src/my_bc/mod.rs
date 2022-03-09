type StorageType = u8;

mod operators;
pub use crate::my_bc::operators::*;

mod base;
pub use crate::my_bc::base::*;

mod eval;
pub use crate::my_bc::eval::*;

#[derive(Debug)]
pub struct Number {
    rdigits: Vec<StorageType>,
    negative: bool,
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", format_base10(self))
    }
}

const INTERNAL_COMPUTATION_BASE: i32 = 8;

fn empty() -> Number {
    Number { rdigits: [].to_vec(), negative: false }
}

fn is_zero(n: &Number) -> bool {
    n.rdigits.len() == 1 && n.rdigits[0] == 0
}

pub fn zero() -> Number {
    Number { rdigits: [0].to_vec(), negative: false }
}

pub fn one() -> Number {
    Number { rdigits: [1].to_vec(), negative: false }
}

fn from_usize(n: usize) -> Number {
    let mut n = n;
    let mut digits = Vec::new();

    while n >= (INTERNAL_COMPUTATION_BASE as usize) {
        let d = (n % (INTERNAL_COMPUTATION_BASE as usize)) as StorageType;

        digits.push(d);
        n = n / (INTERNAL_COMPUTATION_BASE as usize);
    }
    digits.push(n as StorageType);
    Number { rdigits: digits, negative: false }
}

fn clone(n: &Number) -> Number {
    Number { rdigits: n.rdigits.clone(), negative: n.negative }
}
