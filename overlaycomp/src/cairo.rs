use gst::prelude::*;
use std::ops;

const FRAME_WIDTH: i32 = 1920;
const FRAME_HEIGHT: i32 = 1080;

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
        .field("framerate", &gst::Fraction::new(30, 1))
        .build();

    capsfilter.set_property("caps", &caps).unwrap();
    src.set_property_from_str("pattern", "smpte");

    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();
    let src = pipeline.get_by_name("src").unwrap();
    let pad = src.get_static_pad("src").unwrap();

    pad.add_probe(gst::PadProbeType::BUFFER, move |_, probe_info| {
        if let Some(gst::PadProbeData::Buffer(ref mut buffer)) = probe_info.data {
            let buffer = buffer.make_mut();
            let time = buffer.get_pts();
            // let mut time_str = time.to_string();
            // let mut time_str = format!("{:.11}", time.to_string());
            let mut time_str = String::from("");
            let mut map = buffer.map_writable().unwrap();

            let buf_modified = &mut map.to_vec();

            let mut surface = cairo::ImageSurface::create_for_data(
                buf_modified.clone(),
                cairo::Format::Rgb30,
                FRAME_WIDTH,
                FRAME_HEIGHT,
                FRAME_WIDTH * 4,
            )
            .unwrap();

            let ctx = cairo::Context::new(&surface);
            ctx.set_source_rgb(1.0, 1.0, 0.5);
            ctx.rectangle(1650., 960., 200., 50.);
            ctx.fill();

            ctx.set_source_rgb(0., 0.0, 0.);
            ctx.select_font_face("Purisa", cairo::FontSlant::Normal, cairo::FontWeight::Bold);
            ctx.set_font_size(20.);
            ctx.move_to(1730., 990.);

            // TODO : use overlay args to simulate if overlays can be shown with cmd line argument
            let mut overlay_args = vec![
                (0, 4, false, "FOO"),
                (4, 6, false, "BAR"),
                (6, 8, false, "BAZ"),
                (6, 10, false, "QUX"),
            ];

            for arg in overlay_args.iter_mut() {
                if arg.2 == false {
                    if time < arg.1 * gst::SECOND {
                        time_str = arg.3.to_string();
                        break;
                    } else {
                        arg.2 = true;
                    }
                } else {
                    continue;
                }
            }

            ctx.show_text(&time_str);

            //? EXPERIMENT : test update text with buffer pts
            // if time > 2 * gst::SECOND {
            //     time_str = "time's up".to_string();
            // };

            let mut msg = "HWAT";
            ctx.save();
            ctx.move_to(10., 15.);
            if time > 2 * gst::SECOND {
                msg = "WHAT";
            };
            ctx.show_text(msg);

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
