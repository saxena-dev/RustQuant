// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// RustQuant: A Rust library for quantitative finance tools.
// Copyright (C) 2023 https://github.com/avhz
// Dual licensed under Apache 2.0 and MIT.
// See:
//      - LICENSE-APACHE.md
//      - LICENSE-MIT.md
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//! Curves module.
//! Curves (in the financial sense) are functions that map
//! a time to a value, such as a yield curve or a swap curve.
//! They may also be known as term structures.

/// Base curve trait.
pub mod curve;
pub use curve::*;

/// Surface implementations.
/// Surfaces are simply [Curve]s with an additional dimension.
/// For example, a volatility surface is a function of time and strike/moneyness.
pub mod surface;
pub use surface::*;

/// Nelson-Siegel curve model.
pub mod nelson_siegel;
pub use nelson_siegel::*;

/// Nelson-Siegel-Svensson curve model.
/// This model is an extension of the Nelson-Siegel model.
pub mod nelson_siegel_svensson;
pub use nelson_siegel_svensson::*;
