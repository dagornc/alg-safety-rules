//! ALG_SAFETY_RULES — règles de sûreté locales et déterministes.

pub mod rng;
pub mod sim;

pub use rng::PyRandom;
pub use sim::{Action, Params, Resultat, simuler};
