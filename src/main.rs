mod basic;
mod complex;
mod cosine;
mod cross;
mod dot;
mod lerp;
mod linear_combination;
mod matrix;
mod matrix_tools;
mod norms;
mod vector;
mod vector_tests;
mod vector_tools;
mod mat_mult;

use vector_tests::*;

fn main() {
    println!("basic operation tests:");
    test_ops();
    println!("\nlinear_combination tests:");
    test_lc();
    println!("\ndot product tests:");
    test_dot();
    println!("\nlerp tests");
    test_lerp();
    println!("\nNorms tests:");
    test_norms();
    println!("\ncosine tests:");
    tests_cos();
	println!("\ncross prod tests:");
    test_cross();
	println!("\nmatrix prod tests:");
    mat_mult_tests();
}
