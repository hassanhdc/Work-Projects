extern crate gstreamer as gst;

use gst::prelude::*;
use std::io::{self, Read};
use std::thread;

struct CustomData {
    pipeline: gst::Pipeline,
    weak_sink: glib::WeakRef<gst::Element>,
    playing: bool,
    terminate: bool,
    // rate: f64,
}

fn example_main() {
    gst::init().unwrap();

    let pipeline = gst::parse_launch("videotestsrc ! autovideosink name=sink").unwrap();

    pipeline.set_state(gst::State::Playing).unwrap();
    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();

    let bus = pipeline.get_bus().unwrap();

    let weak_sink = pipeline.get_by_name("sink").unwrap().downgrade();

    let mut custom_data = CustomData {
        pipeline,
        weak_sink,
        playing: true,
        terminate: false,
        // rate: 1.0,
    };

    while !custom_data.terminate {
        let msg = bus.timed_pop(gst::MSECOND * 100);
        match msg {
            Some(msg) => handle_message(&mut custom_data, &msg),
            None => handle_keyevent(&mut custom_data),
        }
    }

    custom_data.pipeline.set_state(gst::State::Null).unwrap();
}

fn handle_message(custom_data: &mut CustomData, msg: &gst::Message) {
    use gst::MessageView;
    match msg.view() {
        MessageView::Eos(..) => {
            println!("Received EOS");
            custom_data.terminate = true;
        }

        MessageView::StateChanged(state_change) => {
            if state_change
                .get_src()
                .map(|s| s == custom_data.pipeline)
                .unwrap_or(false)
            {
                let new_state = state_change.get_current();
                let old_state = state_change.get_old();

                println!(
                    "Pipeline state changed from {:?} to {:?}",
                    old_state, new_state
                );
            }
        }
        MessageView::Error(_) => {
            println!("Got error");
            custom_data.terminate = true;
        }
        _ => (),
    }
}

fn handle_keyevent(custom_data: &mut CustomData) {
    let input = io::stdin().bytes();

    let mut b = input.into_iter().map(|s| s.unwrap() as char);

    match b.next() {
        Some(key) => match key {
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
                if let Some(sink) = custom_data.weak_sink.upgrade() {
                    let ev = gst::event::Step::new(gst::format::Buffers(Some(1)), 1.0, true, false);
                    sink.send_event(ev);
                } else {
                    println!("Failed to get sink from pipeline");
                }
            }

            'q' => {
                println!("Quitting..");
                thread::sleep(std::time::Duration::from_millis(500));
                custom_data.terminate = true;
            }

            //? simulate EOS Event
            'e' => {
                if !custom_data.playing {
                    custom_data.pipeline.set_state(gst::State::Playing).unwrap();
                };

                if let Some(sink) = custom_data.weak_sink.upgrade() {
                    let ev = gst::event::Eos::new();
                    sink.send_event(ev);
                }
            }
            _ => (),
        },
        None => (),
    }
}

fn main() {
    example_main();
}
