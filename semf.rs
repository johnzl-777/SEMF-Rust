
/*
 * struct that holds values for formula coefficients
 * a_v = volume term coefficient
 * a_s = surface term coefficient
 * a_c = coloumb term coefficient
 * a_a = asymmetry term coefficient
 * a_p = pairing term constant for generating delta_0
 * k_p = pairing term exponent for generating delta_0
 */

pub struct SemiEmpConsts{
    a_v : f64,
    a_s : f64,
    a_c : f64,
    a_a : f64,
    a_p : f64,
    k_p : f64
}

// Generate commonly used versions of the coefficients,
// found experimentally and through linear fitting
impl SemiEmpConsts {
    pub fn linear_fit_1() -> SemiEmpConsts {
        SemiEmpConsts{
            a_v: 15.8,
            a_s: 18.3,
            a_c: 0.714,
            a_a: 23.2,
            a_p: 12.0,
            k_p: -1.0/2.0
        }
    }

    pub fn linear_fit_2() -> SemiEmpConsts {
        SemiEmpConsts {
            a_v: 15.76,
            a_s: 17.81,
            a_c: 0.711,
            a_a: 23.702,
            a_p: 34.0,
            k_p: -3.0/4.0,
        }
    }
}


fn volume_term(a: u64, a_v: f64) -> f64 {
    a_v * (a as f64)
}

fn surface_term(a: u64, a_s: f64) -> f64 {
    a_s * (a as f64).powf(2.0/3.0)
}

fn coloumb_term(a: u64, z: u64, a_c: f64) -> f64 {
    a_c * ((z * (z - 1)) as f64)/(a as f64).powf(1.0/3.0)
}

fn asymmetry_term(a: u64, z: u64, a_a: f64) -> f64 {
    a_a * ((a - 2*z).pow(2) as f64)/(a as f64)
}

fn pairing_term(a: u64, z: u64, a_p: f64, k_p: f64) -> f64 {
    let delta_0 = a_p * (a as f64).powf(k_p);
  
    let n = a-z;
    let sign = 
        if ((n % 2) != 0) && ((z % 2) != 0) {
            -1 
        } else if ((n % 2) == 0) && ((z % 2) == 0) {
            1
        } else {
            0
        };
    (sign as f64) * delta_0
}

// Full formula, subtracts the terms in proper order using constants from 
// a `SemiEmpConsts` struct
pub fn semi_emp_bind_eng(a: u64, z: u64, consts: SemiEmpConsts) -> f64 {
    volume_term(a, consts.a_v) -
    surface_term(a, consts.a_s) - 
    coloumb_term(a, z, consts.a_c) - 
    asymmetry_term(a, z, consts.a_a) - 
    pairing_term(a, z, consts.a_p, consts.k_p)
}

// Debugging variant, prints the values of each term before returning a value
pub fn semi_emp_bind_eng_debug(a: u64, z: u64, consts: SemiEmpConsts) -> f64 {
    println!("Volume: {}", volume_term(a, consts.a_v));
    println!("Surface: {}", surface_term(a, consts.a_a));
    println!("Coloumb: {}", coloumb_term(a, z, consts.a_c));
    println!("Asymmetry: {}", asymmetry_term(a, z, consts.a_a));
    println!("Pairing: {}", pairing_term(a, z, consts.a_p, consts.k_p));

    volume_term(a, consts.a_v) -
    surface_term(a, consts.a_s) - 
    coloumb_term(a, z, consts.a_c) - 
    asymmetry_term(a, z, consts.a_a) - 
    pairing_term(a, z, consts.a_p, consts.k_p)
}