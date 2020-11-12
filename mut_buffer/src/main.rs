use gstreamer as gst;
// use gstreamer_video as gst_video;

use gst::prelude::*;

// use byte_slice_cast::*;

fn example_main() {
    gst::init().unwrap();

    let src = gst::ElementFactory::make("videotestsrc", Some("src")).unwrap();
    let sink = gst::ElementFactory::make("autovideosink", Some("sink")).unwrap();
    let capsfilter = gst::ElementFactory::make("capsfilter", None).unwrap();

    let pipeline = gst::Pipeline::new(None);
    pipeline.add_many(&[&src, &capsfilter, &sink]).unwrap();
    gst::Element::link_many(&[&src, &capsfilter, &sink]).unwrap();

    let caps = gst::Caps::builder("video/x-raw")
        .field("width", &640_i32)
        .field("height", &320_i32)
        .field("framerate", &gst::Fraction::new(15, 1))
        .build();

    capsfilter.set_property("caps", &caps).unwrap();
    // src.set_property("num_buffers", &1).unwrap();
    src.set_property_from_str("pattern", "black");

    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();
    let src = pipeline.get_by_name("src").unwrap();
    let pad = src.get_static_pad("src").unwrap();

    pad.add_probe(gst::PadProbeType::BUFFER, move |_, probe_info| {
        if let Some(gst::PadProbeData::Buffer(ref mut buffer)) = probe_info.data {
            let buffer = buffer.make_mut();
            let mut map = buffer.map_writable().unwrap();

            let ref mut modified = map.to_vec();

            let mut row_start = 0;
            let mut row_end = 2560;

            for _ in 0..320 {
                let row = &mut modified[row_start..row_end];
                let pix = &mut row[1280];
                *pix += 255;

                row_start += 2560;
                row_end += 2560;
            }

            map.swap_with_slice(modified);

            // let mut count = 0;
            // for _ in map.to_vec().iter_mut() {
            //     if count < 640 * 2 {
            //         map[count] = 0;
            //     } else {
            //         count = 0;
            //     }
            //     count += 1;
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
