mod basic;
mod complex;
mod cosine;
mod cross;
mod determinant;
mod dot;
mod inverse;
mod lerp;
mod linear_combination;
mod mat_mult;
mod matrix;
mod matrix_tools;
mod norms;
mod rank;
mod row_echelon;
mod trace;
mod transpose;
mod vector;
mod vector_tests;
mod vector_tools;
mod cyclic;

use std::println;

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
    println!("\ntrace tests:");
    trace_tests();
    println!("\ntranspose tests:");
    transpose_test();
    println!("\nrow_echelon tests:");
    row_echelon_test();
    println!("\ndeterminant tests:");
    determinant_test();
    println!("\ninverse tests:");
    inverse_test();
	println!("\nrank tests:");
	rank_test();
	println!("\ncyclic tests:");
	cyclic_tests();
}
