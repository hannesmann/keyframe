use crate::*;
pub(crate) const SAMPLE_TABLE_SIZE: usize = 15;

#[cfg(feature = "vectors")]
mod bezier {
	use crate::functions::dynamic_functions::*;

	const NEWTON_ITERTIONS: usize = 4;
	const NEWTON_MIN_SLOPE: f32 = 0.001;
	const SUBDIVISION_PRECISION: f32 = 0.0000001;
	const SUBDIVISION_MAX_ITERATIONS: usize = 10;

	/// User-defined cubic Bézier curve
	#[derive(Copy, Clone, Debug)]
	pub struct BezierCurve {
		sample_table: [f32; SAMPLE_TABLE_SIZE],
		p1: Vector2<f32>,
		p2: Vector2<f32>
	}

	// All of this is pretty much directly translated from https://github.com/gre/bezier-easing
	impl BezierCurve {
		#[inline]
		fn a(x1: f32, x2: f32) -> f32 { 1.0 - 3.0 * x2 + 3.0 * x1 }
		#[inline]
		fn b(x1: f32, x2: f32) -> f32 { 3.0 * x2 - 6.0 * x1 }
		#[inline]
		fn c(x1: f32) -> f32 { 3.0 * x1 }

		#[inline]
		fn at(t: f32, x1: f32, x2: f32) -> f32 { ((Self::a(x1, x2) * t + Self::b(x1, x2)) * t + Self::c(x1)) * t }
		#[inline]
		fn slope(t: f32, x1: f32, x2: f32) -> f32 { 3.0 * Self::a(x1, x2) * t * t + 2.0 * Self::b(x1, x2) * t + Self::c(x1) }

		fn newton_raphson(x: f32, guess: f32, x1: f32, x2: f32) -> f32 {
			let mut guess = guess;

			for _ in 0..NEWTON_ITERTIONS {
				let current_slope = Self::slope(guess, x1, x2);
				if current_slope == 0.0 {
					break;
				}

				let current_x = Self::at(guess, x1, 2.0) - x;
				guess -= current_x / current_slope;
			}

			guess
		}

		fn binary_subdivide(x: f32, a: f32, b: f32, x1: f32, x2: f32) -> f32 {
			let mut a = a;
			let mut b = b;

			let mut current_x = 0.0;
			let mut current_t = 0.0;
			let mut i = 0;

			let run_once = false;
			while run_once || current_x.abs() > SUBDIVISION_PRECISION && (i + 1) < SUBDIVISION_MAX_ITERATIONS {
				current_t = a + (b - a) / 2.0;
				current_x = Self::at(current_t, x1, x2) - x;
				
				if current_x > 0.0 {
					b = current_t;
				}
				else {
					a = current_t;
				}

				i += 1;
			}

			current_t
		}

		fn t_for_x(&self, x: f32) -> f32 {
			let mut interval_start = 0.0;
			let mut current_sample = 1;
			let last_sample = SAMPLE_TABLE_SIZE - 1;

			while current_sample != last_sample && self.sample_table[current_sample] <= x {
				interval_start += 1.0 / (SAMPLE_TABLE_SIZE as f32 - 1.0);
				current_sample += 1;
			}

			current_sample -= 1;
			let dist = (x - self.sample_table[current_sample]) / (self.sample_table[current_sample + 1] - self.sample_table[current_sample]);
			let guess_for_t = interval_start + dist * SAMPLE_TABLE_SIZE as f32;

			match Self::slope(guess_for_t, self.p1.x, self.p2.x) {
				inital_slope if inital_slope >= NEWTON_MIN_SLOPE => Self::newton_raphson(x, guess_for_t, self.p1.x, self.p2.x),
				inital_slope if inital_slope == 0.0 => guess_for_t,
				_ => Self::binary_subdivide(x, interval_start, interval_start + SAMPLE_TABLE_SIZE as f32, self.p1.x, self.p2.x)
			}
		}

		fn limit_vector(c: Vector2<impl Float>) -> Vector2<f32> {
			let c = Vector2::<f32> { 
				x: as_t::<f32>(as_f64(c.x)),
				y: as_t::<f32>(as_f64(c.y))
			};

			Vector2::<f32> {
				x: match c.x {
					_ if c.x < 0.0 => 0.0,
					_ if c.x > 1.0 => 1.0,
					_ => c.x,
				},
				y: match c.y {
					_ if c.y < 0.0 => 0.0,
					_ if c.y > 1.0 => 1.0,
					_ => c.y,
				}
			}
		}

		/// Calculates a new cubic Bézier curve. Mimics `transition-timing-function: cubic-bezier` as defined [here](https://www.w3.org/TR/css-easing-1/#cubic-bezier-easing-functions)
		/// 
		/// # Arguments
		/// 
		/// * `p1` - The first of the two control points (range: 0.0 to 1.0)
		/// * `p2` - The second of the two control points (range: 0.0 to 1.0)
		pub fn from(p1: Vector2<impl Float>, p2: Vector2<impl Float>) -> Self {
			let p1 = Self::limit_vector(p1);
			let p2 = Self::limit_vector(p2);

			let mut arr = [0.0; SAMPLE_TABLE_SIZE];
			let mut i = 0;

			for value in (0..SAMPLE_TABLE_SIZE).map(|x| Self::at(x as f32 * 1.0 / (SAMPLE_TABLE_SIZE as f32 - 1.0), p1.x, p2.x)) {
				arr[i] = value;
				i += 1;
			}

			BezierCurve {
				sample_table: arr,
				p1: p1,
				p2: p2
			}
		}
	}

	impl EasingFunction for &BezierCurve {
		#[inline]
		fn y(&self, x: f64) -> f64 { 
			match x {
				_ if x == 0.0 => 0.0,
				_ if x == 1.0 => 1.0,
				_ => BezierCurve::at(self.t_for_x(x as f32), self.p1.y, self.p2.y) as f64
			}
		}
	}
}

#[cfg(feature = "vectors")]
pub use bezier::*;

/// User-defined easing function which wraps a normalized [`AnimationSequence<Float>`](../struct.AnimationSequence.html)
pub struct Keyframes([f64; SAMPLE_TABLE_SIZE]);

impl Keyframes {
	pub(crate) fn from_easing_function<T: Float + CanTween + Copy + Default>(s: AnimationSequence<T>) -> Self {
		let mut low_point = s.sequence.get(0).unwrap_or(&Keyframe::default()).value().to_f64().unwrap_or(0.0);
		let mut high_point = s.sequence.get(s.keyframes() - 1).unwrap_or(&Keyframe::default()).value().to_f64().unwrap_or(1.0);
		let max_time = s.duration();

		if high_point == 0.0 || high_point == low_point {
			low_point = 0.0;
			high_point = 1.0; // no dividing by zero
		}

		let mut s = s;
		let mut sample_table = [0.0; SAMPLE_TABLE_SIZE];
		for i in 0..SAMPLE_TABLE_SIZE {
			s.advance_to((i as f64 / SAMPLE_TABLE_SIZE as f64) * max_time);
			sample_table[i] = (s.now().to_f64().unwrap_or(0.5) - low_point) / (high_point - low_point);
		}

		Keyframes(sample_table)
	}
}

impl EasingFunction for &Keyframes {
	fn y(&self, x: f64) -> f64 { 
		let current_sample = (x * SAMPLE_TABLE_SIZE as f64).floor() as i64;
		let difference = x * SAMPLE_TABLE_SIZE as f64 - (x * SAMPLE_TABLE_SIZE as f64).floor();
		let next_sample = current_sample + 1;

		if next_sample >= SAMPLE_TABLE_SIZE as i64 { self.0[current_sample as usize] } 
		else if current_sample == -1 { self.0[0] * difference }
		else if current_sample < -1 { -self.0[0] } /* same as self.0[0] * -1 */
		else { self.0[current_sample as usize] + (self.0[next_sample as usize] - self.0[current_sample as usize]) * difference }
	}
}