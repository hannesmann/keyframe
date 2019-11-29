pub(crate) use num_traits::Float;
#[cfg(feature = "vectors")]
pub(crate) use mint::{Vector2, Vector3, Vector4};

pub mod tweening;
use tweening::*;

pub struct Keyframe<T: CanTween> {
	value: T,
	time: f64,
	function: Box<dyn EasingFunction>
}