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
    let overlay = gst::ElementFactory::make("overlaycomposition", None)
        .map_err(|_| MissingElement("overlaycomposition"))?;
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
        .field("framerate", &gst::Fraction::new(15, 1))
        .build();
    capsfilter.set_property("caps", &caps).unwrap();

    src.set_property_from_str("pattern", "white");

    let fontmap = pangocairo::FontMap::new().unwrap();

    let context = fontmap.create_context().unwrap();

    let layout = LayoutWrapper(pango::Layout::new(&context));
    let layout2 = LayoutWrapper(pango::Layout::new(&context));
    let font_desc = pango::FontDescription::from_string("Sans Bold 26");
    layout.set_font_description(Some(&font_desc));
    layout2.set_font_description(Some(&font_desc));
    layout.set_text("World");
    layout2.set_text("Hello");

    let drawer = Arc::new(Mutex::new(DrawingContext {
        layout: glib::SendUniqueCell::new(layout).unwrap(),
        info: None,
    }));

    let drawer2 = Arc::new(Mutex::new(DrawingContext {
        layout: glib::SendUniqueCell::new(layout2).unwrap(),
        info: None,
    }));

    let drawer_clone = drawer.clone();
    let drawer2_clone = drawer2.clone();

    overlay
        .connect("draw", false, move |args| {
            let drawer = &drawer_clone;
            let drawer2 = &drawer2_clone;

            let drawer = drawer.lock().unwrap();
            let drawer2 = drawer2.lock().unwrap();

            let _overlay = args[0].get::<gst::Element>().unwrap().unwrap();
            let sample = args[1].get::<gst::Sample>().unwrap().unwrap();
            let buffer = sample.get_buffer().unwrap();
            let _timestamp = buffer.get_pts();

            let info = drawer.info.as_ref().unwrap();
            let layout = drawer.layout.borrow();
            let layout2 = drawer2.layout.borrow();

            let frame_width = info.width() as usize;
            let frame_height = info.height() as usize;
            let stride = 4 * frame_width;
            let frame_size = stride * frame_height;

            let mut buffer = gst::Buffer::with_size(frame_size).unwrap();

            gst_video::VideoMeta::add(
                buffer.get_mut().unwrap(),
                gst_video::VideoFrameFlags::empty(),
                gst_video::VideoFormat::Bgra,
                frame_width as u32,
                frame_height as u32,
            )
            .unwrap();

            let buffer = buffer.into_mapped_buffer_writable().unwrap();
            let buffer = {
                let buffer_ptr = unsafe { buffer.get_buffer().as_ptr() };
                let surface = cairo::ImageSurface::create_for_data(
                    buffer,
                    cairo::Format::ARgb32,
                    frame_width as i32,
                    frame_height as i32,
                    stride as i32,
                )
                .unwrap();

                let cr = cairo::Context::new(&surface);
                let cr2 = cairo::Context::new(&surface);

                // cr.save();
                // cr.set_operator(cairo::Operator::Clear);
                // cr.paint();
                // cr.restore();

                // cr2.save();
                // cr2.set_operator(cairo::Operator::Clear);
                // cr2.paint();
                // cr2.restore();

                cr.translate(
                    f64::from(info.width()) / 2.0,
                    f64::from(info.height()) / 2.0,
                );
                cr2.translate(
                    f64::from(info.width()) / 2.0,
                    f64::from(info.height()) / 2.5,
                );

                pangocairo::functions::show_layout(&cr, &**layout);
                layout.set_text(&_timestamp.to_string());
                pangocairo::functions::show_layout(&cr2, &**layout2);

                drop(cr);
                drop(cr2);

                unsafe {
                    assert_eq!(
                        cairo_sys::cairo_surface_get_reference_count(surface.to_raw_none()),
                        1
                    );
                    let buffer = glib::translate::from_glib_none(buffer_ptr);
                    drop(surface);
                    buffer
                }
            };

            let rect = gst_video::VideoOverlayRectangle::new_raw(
                &buffer,
                0,
                0,
                frame_width as u32,
                frame_height as u32,
                gst_video::VideoOverlayFormatFlags::PREMULTIPLIED_ALPHA,
            );

            let rect2 = gst_video::VideoOverlayRectangle::new_raw(
                &buffer,
                100,
                100,
                frame_width as u32,
                frame_height as u32,
                gst_video::VideoOverlayFormatFlags::PREMULTIPLIED_ALPHA,
            );

            Some(
                gst_video::VideoOverlayComposition::new(&[rect])
                    .unwrap()
                    .to_value(),
            )
        })
        .unwrap();

    overlay
        .connect("caps-changed", false, move |args| {
            let _overlay = args[0].get::<gst::Element>().unwrap().unwrap();
            let caps = args[1].get::<gst::Caps>().unwrap().unwrap();

            let mut drawer = drawer.lock().unwrap();
            drawer.info = Some(gst_video::VideoInfo::from_caps(&caps).unwrap());

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
