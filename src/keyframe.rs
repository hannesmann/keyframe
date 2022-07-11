use alloc::{boxed::Box, fmt, sync::Arc};
use num_traits::Float;

use crate::{
	as_f64, ease_with_scaled_time,
	easing::{EaseInOut, Linear},
	CanTween, EasingFunction,
};

/// Intermediate step in an animation sequence
#[derive(Clone)]
pub struct Keyframe<T> {
	value: T,
	pub(crate) time: f64,
	function: Arc<dyn EasingFunction + Send + Sync>,
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
			value,
			time: if time < F::zero() { 0.0 } else { as_f64(time) },
			function: Arc::new(function),
		}
	}

	/// Same as [`new`](#method.new), but allows you to supply an easing function which size is not known at compile time.
	///
	/// # Arguments
	/// * `value` - The value that this keyframe will be tweened to/from
	/// * `time` - The start time in seconds of this keyframe
	/// * `function` - The easing function to use from the start of this keyframe to the start of the next keyframe
	#[inline]
	pub fn new_dynamic<F: Float>(value: T, time: F, function: Box<dyn EasingFunction + 'static + Send + Sync>) -> Self {
		Keyframe::<T> {
			value,
			time: if time < F::zero() { 0.0 } else { as_f64(time) },
			function: function.into(),
		}
	}

	/// The value of this keyframe
	#[inline]
	pub fn value(&self) -> T
	where
		T: Clone,
	{
		self.value.clone()
	}

	/// The time in seconds at which this keyframe starts in a sequence
	#[inline]
	pub fn time(&self) -> f64 {
		self.time
	}

	/// The easing function that will be used when tweening to another keyframe
	#[inline]
	pub fn function(&self) -> &dyn EasingFunction {
		self.function.as_ref()
	}

	/// Returns the value between this keyframe and the next keyframe at the specified time
	///
	/// # Note
	///
	/// The following applies if:
	/// * The requested time is before the start time of this keyframe: the value of this keyframe is returned
	/// * The requested time is after the start time of next keyframe: the value of the next keyframe is returned
	/// * The start time of the next keyframe is before the start time of this keyframe: the value of the next keyframe is returned
	#[inline]
	pub fn tween_to(&self, next: &Keyframe<T>, time: impl Float) -> T
	where
		T: CanTween + Clone,
	{
		match as_f64(time) {
			// If the requested time starts before this keyframe
			time if time < self.time => self.value.clone(),
			// If the requested time starts after the next keyframe
			time if time > next.time => next.value.clone(),
			// If the next keyframe starts before this keyframe
			_ if next.time < self.time => next.value.clone(),

			time => T::ease(
				self.value.clone(),
				next.value.clone(),
				self.function.y(ease_with_scaled_time(
					Linear,
					0.0,
					1.0,
					time - self.time,
					next.time - self.time,
				)),
			),
		}
	}
}

impl<V, T: Float> From<(V, T)> for Keyframe<V> {
	/// Creates a new keyframe from a tuple of (value, time).
	/// `EaseInOut` will be used as the easing function.
	/// If the time value is negative the keyframe will start at 0.0.
	#[inline]
	fn from(tuple: (V, T)) -> Self {
		Keyframe::new(tuple.0, as_f64(tuple.1), EaseInOut)
	}
}

impl<V, T: Float, F: EasingFunction + 'static + Send + Sync> From<(V, T, F)> for Keyframe<V> {
	/// Creates a new keyframe from a tuple of (value, time, function).
	/// If the time value is negative the keyframe will start at 0.0.
	#[inline]
	fn from(tuple: (V, T, F)) -> Self {
		Keyframe::new(tuple.0, as_f64(tuple.1), tuple.2)
	}
}

impl<V, T: Float> From<(V, T, Box<dyn EasingFunction + 'static + Send + Sync>)> for Keyframe<V> {
	/// Creates a new keyframe from a tuple of (value, time, function).
	/// If the time value is negative the keyframe will start at 0.0.
	#[inline]
	fn from(tuple: (V, T, Box<dyn EasingFunction + 'static + Send + Sync>)) -> Self {
		Keyframe::new_dynamic(tuple.0, as_f64(tuple.1), tuple.2)
	}
}

impl<T: fmt::Display> fmt::Display for Keyframe<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "Keyframe at {:.2} s: {}", self.time, self.value)
	}
}

impl<T: core::fmt::Debug> fmt::Debug for Keyframe<T> {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		write!(f, "Keyframe {{ value: {:?}, time: {:.2} }}", self.value, self.time)
	}
}
