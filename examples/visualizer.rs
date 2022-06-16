use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use ggez::{Context, ContextBuilder, GameError, GameResult};

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::EventHandler;
use ggez::graphics::*;
use ggez::mint::Point2;

use keyframe::{functions::*, keyframes, AnimationSequence};

fn main() -> GameResult {
	let (ctx, event_loop) = ContextBuilder::new("visualizer", "Hannes Mann")
		.window_mode(
			WindowMode::default()
				.dimensions(848.0, 480.0)
				.min_dimensions(500.0, 340.0)
				.resizable(true),
		)
		.window_setup(
			WindowSetup::default()
				.title("Sequence Visualizer")
				.samples(ggez::conf::NumSamples::Four)
				.vsync(false),
		)
		.build()?;

	let vis = Visualizer {
		example: VisualizerExample::EaseInOutFourPoint,
		keyframes: match_sequence(&VisualizerExample::EaseInOutFourPoint),
		time_in_crate: 0.0,
	};
	ggez::event::run(ctx, event_loop, vis)
}

#[derive(Debug, FromPrimitive, Clone, PartialEq)]
#[repr(i32)]
enum VisualizerExample {
	LinearTwoPoint,
	LinearFourPoint,
	EaseInOutFourPoint,
	LinearCircle30Point,
	BezierFourPoint,
	KeyframesFunctionFourPoint,
	Last,
}

fn match_sequence(example: &VisualizerExample) -> AnimationSequence<Point2<f32>> {
	match example {
		VisualizerExample::LinearTwoPoint => {
			keyframes![([0.0, 0.0].into(), 0.0, Linear), ([1.0, 1.0].into(), 1.0, Linear)]
		}
		VisualizerExample::LinearFourPoint => keyframes![
			([0.0, 0.0].into(), 0.0, Linear),
			([0.2, 0.4].into(), 0.3, Linear),
			([0.8, 0.4].into(), 0.8, Linear),
			([1.0, 1.0].into(), 1.0, Linear)
		],
		VisualizerExample::EaseInOutFourPoint => keyframes![
			([0.0, 0.0].into(), 0.0),
			([0.2, 0.4].into(), 0.3),
			([0.8, 0.4].into(), 0.8),
			([1.0, 1.0].into(), 1.0)
		],
		VisualizerExample::LinearCircle30Point => {
			let mut keyframes = Vec::new();

			for i in 0..=30 {
				let sin = ((i as f32 / 30.0) * std::f32::consts::PI * 2.0).sin();
				let cos = ((i as f32 / 30.0) * std::f32::consts::PI * 2.0).cos();
				keyframes.push(([sin * 0.5 + 0.5, cos * 0.5 + 0.5].into(), i as f64 / 30.0, Linear).into());
			}

			AnimationSequence::from(keyframes)
		}
		VisualizerExample::BezierFourPoint => {
			// https://easings.net/en#easeInCirc
			let bezier = BezierCurve::from([0.6, 0.04].into(), [0.98, 0.335].into());

			keyframes![
				([0.0, 0.0].into(), 0.0, bezier),
				([0.2, 0.4].into(), 0.3, bezier),
				([0.8, 0.4].into(), 0.8, bezier),
				([1.0, 1.0].into(), 1.0, bezier)
			]
		}
		VisualizerExample::KeyframesFunctionFourPoint => {
			// The easing function is normalized after creation, this is the same as specifying:
			// (0.0, 0.0, Linear)
			// (0.4, 0.4, Linear)
			// (0.4, 0.8, Linear)
			// (1.0, 1.0, Linear)
			let function = keyframes![
				(-0.0, 0.0, Linear),
				(-0.8, 0.4, Linear),
				(-0.8, 0.8, Linear),
				(-2.0, 1.0, Linear)
			]
			.to_easing_function();

			keyframes![
				([0.0, 0.0].into(), 0.0, function),
				([0.2, 0.4].into(), 0.3, function),
				([0.8, 0.4].into(), 0.8, function),
				([1.0, 1.0].into(), 1.0, function)
			]
		}
		_ => keyframes![],
	}
}

struct Visualizer {
	example: VisualizerExample,
	keyframes: AnimationSequence<Point2<f32>>,
	time_in_crate: f64,
}

impl EventHandler<GameError> for Visualizer {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		self.time_in_crate = 0.0;

		let now = std::time::Instant::now();
		self.keyframes
			.advance_and_maybe_reverse(ggez::timer::delta(ctx).as_secs_f64() * 0.5);
		self.time_in_crate += (std::time::Instant::now() - now).as_secs_f64();

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		let screen_size = drawable_size(ctx);
		clear(ctx, Color::WHITE);

		let t = Text::new(TextFragment {
			text: format!(
				"{:?} {:.2}/{:.2} s",
				self.example,
				self.keyframes.time() * 2.0,
				self.keyframes.duration() * 2.0
			),
			font: None,
			scale: Some(PxScale::from(40.0)),
			..Default::default()
		});
		let size = t.dimensions(ctx);
		draw(
			ctx,
			&t,
			DrawParam::default()
				.dest([(screen_size.0 / 2.0 - size.w as f32 / 2.0).round(), 20.0])
				.color(Color::BLACK),
		)?;

		let t = Text::new(TextFragment {
			text: format!("{:?} -> {:?}", self.keyframes.pair().0, self.keyframes.pair().1),
			font: None,
			scale: Some(PxScale::from(15.0)),
			..Default::default()
		});
		let size = t.dimensions(ctx);
		draw(
			ctx,
			&t,
			DrawParam::default()
				.dest([(screen_size.0 / 2.0 - size.w as f32 / 2.0).round(), 60.0])
				.color(Color::BLACK),
		)?;

		let t = Text::new(TextFragment {
			text: "Press left or right to switch example".to_owned(),
			font: None,
			scale: Some(PxScale::from(30.0)),
			..Default::default()
		});
		let size = t.dimensions(ctx);
		draw(
			ctx,
			&t,
			DrawParam::default()
				.dest([
					(screen_size.0 / 2.0 - size.w as f32 / 2.0).round(),
					screen_size.1 - 60.0,
				])
				.color(Color::BLACK),
		)?;

		let area = [100.0, 160.0, screen_size.0 - 100.0 * 2.0, screen_size.1 - 160.0 * 2.0];
		let now = std::time::Instant::now();
		let kf_now = self.keyframes.now_strict().unwrap();
		let point: Point2<f32> = [area[0] + kf_now.x * area[2], area[1] + (1.0 - kf_now.y) * area[3]].into();
		self.time_in_crate += (std::time::Instant::now() - now).as_secs_f64();

		let circle = Mesh::new_circle(
			ctx,
			DrawMode::Fill(FillOptions::DEFAULT),
			point,
			4.0,
			2.0,
			Color::new(0.83, 0.17, 0.12, 1.0),
		)?;
		draw(ctx, &circle, DrawParam::default())?;

		for k in &self.keyframes {
			let text = Text::new(TextFragment {
				text: format!("({:.1}, {:.1}) at {:.1} s", k.value().x, k.value().y, k.time() * 2.0),
				font: None,
				scale: Some(PxScale::from(14.0)),
				..Default::default()
			});

			let p: Point2<f32> = [
				(area[0] + k.value().x * area[2] - text.dimensions(ctx).w as f32 / 2.0).round(),
				(area[1] + (1.0 - k.value().y) * area[3] - 20.0).round(),
			]
			.into();

			draw(
				ctx,
				&text,
				DrawParam::default().dest(p).color(Color::new(0.83, 0.17, 0.12, 1.0)),
			)?;
		}

		let t = Text::new(TextFragment {
			text: format!(
				"FPS: {:.1} (ft: {:.2} ms, c: {:.2} ms)",
				ggez::timer::fps(ctx),
				ggez::timer::delta(ctx).as_secs_f64() * 1000.0,
				self.time_in_crate * 1000.0
			),
			font: None,
			scale: Some(PxScale::from(20.0)),
			..Default::default()
		});
		draw(
			ctx,
			&t,
			DrawParam::default()
				.dest([10.0, 10.0])
				.color(Color::new(0.1, 0.9, 0.1, 1.0)),
		)?;

		present(ctx)
	}

	fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
		set_screen_coordinates(ctx, [0.0, 0.0, width, height].into()).expect("Couldn't resize screen");
	}

	fn key_down_event(
		&mut self,
		_ctx: &mut Context,
		keycode: ggez::input::keyboard::KeyCode,
		_keymods: ggez::input::keyboard::KeyMods,
		_repeat: bool,
	) {
		let now = std::time::Instant::now();
		let mut example: i32 = unsafe { std::mem::transmute(self.example.clone()) };

		example += match keycode {
			ggez::input::keyboard::KeyCode::Left => -1,
			ggez::input::keyboard::KeyCode::Right => 1,
			_ => 0,
		};

		if example < 0 {
			example = unsafe { std::mem::transmute::<VisualizerExample, i32>(VisualizerExample::Last) - 1 };
		} else if example >= unsafe { std::mem::transmute::<VisualizerExample, i32>(VisualizerExample::Last) } {
			example = 0;
		}

		if self.example != FromPrimitive::from_i32(example).unwrap_or(VisualizerExample::LinearTwoPoint) {
			self.example = FromPrimitive::from_i32(example).unwrap_or(VisualizerExample::LinearTwoPoint);
			self.keyframes = match_sequence(&self.example);
			self.time_in_crate += (std::time::Instant::now() - now).as_secs_f64();
		}
	}
}
