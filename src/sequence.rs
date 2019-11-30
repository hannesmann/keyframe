use crate::*;

use std::iter::FromIterator;

/// Category of animation sequence error
pub enum AnimationSequenceError {
	/// An attempt was made to insert a keyframe into the sequence when another keyframe already exists with the same start time
	TimeCollision(f64)
}

/// A collection of keyframes that can be played back in sequence
pub struct AnimationSequence<T: CanTween + Copy + Default> {
	sequence: Vec<Keyframe<T>>,

	// Current time
	time: f64,
	// The start time of the last keyframe
	max_time: f64
}

impl<T: CanTween + Copy + Default> AnimationSequence<T> {
	/// Inserts a new keyframe into the animation sequence
	pub fn push(&mut self, keyframe: Keyframe<T>) -> Result<(), AnimationSequenceError> {
		match (&self.sequence).into_iter().any(|k| k.time::<f64>() == keyframe.time()) {
			true => Err(AnimationSequenceError::TimeCollision(keyframe.time())),
			false => {
				if self.max_time < keyframe.time() {
					self.max_time = keyframe.time();
				}
				
				self.sequence.push(keyframe);
				Ok(())
			}
		}
	}
}

impl<T: CanTween + Copy + Default> FromIterator<Keyframe<T>> for AnimationSequence<T> {
	fn from_iter<I: IntoIterator<Item = Keyframe<T>>>(iter: I) -> Self {
		let mut me = Self::default();
		for k in iter { me.push(k).ok(); } // Ignore the error, collisions will be discarded
		me
	}
}

impl<T: CanTween + Copy + Default> Default for AnimationSequence<T> {
	/// Creates an empty animation sequence
	fn default() -> Self { 
		AnimationSequence::<T> {
			sequence: Vec::new(),
			time: 0.0,
			max_time: 0.0
		}
	}
}