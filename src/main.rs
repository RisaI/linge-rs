use lingers::{prelude::*};

fn main() {
    let a: DVector<f64> = vec![ 1., 0., 1. ].into();
    let b: DVector<f64> = vec![ 1.; 3 ].into();

    println!("{}", a.dot_euclid(&b));
    println!("{}", (a + &b).norm());
}


