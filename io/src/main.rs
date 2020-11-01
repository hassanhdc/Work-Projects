extern crate gstreamer as gst;

use gst::prelude::*;
use std::io::{self, Read, Write};
use std::thread;

struct CustomData {
    pipeline: gst::Pipeline,
    sink: glib::WeakRef<gst::Element>,
    playing: bool,
    // rate: f64,
}

fn example_main() {
    gst::init().unwrap();

    let main_loop = glib::MainLoop::new(None, false);

    let pipeline = gst::parse_launch("videotestsrc ! autovideosink name=sink").unwrap();

    pipeline.set_state(gst::State::Playing).unwrap();
    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();

    let bus = pipeline.get_bus().unwrap();

    let sink = pipeline.get_by_name("sink").unwrap();
    let weak_sink = sink.downgrade();

    let mut custom_data = CustomData {
        pipeline,
        sink: weak_sink,
        playing: true,
        // rate: 1.0,
    };

    let pipeline_weak = custom_data.pipeline.downgrade();

    let main_loop_clone = main_loop.clone();

    bus.add_watch(move |_, msg| {
        use gst::MessageView;
        let main_loop = &main_loop_clone.clone();

        match msg.view() {
            MessageView::Eos(..) => {
                println!("Received EOS event");
                main_loop.quit();
            }
            MessageView::Error(_) => {
                main_loop.quit();
            }
            MessageView::StateChanged(state_changed) => {
                if state_changed
                    .get_src()
                    .map(|s| s == pipeline_weak.upgrade().unwrap())
                    .unwrap_or(false)
                {
                    let new_state = state_changed.get_current();
                    let old_state = state_changed.get_old();

                    println!(
                        "Pipeline state changed from {:?} to {:?}",
                        &old_state, &new_state
                    );
                }
            }
            _ => (),
        };

        glib::Continue(true)
    })
    .expect("Failed to add watch");

    handle_keyevent(&mut custom_data);
    main_loop.run();

    custom_data.pipeline.set_state(gst::State::Null).unwrap();
}

fn handle_keyevent(custom_data: &mut CustomData) {
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;

        match &c {
            'p' => {
                if custom_data.playing {
                    custom_data.playing = false;
                    custom_data.pipeline.set_state(gst::State::Paused).unwrap();
                } else {
                    custom_data.playing = true;
                    custom_data.pipeline.set_state(gst::State::Playing).unwrap();
                }
            }
            'n' => {
                if let Some(sink) = custom_data.sink.upgrade() {
                    let ev = gst::event::Step::new(gst::format::Buffers(Some(1)), 1.0, true, false);
                    sink.send_event(ev);
                } else {
                    println!("Failed to get sink from pipeline");
                }
            }
            'q' => {
                if !custom_data.playing {
                    custom_data.pipeline.set_state(gst::State::Playing).unwrap();
                };

                if let Some(sink) = custom_data.sink.upgrade() {
                    let ev = gst::event::Eos::new();
                    sink.send_event(ev);
                    break;
                }
            }
            _ => (),
        }
    }
    println!("exiting for loop in keyevent");
}
fn main() {
    example_main();
}
