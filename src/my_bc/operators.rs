use crate::my_bc::{ StorageType, Number, clone, empty, is_zero, zero, one, INTERNAL_COMPUTATION_BASE };

pub fn negate(n: &Number) -> Number {
    let mut result = clone(n);
    
    result.negative = !result.negative;
    result
}

pub fn compare(lhs: &Number, rhs: &Number) -> i32 {
    let k = if lhs.negative { -1 } else { 1 };

    if lhs.negative == rhs.negative {
        if lhs.rdigits.len() > rhs.rdigits.len() {
            1 * k
        } else if lhs.rdigits.len() < rhs.rdigits.len() {
            -1 * k
        } else {
            for pair in std::iter::zip(&lhs.rdigits, &rhs.rdigits).rev() {
                if pair.0 > pair.1 {
                    return 1 * k
                } else if pair.0 < pair.1 {
                    return -1 * k
                }
            }
            0
        }
    } else if lhs.negative {
        -1
    } else /* if rhs.negative */ {
        1
    }
}

fn add_internal(lhs: &Number, rhs: &Number) -> Number {
    //fn aux_bak(lhs: StorageType, rhs: StorageType, carry: bool) -> (StorageType, bool) {
    //    if carry {
    //        if let Some(sum) = lhs.checked_add(1) {
    //            aux(sum, rhs, false)
    //        } else {
    //            (rhs, true)
    //        }
    //    } else {
    //        if let Some(sum) = lhs.checked_add(rhs) {
    //            (sum, false)
    //        } else {
    //            (lhs - (StorageType::MAX - rhs + 1), true)
    //        }
    //    }
    //}
    fn aux(lhs: StorageType, rhs: StorageType, carry: bool) -> (StorageType, bool) {
        let sum = (lhs as i32) + (rhs as i32) + (if carry { 1 } else { 0 });

        if sum >= INTERNAL_COMPUTATION_BASE {
            ((sum - INTERNAL_COMPUTATION_BASE) as StorageType, true)
        } else {
            (sum as StorageType, false)
        }
    }
    
    let mut i: usize = 0;
    let mut result = empty();
    let mut carry = false;

    while i < lhs.rdigits.len() || i < rhs.rdigits.len() {
        let lhsi: StorageType = if i < lhs.rdigits.len() { lhs.rdigits[i] } else { 0 };
        let rhsi: StorageType = if i < rhs.rdigits.len() { rhs.rdigits[i] } else { 0 };
        let (sum, b) = aux(lhsi, rhsi, carry);

        result.rdigits.push(sum);
        carry = b;
        i = i + 1;
    }
    if carry {
        result.rdigits.push(1);
    }
    result
}

pub fn add(lhs: &Number, rhs: &Number) -> Number {
    if !lhs.negative && !rhs.negative {
        add_internal(lhs, rhs)
    } else if lhs.negative && rhs.negative {
        negate(&add_internal(lhs, rhs))
    } else if lhs.negative {
        sub(rhs, &negate(lhs))
    } else /* if rhs.negative */ {
        sub(lhs, &negate(rhs))
    }
}

fn sub_internal(lhs: &Number, rhs: &Number) -> Number {
    //fn aux_bak(lhs: StorageType, rhs: StorageType, carry: bool) -> (StorageType, bool) {
    //    if carry {
    //        if lhs > 0 {
    //            aux(lhs - 1, rhs, false)
    //        } else {
    //            (StorageType::MAX, true)
    //        }
    //    } else {
    //        if lhs >= rhs {
    //            (lhs - rhs, false)
    //        } else {
    //            (StorageType::MAX - (rhs - lhs) + 1, true)
    //        }
    //    }
    //}
    fn aux(lhs: StorageType, rhs: StorageType, carry: bool) -> (StorageType, bool) {
        let diff = (lhs as i32) - (rhs as i32) - (if carry { 1 } else { 0 });

        if diff < 0 {
            ((diff + INTERNAL_COMPUTATION_BASE) as StorageType, true)
        } else {
            (diff as StorageType, false)
        }
    }
    
    let mut i: usize = 0;
    let mut result = empty();
    let mut carry = false;

    while i < lhs.rdigits.len() || i < rhs.rdigits.len() {
        let lhsi: StorageType = if i < lhs.rdigits.len() { lhs.rdigits[i] } else { 0 };
        let rhsi: StorageType = if i < rhs.rdigits.len() { rhs.rdigits[i] } else { 0 };
        let (diff, b) = aux(lhsi, rhsi, carry);

        result.rdigits.push(diff);
        carry = b;
        i = i + 1;
    }
    debug_assert!(i > 0);
    debug_assert_eq!(carry, false);
    while result.rdigits.len() > 1 && result.rdigits.last() == Some(&0) {
        result.rdigits.pop();
    }
    result
}

pub fn sub(lhs: &Number, rhs: &Number) -> Number {
    if !lhs.negative && !rhs.negative {
        match compare(lhs, rhs) {
            value if value > 0 => sub_internal(lhs, rhs),
            value if value < 0 => negate(&sub_internal(rhs, lhs)),
            _ => zero(),
        }
    } else if lhs.negative && rhs.negative {
        add(lhs, &negate(&rhs))
    } else if lhs.negative {
        negate(&add(&negate(lhs), rhs))
    } else /* if rhs.negative */ {
        add(lhs, &negate(rhs))
    }
}

fn mul_internal(lhs: &Number, rhs: &Number) -> Number {
    let mut result = zero();
    let mut count = clone(rhs);

    for len in (1..=rhs.rdigits.len()).rev() {
        let mut sub_rhs = empty();
        let mut add_rhs = clone(lhs);

        sub_rhs.rdigits.resize(len, 0);
        sub_rhs.rdigits[len - 1] = 1;
        for _ in 1..len {
            add_rhs.rdigits.insert(0, 0);
        }
        while compare(&count, &sub_rhs) >= 0 {
            result = add(&result, &add_rhs);
            count = sub(&count, &sub_rhs);
        }
    }
    result
}

pub fn mul(lhs: &Number, rhs: &Number) -> Number {
    if is_zero(lhs) || is_zero(rhs) {
        zero()
    } else if !lhs.negative && !rhs.negative {
        mul_internal(lhs, rhs)
    } else if lhs.negative && rhs.negative {
        mul_internal(&negate(lhs), &negate(rhs))
    } else if lhs.negative {
        negate(&mul_internal(&negate(lhs), rhs))
    } else /* if rhs.negative */ {
        negate(&mul_internal(lhs, &negate(rhs)))
    }
}

fn div_modulo_internal(lhs: &Number, rhs: &Number) -> (Number, Number) {
    let mut quotient = zero();
    let mut remainder = clone(lhs);
    let one = one();

    while compare(&remainder, rhs) >= 0 {
        let mut k = clone(&one);
        let mut kb = clone(rhs);
        let mut c = zero();

        while compare(&kb, &remainder) <= 0 {
            k.rdigits.insert(0, 0);
            kb.rdigits.insert(0, 0);
        }
        debug_assert!(kb.rdigits.len() > 1);
        k.rdigits.remove(0);
        kb.rdigits.remove(0);
        while compare(&mul(&kb, &c), &remainder) <= 0 {
            c = add(&c, &one);
        }
        debug_assert!(compare(&c, &zero()) > 0);
        c = sub(&c, &one);

        quotient = add(&quotient, &mul(&k, &c));
        remainder = sub(&remainder, &mul(&kb, &c));
    }
    (quotient, remainder)
}

pub fn div_modulo(lhs: &Number, rhs: &Number) -> Result<(Number, Number), String> {
    if is_zero(rhs) {
        Err(String::from("division by zero"))
    } else if is_zero(lhs) {
        Ok((zero(), zero()))
    } else if !lhs.negative && !rhs.negative {
        Ok(div_modulo_internal(lhs, rhs))
    } else if lhs.negative && rhs.negative {
        let (quotient, remainder) = div_modulo_internal(&negate(lhs), &negate(rhs));
        Ok((quotient, negate(&remainder)))
    } else if lhs.negative {
        let (quotient, remainder) = div_modulo_internal(&negate(lhs), rhs);
        Ok((negate(&quotient), negate(&remainder)))
    } else /* if rhs.negative */ {
        let (quotient, remainder) = div_modulo_internal(lhs, &negate(rhs));
        Ok((negate(&quotient), remainder))
    }
}

pub fn div(lhs: &Number, rhs: &Number) -> Result<Number, String> {
    match div_modulo(lhs, rhs) {
        Ok((q, _)) => Ok(q),
        Err(err) => Err(err),
    }
}

pub fn modulo(lhs: &Number, rhs: &Number) -> Result<Number, String> {
    match div_modulo(lhs, rhs) {
        Ok((_, r)) => Ok(r),
        Err(err) => Err(err),
    }
}
