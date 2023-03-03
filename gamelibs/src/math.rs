use nalgebra_glm::*;
use std::f64::consts::PI;

/// Defines the curve type based on the information in [**this video**](https://www.youtube.com/watch?v=KPoeNZZ6H4s).
/// 
/// * `CurveType::Linear` is a 1:1 I/O response.
/// 
/// * `CurveType::Bezier` is a quadratic Bezier response.
/// 
/// * `CurveType::SmoothDamped` is the Unity critical damping response.
/// 
/// * `CurveType::Mechanical` is a useful simulation of mechanic movement; see `CurveType::Custom`.
/// 
/// * `CurveType::Custom{f, z, r}` is a custom curve with the following `f64` type properties:
///     * `f` is the equivalent of FFT filtering, with lower values having slower dynamics.
///         * This should be kept to a value where `0.01 < f < 10`.
///     * `z` is the damping coefficient, where it slows oscillation caused by overshooting the base function.
///         * At a value of `0`, the vibration never dies and the object permanently oscillates.
///         * At values where `0 < z < 1`, the system is underdamped, causing less oscillation over time the higher it is set.
///         * At a value of `1`, the system is critically damping.
///         * At values where `1 < z`, the system is overdamped, causing the output function to slowly approach the input over time.
///     * `r` is the initial response value, where:
///         * At a value of `0`, the initial response is dampened and requires time to start moving (easing in).
///         * At values where `0 < r < 1`, the initial response has an immediate non-zero derivative, but is still dampened.
///         * At a value of `1`, the initial response follows the input function.
///         * At values where `1 < r`, the initial response causes an overshooting of the intended ceiling of the input function.
///         * At values where `r < 0`, the initial response is negative, causing an anticipation of the intended intended movement of the input function.
enum CurveStyle {
    Linear,
    Bezier,
    SmoothDamped,
    Mechanical{f: f64, z: f64},
    Custom{f: f64, z: f64, r: f64},
}

struct CurveType {
    f: f64,
    z: f64,
    r: f64,
    _w: f64,
    _z: f64,
    _d: f64,
}

impl CurveType {
    fn from_style(c: CurveStyle) -> Self {
        let get_fzr = 
            match c {
                CurveStyle::Linear => CurveType {
                    f: 10.0,
                    z: 0.0,
                    r: 1.0,
                    _w: 0.0,
                    _z: 0.0,
                    _d: 0.0,
                },
                CurveStyle::Bezier => CurveType { // TODO set proper vals
                    f: 0.0,
                    z: 0.0,
                    r: 0.0,
                    _w: 0.0,
                    _z: 0.0,
                    _d: 0.0,
                },
                CurveStyle::SmoothDamped => CurveType {
                    f: 1.0,
                    z: 1.0,
                    r: 0.0,
                    _w: 0.0,
                    _z: 0.0,
                    _d: 0.0,
                },
                CurveStyle::Mechanical{f, z} => CurveType {
                    f, z,
                    r: 2.0,
                    _w: 0.0,
                    _z: 0.0,
                    _d: 0.0,
                },
                CurveStyle::Custom{f, z, r} => CurveType {
                    f, z, r,
                    _w: 0.0,
                    _z: 0.0,
                    _d: 0.0,
                }};
        
        let f = get_fzr.f;
        let z = get_fzr.z;
        let r = get_fzr.r;
        let _w = 2.0 * PI * f;
        let _z = 0.0;
        let _d = _w * f64::sqrt(f64::abs(z * z - 1.0));
        
        CurveType {
            f: z / (PI * f),
            r: 1.0 / (_w * _w),
            z: (r * z) / _w,
            _w, _z, _d
        }
    }
}

struct WeightedNextBundle <F: Fn(f64) -> f64> {
    base_func: F,
    time: f64,
    curve: CurveType,
    last_pos: DVec3,
    last_vel: DVec3,
    last_acc: DVec3,
}

fn calc_weighted_next<F: Fn(f64) -> f64>(w: WeightedNextBundle<F>) ->
(DVec3, DVec3) {
    /* Var initialization and definition */
    let k1: f64 = w.curve.f;
    let k2: f64 = w.curve.z;
    let k3: f64 = w.curve.r;
    let _w: f64 = w.curve._w;
    let _z: f64 = w.curve._z;
    let _d: f64 = w.curve._d;
    let t: f64 = w.time;
    let x: f64 = (w.base_func)(t);
    let xd: f64 = derivative(w.base_func, t);
    let mut y = w.last_pos;
    let mut yd = w.last_vel;

    let k1_stable: f64;
    let k2_stable: f64;

    if _w * t < _z { // Clamp k2 (same as old k2_stable method)
        k1_stable = k1;
        k2_stable = f64::max(
            k2, f64::max(
            t * t * 0.5 + t * k1 * 0.5,
            t * k1)
        );
    } else { // Pole matching algorithm
        let t1: f64 = f64::exp(-_z * _w * t);
        let temp: f64;
        if _z <= 1.0 {
            temp = f64::cos(t * _d);
        } else {
            temp = f64::cosh(t * _d);
        }
        let alpha = 2.0 * t1 * temp;
        let beta = t1 * t1;
        let t2 = t / (1.0 + beta - alpha);
        k1_stable = (1.0 - beta) * t2;
        k2_stable = t * t2;

    }

    /* Update position */
    y.x = y.x + t * yd.x;
    y.y = y.y + t * yd.y;
    y.z = y.z + t * yd.z;
    yd.x = yd.x + t * (x + k3 * xd - y.x - k1_stable * yd.x) / k2_stable;
    yd.y = yd.y + t * (x + k3 * xd - y.y - k1_stable * yd.y) / k2_stable;
    yd.z = yd.z + t * (x + k3 * xd - y.z - k1_stable * yd.z) / k2_stable;

    return (y, yd)
}

/// Returns the derivative of a given function f(x) using Newtonian approximation
fn derivative<F: Fn(f64) -> f64>(
    f: F,   // the function to be derived
    x: f64, // the argument to be derived from
) -> f64 {
    const DELTA: f64 = f64::MIN_POSITIVE;
    let x1: f64 = x - DELTA;
    let x2: f64 = x + DELTA;
    let y1: f64 = f(x1);
    let y2: f64 = f(x2);
    return (y2 - y1) / (x2 - x1)
}

/*
 * q_rsqrt is a 64-bit port of the Q_rsqrt Quake inverse square algorithm complete with a new mystery constant (sqrt(2^1023) in hex for brevity).
 * For values between 0 and 1, q_rsqrt is within a negligable margin of error when compared to the rsqrt calculation, while being MUCH faster and less demanding.
 */
pub fn q_rsqrt(f_in: f64) -> f64 {
    let f_in_as_bits: u64 = f_in.to_bits(); // evil floating point bit hack
    let f_in_as_bits: u64 = 0x5fe6a09e667f3bc8 - (f_in_as_bits >> 1); // what the fuck? (now with more 64-bit)
    let f_out: f64 = f64::from_bits(f_in_as_bits);
    let f_out: f64 = f_out * (1.5 - 0.5 * f_in * f_out * f_out); // 1st iteration
    let f_out: f64 = f_out * (1.5 - 0.5 * f_in * f_out * f_out); // 2nd iteration, can be removed
    let f_out: f64 = f_out * (1.5 - 0.5 * f_in * f_out * f_out); // 3rd iteration, can be removed; provides full precision.
    return f_out;
}
