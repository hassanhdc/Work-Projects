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
        .field("width", &800i32)
        .field("height", &800i32)
        .build();
    capsfilter.set_property("caps", &caps).unwrap();

    src.set_property_from_str("pattern", "white");

    let fontmap = pangocairo::FontMap::new().unwrap();

    let context = fontmap.create_context().unwrap();
    let layout_time = LayoutWrapper(pango::Layout::new(&context));
    let layout_message = LayoutWrapper(pango::Layout::new(&context));

    let font_desc = pango::FontDescription::from_string("Sans Bold 12");
    layout_time.set_font_description(Some(&font_desc));
    layout_message.set_font_description(Some(&font_desc));
    layout_message.set_text("Hello World");

    let drawer_time = Arc::new(Mutex::new(DrawingContext {
        layout: glib::SendUniqueCell::new(layout_time).unwrap(),
        info: None,
    }));
    let drawer_msg = Arc::new(Mutex::new(DrawingContext {
        layout: glib::SendUniqueCell::new(layout_message).unwrap(),
        info: None,
    }));

    let drawer_clone_time = drawer_time.clone();
    let drawer_clone_msg = drawer_msg.clone();

    overlay
        .connect("draw", false, move |args| {
            let drawer_time = &drawer_clone_time;
            let drawer_msg = &drawer_clone_msg;
            let drawer_time = drawer_time.lock().unwrap();
            let drawer_msg = drawer_msg.lock().unwrap();

            let _overlay = args[0].get::<gst::Element>().unwrap().unwrap();

            let cr = args[1].get::<cairo::Context>().unwrap().unwrap();
            let timestamp = args[2].get_some::<gst::ClockTime>().unwrap();
            let _duration = args[3].get_some::<gst::ClockTime>().unwrap();

            let layout_time = drawer_time.layout.borrow();
            let layout_msg = drawer_msg.layout.borrow();

            cr.set_source_rgba(1.0, 0.5, 0.0, 0.8);

            pangocairo::functions::update_layout(&cr, &**layout_time);
            pangocairo::functions::update_layout(&cr, &**layout_msg);

            cr.move_to(650., 770.);

            pangocairo::functions::show_layout(&cr, &**layout_time);
            pangocairo::functions::show_layout(&cr, &**layout_msg);

            layout_time.

            let time_str = timestamp.to_string();
            let time = format!("{:.11}", time_str);

            layout_time.set_text(&time);

            None
        })
        .unwrap();

    overlay
        .connect("caps-changed", false, move |args| {
            let _overlay = args[0].get::<gst::Element>().unwrap().unwrap();
            let caps = args[1].get::<gst::Caps>().unwrap().unwrap();

            let mut drawer_time = drawer_time.lock().unwrap();
            // let mut drawer_msg = drawer_msg.lock().unwrap();
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
