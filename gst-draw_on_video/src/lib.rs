use std::ops;
use std::sync::{Arc, Mutex};

use anyhow::Error;
use derive_more::{Display, Error};

use gst::prelude::*;
use pango::prelude::*;

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

pub struct DrawingContext {
    pub layout: glib::SendUniqueCell<LayoutWrapper>,
    pub info: Option<gst_video::VideoInfo>,
}

#[derive(Debug)]
pub struct LayoutWrapper(pub pango::Layout);

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
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct text {
    content: String,
    x: f64,
    y: f64,
    rgba: (f64, f64, f64, f64),
    present_time: (u64, u64),
    pub(crate) rendered: bool,
}
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    rgba: (f64, f64, f64, f64),
    present_time: (u64, u64),
    pub(crate) rendered: bool,
}

#[derive(Clone)]
pub enum OverlayElement {
    Rectangle(rect),
    Text(text),
}

impl OverlayElement {
    pub fn create_rectangle(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        rgba: (f64, f64, f64, f64),
        present_time: (u64, u64),
    ) -> Self {
        let rect = rect {
            x,
            y,
            width,
            height,
            rgba,
            present_time,
            rendered: false,
        };
        OverlayElement::Rectangle(rect)
    }

    pub fn create_text(
        content: String,
        x: f64,
        y: f64,
        rgba: (f64, f64, f64, f64),
        present_time: (u64, u64),
    ) -> Self {
        let text = text {
            content,
            x,
            y,
            rgba,
            present_time,
            rendered: false,
        };
        OverlayElement::Text(text)
    }
}

pub struct VideoContext {}

impl VideoContext {
    pub fn new(width: i32, height: i32, framerate: i32) -> Result<gst::Pipeline, Error> {
        gst::init()?;
        let pipeline = gst::Pipeline::new(None);

        // initiate elements
        let src = gst::ElementFactory::make("videotestsrc", None)
            .map_err(|_| MissingElement("videotestsrc"))?;
        let overlay = gst::ElementFactory::make("cairooverlay", Some("overlay"))
            .map_err(|_| MissingElement("cairooverlay"))?;
        let capsfilter = gst::ElementFactory::make("capsfilter", None)
            .map_err(|_| MissingElement("capsfilter"))?;
        let videoconvert = gst::ElementFactory::make("videoconvert", None)
            .map_err(|_| MissingElement("videoconvert"))?;
        let sink = gst::ElementFactory::make("autovideosink", None)
            .map_err(|_| MissingElement("autovideosink"))?;

        pipeline.add_many(&[&src, &overlay, &capsfilter, &videoconvert, &sink])?;
        gst::Element::link_many(&[&src, &overlay, &capsfilter, &videoconvert, &sink])?;

        let caps = gst::Caps::builder("video/x-raw")
            .field("width", &width)
            .field("height", &height)
            .field("framerate", &gst::Fraction::new(framerate, 1))
            .build();
        capsfilter.set_property("caps", &caps).unwrap();
        src.set_property_from_str("pattern", "smpte");

        Ok(pipeline)
    }

    pub fn draw_on(overlay: gst::Element, overlay_element: Vec<OverlayElement>) {
        let fontmap = pangocairo::FontMap::new().unwrap();
        let context = fontmap.create_context().unwrap();
        let layout = LayoutWrapper(pango::Layout::new(&context));
        let font_desc = pango::FontDescription::from_string("Sans Bold 12");
        layout.set_font_description(Some(&font_desc));

        let drawer = Arc::new(Mutex::new(DrawingContext {
            layout: glib::SendUniqueCell::new(layout).unwrap(),
            info: None,
        }));

        // let draw_args = Box::new(overlay_element);

        // get a copy of the wrapper object so it can be moved into the callback
        let drawer_clone = drawer.clone();
        overlay
            .connect("draw", false, move |args| {
                let drawer = &drawer_clone;
                let drawer = drawer.lock().unwrap();

                let timestamp = args[2].get_some::<gst::ClockTime>().unwrap();
                let ctx = args[1].get::<cairo::Context>().unwrap().unwrap();
                let layout = drawer.layout.borrow();

                // create an empty string as a placeholder for render time comparison
                // let mut time_str = String::from("");

                // create new surface to draw on
                let surface = cairo::ImageSurface::create(cairo::Format::Rgb30, 20, 20).unwrap();
                ctx.set_source_surface(&surface, 0., 0.);

                // format time as string to display on video frame
                let timestamp_str = format!("{:.11}", timestamp.to_string());
                ctx.set_source_rgba(1.0, 1.0, 1.0, 1.);
                ctx.move_to(1800., 970.);
                layout.set_text(&timestamp_str);
                pangocairo::functions::show_layout(&ctx, &**layout);

                let draw_args = overlay_element.clone();

                for ele in draw_args.into_iter() {
                    match ele {
                        OverlayElement::Rectangle(mut rect) => {
                            if !rect.rendered {
                                if (timestamp < rect.present_time.1 * gst::SECOND)
                                    && (timestamp > rect.present_time.0 * gst::SECOND)
                                {
                                    ctx.set_source_rgba(
                                        rect.rgba.0,
                                        rect.rgba.1,
                                        rect.rgba.2,
                                        rect.rgba.3,
                                    );
                                    ctx.rectangle(rect.x, rect.y, rect.width, rect.height);
                                    ctx.fill();
                                } else {
                                    rect.rendered = true;
                                }
                            } else {
                                continue;
                            }
                        }
                        OverlayElement::Text(mut text) => {
                            if !text.rendered {
                                if (timestamp < text.present_time.1 * gst::SECOND)
                                    && (timestamp > text.present_time.0 * gst::SECOND)
                                {
                                    ctx.move_to(text.x, text.y);
                                    ctx.set_source_rgba(
                                        text.rgba.0,
                                        text.rgba.1,
                                        text.rgba.2,
                                        text.rgba.3,
                                    );
                                    layout.set_text(text.content.as_str());
                                    pangocairo::functions::show_layout(&ctx, &**layout);
                                } else {
                                    text.rendered = true;
                                }
                            } else {
                                continue;
                            }
                        }
                    }
                }
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
    }
}
