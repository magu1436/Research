mod simulation;

use std::{collections::HashMap, vec};

use arrayfire::*;

use simulation::*;

use crate::simulation::calculator::get_nth_bit_from_decimal;
use crate::environment::init_backend;
use crate::simulation::execute_new::execute;
fn main() {

    init_backend(Backend::CUDA);
    info();

    // const N: usize = 1;
    // let var_dim: Dim4 = Dim4::new(&[N as u64, 1, 1, 1]);
    // let c: Array<f64> = Array::new(&[2.0], var_dim);   // 重み
    // let e: Array<f64> = Array::new(&[2.0], var_dim);   // 事業総資産
    // let b: Array<f64> = Array::new(&[10.0], var_dim);  // 債権
    // let pi1: Array<f64> = Array::new(&[
    //     0.0,
    // ], Dim4::new(&[N as u64, N as u64, 1, 1]));
    // const M: i32 = 5;    // 予算
    // const A: usize = 3;  // 実数一つあたりに確保するビット数
    // const Q_N: usize = 4 * A * N + A;

    // let mut var2index: HashMap<&str, usize> = HashMap::new();
    // var2index.insert("p1", 0);
    // var2index.insert("x", 1);
    // var2index.insert("s", 2);
    // var2index.insert("t", 3);

    // let r_vec = | var: &str, l: i32 | -> Array<f64> {
    //     let mut vec: Vec<f64> = vec![];
    //     for i in 0..N {
    //         let mut v = 0.0;
    //         for k in 0..A {
    //             let q = get_nth_bit_from_decimal(l, var2index[var] * A * N + i * A + k, Q_N);
    //             v += 2_f64.powi(k as i32) * (q as f64);
    //         }
    //         vec.push(v);
    //     }

    //     return Array::new(&vec, var_dim);
    // };

    // let objective_function = | l: i32 | -> f64 {
    //     let p1 = r_vec("p1", l);
    //     let x = r_vec("x", l);
    //     let s = r_vec("s", l);
    //     let t = r_vec("t", l);
    //     let mut u: f64 = 0.0;
    //     for i in 0..A {
    //         u += 2_f64.powi(i as i32) * (get_nth_bit_from_decimal(l, 4 * N * A + i, Q_N) as f64);
    //     }

    //     print(&p1);

    //     let z = 
    //         sum_all(&(-1 * matmul(&c, &p1, MatProp::TRANS, MatProp::NONE))).0
    //         + sum_all(&pow(&(&p1 - matmul(&pi1, &p1, MatProp::NONE, MatProp::NONE) - &x - &e - &s), &2.0, false)).0
    //         + sum_all(&pow(&(&p1 + &t + &b), &2.0, false)).0
    //         + (sum_all(&x).0 + u - (M as f64)).powi(2);

    //     return z;
    // };

    // let result = execute::execute(Q_N, objective_function);

    // print(&result);

    const NUMS: [i32; 6] = [2, 3, 5, 7, 8, 9];
    let objective_function = | l: i32 | -> f64 {
        let mut sum = 0.0;
        for i in 0..NUMS.len() {
            for j in i+1..NUMS.len() {
                let l_i = 2 * get_nth_bit_from_decimal(l, i, NUMS.len()) - 1;
                let l_j = 2 * get_nth_bit_from_decimal(l, j, NUMS.len()) - 1;
                sum += (l_i * l_j * NUMS[i] * NUMS[j]) as f64;
            }
        }
        return sum;
    };

    let result = execute(NUMS.len(), objective_function);
    println!("Result: ");
    print(&result);
}