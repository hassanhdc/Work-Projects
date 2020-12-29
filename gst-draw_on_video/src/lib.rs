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
}
