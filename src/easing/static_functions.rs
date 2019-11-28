use crate::easing::*;

/// Linear interpolation from point A to point B. Use with `ease(Linear, ...)`.
pub struct Linear;
impl EasingFunction for Linear {
	#[inline] 
	fn y(&self, x: f64) -> f64 { x }
}

#[cfg(feature = "shorthand-easing-functions")]
pub(crate) mod shorthand_functions {
	use crate::easing::*;

	/// Linear interpolation from point A to point B.
	#[inline]
	pub fn linear<V: CanEase, T: Float>(from: V, to: V, time: T) -> V { ease(Linear, from, to, time) } 
}
#[cfg(feature = "shorthand-easing-functions")]
pub use shorthand_functions::*;