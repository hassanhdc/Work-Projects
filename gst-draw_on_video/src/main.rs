extern crate vid_overlay;

use gst::prelude::*;

use anyhow::Error;
use derive_more::{Display, Error};

use vid_overlay::{OverlayElement, VideoContext};

#[derive(Debug, Display, Error)]
#[display(fmt = "Missing element {}", _0)]
struct MissingElement(#[error(not(source))] &'static str);

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    source: glib::Error,
}

fn draw_elements() -> gst::Pipeline {
    let pipeline = VideoContext::new(1920i32, 1080i32, 30i32).unwrap();
    let overlay = pipeline.get_by_name("overlay").unwrap();

    let rect =
        OverlayElement::create_rectangle(670., 100., 300., 100., (0.2, 0.8, 1.0, 1.), (3, 6));
    let rect2 =
        OverlayElement::create_rectangle(1070., 200., 300., 100., (1.0, 0.3, 0.6, 1.), (5, 10));
    let rect3 =
        OverlayElement::create_rectangle(1070., 550., 300., 100., (0.1, 0.5, 1.0, 1.), (7, 13));
    let rect4 =
        OverlayElement::create_rectangle(1600., 700., 500., 50., (0.9, 0.5, 1.0, 1.), (4, 12));
    let rect5 =
        OverlayElement::create_rectangle(1800., 550., 900., 100., (0.1, 0.9, 1.0, 1.), (1, 8));

    let txt =
        OverlayElement::create_text("Foo Bar".to_string(), 200., 0., (0.1, 0.5, 1.0, 1.), (0, 5));
    let txt2 = OverlayElement::create_text(
        "This is the longest paragraph ever to be typed and I can't think of anything else to type so I am just clickity clackity yappin' on keyboard".to_string(),
        200.,
        100.,
        (0.1, 0.5, 1.0, 1.),
        (0, 5),
    );

    VideoContext::draw_on(overlay, vec![rect, rect2, rect3, rect4, rect5, txt, txt2]);

    pipeline
}

fn main_loop(pipeline: gst::Pipeline) -> Result<(), Error> {
    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;
                return Err(ErrorMessage {
                    src: msg
                        .get_src()
                        .map(|s| String::from(s.get_path_string()))
                        .unwrap_or_else(|| String::from("None")),
                    error: err.get_error().to_string(),
                    debug: err.get_debug(),
                    source: err.get_error(),
                }
                .into());
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null)?;

    Ok(())
}

fn main() {
    let pipeline = draw_elements();
    match main_loop(pipeline) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}
