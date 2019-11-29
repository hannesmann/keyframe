use crate::tweening::*;

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

/// Accelerating cubically from point A to point B. Use with `ease(EaseIn, ...)`.
/// 
/// <div class="function-preview" data-function="t * t * t"></div>
pub struct EaseIn;
impl EasingFunction for EaseIn {
	#[inline] 
	fn y(&self, x: f64) -> f64 { x * x * x }
}

/// Decelerating cubically from point A to point B. Use with `ease(EaseOut, ...)`.
/// 
/// <div class="function-preview" data-function="(--t) * t * t + 1"></div>
pub struct EaseOut;
impl EasingFunction for EaseOut {
	#[inline] 
	fn y(&self, x: f64) -> f64 { 
		let x_minus_one = x - 1.0;
		1.0 + x_minus_one * x_minus_one * x_minus_one
	}
}

/// Accelerating then decelerating cubically from point A to point B. Use with `ease(EaseInOut, ...)`.
/// 
/// <div class="function-preview" data-function="t<.5 ? 4*t*t*t : (t-1)*(2*t-2)*(2*t-2)+1"></div>
pub struct EaseInOut;
impl EasingFunction for EaseInOut {
	#[inline] 
	fn y(&self, x: f64) -> f64 { 
		if x < 0.5 { 4.0 * x * x * x } 
		else { 
			let x_minus_one = x - 1.0;
			x_minus_one * (2.0 * x - 2.0) * (2.0 * x - 2.0) + 1.0
		}
	}
}

/// Returns the value at a specified X position on an accelerating curve between point A and point B. 
/// Time is limited to a range between 0.0 and 1.0.
/// 
/// <div class="function-preview" data-function="t * t * t"></div>
#[inline]
pub fn ease_in<V: CanTween, T: Float>(from: V, to: V, time: T) -> V {
	ease(EaseIn, from, to, time)
}

/// Returns the value at a specified X position on a decelerating curve between point A and point B. 
/// Time is limited to a range between 0.0 and 1.0.
/// 
/// <div class="function-preview" data-function="(--t) * t * t + 1"></div>
#[inline]
pub fn ease_out<V: CanTween, T: Float>(from: V, to: V, time: T) -> V {
	ease(EaseOut, from, to, time)
}

/// Returns the value at a specified X position on an accelerating and decelerating curve between point A and point B. 
/// Time is limited to a range between 0.0 and 1.0.
/// 
/// <div class="function-preview" data-function="t<.5 ? 4*t*t*t : (t-1)*(2*t-2)*(2*t-2)+1"></div>
#[inline]
pub fn ease_in_out<V: CanTween, T: Float>(from: V, to: V, time: T) -> V {
	ease(EaseInOut, from, to, time)
}

/// Accelerating quartically from point A to point B. Use with `ease(EaseInQuart, ...)`.
/// 
/// <div class="function-preview" data-function="t*t*t*t"></div>
pub struct EaseInQuart;
impl EasingFunction for EaseInQuart {
	#[inline] 
	fn y(&self, x: f64) -> f64 { x * x * x * x }
}

/// Decelerating quartically from point A to point B. Use with `ease(EaseOutQuart, ...)`.
/// 
/// <div class="function-preview" data-function="1-(--t)*t*t*t"></div>
pub struct EaseOutQuart;
impl EasingFunction for EaseOutQuart {
	#[inline] 
	fn y(&self, x: f64) -> f64 { 
		let x_minus_one = x - 1.0;
		1.0 - x_minus_one * x_minus_one * x_minus_one * x_minus_one
	}
}

/// Accelerating then decelerating quartically from point A to point B. Use with `ease(EaseInOutQuart, ...)`.
/// 
/// <div class="function-preview" data-function="t<.5 ? 8*t*t*t*t : 1-8*(--t)*t*t*t"></div>
pub struct EaseInOutQuart;
impl EasingFunction for EaseInOutQuart {
	#[inline] 
	fn y(&self, x: f64) -> f64 { 
		if x < 0.5 { 8.0 * x * x * x * x } 
		else { 
			let x_minus_one = x - 1.0;
			1.0 - 8.0 * x_minus_one * x_minus_one * x_minus_one * x_minus_one
		}
	}
}

/// Accelerating quintically from point A to point B. Use with `ease(EaseInQuint, ...)`.
/// 
/// <div class="function-preview" data-function="t*t*t*t*t"></div>
pub struct EaseInQuint;
impl EasingFunction for EaseInQuint {
	#[inline] 
	fn y(&self, x: f64) -> f64 { x * x * x * x * x }
}

/// Decelerating quintically from point A to point B. Use with `ease(EaseOutQuint, ...)`.
/// 
/// <div class="function-preview" data-function="1+(--t)*t*t*t*t"></div>
pub struct EaseOutQuint;
impl EasingFunction for EaseOutQuint {
	#[inline] 
	fn y(&self, x: f64) -> f64 { 
		let x_minus_one = x - 1.0;
		1.0 + x_minus_one * x_minus_one * x_minus_one * x_minus_one * x_minus_one
	}
}

/// Accelerating then decelerating quintically from point A to point B. Use with `ease(EaseInOutQuint, ...)`.
/// 
/// <div class="function-preview" data-function="t<.5 ? 16*t*t*t*t*t : 1+16*(--t)*t*t*t*t"></div>
pub struct EaseInOutQuint;
impl EasingFunction for EaseInOutQuint {
	#[inline] 
	fn y(&self, x: f64) -> f64 { 
		if x < 0.5 { 16.0 * x * x * x * x * x } 
		else { 
			let x_minus_one = x - 1.0;
			1.0 + 16.0 * x_minus_one * x_minus_one * x_minus_one * x_minus_one * x_minus_one
		}
	}
}