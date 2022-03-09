use crate::my_bc::{ Number, from_usize, clone, is_zero, zero, negate, compare, add, div_modulo, INTERNAL_COMPUTATION_BASE };

pub const BASE_2: &'static str = "01";
pub const BASE_8: &'static str = "01234567";
pub const BASE_10: &'static str = "0123456789";
pub const BASE_16: &'static str = "0123456789ABCDEF";

pub fn from_base(n: &str, base: &str) -> Option<Number> {
    let mut result = zero();

    for c in n.chars() {
        if let Some(index) = base.find(c) {
            let digit = from_usize(index);
            let clone = clone(&result);
    
            for _ in 1..base.len() {
                result = add(&result, &clone)
            }
            result = add(&result, &digit);
        } else {
            return None;
        }
    }
    Some(result)
}

pub fn from_base10(n: &str) -> Option<Number> {
    from_base(n, BASE_10)
}

fn to_usize(n: &Number) -> usize {
    let mut result: usize = 0;

    for d in n.rdigits.iter().rev() {
        result = result * (INTERNAL_COMPUTATION_BASE as usize) + (*d as usize);
    }
    result
}

pub fn format_base(n: &Number, base: &str) -> String {
    let mut s = String::new();
    let mut n = clone(n);

    if n.negative && !is_zero(&n) {
        s += "-";
        n = negate(&n);
    }
    fn aux(n: &Number, base_len: &Number, base: &str) -> String {
        let mut s = String::new();
        if let Ok((q, r)) = div_modulo(n, base_len) {
            if compare(n, base_len) >= 0 {
                s.push_str(&aux(&q, base_len, base));
            }
            s.push(base.chars().nth(to_usize(&r)).expect("index out of range"));
            s
        } else {
            panic!("unexpected error")
        }
    }
    s.push_str(&aux(&n, &from_usize(base.len()), base));
    s
}

pub fn format_base10(n: &Number) -> String {
    format_base(n, BASE_10)
}
