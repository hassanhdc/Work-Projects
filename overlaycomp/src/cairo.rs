use gst::prelude::*;
use std::ops;

const FRAME_WIDTH: i32 = 600;
const FRAME_HEIGHT: i32 = 600;

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
    src.set_property_from_str("pattern", "smpte");

    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();
    let src = pipeline.get_by_name("src").unwrap();
    let pad = src.get_static_pad("src").unwrap();

    pad.add_probe(gst::PadProbeType::BUFFER, move |_, probe_info| {
        if let Some(gst::PadProbeData::Buffer(ref mut buffer)) = probe_info.data {
            let buffer = buffer.make_mut();
            let time = buffer.get_pts().to_string();
            let mut map = buffer.map_writable().unwrap();

            let buf_modified = &mut map.to_vec();

            let clone_buf = buf_modified.clone();

            let mut surface = cairo::ImageSurface::create_for_data(
                clone_buf,
                cairo::Format::ARgb32,
                FRAME_WIDTH,
                FRAME_HEIGHT,
                FRAME_WIDTH * 4,
            )
            .unwrap();

            let ctx = cairo::Context::new(&surface);
            ctx.set_source_rgb(0.05, 0.05, 0.05);
            ctx.rectangle(300., 550., 300., 50.);
            // ctx.paint();
            // ctx.fill();

            ctx.set_source_rgb(1.0, 1.0, 1.0);
            ctx.select_font_face("Purisa", cairo::FontSlant::Normal, cairo::FontWeight::Bold);

            ctx.set_font_size(20.);

            ctx.move_to(400., 590.);
            ctx.show_text(&time);

            drop(ctx);

            let mut raw = surface.get_data().unwrap().to_vec();

            drop(surface);

            map.swap_with_slice(&mut raw);

            // {
            //     let mut surface =
            //         cairo::ImageSurface::create(cairo::Format::ARgb32, FRAME_WIDTH, FRAME_HEIGHT)
            //             .unwrap();
            //     let ctx = cairo::Context::new(&surface);
            //     ctx.set_source_rgb(1., 0., 0.);
            //     ctx.select_font_face(
            //         "Purisa",
            //         cairo::FontSlant::Normal,
            //         cairo::FontWeight::Normal,
            //     );
            //     ctx.set_font_size(18.);
            //     ctx.move_to(400., 590.);
            //     ctx.show_text("What is love");
            //     // ctx.rectangle(20., 20., 100., 100.);
            //     // ctx.fill();
            //     drop(ctx);
            //     let stride = surface.get_stride();
            //     let data = surface.get_data().unwrap().to_vec();
            //     let data = &data[..];
            //     let mut target_data = vec![0; (640 * 4 * 640)];
            //     target_data.copy_from_slice(data);
            //     map.swap_with_slice(&mut target_data);
            // }
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
