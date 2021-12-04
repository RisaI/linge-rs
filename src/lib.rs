pub mod traits;
pub mod impls;
pub mod precond;
pub mod solvers;
pub mod macros;

// Fields
pub use simba::{scalar::Field, scalar::RealField, scalar::ComplexField};

// Complex numbers
pub use num_complex::Complex;
pub type C32 = Complex<f32>;
pub type C64 = Complex<f64>;

// Prelud
pub mod prelude;
