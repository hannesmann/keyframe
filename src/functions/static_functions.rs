use crate::easing::*;

// Based on https://gist.githubusercontent.com/gre/1650294/raw/01bf897e14c41f90c8fcda739fdc793790138446/easing.js
//      and https://github.com/warrenm/AHEasing/blob/master/AHEasing/easing.c

/// Linear interpolation from point A to point B
///
/// <div class="function-preview" data-function="t"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct Linear;
impl EasingFunction for Linear {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		x
	}
}

/// Step function, returns the closest to either point A or B
///
/// <div class="function-preview" data-function="Math.round(t)"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct Step;
impl EasingFunction for Step {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		x.round()
	}
}

/// Hold function, always returns A
///
/// <div class="function-preview" data-function="0"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct Hold;
impl EasingFunction for Hold {
	#[inline]
	fn y(&self, _x: f64) -> f64 {
		0.0
	}
}

/// Accelerating quadratically from point A to point B
///
/// <div class="function-preview" data-function="t * t"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct EaseInQuad;
impl EasingFunction for EaseInQuad {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		x * x
	}
}

/// Decelerating quadratically from point A to point B
///
/// <div class="function-preview" data-function="t * (2-t)"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct EaseOutQuad;
impl EasingFunction for EaseOutQuad {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		x * (2.0 - x)
	}
}

/// Accelerating then decelerating quadratically from point A to point B
///
/// <div class="function-preview" data-function="t<.5 ? 2*t*t : -1+(4-2*t)*t"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct EaseInOutQuad;
impl EasingFunction for EaseInOutQuad {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		if x < 0.5 {
			2.0 * x * x
		} else {
			-1.0 + (4.0 - 2.0 * x) * x
		}
	}
}

/// Accelerating cubically from point A to point B
///
/// <div class="function-preview" data-function="t * t * t"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct EaseInCubic;
impl EasingFunction for EaseInCubic {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		x * x * x
	}
}

/// Decelerating cubically from point A to point B
///
/// <div class="function-preview" data-function="(--t) * t * t + 1"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct EaseOutCubic;
impl EasingFunction for EaseOutCubic {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		let x_minus_one = x - 1.0;
		1.0 + x_minus_one * x_minus_one * x_minus_one
	}
}

/// Accelerating then decelerating cubically from point A to point B
///
/// <div class="function-preview" data-function="t<.5 ? 4*t*t*t : (t-1)*(2*t-2)*(2*t-2)+1"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct EaseInOutCubic;
impl EasingFunction for EaseInOutCubic {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		if x < 0.5 {
			4.0 * x * x * x
		} else {
			let x_minus_one = x - 1.0;
			x_minus_one * (2.0 * x - 2.0) * (2.0 * x - 2.0) + 1.0
		}
	}
}

/// Accelerating quartically from point A to point B
///
/// <div class="function-preview" data-function="t*t*t*t"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct EaseInQuart;
impl EasingFunction for EaseInQuart {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		x * x * x * x
	}
}

/// Decelerating quartically from point A to point B
///
/// <div class="function-preview" data-function="1-(--t)*t*t*t"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct EaseOutQuart;
impl EasingFunction for EaseOutQuart {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		let x_minus_one = x - 1.0;
		1.0 - x_minus_one * x_minus_one * x_minus_one * x_minus_one
	}
}

/// Accelerating then decelerating quartically from point A to point B
///
/// <div class="function-preview" data-function="t<.5 ? 8*t*t*t*t : 1-8*(--t)*t*t*t"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct EaseInOutQuart;
impl EasingFunction for EaseInOutQuart {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		if x < 0.5 {
			8.0 * x * x * x * x
		} else {
			let x_minus_one = x - 1.0;
			1.0 - 8.0 * x_minus_one * x_minus_one * x_minus_one * x_minus_one
		}
	}
}

/// Accelerating quintically from point A to point B
///
/// <div class="function-preview" data-function="t*t*t*t*t"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct EaseInQuint;
impl EasingFunction for EaseInQuint {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		x * x * x * x * x
	}
}

/// Decelerating quintically from point A to point B
///
/// <div class="function-preview" data-function="1+(--t)*t*t*t*t"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct EaseOutQuint;
impl EasingFunction for EaseOutQuint {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		let x_minus_one = x - 1.0;
		1.0 + x_minus_one * x_minus_one * x_minus_one * x_minus_one * x_minus_one
	}
}

/// Accelerating then decelerating quintically from point A to point B
///
/// <div class="function-preview" data-function="t<.5 ? 16*t*t*t*t*t : 1+16*(--t)*t*t*t*t"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct EaseInOutQuint;
impl EasingFunction for EaseInOutQuint {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		if x < 0.5 {
			16.0 * x * x * x * x * x
		} else {
			let x_minus_one = x - 1.0;
			1.0 + 16.0 * x_minus_one * x_minus_one * x_minus_one * x_minus_one * x_minus_one
		}
	}
}

/// Accelerating on 1/4 of a sine wave from point A to point B
///
/// <div class="function-preview" data-function="Math.sin((t - 1) * Math.PI / 2) + 1"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct EaseIn;
impl EasingFunction for EaseIn {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		((x - 1.0) * core::f64::consts::FRAC_PI_2).sin() + 1.0
	}
}

/// Decelerating on 1/4 of a sine wave from point A to point B
///
/// <div class="function-preview" data-function="Math.sin(t * Math.PI / 2)"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct EaseOut;
impl EasingFunction for EaseOut {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		(x * core::f64::consts::FRAC_PI_2).sin()
	}
}

/// Accelerating then decelerating on 1/2 of a sine wave from point A to point B
///
/// <div class="function-preview" data-function=".5 * (1 - Math.cos(t * Math.PI))"></div>
#[derive(Copy, Clone, Debug, Default)]
pub struct EaseInOut;
impl EasingFunction for EaseInOut {
	#[inline]
	fn y(&self, x: f64) -> f64 {
		0.5 * (1.0 - (x * core::f64::consts::PI).cos())
	}
}
