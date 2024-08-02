extern crate rustfft;
use rustfft::{FftPlanner, num_complex::Complex};

fn polynomial_multiply(a: &[f64], b: &[f64]) -> Vec<f64> {
    let n = a.len() + b.len() - 1;
    let fft_size = n.next_power_of_two();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(fft_size);
    let ifft = planner.plan_fft_inverse(fft_size);

    let mut a_complex: Vec<Complex<f64>> = a.iter().map(|&x| Complex::new(x, 0.0)).collect();
    a_complex.resize(fft_size, Complex::new(0.0, 0.0));
    let mut b_complex: Vec<Complex<f64>> = b.iter().map(|&x| Complex::new(x, 0.0)).collect();
    b_complex.resize(fft_size, Complex::new(0.0, 0.0));

    fft.process(&mut a_complex);
    fft.process(&mut b_complex);

    let mut result_complex = vec![Complex::new(0.0, 0.0); fft_size];
    for i in 0..fft_size {
        result_complex[i] = a_complex[i] * b_complex[i];
    }

    ifft.process(&mut result_complex);

    result_complex
        .iter()
        .map(|x| (x.re / fft_size as f64).round())  // Round to the nearest integer
        .take(n)
        .collect()
}

fn main() {
    let poly1 = vec![1.0, 2.0, 3.0];
    let poly2 = vec![4.0, 5.0, 6.0];

    let result = polynomial_multiply(&poly1, &poly2);
    println!("Result: {:?}", result);
}



