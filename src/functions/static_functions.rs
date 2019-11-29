use crate::easing::*;

// Based on https://gist.githubusercontent.com/gre/1650294/raw/01bf897e14c41f90c8fcda739fdc793790138446/easing.js

/// Linear interpolation from point A to point B. Use with `ease(Linear, ...)`
/// 
/// <div class="function-preview" data-function="t"></div>
pub struct Linear;
impl EasingFunction for Linear {
	fn y(&self, x: f64) -> f64 { x }
}

/// Accelerating quadratically from point A to point B. Use with `ease(EaseInQuad, ...)`
/// 
/// <div class="function-preview" data-function="t * t"></div>
pub struct EaseInQuad;
impl EasingFunction for EaseInQuad {
	fn y(&self, x: f64) -> f64 { x * x }
}

/// Decelerating quadratically from point A to point B. Use with `ease(EaseOutQuad, ...)`
/// 
/// <div class="function-preview" data-function="t * (2-t)"></div>
pub struct EaseOutQuad;
impl EasingFunction for EaseOutQuad {
	fn y(&self, x: f64) -> f64 { x * (2.0 - x) }
}

/// Accelerating then decelerating quadratically from point A to point B. Use with `ease(EaseInOutQuad, ...)`
/// 
/// <div class="function-preview" data-function="t<.5 ? 2*t*t : -1+(4-2*t)*t"></div>
pub struct EaseInOutQuad;
impl EasingFunction for EaseInOutQuad {
	fn y(&self, x: f64) -> f64 { 
		if x < 0.5 { 2.0 * x * x } else { -1.0 + (4.0 - 2.0 * x) * x }
	}
}

/// Accelerating cubically from point A to point B. Use with `ease(EaseIn, ...)`
/// 
/// <div class="function-preview" data-function="t * t * t"></div>
pub struct EaseIn;
impl EasingFunction for EaseIn {
	fn y(&self, x: f64) -> f64 { x * x * x }
}

/// Decelerating cubically from point A to point B. Use with `ease(EaseOut, ...)`
/// 
/// <div class="function-preview" data-function="(--t) * t * t + 1"></div>
pub struct EaseOut;
impl EasingFunction for EaseOut {
	fn y(&self, x: f64) -> f64 { 
		let x_minus_one = x - 1.0;
		1.0 + x_minus_one * x_minus_one * x_minus_one
	}
}

/// Accelerating then decelerating cubically from point A to point B. Use with `ease(EaseInOut, ...)`
/// 
/// <div class="function-preview" data-function="t<.5 ? 4*t*t*t : (t-1)*(2*t-2)*(2*t-2)+1"></div>
pub struct EaseInOut;
impl EasingFunction for EaseInOut {
	fn y(&self, x: f64) -> f64 { 
		if x < 0.5 { 4.0 * x * x * x } 
		else { 
			let x_minus_one = x - 1.0;
			x_minus_one * (2.0 * x - 2.0) * (2.0 * x - 2.0) + 1.0
		}
	}
}

/// Accelerating quartically from point A to point B. Use with `ease(EaseInQuart, ...)`
/// 
/// <div class="function-preview" data-function="t*t*t*t"></div>
pub struct EaseInQuart;
impl EasingFunction for EaseInQuart {
	fn y(&self, x: f64) -> f64 { x * x * x * x }
}

/// Decelerating quartically from point A to point B. Use with `ease(EaseOutQuart, ...)`
/// 
/// <div class="function-preview" data-function="1-(--t)*t*t*t"></div>
pub struct EaseOutQuart;
impl EasingFunction for EaseOutQuart {
	fn y(&self, x: f64) -> f64 { 
		let x_minus_one = x - 1.0;
		1.0 - x_minus_one * x_minus_one * x_minus_one * x_minus_one
	}
}

/// Accelerating then decelerating quartically from point A to point B. Use with `ease(EaseInOutQuart, ...)`
/// 
/// <div class="function-preview" data-function="t<.5 ? 8*t*t*t*t : 1-8*(--t)*t*t*t"></div>
pub struct EaseInOutQuart;
impl EasingFunction for EaseInOutQuart {
	fn y(&self, x: f64) -> f64 { 
		if x < 0.5 { 8.0 * x * x * x * x } 
		else { 
			let x_minus_one = x - 1.0;
			1.0 - 8.0 * x_minus_one * x_minus_one * x_minus_one * x_minus_one
		}
	}
}

/// Accelerating quintically from point A to point B. Use with `ease(EaseInQuint, ...)`
/// 
/// <div class="function-preview" data-function="t*t*t*t*t"></div>
pub struct EaseInQuint;
impl EasingFunction for EaseInQuint {
	fn y(&self, x: f64) -> f64 { x * x * x * x * x }
}

/// Decelerating quintically from point A to point B. Use with `ease(EaseOutQuint, ...)`
/// 
/// <div class="function-preview" data-function="1+(--t)*t*t*t*t"></div>
pub struct EaseOutQuint;
impl EasingFunction for EaseOutQuint {
	fn y(&self, x: f64) -> f64 { 
		let x_minus_one = x - 1.0;
		1.0 + x_minus_one * x_minus_one * x_minus_one * x_minus_one * x_minus_one
	}
}

/// Accelerating then decelerating quintically from point A to point B. Use with `ease(EaseInOutQuint, ...)`
/// 
/// <div class="function-preview" data-function="t<.5 ? 16*t*t*t*t*t : 1+16*(--t)*t*t*t*t"></div>
pub struct EaseInOutQuint;
impl EasingFunction for EaseInOutQuint {
	fn y(&self, x: f64) -> f64 { 
		if x < 0.5 { 16.0 * x * x * x * x * x } 
		else { 
			let x_minus_one = x - 1.0;
			1.0 + 16.0 * x_minus_one * x_minus_one * x_minus_one * x_minus_one * x_minus_one
		}
	}
}