use arrayfire::*;
use num_complex::Complex64;

use super::calculator::get_nth_bit_from_decimal;
use super::environment::assert_backend_initialized;

pub fn execute<F>(bit_amount: usize, objective_function: F) -> Array<f64>
where
    F: Fn(i32) -> f64,
{
    assert_backend_initialized();

    // 定数群
    // 後々, 引数で受け取る仕様に変更
    let step: u32 = 1000;
    let tau: f64 = 1.0;
    let dt: f64 = tau / (step as f64);
    let b0: f64 = 30.0;

    let pattern = 2_u64.pow(bit_amount as u32) as usize;

    let hamiltonian_size = Dim4::new(&[
        pattern as u64,
        pattern as u64,
        1,
        1,
    ]);

    // 対角成分の作成
    // 将来的にはT行列にくみこむ
    let mut hll_vec = vec![];
    for l in 0..pattern {
        hll_vec.push(objective_function(l as i32));
    }
    let hll: Array<f64> = Array::new(&hll_vec, Dim4::new(&[hamiltonian_size[0], 1, 1, 1]));

    // 非対角成分の作成
    let mut io_vec = vec![vec![0; pattern]; pattern];
    for i in 0..pattern {
        for j in 0..pattern {
            let mut k = 0;
            for l in 0..bit_amount {
                k += (2 * get_nth_bit_from_decimal(i as i32, l, bit_amount) - 1) *
                     (2 * get_nth_bit_from_decimal(j as i32, l, bit_amount) - 1);
                if k == (bit_amount - 2) as i32 {
                    io_vec[i][j] = 1;
                }
            }
        }
    }
    let io: Array<u32> = Array::new(&io_vec.concat(), hamiltonian_size);

    // 計算用行列の定義
    // 後々は, T行列はハミルトニアンにまとめる
    let mut t_comp = constant(Complex64::new(0.0, 0.0), hamiltonian_size);
    let mut f0 = constant(
        Complex64::new(1.0, 0.0),
        Dim4::new(&[pattern as u64, 1, 1, 1]),
    ) / (pattern as f64).sqrt();
    let mut t: f64;

    for time in 0..step {
        t = (time as f64) * dt;
        let a = t / tau;
        let b = b0 * (1.0 - a);
        // H行列の作成
        // H = -b * (IO - I) + a * diag(HLL)
        let h: Array<f64> = -b * (&io - &identity::<f64>(hamiltonian_size)) + a * diag_create(&hll, 0);
        // T行列の更新
        // T = I - i * dt / 2 * H
        assign_seq(&mut t_comp, &[Seq::new(0, pattern as u32 - 1, 1), Seq::default()], &(&identity::<Complex64>(hamiltonian_size) - 0.5 * Complex64::new(0.0, 1.0) * dt * &h));

        let f = matmul(&t_comp, &f0, MatProp::NONE, MatProp::NONE);
        assign_seq(&mut f0, &[Seq::new(0, pattern as u32 - 1, 1)], &f);
    }

    // 結果の変換
    let nr = norm(&f0, NormType::VECTOR_2, 2.0, 0.0);
    f0 = (1.0 / nr) * f0;
    let amp = abs(&f0);
    let prob = mul(&amp, &amp, true);

    return prob;

}