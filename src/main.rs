use linge_rs::traits::Vector;

fn main() {
    println!("Hello, world!");

    let b = linge_rs::impls::DVector::<f64>::zero_vec(10);

    println!("{}", b.cnorm());
}
