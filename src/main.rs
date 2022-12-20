use fixed_vec_deque::FixedVecDeque;
use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer as AudioBuffer;
use nannou_laser as laser;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

const MAX_POINTS: usize = 2048;

fn main() {
    nannou::app(model).run();
}

type AudioData = [f32; 2];

type CurrentPoints = Arc<Mutex<FixedVecDeque<[AudioData; MAX_POINTS]>>>;

struct Model {
    audio_input: audio::Stream<CurrentPoints>,
    current_points: CurrentPoints,
}

fn model(app: &App) -> Model {
    // Create a window to receive mouse events.
    app.new_window().view(view).build().unwrap();

    // Initialise the audio host so we can spawn an audio stream.
    let points = Arc::new(Mutex::new(FixedVecDeque::new()));
    let audio_host = audio::Host::new();

    let stream = audio_host
        .new_input_stream(points.clone())
        .capture(audio_capture_fn)
        .build()
        .unwrap();

    stream.play().unwrap();

    Model {
        audio_input: stream,
        current_points: points,
    }
}

// A function that captures the audio from the buffer and
// writes it into the the WavWriter.
fn audio_capture_fn(audio: &mut CurrentPoints, buffer: &AudioBuffer) {
    // When the program ends, writer is dropped and data gets written to the disk.
    let mut points = audio.lock().unwrap();
    for frame in buffer.frames() {
        let mut point = [0f32, 0f32];
        let mut x = 0f32;
        let mut y = 0f32;
        for (idx, sample) in frame.iter().enumerate() {
            if idx == 0 {
                x = sample * 500f32;
            } else if idx == 1 {
                y = sample * 500f32;
            }
            point = [x, y];
        }
        *points.push_back() = point;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Visualise the point in the window.
    let draw = app.draw();
    draw.background().color(BLACK);
    let mut pts = model.current_points.lock().unwrap();
    let mut pt_iter = pts.iter();
    let mut p1 = pt_iter.next();
    for p2 in pt_iter {
        if let Some(p1) = p1 {
            let p1 = Point2::from(p1.clone());
            let p2 = Point2::from(p2.clone());
            if p1.distance(p2) < 25f32 {
                draw.line()
                    .start(p1)
                    .end(p2)
                    .stroke_weight(1f32)
                    .color(LIMEGREEN)
                    .finish();
            }
        }
        p1 = Some(p2);
    }
    pts.clear();

    draw.to_frame(app, &frame).unwrap();
}
