pub(crate) use crate::*;

use std::borrow::Borrow;

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
pub fn ease_with_unbounded_time<V: CanTween, F: EasingFunction>(function: impl Borrow<F>, from: V, to: V, time: impl Float) -> V {
	V::ease(from, to, function.borrow().y(as_f64(time)))
}

/// Returns the value at a specified X position on the curve between point A and point B. 
/// Time is limited to a range between 0.0 and 1.0.
#[inline]
pub fn ease<V: CanTween, T: Float, F: EasingFunction>(function: impl Borrow<F>, from: V, to: V, time: T) -> V {
	ease_with_unbounded_time(function, from, to, match time {
		_ if time < T::zero() => T::zero(),
		 _ if time > T::one() => T::one(),
		_ => time
	})
}

/// Returns the value at a specified X position on the curve between point A and point B. 
/// Time is limited to a range between 0.0 and `max_time`.
#[inline]
pub fn ease_with_scaled_time<V: CanTween, T: Float, F: EasingFunction>(function: impl Borrow<F>, from: V, to: V, time: T, max_time: T) -> V {
	ease(function, from, to, match time {
		_ if time < T::zero() => T::zero(),
		_ if time > max_time => T::one(),
		_ => time / max_time
	})
}

#[cfg(feature = "mint_types")]
mod mint_type_impls {
	use crate::easing::*;

	impl<V: CanTween> CanTween for Vector2<V> {
		#[inline]
		fn ease(from: Self, to: Self, time: impl Float) -> Self {
			Self {
				x: V::ease(from.x, to.x, time),
				y: V::ease(from.y, to.y, time)
			}
		}
	}

	impl<V: CanTween> CanTween for Vector3<V> {
		#[inline]
		fn ease(from: Self, to: Self, time: impl Float) -> Self {
			Self {
				x: V::ease(from.x, to.x, time),
				y: V::ease(from.y, to.y, time),
				z: V::ease(from.z, to.z, time)
			}
		}
	}

	impl<V: CanTween> CanTween for Vector4<V> {
		#[inline]
		fn ease(from: Self, to: Self, time: impl Float) -> Self {
			Self {
				x: V::ease(from.x, to.x, time),
				y: V::ease(from.y, to.y, time),
				z: V::ease(from.z, to.z, time),
				w: V::ease(from.w, to.w, time)
			}
		}
	}

	impl<V: CanTween> CanTween for Point2<V> {
		#[inline]
		fn ease(from: Self, to: Self, time: impl Float) -> Self {
			Self {
				x: V::ease(from.x, to.x, time),
				y: V::ease(from.y, to.y, time)
			}
		}
	}

	impl<V: CanTween> CanTween for Point3<V> {
		#[inline]
		fn ease(from: Self, to: Self, time: impl Float) -> Self {
			Self {
				x: V::ease(from.x, to.x, time),
				y: V::ease(from.y, to.y, time),
				z: V::ease(from.z, to.z, time)
			}
		}
	}
}

#[cfg(feature = "mint_types")]
pub use mint_type_impls::*;