//! Portable numeric helpers. All transcendental calls go through `libm`.

/// Square root. Domain checks belong to the interpreter.
#[must_use]
pub fn sqrt(x: f64) -> f64 {
    libm::sqrt(x)
}

/// Sine.
#[must_use]
pub fn sin(x: f64) -> f64 {
    libm::sin(x)
}

/// Cosine.
#[must_use]
pub fn cos(x: f64) -> f64 {
    libm::cos(x)
}

/// Floor toward negative infinity.
#[must_use]
pub fn floor(x: f64) -> f64 {
    libm::floor(x)
}
