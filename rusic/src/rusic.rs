mod toolbar;

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

use toolbar::MusicToolbar;

struct App {
    adjustment: gtk::Adjustment,
    cover: gtk::Image,
    window: gtk::ApplicationWindow,
    toolbar: MusicToolbar,
}
impl App {
    fn new(application: gtk::Application) -> Self {
        let window = gtk::ApplicationWindow::new(&application);
        window.set_title("Rusic");
        window.set_default_size(400, 100);
        window.set_border_width(15);

        let toolbar = MusicToolbar::new();
        use gtk::Orientation::{Horizontal, Vertical};
        let vbox = gtk::Box::new(Vertical, 0);
        vbox.add(toolbar.toolbar());

        let cover = gtk::Image::new();
        cover.set_from_file("cover.jpg");
        vbox.add(&cover);

        let adjustment = gtk::Adjustment::new(0.0, 0.0, 0.0, 10.0, 0.0, 0.0);
        let scale = gtk::Scale::new(Horizontal, Some(&adjustment));
        scale.set_draw_value(false);
        vbox.add(&scale);

        window.add(&vbox);
        window.show_all();

        let app = App {
            toolbar,
            window,
            adjustment,
            cover,
        };
        //test
        app.connect_events();
        app.connect_toolbar_events();

        app
    }
    fn connect_events(&self) {}
    pub fn connect_toolbar_events(&self) {
        let window = self.window.clone();
        self.toolbar.quit_button.connect_clicked(move |_| unsafe {
            window.destroy();
        });

        let play_button = self.toolbar.play_button.clone();
        self.toolbar.play_button.connect_clicked(move |_| {
            let play_stock = Some(glib::GString::from("media-playback-start"));

            if play_button.get_icon_name() == play_stock {
                play_button.set_icon_name(Some("media-playback-pause"));
                play_button.show_now();
            } else {
                play_button.set_icon_name(Some("media-playback-start"));
            }
        });
    }
}

fn main() {
    let application = gtk::Application::new(Some("com.Rusic"), Default::default())
        .expect("Unable to initialize app..");

    application.connect_activate(|app| {
        App::new(app.clone());
    });

    application.run(&args().collect::<Vec<_>>());
}
