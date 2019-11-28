use crate::*;

/// Implementation of a 2D curve function for use in easing between two points. 
pub trait EasingFunction {
	/// Based on an X position, calculate the Y position. 
	/// 0.0-1.0 is start and end on both axes, both are allowed to go out of bounds. 
	/// 
	/// # Note
	/// 
	/// Because this method has a `&self` argument this trait can be used to both implement a static curve function (e.g. a linear interpolation) 
	/// or a dynamic curve function (e.g. a bezier curve with an arbitrary number of points).
	/// 
	/// Since a static curve function will have zero size the size of a `Box<dyn EasingFunction>` will be the same size as a vtable. 
	/// This also means you can specify a static curve function with only the name of the type (e.g. `ease(EaseInOut, 0.0, 1.0, 0.5)`).
	fn y(&self, x: f64) -> f64;
}

#[inline]
fn as_f64<T: Float>(value: T) -> f64 { value.to_f64().expect(&format!("Value not representable in f64")) }

#[inline]
fn as_t<T: Float>(value: f64) -> T { 
	match value {
		_ if value > as_f64(T::max_value()) => T::max_value(),
		_ if value < as_f64(T::min_value()) => T::min_value(),
		_ => T::from(value).expect(&format!("{} not representable in chosen float type", value)) 
	}
}

/// Types that can be used with easing function.
pub trait CanEase {
	fn ease<T: Float>(from: Self, to: Self, position: T) -> Self;
}

impl CanEase for f32 {
	fn ease<T: Float>(from: Self, to: Self, position: T) -> Self {
		as_t(as_f64(from + (to - from)) * as_f64(position))
	}
}

impl CanEase for f64 {
	fn ease<T: Float>(from: Self, to: Self, position: T) -> Self {
		as_t(as_f64(from + (to - from)) * as_f64(position))
	}
}

/// Returns the value at a specified X position on the curve between point A and point B. 
/// Bounds are 0.0-1.0 for the X position but it can go out of bounds.
pub fn ease_with_unbounded_x<V: CanEase, X: Float>(function: impl EasingFunction, from: V, to: V, x: X) -> V {
	V::ease::<X>(from, to, as_t(function.y(as_f64(x))))
}

/// Returns the value at a specified X position on the curve between point A and point B. 
/// The X position is limited to a range between 0.0 and 1.0.
pub fn ease<V: CanEase, X: Float>(function: impl EasingFunction, from: V, to: V, x: X) -> V {
	ease_with_unbounded_x(function, from, to, match x {
		_ if x < X::zero() => X::zero(),
		 _ if x > X::one() => X::one(),
		_ => x
	})
}

/// Returns the value at a specified X position on the curve between point A and point B. 
/// The X position is limited to a range between 0.0 and `max_x`.
pub fn ease_with_scaled_x<V: CanEase, X: Float>(function: impl EasingFunction, from: V, to: V, x: X, max_x: X) -> V {
	ease(function, from, to, match x {
		_ if x < X::zero() => X::zero(),
		 _ if x > max_x => X::one(),
		_ => x / max_x
	})
}

/// Linear interpolation from point A to point B. Use with `ease(Linear, ...)`.
pub struct Linear;
impl EasingFunction for Linear {
	#[inline] 
	fn y(&self, x: f64) -> f64 { x }
}

#[cfg(feature = "vectors")]
mod vector_impls {
	use crate::easing::*;

	impl<V: Float> CanEase for Vector2<V> {
		fn ease<T: Float>(from: Self, to: Self, position: T) -> Self {
			Self {
				x: as_t(as_f64(from.x + (to.x - from.x)) * as_f64(position)),
				y: as_t(as_f64(from.y + (to.y - from.y)) * as_f64(position))
			}
		}
	}

	impl<V: Float> CanEase for Vector3<V> {
		fn ease<T: Float>(from: Self, to: Self, position: T) -> Self {
			Self {
				x: as_t(as_f64(from.x + (to.x - from.x)) * as_f64(position)),
				y: as_t(as_f64(from.y + (to.y - from.y)) * as_f64(position)),
				z: as_t(as_f64(from.z + (to.z - from.z)) * as_f64(position))
			}
		}
	}

	impl<V: Float> CanEase for Vector4<V> {
		fn ease<T: Float>(from: Self, to: Self, position: T) -> Self {
			Self {
				x: as_t(as_f64(from.x + (to.x - from.x)) * as_f64(position)),
				y: as_t(as_f64(from.y + (to.y - from.y)) * as_f64(position)),
				z: as_t(as_f64(from.z + (to.z - from.z)) * as_f64(position)),
				w: as_t(as_f64(from.w + (to.w - from.w)) * as_f64(position))
			}
		}
	}
}
#[cfg(feature = "vectors")]
pub use vector_impls::*;

#[cfg(feature = "shorthand-easing-functions")]
pub(crate) mod shorthand_functions {
	use crate::easing::*;

	/// Linear interpolation from point A to point B.
	pub fn linear<V: CanEase, T: Float>(from: V, to: V, time: T) -> V { ease(Linear, from, to, time) } 
}
#[cfg(feature = "shorthand-easing-functions")]
pub use shorthand_functions::*;