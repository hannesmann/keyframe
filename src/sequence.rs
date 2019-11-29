use crate::*;

/// A collection of keyframes that can be played back in sequence
pub struct AnimationSequence<T: CanTween> {
	sequence: Vec<T>
}