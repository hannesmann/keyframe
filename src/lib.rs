pub(crate) use num_traits::Float;
#[cfg(feature = "vectors")]
pub(crate) use mint::{Vector2, Vector3, Vector4};

pub mod easing;
use easing::*;

pub struct Keyframe<T: CanEase> {
	value: T,
	time: f64,
	function: Box<dyn EasingFunction>
}