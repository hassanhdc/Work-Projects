use gst::prelude::*;

use pango::prelude::*;

use std::ops;
use std::sync::{Arc, Mutex};

const FRAME_WIDTH: i32 = 640;
const FRAME_HEIGHT: i32 = 640;
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

fn example_main() {
    gst::init().unwrap();

    let src = gst::ElementFactory::make("videotestsrc", Some("src")).unwrap();
    let sink = gst::ElementFactory::make("autovideosink", Some("sink")).unwrap();
    let capsfilter = gst::ElementFactory::make("capsfilter", None).unwrap();

    let pipeline = gst::Pipeline::new(None);
    pipeline.add_many(&[&src, &capsfilter, &sink]).unwrap();
    gst::Element::link_many(&[&src, &capsfilter, &sink]).unwrap();

    let caps = gst::Caps::builder("video/x-raw")
        .field("width", &FRAME_WIDTH)
        .field("height", &FRAME_HEIGHT)
        .field("framerate", &gst::Fraction::new(15, 1))
        .build();

    capsfilter.set_property("caps", &caps).unwrap();
    src.set_property_from_str("pattern", "white");

    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();
    let src = pipeline.get_by_name("src").unwrap();
    let pad = src.get_static_pad("src").unwrap();

    let fontmap = pangocairo::FontMap::new().unwrap();

    let context = fontmap.create_context().unwrap();
    let layout = LayoutWrapper(pango::Layout::new(&context));

    let font_desc = pango::FontDescription::from_string("Sans Bold 24");
    layout.set_font_description(Some(&font_desc));
    layout.set_text("GStreamer");

    let drawer = Arc::new(Mutex::new(DrawingContext {
        layout: glib::SendUniqueCell::new(layout).unwrap(),
        info: None,
    }));

    let drawer_clone = drawer.clone();

    pad.add_probe(gst::PadProbeType::BUFFER, move |_, probe_info| {
        if let Some(gst::PadProbeData::Buffer(ref mut buffer)) = probe_info.data {
            let buffer = buffer.make_mut();
            let mut map = buffer.map_writable().unwrap();

            let buf_modified = &mut map.to_vec();

            let drawer = &drawer_clone;
            let drawer = drawer.lock().unwrap();

            let layout = drawer.layout.borrow();

            //? _____Draw a square at (x,y) position in the buffer frame_____ (Attempt: 2)
            // {
            //     let x_pos = 320;
            //     let y_pos = 320;
            //     let square_size = 50;
            //     let mut buf_idx = vec![];
            //     for j in 0..=square_size {
            //         let start_idx = ((y_pos + j) * 2560) + (x_pos * 4);
            //         let end_idx = start_idx + (square_size * 4);
            //         buf_idx.push((start_idx, end_idx));
            //         let line = &mut buf_modified[start_idx..=end_idx];
            //         for val in line {
            //             *val = 0;
            //         }
            //     }
            //     map.swap_with_slice(buf_modified);
            // }

            let buf_idx = &mut buf_modified[0..(2560 * 50)];
            let mut cloned_buf = vec![0; (2560 * 50)];
            cloned_buf.copy_from_slice(buf_idx);

            let surface = cairo::ImageSurface::create_for_data(
                cloned_buf,
                cairo::Format::ARgb32,
                640 as i32,
                50 as i32,
                2560 as i32,
            )
            .unwrap();

            let cr = cairo::Context::new(&surface);

            cr.paint();

            cr.set_source_rgb(1.0, 0., 0.);

            cr.translate(f64::from(FRAME_WIDTH) / 2.0, 50.);

            pangocairo::functions::show_layout(&cr, &**layout);

            map.swap_with_slice(&mut cloned_buf);
            drop(cr);
            drop(surface);
        };

        gst::PadProbeReturn::Ok
    });

    pipeline.set_state(gst::State::Playing).unwrap();

    let bus = pipeline.get_bus().unwrap();
    for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(_) => break,
            MessageView::Error(_) => break,
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null).unwrap();
}

fn main() {
    example_main();
}
