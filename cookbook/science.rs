#!/usr/bin/env run-cargo-script
// cargo-deps: ndarray, num

#[macro_use(array)]
extern crate ndarray;
extern crate num;

use ndarray::Array;

fn vector_sum() {
    let a = Array::from_vec(vec![1., 2., 3., 4., 5.]);
    let b = Array::from_vec(vec![5., 4., 3., 2., 1.]);
    let mut c = Array::from_vec(vec![1., 2., 3., 4., 5.]);
    let mut d = Array::from_vec(vec![5., 4., 3., 2., 1.]);

    let z = a + b;
    let w = &c + &d;

    let epsilon = 1e-8;
    for elem in z.iter() {
        let diff: f32 = *elem - 6.;
        assert!(diff.abs() < epsilon);
    }

    println!("c = {}", c);
    c[0] = 10.;
    d[1] = 10.;

    for elem in w.iter() {
        let diff: f32 = *elem - 6.;
        assert!(diff.abs() < epsilon);
    }
}

use ndarray::{Array1, ArrayView1};

fn l1_norm(x: ArrayView1<f64>) -> f64 {
    x.fold(0., |acc, elem| acc + elem.abs())
}

fn l2_norm(x: ArrayView1<f64>) -> f64 {
    x.dot(&x).sqrt()
}

fn normalize(mut x: Array1<f64>) -> Array1<f64> {
    let norm = l2_norm(x.view());
    x.mapv_inplace(|e| e/norm);
    x
}

fn vector_norm() {
    let x = array![1., 2., 3., 4., 5.];
    println!("||x||_2 = {}", l2_norm(x.view()));
    println!("||x||_1 = {}", l1_norm(x.view()));
    println!("Normalizing x yields {:?}", normalize(x));
}

fn metrix_sum() {
    use ndarray::arr2;

    let a = arr2(&[
        [1, 2, 3],
        [4, 5, 6],
    ]);

    let b = arr2(&[
        [6, 5, 4],
        [3, 2, 1],
    ]);

    println!("Sum: {}", a + b);
}

fn matrix_dot_product() {
    use ndarray::arr2;

    let a = arr2(&[
        [1, 2, 3],
        [4, 5, 6],
    ]);

    let b = arr2(&[
        [6, 3],
        [5, 2],
        [4, 1],
    ]);

    println!("Dot Product: {}", a.dot(&b));
}

fn scalar_product() {
    use ndarray::{arr1, arr2};

    let scaler = 4;
    let vector = arr1(&[1, 2, 3]);
    let matrix = arr2(&[
        [4, 5, 6],
        [7, 8, 9],
    ]);

    let new_vector: Array1<_> = scaler * vector;
    println!("New Vector: {}", new_vector);

    let new_matrix = matrix.dot(&new_vector);
    println!("New Matrix: {}", new_matrix);
}

fn find_hypotenuse() {
    let angle: f64 = 2.0;
    let side_length = 80.0;

    let hypotenuse = side_length / angle.sin();
    println!("Hypotenuse: {}", hypotenuse);
}

fn verify_tanx() {
    let x: f64 = 6.0;

    let a = x.tan();
    let b = x.sin() / x.cos();
    assert_eq!(a, b);
}

fn earth_distance() {
    let earth_radius_kilometer = 6371.0_f64;
    let (paris_lat_deg, paris_lon_deg) = (48.85341_f64, -2.34880_f64);
    let (london_lat_deg, london_lon_deg) = (51.50853_f64, -0.12574_f64);

    let paris_lat = paris_lat_deg.to_radians();
    let london_lat = london_lat_deg.to_radians();

    let delta_lat = (paris_lat_deg - london_lat_deg).to_radians();
    let delta_lon = (paris_lon_deg - london_lon_deg).to_radians();

    let central_angle_inner = (delta_lat / 2.0).sin().powi(2)
        + paris_lat.cos() * london_lat.cos() * (delta_lon / 2.0).sin().powi(2);

    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    let distance = earth_radius_kilometer * central_angle;

    println!(
        "Distance between Paris and London on the surface of Earth is {:.1} kilometers",
        distance,
    );
}

use num::complex::Complex;

fn complex_num() {
    let complex_int = Complex::new(10, 20);
    let complex_float = Complex::new(19.1, 29.2);

    println!("Complex Integer: {}", complex_int);
    println!("Complex Float: {}", complex_float);
}

fn complex_sum() {
    let complex_num1 = Complex::new(10.0, 20.0);
    let complex_num2 = Complex::new(3.1, -4.2);

    let sum = complex_num1 + complex_num2;

    println!("Sum: {}", sum);
}

fn complex_fn() {
    use std::f64::consts::PI;

    let x = Complex::new(0.0, 2.0 * PI);

    println!("e^(2i * pi) = {}", x.exp());
}

fn main() {
    vector_sum();
    vector_norm();
    metrix_sum();
    matrix_dot_product();
    scalar_product();

    find_hypotenuse();
    verify_tanx();
    earth_distance();

    complex_num();
    complex_sum();
    complex_fn();

    // statistics
    // miscellaneous
}
