use gstreamer as gst;

use gst::prelude::*;

const FRAME_WIDTH: i32 = 640;
const FRAME_HEIGHT: i32 = 640;

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
    src.set_property_from_str("pattern", "black");

    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();
    let src = pipeline.get_by_name("src").unwrap();
    let pad = src.get_static_pad("src").unwrap();

    pad.add_probe(gst::PadProbeType::BUFFER, move |_, probe_info| {
        if let Some(gst::PadProbeData::Buffer(ref mut buffer)) = probe_info.data {
            let buffer = buffer.make_mut();
            let mut map = buffer.map_writable().unwrap();

            let ref mut buf_modified = map.to_vec();

            //& redundant block of code
            { // let mut row_start = 0;
                 // let mut row_end = 2560;
                 // for _ in 0..320 {
                 //     // let row = &mut modified[row_start..row_end];
                 //     // let pix = &mut row[1280];
                 //     // *pix += 255;
                 //     // row_start += 2560;
                 //     // row_end += 2560;
                 //     let row = modified.chunks_exact_mut(4);
                 //     let mut count = 0;
                 //     for arr in row {
                 //         if count == 120 {
                 //             for pix in arr {
                 //                 *pix = 255;
                 //             }
                 //         }
                 //         count += 1;
                 //     }
                 // }
            }

            //& redundant block of code
            { // let mut count = 0;
                 // for _ in map.to_vec().iter_mut() {
                 //     if count < 640 * 2 {
                 //         map[count] = 0;
                 //     } else {
                 //         count = 0;
                 //     }
                 //     count += 1;
                 // }
            }

            //? _____Draw a square at (x,y) position in the buffer frame_____ (Attempt: 1)
            {
                //? draw dimensions for square
                let lines = FRAME_WIDTH * 4;
                let draw_x = 320;
                let draw_y = 320;

                //^ note : square size cannot be greater than any of the draw dimensions
                let square_size = 50;

                //^ counter for vertical lines in the frame
                let mut vertical_lines = 0;

                //^ Iterates over each line (note: accounts for stride per line)
                for line in buf_modified.chunks_exact_mut(lines as usize) {
                    //^ since the square grows from the center i.e. half in each direction and
                    //^ there is an exact '4' chunks per iteration, square_size is divided by 8 (4 * 2)

                    if (vertical_lines > draw_y - (square_size / 8))
                        && (vertical_lines < draw_y + (square_size / 8))
                    {
                        for pix in &mut line[((draw_x * 4) - (square_size / 2))
                            ..=((draw_x * 4) + (square_size / 2))]
                            .chunks_exact_mut(4)
                        {
                            //^ change each of 'ARGB' values per pixel to white
                            pix[0] = 255;
                            pix[1] = 255;
                            pix[2] = 255;
                            pix[3] = 255;
                        }
                    }
                    vertical_lines += 1
                }

                map.swap_with_slice(buf_modified);
            }

            //? _____Draw a square at (x,y) position in the buffer frame_____ (Attempt: 2)
            {
                let square_size = 319;

                let draw_x = 320;
                let draw_y = 320;

                for x in 0..=square_size {
                    let start_idx = ((draw_y + x) * 2560) + (draw_x * 4);
                    let end_idx = start_idx + (square_size * 4);

                    let line = &mut buf_modified[start_idx..=end_idx];
                    for val in line {
                        *val = 255;
                    }
                }

                map.swap_with_slice(buf_modified);
            }
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
