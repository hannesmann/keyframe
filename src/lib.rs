pub(crate) use num_traits::Float;
#[cfg(feature = "vectors")]
pub(crate) use mint::{Vector2, Vector3, Vector4};

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
pub struct Keyframe<T: CanTween + Copy + Default + Send + Sync> {
	value: T,
	time: f64,
	function: Box<dyn EasingFunction + Send + Sync>
}

impl<T: CanTween + Copy + Default + Send + Sync> Keyframe<T> {
	/// Creates a new keyframe from the specified values.
	/// If the time value is negative the keyframe will start at 0.0.
	/// 
	/// # Arguments
	/// * `value` - The value that this keyframe will be tweened to/from
	/// * `time` - The start time in seconds of this keyframe
	/// * `function` - The easing function to use from the start of this keyframe to the start of the next keyframe
	pub fn new<F: Float>(value: T, time: F, function: impl EasingFunction + 'static + Send + Sync) -> Self {
		Keyframe::<T> {
			value: value,
			time: if time < F::zero() { 0.0 } else { as_f64(time) },
			function: Box::new(function)
		}
	}

	/// The value of this keyframe
	pub fn value(&self) -> T { self.value }

	/// The time in seconds at which this keyframe starts in a sequence
	pub fn time(&self) -> f64 { self.time }

	/// The easing function that will be used when tweening to another keyframe
	pub fn function(&self) -> &dyn EasingFunction { self.function.as_ref() }

	/// Returns the value between this keyframe and the next keyframe at the specified time
	/// 
	/// # Note
	/// 
	/// The following applies if:
	/// * The requested time is before the start time of this keyframe: the value of this keyframe is returned
	/// * The requested time is after the start time of next keyframe: the value of the next keyframe is returned
	/// * The start time of the next keyframe is before the start time of this keyframe: the value of the next keyframe is returned
	pub fn tween_to(&self, next: &Keyframe<T>, time: impl Float) -> T {
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

impl<V: CanTween + Copy + Default + Send + Sync, T: Float, F: EasingFunction + 'static + Send + Sync> From<(V, T, F)> for Keyframe<V> {
	/// Creates a new keyframe from a tuple of (value, time, function).
	/// If the time value is negative the keyframe will start at 0.0.
	fn from(tuple: (V, T, F)) -> Self { Keyframe::new(tuple.0, as_f64(tuple.1), tuple.2) }
}

impl<T: CanTween + Copy + Default + Send + Sync> Default for Keyframe<T> {
	fn default() -> Self { Keyframe::new(T::default(), 0.0, Linear) }
}

impl<T: CanTween + Copy + Default + Send + Sync + fmt::Display> fmt::Display for Keyframe<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "Keyframe at {} s: {}", self.time, self.value)
	}
}

mod sequence;
pub use sequence::*;

// TODO: KeyframeSequence, impl CanTween for KeyframeSequence, animation! macro, examples