pub(crate) use crate::*;

/// Implementation of a 2D curve function for easing between two points
pub trait EasingFunction {
	/// For an X position on the curve, calculate the Y position. 
	/// 0.0-1.0 is start and end on both axes but values can go out of bounds. 
	/// 
	/// # Note
	/// 
	/// Because this method has a `&self` argument this trait can be used to both implement a "static" curve function (e.g. a linear interpolation) 
	/// or a "dynamic" curve function (e.g. a bezier curve with user defined inputs).
	/// 
	/// Since a static curve function will have zero size the size of a `dyn EasingFunction` will be the same size as a vtable. 
	/// This also means you can specify a static curve function with only the name of the type (e.g. `ease(EaseInOut, 0.0, 1.0, 0.5)`).
	fn y(&self, x: f64) -> f64;
}

/// Type that can be used with an easing function
pub trait CanTween {
	fn ease(from: Self, to: Self, time: impl Float) -> Self;
}

impl CanTween for f32 {
	#[inline]
	fn ease(from: Self, to: Self, time: impl Float) -> Self {
		as_t(as_f64(from) + as_f64(to - from) * as_f64(time))
	}
}

impl CanTween for f64 {
	#[inline]
	fn ease(from: Self, to: Self, time: impl Float) -> Self {
		as_t(as_f64(from) + as_f64(to - from) * as_f64(time))	
	}
}

/// Returns the value at a specified X position on the curve between point A and point B. 
/// Bounds are 0.0-1.0 for the time argument but it can go out of bounds.
#[inline]
pub fn ease_with_unbounded_time<V: CanTween>(function: impl EasingFunction, from: V, to: V, time: impl Float) -> V {
	V::ease(from, to, function.y(as_f64(time)))
}

/// Returns the value at a specified X position on the curve between point A and point B. 
/// Time is limited to a range between 0.0 and 1.0.
#[inline]
pub fn ease<V: CanTween, T: Float>(function: impl EasingFunction, from: V, to: V, time: T) -> V {
	ease_with_unbounded_time(function, from, to, match time {
		_ if time < T::zero() => T::zero(),
		 _ if time > T::one() => T::one(),
		_ => time
	})
}

/// Returns the value at a specified X position on the curve between point A and point B. 
/// Time is limited to a range between 0.0 and `max_time`.
#[inline]
pub fn ease_with_scaled_time<V: CanTween, T: Float>(function: impl EasingFunction, from: V, to: V, time: T, max_time: T) -> V {
	ease(function, from, to, match time {
		_ if time < T::zero() => T::zero(),
		 _ if time > max_time => T::one(),
		_ => time / max_time
	})
}

/// Returns the value at a specified X position on an accelerating curve between point A and point B. 
/// Time is limited to a range between 0.0 and 1.0.
/// 
/// <div class="function-preview" data-function="t * t * t"></div>
#[inline]
pub fn ease_in<V: CanTween, T: Float>(from: V, to: V, time: T) -> V {
	ease(functions::EaseIn, from, to, time)
}

/// Returns the value at a specified X position on a decelerating curve between point A and point B. 
/// Time is limited to a range between 0.0 and 1.0.
/// 
/// <div class="function-preview" data-function="(--t) * t * t + 1"></div>
#[inline]
pub fn ease_out<V: CanTween, T: Float>(from: V, to: V, time: T) -> V {
	ease(functions::EaseOut, from, to, time)
}

/// Returns the value at a specified X position on an accelerating and decelerating curve between point A and point B. 
/// Time is limited to a range between 0.0 and 1.0.
/// 
/// <div class="function-preview" data-function="t<.5 ? 4*t*t*t : (t-1)*(2*t-2)*(2*t-2)+1"></div>
#[inline]
pub fn ease_in_out<V: CanTween, T: Float>(from: V, to: V, time: T) -> V {
	ease(functions::EaseInOut, from, to, time)
}

#[cfg(feature = "vectors")]
mod vector_impls {
	use crate::easing::*;

	impl<V: Float> CanTween for Vector2<V> {
		#[inline]
		fn ease(from: Self, to: Self, time: impl Float) -> Self {
			Self {
				x: as_t(as_f64(from.x + (to.x - from.x)) * as_f64(time)),
				y: as_t(as_f64(from.y + (to.y - from.y)) * as_f64(time))
			}
		}
	}

	impl<V: Float> CanTween for Vector3<V> {
		#[inline]
		fn ease(from: Self, to: Self, time: impl Float) -> Self {
			Self {
				x: as_t(as_f64(from.x + (to.x - from.x)) * as_f64(time)),
				y: as_t(as_f64(from.y + (to.y - from.y)) * as_f64(time)),
				z: as_t(as_f64(from.z + (to.z - from.z)) * as_f64(time))
			}
		}
	}

	impl<V: Float> CanTween for Vector4<V> {
		#[inline]
		fn ease(from: Self, to: Self, time: impl Float) -> Self {
			Self {
				x: as_t(as_f64(from.x + (to.x - from.x)) * as_f64(time)),
				y: as_t(as_f64(from.y + (to.y - from.y)) * as_f64(time)),
				z: as_t(as_f64(from.z + (to.z - from.z)) * as_f64(time)),
				w: as_t(as_f64(from.w + (to.w - from.w)) * as_f64(time))
			}
		}
	}
}

#[cfg(feature = "vectors")]
pub use vector_impls::*;