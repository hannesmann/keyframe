use crate::*;

pub type CurvePoint = Vector2<f64>;

/// Implementation of a 2D curve function for use in easing between two points. 
pub trait EasingFunction {
	/// Based on a number of points and an X position, calculate the Y position. 
	/// 0.0 is start and 1.0 is end on both axes. 
	/// 
	/// # Note
	/// This function can choose to ignore `curve` if it only implements a single static curve. In that case, `&[]` should be used for the `curve` argument.
	fn y_for_unbounded_x(curve: &[CurvePoint], x: f64) -> f64;

	/// Based on a number of points and an X position, calculate the Y position. 
	/// The X position is limited to a range between 0.0 and 1.0.
	fn y(curve: &[CurvePoint], x: f64) -> f64 {
		Self::y_for_unbounded_x(curve, match x {
			_ if x < 0.0 => { 0.0 },
			_ if x > 1.0 => { 1.0 },
			_ => { x }
		})
	}

	/// Based on a number of points and an X position, calculate the Y position. 
	/// The X position is limited to a range between 0.0 and `max_x`, while the curve is limited to a range between 0.0 and 1.0.
	fn y_for_scaled_x(curve: &[CurvePoint], x: f64, max_x: f64) -> f64 {
		Self::y(curve, match x {
			_ if x < 0.0 => { 0.0 },
			_ if x > max_x => { max_x },
			_ => { x / max_x }
		})
	}
}