use std::ops::{Div, Rem};

pub fn gcd(u: i64, v: i64) -> i64 {
    // `wrapping_abs` gives a number's absolute value, unless that's 2³¹. 2³¹
    // won't fit in `i64`, so it gives -2³¹ instead.
    let mut v = v.wrapping_abs() as u64;
    if u == 0 {
        return v as i64;
    }
    let mut u = u.wrapping_abs() as u64;
    if v == 0 {
        return u as i64;
    }

    // `|` is bitwise OR. `trailing_zeros` quickly counts a binary number's
    // trailing zeros, giving its prime factorization's exponent on two.
    let gcd_exponent_on_two = (u | v).trailing_zeros();

    // `>>=` divides the left by two to the power of the right, storing that in
    // the left variable. `u` divided by its prime factorization's power of two
    // turns it odd.
    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();

    while u != v {
        if u < v {
            // Swap the variables' values with each other.
            core::mem::swap(&mut u, &mut v);
        }
        u -= v;
        u >>= u.trailing_zeros();
    }

    // `<<` multiplies the left by two to the power of the right.
    (u << gcd_exponent_on_two) as i64
}

pub fn lcm(u: i64, v: i64) -> i64 {
    if u > v {
        (u / gcd(u, v)) * v
    } else {
        (v / gcd(v, u)) * u
    }
}

pub fn polygon_area(vertices: &[(i64, i64)]) -> f64 {
    let n = vertices.len();
    let mut sum = 0.0;

    for cur in 0..n {
        let next = (cur + 1) % n;
        sum += (vertices[cur].0 as f64 * vertices[next].1 as f64)
            - (vertices[next].0 as f64 * vertices[cur].1 as f64);
    }

    0.5 * sum.abs()
}

pub fn picks_theorem(area: f64, vertices: &[(i64, i64)]) -> u64 {
    let boundary_points = vertices.len() as u64;
    let interior_points = area - (boundary_points / 2) as f64 + 1.0;

    interior_points as u64
}

pub fn div_rem<T>(first: T, second: T) -> (T, T)
where
    T: Div<Output = T> + Rem<Output = T> + Copy,
{
    (first / second, first % second)
}
