use lingers::{prelude::*};

fn main() {
    let a: DVector<f64> = [ 1., 0., 1. ].into();
    let b: DVector<f64> = vec![ 1.; 3 ].into();

    let _mat: SMatrix<f64, 3, 3> = [
        [1., 0.5, 1. ],
        [0., 1. , 0.5],
        [0., 0. , 1. ]
    ].into();

    println!("{}", a.dot_euclid(&b));
    println!("{}", (a + &b).norm());
}


