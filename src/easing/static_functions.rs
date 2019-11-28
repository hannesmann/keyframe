use crate::easing::*;

// Based on https://gist.githubusercontent.com/gre/1650294/raw/01bf897e14c41f90c8fcda739fdc793790138446/easing.js

/// Linear interpolation from point A to point B. Use with `ease(Linear, ...)`.
pub struct Linear;
impl EasingFunction for Linear {
	#[inline] 
	fn y(&self, x: f64) -> f64 { x }
}

/// Accelerating quadratically from point A to point B. Use with `ease(EaseInQuad, ...)`.
pub struct EaseInQuad;
impl EasingFunction for EaseInQuad {
	#[inline] 
	fn y(&self, x: f64) -> f64 { x * x }
}

/// Decelerating quadratically from point A to point B. Use with `ease(EaseOutQuad, ...)`.
pub struct EaseOutQuad;
impl EasingFunction for EaseOutQuad {
	#[inline] 
	fn y(&self, x: f64) -> f64 { x * (2.0 - x) }
}

/// Accelerating then decelerating quadratically from point A to point B. Use with `ease(EaseInOutQuad, ...)`.
pub struct EaseInOutQuad;
impl EasingFunction for EaseInOutQuad {
	#[inline] 
	fn y(&self, x: f64) -> f64 { 
		if x < 0.5 { 2.0 * x * x } else { -1.0 + (4.0 - 2.0 * x) * x }
	}
}

#[cfg(feature = "shorthand-easing-functions")]
pub(crate) mod shorthand_functions {
	use crate::easing::*;

	/// Linear interpolation from point A to point B.
	#[inline]
	pub fn linear<V: CanEase, T: Float>(from: V, to: V, time: T) -> V { ease(Linear, from, to, time) } 

	/// Accelerating quadratically from point A to point B.
	#[inline]
	pub fn ease_in<V: CanEase, T: Float>(from: V, to: V, time: T) -> V { ease(EaseInQuad, from, to, time) }
	
	/// Decelerating quadratically from point A to point B.
	#[inline]
	pub fn ease_out<V: CanEase, T: Float>(from: V, to: V, time: T) -> V { ease(EaseOutQuad, from, to, time) }

	/// Accelerating then decelerating quadratically from point A to point B.
	#[inline]
	pub fn ease_in_out<V: CanEase, T: Float>(from: V, to: V, time: T) -> V { ease(EaseInOutQuad, from, to, time) }	
	
}
#[cfg(feature = "shorthand-easing-functions")]
pub use shorthand_functions::*;
