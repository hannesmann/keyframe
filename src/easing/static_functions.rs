use crate::easing::*;

// Based on https://gist.githubusercontent.com/gre/1650294/raw/01bf897e14c41f90c8fcda739fdc793790138446/easing.js

/// Linear interpolation from point A to point B. Use with `ease(Linear, ...)`.
/// 
/// <div class="function-preview" data-function="t"></div>
pub struct Linear;
impl EasingFunction for Linear {
	#[inline] 
	fn y(&self, x: f64) -> f64 { x }
}

/// Accelerating quadratically from point A to point B. Use with `ease(EaseInQuad, ...)`.
/// 
/// <div class="function-preview" data-function="t * t"></div>
pub struct EaseInQuad;
impl EasingFunction for EaseInQuad {
	#[inline] 
	fn y(&self, x: f64) -> f64 { x * x }
}

/// Decelerating quadratically from point A to point B. Use with `ease(EaseOutQuad, ...)`.
/// 
/// <div class="function-preview" data-function="t * (2-t)"></div>
pub struct EaseOutQuad;
impl EasingFunction for EaseOutQuad {
	#[inline] 
	fn y(&self, x: f64) -> f64 { x * (2.0 - x) }
}

/// Accelerating then decelerating quadratically from point A to point B. Use with `ease(EaseInOutQuad, ...)`.
/// 
/// <div class="function-preview" data-function="t<.5 ? 2*t*t : -1+(4-2*t)*t"></div>
pub struct EaseInOutQuad;
impl EasingFunction for EaseInOutQuad {
	#[inline] 
	fn y(&self, x: f64) -> f64 { 
		if x < 0.5 { 2.0 * x * x } else { -1.0 + (4.0 - 2.0 * x) * x }
	}
}
