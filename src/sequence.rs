use crate::*;

use std::iter::FromIterator;

/// Category of animation sequence error
pub enum AnimationSequenceError {
	/// An attempt was made to insert a keyframe into the sequence when another keyframe already exists with the same start time
	TimeCollision(f64)
}

/// A collection of keyframes that can be played back in sequence
pub struct AnimationSequence<T: CanTween + Copy + Default> {
	pub(crate) sequence: Vec<Keyframe<T>>,

	// Current item we're animating
	keyframe: Option<usize>,
	// Current time
	time: f64
}

impl<T: CanTween + Copy + Default> AnimationSequence<T> {
	/// Creates a new empty animation sequence
	#[inline]
	pub fn new() -> Self { 	
		AnimationSequence::<T> {
			sequence: Vec::new(),
			keyframe: None,

			time: 0.0
		}
	}

	fn update_current_keyframe(&mut self) {
		if let Some(k) = self.keyframe {
			if self.keyframes() <= k || self.sequence[k].time() > self.time {
				self.keyframe = None;
			}
		}

		for i in self.keyframe.unwrap_or(0)..self.keyframes() {
			if self.sequence[i].time > self.time { break } else { self.keyframe = Some(i) }
		}
	}

	fn insert_into_vec(&mut self, keyframe: Keyframe<T>) -> Result<(), AnimationSequenceError> {
		if self.has_keyframe_at(keyframe.time()) { 
			Err(AnimationSequenceError::TimeCollision(keyframe.time()))
		}
		else {
			self.sequence.push(keyframe);
			Ok(())
		}
	}

	/// Inserts a new keyframe into the animation sequence
	pub fn insert(&mut self, keyframe: Keyframe<T>) -> Result<(), AnimationSequenceError> {
		if self.has_keyframe_at(keyframe.time()) { 
			Err(AnimationSequenceError::TimeCollision(keyframe.time()))
		}
		else {
			if keyframe.time() > self.sequence.last().unwrap_or(&Keyframe::default()).time() {
				self.sequence.insert(self.sequence.len(), keyframe);
			} 
			else if keyframe.time() < self.sequence.first().unwrap_or(&Keyframe::default()).time() {
				self.sequence.insert(0, keyframe);
			}
			else {
				self.sequence.push(keyframe);

				// Gives us the following guarantees:
				// * the item that comes next also has a later time
				// * the first item has the earliest time
				// * the last item has the last time (useful for remove_at)
				self.sequence.sort_unstable_by(|k, k2| k.time.partial_cmp(&k2.time).unwrap_or(std::cmp::Ordering::Equal));
			}

			self.update_current_keyframe();
			Ok(())
		}
	}

	/// Inserts several keyframes from an iterator all at once. 
	/// This is faster because sorting only needs to be done after all the keyframes have been inserted.
	pub fn insert_many(&mut self, keyframes: impl IntoIterator<Item = impl Into<Keyframe<T>>>) -> Result<(), AnimationSequenceError> {
		for k in keyframes { self.insert_into_vec(k.into())?; }
		self.sequence.sort_unstable_by(|k, k2| k.time.partial_cmp(&k2.time).unwrap_or(std::cmp::Ordering::Equal));
		self.update_current_keyframe();
		Ok(())
	}	

	/// Removes the keyframe from the sequence at the specified time. Returns true if a keyframe was actually removed
	pub fn remove(&mut self, timestamp: f64) -> bool { self.retain(|t| t != timestamp) }
	/// Removes all keyframes from this sequence
	pub fn clear(&mut self) { self.retain(|_| false); }

	/// Retains only the keyframes specified by the predicate. Works the same as `Vec::retain`.
	/// Returns true only if a keyframe was actually removed.
	pub fn retain<F: FnMut(f64) -> bool>(&mut self, mut f: F) -> bool {
		let old_len = self.keyframes();
		self.sequence.retain(|k| f(k.time()));

		if old_len != self.keyframes() {
			if self.time > self.duration() {
				self.time = self.duration();
			}
			self.update_current_keyframe();
			true
		}
		else { false }
	}

	/// If this sequence has a keyframe at the exact timestamp
	#[inline]
	pub fn has_keyframe_at(&self, timestamp: f64) -> bool { self.into_iter().any(|k| k.time() == timestamp) }

	/// The number of keyframes in this sequence
	#[inline]
	pub fn keyframes(&self) -> usize { self.sequence.len() }

	/// The current pair of keyframes that are being animated (current, next)
	/// 
	/// # Note
	/// 
	/// The following applies if:
	/// * There are no keyframes in this sequence: (`None`, `None`) is returned
	/// * The sequence has not reached the first keyframe: (`None`, current) is returned
	/// * There is only one keyframe in this sequence and the sequence has reached it: (current, `None`) is returned
	/// * The sequence has finished: (current, `None`) is returned
	#[inline]
	pub fn pair(&self) -> (Option<&Keyframe<T>>, Option<&Keyframe<T>>) {
		match self.keyframe {
			Some(c) if c == self.sequence.len() - 1 => (Some(&self.sequence[c]), None), 
			Some(c) => (Some(&self.sequence[c]), Some(&self.sequence[c + 1])),
			None if self.sequence.len() > 0 => (None, Some(&self.sequence[0])),
			None => (None, None)
		}
	}

	/// The current value of this sequence
	#[inline]
	pub fn now(&self) -> T {
		match self.pair() {
			(Some(s1), Some(s2)) => s1.tween_to(s2, self.time),
			(Some(s1), None) => s1.value(),
			(None, Some(s2)) => Keyframe::default().tween_to(s2, self.time),
			(None, None) => T::default()
		}
	}

	/// Advances this sequence by the duration specified.
	/// Returns `true` if this sequence is now finished.
	#[inline]
	pub fn advance_by(&mut self, duration: f64) -> bool {
		self.advance_to(self.time() + duration)
	}

	/// Advances this sequence to the exact timestamp. 
	/// Returns `true` if this sequence is now finished.
	/// 
	/// # Note
	/// 
	/// The following applies if:
	/// * The timestamp is negative: the sequence is set to `0.0`
	/// * The timestamp is after the duration of the sequence: the sequence is set to `duration()`
	#[inline]
	pub fn advance_to(&mut self, timestamp: f64) -> bool {
		if self.time == timestamp { return self.finished() }
		
		self.time = match timestamp {
			_ if timestamp < 0.0 => 0.0,
			_ if timestamp > self.duration() => self.duration(),
			_ => timestamp
		};

		self.update_current_keyframe();
		self.finished()
	}

	/// The length in seconds of this sequence
	#[inline]
	pub fn duration(&self) -> f64 { 
		// Keyframe::default means that if we don't have any items in this collection (meaning - 1 is out of bounds) the maximum time will be 0.0
		self.sequence.get(self.sequence.len().saturating_sub(1)).unwrap_or(&Keyframe::default()).time 
	}

	/// The current progression of this sequence in seconds
	#[inline]
	pub fn time(&self) -> f64 { self.time }

	/// The current progression of this sequence as a percentage
	#[inline]
	pub fn progress(&self) -> f64 { 
		if self.duration() == 0.0 { 0.0 } else { self.time / self.duration() }
	}

	/// If this sequence has finished and is at the end. 
	/// It can be reset with `advance_to(0.0)`.
	#[inline]
	pub fn finished(&self) -> bool { self.time == self.duration() }

	/// Consumes this sequence and creates a new animation sequence which plays in reverse order
	pub fn reverse(self) -> AnimationSequence<T> {
		let max_time = self.duration();

		Self::from_iter(self.sequence
			.into_iter()
			.rev()
			.map(|mut k| {
				k.time = max_time - k.time;
				k
			})
		)
	}
}

impl<T: Float + CanTween + Copy + Default> AnimationSequence<T> {
	/// Consumes this sequence and creates a normalized easing function which controls the 2D curve according to the keyframes in this sequence
	/// 
	/// # Note
	/// 
	/// This function is only implemented for one-dimensional float types, since each value corresponds to a Y position
	pub fn to_easing_function(self) -> Keyframes { Keyframes::from_easing_function(self) }
}

impl<T: CanTween + Copy + Default> From<Vec<Keyframe<T>>> for AnimationSequence<T> {
	/// Creates a new animation sequence from a vector of keyframes
	fn from(vec: Vec<Keyframe<T>>) -> Self {
		let mut me = AnimationSequence::<T> {
			sequence: vec,
			keyframe: None,

			time: 0.0
		};

		me.sequence.sort_unstable_by(|k, k2| k.time.partial_cmp(&k2.time).unwrap_or(std::cmp::Ordering::Equal));
		me.sequence.dedup_by_key(|k| k.time());
		me.update_current_keyframe();

		me
	}
}

impl<T: CanTween + Copy + Default> Default for AnimationSequence<T> {
	#[inline]
	fn default() -> Self { Self::new() }
}

impl<T: CanTween + Copy + Default, I: Into<Keyframe<T>>> FromIterator<I> for AnimationSequence<T> {
	/// Creates a new animation sequence from an iterator
	fn from_iter<I2: IntoIterator<Item = I>>(iter: I2) -> Self {
		let mut me = Self::new();
		me.insert_many(iter).ok(); // Ignore errors, collisions will be discarded
		me
	}
}

impl<'a, T: CanTween + Copy + Default> IntoIterator for &'a AnimationSequence<T> {
	type Item = &'a Keyframe<T>;
	type IntoIter = std::slice::Iter<'a, Keyframe<T>>;

	#[inline]
	fn into_iter(self) -> Self::IntoIter {
		self.sequence.as_slice().into_iter()
	}
}

/// Creates an animation sequence containing any arguments that can be made into keyframes
/// 
/// # Note
/// 
/// While this macro can be used with [`Keyframe::new`](struct.Keyframe.html#method.new) it's recommended to specify your keyframes with tuples (for shorter code) like this: 
/// ```ignore
/// keyframes![(0.0, 0.0), (0.5, 1.0), (1.0, 2.0), (1.5, 3.0, EaseOut), (2.0, 4.0)]
/// ```
#[macro_export]
macro_rules! keyframes {
	() => (AnimationSequence::<f64>::default());
	($($k: expr),*) => {{
		let mut vec = Vec::new();
		$( vec.push($k.into()); )*
		AnimationSequence::from(vec)
	}};
}