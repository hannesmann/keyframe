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

pub use num_traits;
#[cfg(feature = "mint_types")]
pub use mint;

pub(crate) use num_traits::Float;
#[cfg(feature = "mint_types")]
pub(crate) use mint::{Vector2, Vector3, Vector4, Point2, Point3};

use std::fmt;

pub(crate) fn as_f64(value: impl Float) -> f64 { value.to_f64().expect(&format!("Value not representable in f64")) }
pub(crate) fn as_t<T: Float>(value: f64) -> T {
	match value {
		_ if value > as_f64(T::max_value()) => T::max_value(),
		_ if value < as_f64(T::min_value()) => T::min_value(),
		_ => T::from(value).expect(&format!("{} not representable in chosen float type", value))
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

/// Intermediate step in an animation sequence
pub struct Keyframe<T> {
	value: T,
	time: f64,
	function: Box<dyn EasingFunction + Send + Sync>
}

impl<T> Keyframe<T> {
	/// Creates a new keyframe from the specified values.
	/// If the time value is negative the keyframe will start at 0.0.
	///
	/// # Arguments
	/// * `value` - The value that this keyframe will be tweened to/from
	/// * `time` - The start time in seconds of this keyframe
	/// * `function` - The easing function to use from the start of this keyframe to the start of the next keyframe
	#[inline]
	pub fn new<F: Float>(value: T, time: F, function: impl EasingFunction + 'static + Send + Sync) -> Self {
		Keyframe::<T> {
			value: value,
			time: if time < F::zero() { 0.0 } else { as_f64(time) },
			function: Box::new(function)
		}
	}

	/// The value of this keyframe
	#[inline]
	pub fn value(&self) -> T where T: Copy { self.value }

	/// The time in seconds at which this keyframe starts in a sequence
	#[inline]
	pub fn time(&self) -> f64 { self.time }

	/// The easing function that will be used when tweening to another keyframe
	#[inline]
	pub fn function(&self) -> &dyn EasingFunction { self.function.as_ref() }

	/// Returns the value between this keyframe and the next keyframe at the specified time
	///
	/// # Note
	///
	/// The following applies if:
	/// * The requested time is before the start time of this keyframe: the value of this keyframe is returned
	/// * The requested time is after the start time of next keyframe: the value of the next keyframe is returned
	/// * The start time of the next keyframe is before the start time of this keyframe: the value of the next keyframe is returned
	#[inline]
	pub fn tween_to(&self, next: &Keyframe<T>, time: impl Float) -> T where T: CanTween + Copy {
		match as_f64(time) {
			// If the requested time starts before this keyframe
			time if time < self.time => self.value,
			// If the requested time starts after the next keyframe
			time if time > next.time => next.value,
			// If the next keyframe starts before this keyframe
			_ if next.time < self.time => next.value,

			time => T::ease(self.value, next.value, self.function.y(ease_with_scaled_time(Linear, 0.0, 1.0, time - self.time, next.time - self.time)))
		}
	}
}

impl<V, T: Float> From<(V, T)> for Keyframe<V> {
	/// Creates a new keyframe from a tuple of (value, time).
	/// `EaseInOut` will be used as the easing function.
	/// If the time value is negative the keyframe will start at 0.0.
	#[inline]
	fn from(tuple: (V, T)) -> Self { Keyframe::new(tuple.0, as_f64(tuple.1), EaseInOut) }
}

impl<V, T: Float, F: EasingFunction + 'static + Send + Sync> From<(V, T, F)> for Keyframe<V> {
	/// Creates a new keyframe from a tuple of (value, time, function).
	/// If the time value is negative the keyframe will start at 0.0.
	#[inline]
	fn from(tuple: (V, T, F)) -> Self { Keyframe::new(tuple.0, as_f64(tuple.1), tuple.2) }
}

impl<T: fmt::Display> fmt::Display for Keyframe<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "Keyframe at {:.2} s: {}", self.time, self.value)
	}
}

impl<T: fmt::Debug> fmt::Debug for Keyframe<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "Keyframe {{ value: {:?}, time: {:.2} }}", self.value, self.time)
	}
}

mod sequence;
pub use sequence::*;