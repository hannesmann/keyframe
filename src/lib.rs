//! A simple library for animation in Rust
//!
//! ## Usage
//!
//! Tweening between two values is done with [`ease(function, from, to, time)`](fn.ease.html).
//! `from` and `to` can be any type that implements [`CanTween`](trait.CanTween.html), such as `f64` or `mint::Vector2`, while `time` needs to be a floating-point value between zero and one.
//! `function` specifies the transition between `from` and `to` and is any type that implements [`EasingFunction`](trait.EasingFunction.html).
//!
//! [`AnimationSequence`](struct.AnimationSequence.html) can be used to create more complex animations that keep track of keyframes, time, etc.
//! You can create animation sequences with the [`keyframes![...]`](macro.keyframes.html) macro, from an iterator or from a vector.
//!
//! ## Examples
//!
//! An example visualizer is included in `examples/`. Run `cargo run --example visualizer --release` to start it. (ggez is really slow in debug mode!)
//!
//! Tweening:
//!
//! ```rust
//! use keyframe::{ease, functions::EaseInOut};
//!
//! fn example() -> f64 {
//!     let a = 0.0;
//!     let b = 2.0;
//!     let time = 0.5;
//!
//!     ease(EaseInOut, a, b, time)
//! }
//! ```
//!
//! Animation sequences:
//!
//! ```rust
//! use keyframe::{keyframes, Keyframe, AnimationSequence, functions::Linear};
//!
//! fn example() {
//!    // (value, time) or (value, time, function)
//!    let mut sequence = keyframes![
//!         (0.5, 0.0), // <-- EaseInOut used from 0.0 to 0.3
//!         (1.5, 0.3, Linear), // <-- Linear used from 0.3 to 1.0
//!         (2.5, 1.0) // <-- Easing function here is never used, since we're at the end
//!    ];
//!
//!   sequence.advance_by(0.65);
//!
//!    assert_eq!(sequence.now(), 2.0);
//!    assert_eq!(sequence.duration(), 1.0);
//! }
//! ```
//!
//! Custom structures:
//!
//! ```rust
//! use keyframe::mint::Point2;
//! // This macro works with any structure as long as it only consists of types that implement "CanTween"
//! use keyframe_derive::CanTween;
//!
//! #[derive(CanTween)]
//! struct MySubStructure {
//!     a: f32
//! }
//!
//! #[derive(CanTween)]
//! struct MyStructure {
//!     a: f64,
//!     b: Point2<f64>,
//!     c: f32,
//!     d: Vec<MySubStructure> // BEWARE! This will panic if "from" and "to" are different lengths.
//! }
//!
//! // Also works with unnamed structures
//! #[derive(CanTween)]
//! struct UnnamedStructure(MyStructure, f64);
//! ```
#![no_std]

#[cfg(feature = "mint_types")]
pub use mint;
pub use num_traits;

#[cfg(feature = "mint_types")]
pub(crate) use mint::{Point2, Point3, Vector2, Vector3, Vector4};
pub(crate) use num_traits::Float;

#[cfg(feature = "alloc")]
extern crate alloc;

pub(crate) fn as_f64(value: impl Float) -> f64 {
	value.to_f64().expect("Value not representable in f64")
}
pub(crate) fn as_t<T: Float>(value: f64) -> T {
	match value {
		_ if value > as_f64(T::max_value()) => T::max_value(),
		_ if value < as_f64(T::min_value()) => T::min_value(),
		#[cfg(feature = "alloc")]
		_ => T::from(value).expect(&alloc::format!(
			"{} not representable in chosen float type",
			value
		)),
		#[cfg(not(feature = "alloc"))]
		_ => T::from(value).expect("value not representable in chosen float type"),
	}
}

/// Definitions for various easing functions
///
/// <div class="function-preview" data-function="t" data-struct="Linear"></div>
/// <div class="function-preview" data-function="t * t * t" data-struct="EaseIn"></div>
/// <div class="function-preview" data-function="(--t) * t * t + 1" data-struct="EaseOut"></div>
/// <div class="function-preview" data-function="t<.5 ? 4*t*t*t : (t-1)*(2*t-2)*(2*t-2)+1" data-struct="EaseInOut"></div>
pub mod functions;
use functions::*;

mod easing;
pub use easing::*;

#[cfg(feature = "alloc")]
mod keyframe;
#[cfg(feature = "alloc")]
pub use keyframe::*;

#[cfg(feature = "alloc")]
mod sequence;
#[cfg(feature = "alloc")]
pub use sequence::*;
