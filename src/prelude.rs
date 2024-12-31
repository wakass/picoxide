//Just use everthing from the error crate
pub use crate::error::*;

//Generic type wrapping, useful for external modules (i think)
pub struct W<T>(pub T);
