use crate::complex::{Comp, Complex};
use crate::cosine::*;
use crate::cross::cross_product;
use crate::cyclic::Cyclic;
use crate::lerp::lerp;
use crate::linear_combination;
use crate::matrix::Matrix;
use crate::vector::Vector;
use std::println;

pub fn test_ops() {
    let mut v = Vector::from(vec![1., 3., 4.]);
	let w = Vector::from(vec![1., 3., 5.]);
	println!("Adding {} to {} :", w, v);
    v += &w;
    println!("{v}");
	println!("Multiplying {v} by 2 :");
    v *= 2.;
    println!("{v}");
	println!("Subtracting {} to {} :", w, v);
    v -= w;
    println!("{v}");
}

pub fn test_lc() {
    let a: Vec<Vector<Comp>> = vec![];
    let b: Vec<f64> = vec![];
    let v: Vector<Comp> = linear_combination::linear_combination::<Comp, f64>(&a, &b);
	println!("Calculating empty linear combiantion:");
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
	println!("Calculating {}*{} + {}*{} + {}*{} + {}*{} :", d[0], c[0], d[1], c[1], d[2], c[2], d[3], c[3]);
    println!("{v2}");

    let e1 = Vector::from(vec![1., 0., 0.]);
    let e2 = Vector::from(vec![0., 1., 0.]);
    let e3 = Vector::from(vec![0., 0., 1.]);
    let v1 = Vector::from(vec![1., 2., 3.]);
    let v2 = Vector::from(vec![0., 10., -100.]);
	println!("Calculating {}*{} + {}*{} + {}*{} :", e1, 10., e2, -2., e3, 0.5);
    println!(
        "{}",
        linear_combination::linear_combination::<f32, f32>(&[e1, e2, e3], &[10., -2., 0.5])
    );
    // [10.]
    // [-2.]
    // [0.5]
	println!("Calculating {}*{} + {}*{} :", v1, 10., v2, -2.);
    println!(
        "{}",
        linear_combination::linear_combination::<f32, f32>(&[v1, v2], &[10., -2.])
    );
    // [10.]
    // [0.]
    // [230.]
}

pub fn test_lerp() {
	let v = Matrix::from_multiple(vec![vec![2., 1.], vec![3., 4.]]).unwrap();
	let w = Matrix::from_multiple(vec![vec![20., 10.], vec![30., 40.]]).unwrap();
	println!("Calculating linear interpolation {} *\n{}+ (1 - {}) *\n{}:", 0.5, v, 0.5, w);
    println!("{}",lerp(v, w, 0.5));

	let v = Matrix::from_multiple(vec![
                vec![Complex { re: 1.0, im: 0.1 }, Complex { re: 0.0, im: -1. }],
                vec![Complex { re: 2.0, im: 0.1 }, Complex { re: 4.0, im: 4.5 }]
            ]).unwrap();
	let w = Matrix::from_multiple(vec![
                vec![
                    Complex { re: -2.0, im: -10. },
                    Complex { re: 50., im: -40. }
                ],
                vec![Complex { re: 10., im: -5. }, Complex { re: -40., im: 100. }]
            ])
            .unwrap();
	println!("Calculating linear interpolation {} *\n{}+ (1 - {}) *\n{}:", 0.5, v, 0.5, w);
    println!("{}", lerp(v, w, 0.3));
}

pub fn test_dot() {
    let mut u = Vector::from(vec![0., 0.]);
    let mut v = Vector::from(vec![1., 1.]);
	println!("Calculating dot product of {} and {}", v, u);
    println!("{}", u.dot(&v));
    // 0.0
    u = Vector::from(vec![1., 1.]);
    v = Vector::from(vec![1., 1.]);
	println!("Calculating dot product of {} and {}", v, u);
    println!("{}", u.dot(&v));
    // 2.0
    u = Vector::from(vec![-1., 6.]);
    v = Vector::from(vec![3., 2.]);
	println!("Calculating dot product of {} and {}", v, u);
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
	println!("Calculating dot product of {} and {}", w, t);
    println!("{} {}", w.dot(&t), w.dot_conj(&t));
    // 1.86 + 6.28i, 8.14 + 6.28i
}

pub fn test_norms() {
    let mut u = Vector::from(vec![0., 0., 0.]);
	println!("Calculating norms of {u}");
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 0.0, 0.0, 0.0
    u = Vector::from(vec![1., 2., 3.]);
	println!("Calculating norms of {u}");
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 6.0, 3.74165738, 3.0
    u = Vector::from(vec![-1., -2.]);
	println!("Calculating norms of {u}");
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 3.0, 2.236067977, 2.0
    let mut v = Vector::from(vec![Complex { re: -1., im: -2. }]);
	println!("Calculating norms of {v}");
    println!("{}, {}, {}", v.norm_1(), v.norm(), v.norm_inf());
    v = Vector::from(vec![
        Complex { re: -1., im: -2. },
        Complex { re: 0., im: 3. },
    ]);
	println!("Calculating norms of {v}");
    println!("{}, {}, {}", v.norm_1(), v.norm(), v.norm_inf());
}

pub fn tests_cos() {
    let mut u = Vector::from(vec![1., 0.]);
    let mut v = Vector::from(vec![1., 0.]);
	println!("Calculating cosine between {} and {}", u, v);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    u = Vector::from(vec![1., 0.]);
    v = Vector::from(vec![0., 1.]);
	println!("Calculating cosine between {} and {}", u, v);
    println!("{}", angle_cos(&u, &v));
    // 0.0
    u = Vector::from(vec![-1., 1.]);
    v = Vector::from(vec![1., -1.]);
	println!("Calculating cosine between {} and {}", u, v);
    println!("{}", angle_cos(&u, &v));
    // -1.0
    u = Vector::from(vec![2., 1.]);
    v = Vector::from(vec![4., 2.]);
	println!("Calculating cosine between {} and {}", u, v);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    u = Vector::from(vec![1., 2., 3.]);
    v = Vector::from(vec![4., 5., 6.]);
	println!("Calculating cosine between {} and {}", u, v);
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
	println!("Calculating cosines between {} and {}", w, t);
    println!(
        "{} {}",
        cf32_angle_re_cos(&w, &t),
        cf32_angle_abs_cos(&w, &t)
    );
}

pub fn test_cross() {
    let u = Vector::from(vec![0., 0., 1.]);
    let v = Vector::from(vec![1., 0., 0.]);
	println!("Calculating cross product between {} and {}", u, v);
    println!("{}", cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from(vec![1., 2., 3.]);
    let v = Vector::from(vec![4., 5., 6.]);
	println!("Calculating cross product between {} and {}", u, v);
    println!("{}", cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from(vec![4., 2., -3.]);
    let v = Vector::from(vec![-2., -5., 16.]);
	println!("Calculating cross product between {} and {}", u, v);
    println!("{}", cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]
    let u = Vector::from(vec![
        Complex { re: 1.0, im: -2.0 },
        Complex { re: 0.0, im: 10.0 },
        Complex {
            re: 3.140,
            im: -10.,
        },
    ]);
    let v = Vector::from(vec![
        Complex { re: -1.0, im: -2.0 },
        Complex {
            re: 1.75,
            im: -2.75,
        },
        Complex { re: -1.0, im: 2.5 },
    ]);
	println!("Calculating cross product between {} and {}", u, v);
    println!("{}", cross_product(&u, &v));
}

pub fn mat_mult_tests() {
    let u = Matrix::from_multiple(vec![vec![1., 0.], vec![0., 1.]]).unwrap();
    let v = Vector::from(vec![4., 2.]);
	println!("Calculating\n{}* {}", u, v);
    println!("{}", u.mul_vec(v));
    // [4.]
    // [2.]
    let u = Matrix::from_multiple(vec![vec![2., 0.], vec![0., 2.]]).unwrap();
    let v = Vector::from(vec![4., 2.]);
	println!("Calculating\n{}* {}", u, v);
    println!("{}", u.mul_vec(v));
    // [8.]
    // [4.]
    let u = Matrix::from_multiple(vec![vec![2., -2.], vec![-2., 2.]]).unwrap();
    let v = Vector::from(vec![4., 2.]);
	println!("Calculating\n{}* {}", u, v);
    println!("{}", u.mul_vec(v));
    // [4.]
    // [-4.]
    let u = Matrix::from_multiple(vec![vec![1., 0.], vec![0., 1.]]).unwrap();
    let v = Matrix::from_multiple(vec![vec![1., 0.], vec![0., 1.]]).unwrap();
	println!("Calculating\n{}*\n{}", u, v);
    println!("{}", u.mul_mat(v));
    // [1., 0.]
    // [0., 1.]
    let u = Matrix::from_multiple(vec![vec![1., 0.], vec![0., 1.]]).unwrap();
    let v = Matrix::from_multiple(vec![vec![2., 1.], vec![4., 2.]]).unwrap();
	println!("Calculating\n{}*\n{}", u, v);
    println!("{}", u.mul_mat(v));
    // [2., 1.]
    // [4., 2.]
    let u = Matrix::from_multiple(vec![vec![3., -5.], vec![6., 8.]]).unwrap();
    let v = Matrix::from_multiple(vec![vec![2., 1.], vec![4., 2.]]).unwrap();
	println!("Calculating\n{}*\n{}", u, v);
    println!("{}", u.mul_mat(v));
    // [-14., -7.]
    // [44., 22.]
    let u = Matrix::from_multiple(vec![
        vec![Complex { re: 2., im: 1. }, Complex { re: 0., im: 0. }],
        vec![Complex { re: 0., im: 0. }, Complex { re: 2., im: 1. }],
    ])
    .unwrap();
    let v = Matrix::from_multiple(vec![
        vec![Complex { re: 2.0, im: -1. }, Complex { re: 3., im: -1. }],
        vec![Complex { re: 3., im: -1. }, Complex { re: 2.0, im: -1. }],
    ])
    .unwrap();
	println!("Calculating\n{}*\n{}", u, v);
    println!("{}", u.mul_mat(v));
    let u = Matrix::from_multiple(vec![vec![3., -5.], vec![6., 8.], vec![1., 2.]]).unwrap();
    let v = Matrix::from_multiple(vec![vec![2., 1., -4.], vec![4., 2., 2.]]).unwrap();
	println!("Calculating\n{}*\n{}", u, v);
    println!("{}", &u * &v);
	println!("Calculating\n{}*\n{}", v, u);
    println!("{}", v * u);
}

pub fn trace_tests() {
    let u = Matrix::from_multiple(vec![vec![1., 0.], vec![0., 1.]]).unwrap();
	println!("Calculating trace of\n{u}");
    println!("{}", u.trace());
    // 2.0
    let u = Matrix::from_multiple(vec![vec![2., -5., 0.], vec![4., 3., 7.], vec![-2., 3., 4.]])
        .unwrap();
	println!("Calculating trace of\n{u}");
    println!("{}", u.trace());
    // 9.0
    let u = Matrix::from_multiple(vec![
        vec![-2., -8., 4.],
        vec![1., -23., 4.],
        vec![0., 6., 4.],
    ])
    .unwrap();
	println!("Calculating trace of\n{u}");
    println!("{}", u.trace());
    // -21.0
    let u = Matrix::from_multiple(vec![
        vec![
            Complex {
                re: 3.140,
                im: -10.,
            },
            Complex {
                re: 3.140,
                im: -10.,
            },
        ],
        vec![
            Complex {
                re: 3.140,
                im: -10.,
            },
            Complex {
                re: 3.140,
                im: -10.,
            },
        ],
    ])
    .unwrap();
	println!("Calculating trace of\n{u}");
    println!("{}", u.trace());
}

pub fn transpose_test() {
    let u = Matrix::from_multiple(vec![
        vec![
            Complex { re: 1., im: -1. },
            Complex {
                re: 3.140,
                im: -10.,
            },
        ],
        vec![
            Complex { re: 0.0, im: -10.5 },
            Complex {
                re: 3.250,
                im: -10.25,
            },
        ],
    ])
    .unwrap();
	println!("Calculating transpose of\n{u}");
    println!("{}", u.transpose());
	println!("Calculating transpose conjugate of\n{u}");
    println!("{}", u.dagger());
    let u = Matrix::from_multiple(vec![vec![3., -5.], vec![6., 8.], vec![1., 2.]]).unwrap();
	println!("Calculating transpose of\n{u}");
    println!("{}", u.transpose());
}

pub fn row_echelon_test() {
	println!("Subjects examples show that what is actually asked is the reduced row echelon for, function for a non reduced version (solution is not unique) was also done out of frustration for the subject");
    let u =
        Matrix::from_multiple(vec![vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.]]).unwrap();
	println!("Non reduced row echelon form of\n{u}");
    println!("{}", u.stand_row_echelon());
	println!("Reduced row echelon form of\n{u}");
    println!("{}\n", u.row_echelon());
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let u = Matrix::from_multiple(vec![vec![1., 2.], vec![3., 4.]]).unwrap();
    println!("Non reduced row echelon form of\n{u}");
    println!("{}", u.stand_row_echelon());
	println!("Reduced row echelon form of\n{u}");
    println!("{}\n", u.row_echelon());
    // [1.0, 0.0]
    // [0.0, 1.0]
    let u = Matrix::from_multiple(vec![vec![1., 2.], vec![2., 4.]]).unwrap();
    println!("Non reduced row echelon form of\n{u}");
    println!("{}", u.stand_row_echelon());
	println!("Reduced row echelon form of\n{u}");
    println!("{}\n", u.row_echelon());
    // [1.0, 2.0]
    // [0.0, 0.0]
    let u = Matrix::from_multiple(vec![
        vec![8., 5., -2., 4., 28.],
        vec![4., 2.5, 20., 4., -4.],
        vec![8., 5., 1., 4., 17.],
    ])
    .unwrap();
    println!("Non reduced row echelon form of\n{u}");
    println!("{}", u.stand_row_echelon());
	println!("Reduced row echelon form of\n{u}");
    println!("{}\n", u.row_echelon());
	let v = u.transpose();
	println!("Non reduced row echelon form of\n{v}");
    println!("{}", v.stand_row_echelon());
	println!("Reduced row echelon form of\n{v}");
    println!("{}\n", v.row_echelon());
    // [1.0, 0.625, 0.0, 0.0, -12.1666667]
    // [0.0, 0.0, 1.0, 0.0, -3.6666667]
    // [0.0, 0.0, 0.0, 1.0, 29.5 ]
    let u = Matrix::from_multiple(vec![
        vec![
            Complex { re: 1., im: -1. },
            Complex {
                re: 3.140,
                im: -10.,
            },
        ],
        vec![
            Complex { re: 0.0, im: -10.5 },
            Complex {
                re: 3.250,
                im: -10.25,
            },
        ],
    ])
    .unwrap();
    println!("Non reduced row echelon form of\n{u}");
    println!("{}", u.stand_row_echelon());
	println!("Reduced row echelon form of\n{u}");
    println!("{}\n", u.row_echelon());
}

pub fn determinant_test() {
    let u = Matrix::from_multiple(vec![vec![1., -1.], vec![-1., 1.]]).unwrap();
	println!("Calculating determinant of\n{u}");
    println!("{}", u.determinant());
    // 0.0
    let u =
        Matrix::from_multiple(vec![vec![2., 0., 0.], vec![0., 2., 0.], vec![0., 0., 2.]]).unwrap();
	println!("Calculating determinant of\n{u}");
    println!("{}", u.determinant());
    // 8.0
    let u = Matrix::from_multiple(vec![vec![8., 5., -2.], vec![4., 7., 20.], vec![7., 6., 1.]])
        .unwrap();
	println!("Calculating determinant of\n{u}");
    println!("{}", u.determinant());
    // -174.0
    let u = Matrix::from_multiple(vec![
        vec![8., 5., -2., 4.],
        vec![4., 2.5, 20., 4.],
        vec![8., 5., 1., 4.],
        vec![28., -4., 17., 1.],
    ])
    .unwrap();
	println!("Calculating determinant of\n{u}");
    println!("{}", u.determinant());
    // 1032
    let u = Matrix::from_multiple(vec![
        vec![
            Complex { re: 1., im: -1. },
            Complex {
                re: 3.140,
                im: -10.,
            },
        ],
        vec![
            Complex { re: 0.0, im: -10.5 },
            Complex {
                re: 3.250,
                im: -10.25,
            },
        ],
    ])
    .unwrap();
	println!("Calculating determinant of\n{u}");
    println!("{}", u.determinant());
}

pub fn inverse_test() {
    let u =
        Matrix::from_multiple(vec![vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.]]).unwrap();
    let v = u.inverse().unwrap();
	println!("Calculating the inverse of u =\n{u}, let's call it v");
    println!("{}", v);
    println!("u * v =\n{}", &u * &v);
    println!("v * u =\n{}\n", &v * &u);
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let u =
        Matrix::from_multiple(vec![vec![2., 0., 0.], vec![0., 2., 0.], vec![0., 0., 2.]]).unwrap();
    let v = u.inverse().unwrap();
    println!("Calculating the inverse of u =\n{u}, let's call it v");
    println!("{}", v);
    println!("u * v =\n{}", &u * &v);
    println!("v * u =\n{}\n", &v * &u);
    // [0.5, 0.0, 0.0]
    // [0.0, 0.5, 0.0]
    // [0.0, 0.0, 0.5]
    let u = Matrix::from_multiple(vec![vec![8., 5., -2.], vec![4., 7., 20.], vec![7., 6., 1.]])
        .unwrap();
    let v = u.inverse().unwrap();
    println!("Calculating the inverse of u =\n{u}, let's call it v");
    println!("{}", v);
    println!("u * v =\n{}", &u * &v);
    println!("v * u =\n{}\n", &v * &u);
    // [0.649425287, 0.097701149, -0.655172414]
    // [-0.781609195, -0.126436782, 0.965517241]
    // [0.143678161, 0.074712644, -0.206896552]
    let u = Matrix::from_multiple(vec![
        vec![
            Complex { re: 1., im: -1. },
            Complex {
                re: 3.140,
                im: -10.,
            },
        ],
        vec![
            Complex { re: 0.0, im: -10.5 },
            Complex {
                re: 3.250,
                im: -10.25,
            },
        ],
    ])
    .unwrap();
    let v = u.inverse().unwrap();
    println!("Calculating the inverse of u =\n{u}, let's call it v");
    println!("{}", v);
    println!("u * v =\n{}", &u * &v);
    println!("v * u =\n{}\n", &v * &u);
}

pub fn rank_test() {
    let u =
        Matrix::from_multiple(vec![vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.]]).unwrap();
	println!("Calculating rank of\n{u}:\n");
    println!("{}", u.rank());
    // 3
    let u = Matrix::from_multiple(vec![
        vec![1., 2., 0., 0.],
        vec![2., 4., 0., 0.],
        vec![-1., 2., 1., 1.],
    ])
    .unwrap();
    println!("{}", u.rank());
    // 2
    let u = Matrix::from_multiple(vec![
        vec![8., 5., -2.],
        vec![4., 7., 20.],
        vec![7., 6., 1.],
        vec![21., 18., 7.],
    ])
    .unwrap();
	println!("Calculating rank of\n{u}:\n");
    println!("{}", u.rank());
    // 3
    let u = Matrix::from_multiple(vec![
        vec![
            Complex { re: 1., im: -1. },
            Complex {
                re: 3.140,
                im: -10.,
            },
        ],
        vec![
            Complex { re: 0.0, im: -10.5 },
            Complex {
                re: 3.250,
                im: -10.25,
            },
        ],
    ])
    .unwrap();
	println!("Calculating rank of\n{u}:\n");
    println!("{}", u.rank());
}

pub fn cyclic_tests() {
    let m = Matrix::from_multiple(vec![
        vec![
            Cyclic::<11>::new(1),
            Cyclic::<11>::new(9),
            Cyclic::<11>::new(5),
        ],
        vec![
            Cyclic::<11>::new(0),
            Cyclic::<11>::new(9),
            Cyclic::<11>::new(5),
        ],
        vec![
            Cyclic::<11>::new_i64(-1),
            Cyclic::<11>::new(0),
            Cyclic::<11>::new(2),
        ],
    ])
    .unwrap();
    let n = m.inverse().unwrap();
    println!("{}", m.row_echelon());
	println!("{}", &m * &n);
	println!("{}", &n * &m);
	println!("{}", n.rank());
	println!("{}", n.determinant());
	println!("{}\n", m.determinant());
	println!("{}", m);
	println!("{}", n);
}
