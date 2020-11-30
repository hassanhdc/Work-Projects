use gst::prelude::*;

use pango::prelude::*;

use std::ops;
use std::sync::{Arc, Mutex};

use anyhow::Error;
use derive_more::{Display, Error};

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

struct DrawingContext {
    layout: glib::SendUniqueCell<LayoutWrapper>,
    info: Option<gst_video::VideoInfo>,
}

#[derive(Debug)]
struct LayoutWrapper(pango::Layout);

impl ops::Deref for LayoutWrapper {
    type Target = pango::Layout;

    fn deref(&self) -> &pango::Layout {
        &self.0
    }
}

unsafe impl glib::SendUnique for LayoutWrapper {
    fn is_unique(&self) -> bool {
        self.0.ref_count() == 1
    }
}

fn create_pipeline() -> Result<gst::Pipeline, Error> {
    gst::init()?;

    let pipeline = gst::Pipeline::new(None);
    let src = gst::ElementFactory::make("videotestsrc", None)
        .map_err(|_| MissingElement("videotestsrc"))?;
    let overlay = gst::ElementFactory::make("cairooverlay", None)
        .map_err(|_| MissingElement("cairooverlay"))?;
    let capsfilter =
        gst::ElementFactory::make("capsfilter", None).map_err(|_| MissingElement("capsfilter"))?;
    let videoconvert = gst::ElementFactory::make("videoconvert", None)
        .map_err(|_| MissingElement("videoconvert"))?;
    let sink = gst::ElementFactory::make("autovideosink", None)
        .map_err(|_| MissingElement("autovideosink"))?;

    pipeline.add_many(&[&src, &overlay, &capsfilter, &videoconvert, &sink])?;
    gst::Element::link_many(&[&src, &overlay, &capsfilter, &videoconvert, &sink])?;

    let caps = gst::Caps::builder("video/x-raw")
        .field("width", &1920i32)
        .field("height", &1080i32)
        .field("framerate", &gst::Fraction::new(30, 1))
        .build();

    capsfilter.set_property("caps", &caps).unwrap();

    src.set_property_from_str("pattern", "smpte");

    let fontmap = pangocairo::FontMap::new().unwrap();
    let context = fontmap.create_context().unwrap();
    let layout = LayoutWrapper(pango::Layout::new(&context));

    let font_desc = pango::FontDescription::from_string("Sans Bold 12");
    layout.set_font_description(Some(&font_desc));

    let drawer = Arc::new(Mutex::new(DrawingContext {
        layout: glib::SendUniqueCell::new(layout).unwrap(),
        info: None,
    }));

    let drawer_clone = drawer.clone();

    overlay
        .connect("draw", false, move |args| {
            let drawer = &drawer_clone;
            let drawer = drawer.lock().unwrap();

            let timestamp = args[2].get_some::<gst::ClockTime>().unwrap();
            let ctx = args[1].get::<cairo::Context>().unwrap().unwrap();

            let layout = drawer.layout.borrow();

            let surface = cairo::ImageSurface::create(cairo::Format::Rgb30, 20, 20).unwrap();
            ctx.set_source_surface(&surface, 0., 0.);

            ctx.set_source_rgba(1.0, 1.0, 0.0, 1.);
            ctx.move_to(1800., 970.);

            let timestamp_str = format!("{:.11}", timestamp.to_string());
            layout.set_text(&timestamp_str);
            pangocairo::functions::show_layout(&ctx, &**layout);

            ctx.set_source_rgba(1.0, 0.5, 0.0, 1.);
            ctx.move_to(0., 0.);
            let msg = "Foo Bar";
            layout.set_text(msg);
            pangocairo::functions::show_layout(&ctx, &**layout);

            ctx.set_source_rgba(1.0, 1.0, 1.0, 1.);
            ctx.move_to(670., 0.);
            let msg = "Baz Qux";
            layout.set_text(msg);
            pangocairo::functions::show_layout(&ctx, &**layout);

            None
        })
        .unwrap();

    overlay
        .connect("caps-changed", false, move |args| {
            let _overlay = args[0].get::<gst::Element>().unwrap().unwrap();
            let caps = args[1].get::<gst::Caps>().unwrap().unwrap();

            let mut drawer_time = drawer.lock().unwrap();
            drawer_time.info = Some(gst_video::VideoInfo::from_caps(&caps).unwrap());

            None
        })
        .unwrap();

    Ok(pipeline)
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
    match create_pipeline().and_then(main_loop) {
        Ok(r) => r,
        Err(e) => eprintln!("Error! {}", e),
    }
}
