use std::f64::consts::PI;

/// Calculates the energy of a quantum well with sides a, b and quantum numbers n1, n2
pub fn well_energy(n1: i64, n2: i64, a: f64, b: f64) -> f64{
   0.5*( (PI * n1 as f64 /a).powi(2) + (PI * n2 as f64 /b).powi(2))
}