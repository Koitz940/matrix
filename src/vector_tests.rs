use crate::complex::{Comp, Complex};
use crate::cosine::*;
use crate::cross::cross_product;
use crate::lerp::lerp;
use crate::linear_combination;
use crate::matrix::Matrix;
use crate::vector::Vector;
use std::println;

pub fn test_ops() {
    let mut v = Vector::from(vec![1., 3., 4.]);
    v += Vector::from(vec![1., 3., 5.]);
    println!("{v}");
    v *= 2.;
    println!("{v}");
    v -= Vector::from(vec![1., 3., 5.]);
    println!("{v}");
}

pub fn test_lc() {
    let a: Vec<Vector<Comp>> = vec![];
    let b: Vec<f64> = vec![];
    let v: Vector<Comp> = linear_combination::linear_combination::<Comp, f64>(&a, &b);
    println!("{v}");
    let c = vec![
        Vector::from(vec![Complex::<f64> { re: 1.0, im: 1.5 }]),
        Vector::from(vec![Complex::<f64> { re: 1.0, im: 0.0 }]),
        Vector::from(vec![Complex::<f64> { re: -3.14, im: 1.5 }]),
        Vector::from(vec![Complex::<f64> { re: -2.5, im: -1.5 }]),
    ];
    let d: Vec<Comp> = vec![
        Complex::<f64> { re: 1., im: 0. },
        Complex::<f64> { re: 0., im: 2.0 },
        Complex::<f64> { re: -1.5, im: 1.0 },
        Complex::<f64> { re: 0., im: -2.5 },
    ];
    let v2 = linear_combination::linear_combination(&c, &d);
    println!("{v2}");

    let e1 = Vector::from(vec![1., 0., 0.]);
    let e2 = Vector::from(vec![0., 1., 0.]);
    let e3 = Vector::from(vec![0., 0., 1.]);
    let v1 = Vector::from(vec![1., 2., 3.]);
    let v2 = Vector::from(vec![0., 10., -100.]);
    println!(
        "{}",
        linear_combination::linear_combination::<f32, f32>(&[e1, e2, e3], &[10., -2., 0.5])
    );
    // [10.]
    // [-2.]
    // [0.5]
    println!(
        "{}",
        linear_combination::linear_combination::<f32, f32>(&[v1, v2], &[10., -2.])
    );
    // [10.]
    // [0.]
    // [230.]
}

pub fn test_lerp() {
    println!(
        "{}",
        lerp(
            Matrix::from_multiple(vec![vec![2., 1.], vec![3., 4.]]).unwrap(),
            Matrix::from_multiple(vec![vec![20., 10.], vec![30., 40.]]).unwrap(),
            0.5
        )
    );

    println!(
        "{}",
        lerp(
            Matrix::from_multiple(vec![
                vec![Complex { re: 1.0, im: 0.1 }, Complex { re: 0.0, im: -1. }],
                vec![Complex { re: 2.0, im: 0.1 }, Complex { re: 4.0, im: 4.5 }]
            ])
            .unwrap(),
            Matrix::from_multiple(vec![
                vec![
                    Complex { re: -2.0, im: -10. },
                    Complex { re: 50., im: -40. }
                ],
                vec![Complex { re: 10., im: -5. }, Complex { re: -40., im: 100. }]
            ])
            .unwrap(),
            0.3
        )
    );
}

pub fn test_dot() {
    let mut u = Vector::from(vec![0., 0.]);
    let mut v = Vector::from(vec![1., 1.]);
    println!("{}", u.dot(&v));
    // 0.0
    u = Vector::from(vec![1., 1.]);
    v = Vector::from(vec![1., 1.]);
    println!("{}", u.dot(&v));
    // 2.0
    u = Vector::from(vec![-1., 6.]);
    v = Vector::from(vec![3., 2.]);
    println!("{}", u.dot(&v));
    // 9.0

    let w = Vector::from(vec![
        Complex { re: 1., im: 0. },
        Complex { re: 2.0, im: 3.14 },
    ]);
    let t = Vector::from(vec![
        Complex { re: 1., im: -2. },
        Complex { re: 2.0, im: 1. },
    ]);
    println!("{} {}", w.dot(&t), w.dot_conj(&t));
    // 1.86 + 6.28i, 8.14 + 6.28i
}

pub fn test_norms() {
    let mut u = Vector::from(vec![0., 0., 0.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 0.0, 0.0, 0.0
    u = Vector::from(vec![1., 2., 3.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 6.0, 3.74165738, 3.0
    u = Vector::from(vec![-1., -2.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 3.0, 2.236067977, 2.0
    let mut v = Vector::from(vec![Complex { re: -1., im: -2. }]);
    println!("{}, {}, {}", v.norm_1(), v.norm(), v.norm_inf());
    v = Vector::from(vec![
        Complex { re: -1., im: -2. },
        Complex { re: 0., im: 3. },
    ]);
    println!("{}, {}, {}", v.norm_1(), v.norm(), v.norm_inf());
}

pub fn tests_cos() {
    let mut u = Vector::from(vec![1., 0.]);
    let mut v = Vector::from(vec![1., 0.]);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    u = Vector::from(vec![1., 0.]);
    v = Vector::from(vec![0., 1.]);
    println!("{}", angle_cos(&u, &v));
    // 0.0
    u = Vector::from(vec![-1., 1.]);
    v = Vector::from(vec![1., -1.]);
    println!("{}", angle_cos(&u, &v));
    // -1.0
    u = Vector::from(vec![2., 1.]);
    v = Vector::from(vec![4., 2.]);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    u = Vector::from(vec![1., 2., 3.]);
    v = Vector::from(vec![4., 5., 6.]);
    println!("{}", angle_cos(&u, &v));
    // 0.974631846

    let w = Vector::from(vec![
        Complex { re: 1., im: 0. },
        Complex { re: 2.0, im: 3.14 },
    ]);
    let t = Vector::from(vec![
        Complex { re: 1., im: -2. },
        Complex { re: 2.0, im: 1. },
    ]);
    println!(
        "{} {}",
        cf32_angle_re_cos(&w, &t),
        cf32_angle_abs_cos(&w, &t)
    );
}

pub fn test_cross() {
    let u = Vector::from(vec![0., 0., 1.]);
    let v = Vector::from(vec![1., 0., 0.]);
    println!("{}", cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from(vec![1., 2., 3.]);
    let v = Vector::from(vec![4., 5., 6.]);
    println!("{}", cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from(vec![4., 2., -3.]);
    let v = Vector::from(vec![-2., -5., 16.]);
    println!("{}", cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]
	let u = Vector::from(vec![Complex {re: 1.0, im: -2.0}, Complex {re: 0.0, im: 10.0}, Complex {re: 3.140, im: -10.}]);
    let v = Vector::from(vec![Complex {re: -1.0, im: -2.0}, Complex {re: 1.75, im: -2.75}, Complex {re: -1.0, im: 2.5}]);
    println!("{}", cross_product(&u, &v));
}

pub fn mat_mult_tests() {
	let u = Matrix::from_multiple(vec![
	vec![1., 0.],
	vec![0., 1.],
	]).unwrap();
	let v = Vector::from(vec![4., 2.]);
	println!("{}", u.mul_vec(v));
	// [4.]
	// [2.]
	let u = Matrix::from_multiple(vec![
	vec![2., 0.],
	vec![0., 2.],
	]).unwrap();
	let v = Vector::from(vec![4., 2.]);
	println!("{}", u.mul_vec(v));
	// [8.]
	// [4.]
	let u = Matrix::from_multiple(vec![
	vec![2., -2.],
	vec![-2., 2.],
	]).unwrap();
	let v = Vector::from(vec![4., 2.]);
	println!("{}", u.mul_vec(v));
	// [4.]
	// [-4.]
	let u = Matrix::from_multiple(vec![
	vec![1., 0.],
	vec![0., 1.],
	]).unwrap();
	let v = Matrix::from_multiple(vec![
	vec![1., 0.],
	vec![0., 1.],
	]).unwrap();
	println!("{}", u.mul_mat(v));
	// [1., 0.]
	// [0., 1.]
	let u = Matrix::from_multiple(vec![
	vec![1., 0.],
	vec![0., 1.],
	]).unwrap();
	let v = Matrix::from_multiple(vec![
	vec![2., 1.],
	vec![4., 2.],
	]).unwrap();
	println!("{}", u.mul_mat(v));
	// [2., 1.]
	// [4., 2.]
	let u = Matrix::from_multiple(vec![
	vec![3., -5.],
	vec![6., 8.],
	]).unwrap();
	let v = Matrix::from_multiple(vec![
	vec![2., 1.],
	vec![4., 2.],
	]).unwrap();
	println!("{}", u.mul_mat(v));
// [-14., -7.]
// [44., 22.]
}
