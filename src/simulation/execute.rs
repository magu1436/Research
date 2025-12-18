use arrayfire::*;
use num_complex::Complex64;

use super::util::*;
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
    let b0: f64 = 10.0;

    let pattern = 2_u64.pow(bit_amount as u32);

    let hamiltonian_size = Dim4::new(&[
        pattern,
        pattern,
        1,
        1,
    ]);

    // 対角成分の作成
    // 将来的にはT行列にくみこむ
    let mut hll: Array<f64> = constant(0.0, Dim4::new(&[hamiltonian_size[0], 1, 1, 1]));
    for l in 0..pattern {
        hll = create_array_replaced_elem(&hll, l as usize, 0, objective_function(l as i32));
    }

    // テスト用ログ
    print!("HLL: ");
    print(&hll);

    // 非対角成分の作成
    let mut io: Array<u32> = constant(0, hamiltonian_size);
    for i in 0..pattern {
        for j in 0..pattern {
            let mut k = 0;
            for l in 0..bit_amount {
                k += (2 * get_nth_bit_from_decimal(i as i32, l as usize, bit_amount) - 1) *
                     (2 * get_nth_bit_from_decimal(j as i32, l as usize, bit_amount) - 1);
                if k == (bit_amount - 2) as i32 {
                    io = create_array_replaced_elem(&io, i as usize, j as usize, 1);
                }
            }
        }
    }

    // テスト用ログ
    println!("IO: ");
    print(&io);

    // 計算用の行列の定義
    // 後々は, T行列はハミルトニアンとまとめる
    let mut t_comp = constant(
        Complex64::new(0.0, 0.0), 
        hamiltonian_size,
    );
    let mut hamiltonian = constant(
        0.0f64,
        hamiltonian_size,
    );
    let mut f0 = constant(
        Complex64::new(1.0f64, 0.0f64),
        Dim4::new(&[hamiltonian_size[0], 1, 1, 1]),
    ) / (pattern as f64).sqrt();
    let mut t: f64;

    // 計算
    for time in 0..step {
        t = (time as f64) * dt;
        let a = t / tau;
        let b = b0 * (1.0 - a);
        for i in 0..pattern {
            for j in 0..pattern {
                hamiltonian = create_array_replaced_elem(
                    &hamiltonian, 
                    i as usize, 
                    j as usize,
                    -b * ref_in_array(
                        &io, 
                        i as usize, 
                        j as usize
                    ) as f64, 
                );
                t_comp = create_array_replaced_elem(
                    &t_comp, 
                    i as usize, 
                    j as usize, 
                    Complex64::new(
                        0.0,
                        -0.5 * dt * ref_in_array(
                            &hamiltonian, 
                            i as usize, 
                            j as usize
                        ),
                    ),
                );
            }
        }
        for l in 0..pattern {
            hamiltonian = create_array_replaced_elem(
                &hamiltonian, 
                l as usize, 
                l as usize, 
                a * ref_in_array(
                    &hll, 
                    l as usize,
                    0,
                ),
            );
            t_comp = create_array_replaced_elem(
                &t_comp,
                l as usize, 
                l as usize,
                Complex64::new(
                    1.0,
                    -0.5 * dt * ref_in_array(
                        &hamiltonian, 
                        l as usize, 
                        l as usize,
                    ),
                ),
            );
        }
        f0 = matmul(&t_comp, &f0, MatProp::NONE, MatProp::NONE);
    }

    // テスト用ログ
    println!("F0: ");
    print(&f0);

    // 結果の変換
    let nr = norm(&f0, NormType::VECTOR_2, 2.0, 0.0);
    f0 = (1.0 / nr) * f0;
    let amp = abs(&f0);
    let prob = mul(&amp, &amp, true);

    prob
}