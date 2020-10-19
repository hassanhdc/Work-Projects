mod playlist;
mod toolbar;

extern crate gdk_pixbuf;
extern crate gio;
extern crate gtk;
use gtk::{FileChooserAction, FileChooserDialog, FileFilter};
extern crate gtk_sys;
extern crate id3;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;
use std::path::PathBuf;
use std::rc::Rc;

use playlist::Playlist;
use toolbar::MusicToolbar;

struct App {
    adjustment: gtk::Adjustment,
    cover: gtk::Image,
    window: gtk::ApplicationWindow,
    toolbar: MusicToolbar,
    playlist: Rc<Playlist>,
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

        let playlist = Rc::new(Playlist::new());
        vbox.add(playlist.view());

        let cover = gtk::Image::new();
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
            playlist,
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

        //? Change state of Play button to 'Pause' and to 'Play' again on repetitive clicks
        //? And change the 'cover' of the song on 'Pause' and 'Play'
        let play_button = self.toolbar.play_button.clone();
        let playlist = self.playlist.clone();
        let cover = self.cover.clone();

        self.toolbar.play_button.connect_clicked(move |_| {
            let play_stock = Some(glib::GString::from("media-playback-start"));

            if play_button.get_icon_name() == play_stock {
                play_button.set_icon_name(Some("media-playback-pause"));
                toolbar::set_cover(&cover, &playlist);
                println!("{:?}", play_button.get_icon_widget());
            } else {
                play_button.set_icon_name(Some("media-playback-start"));
                println!("{:?}", play_button.get_icon_widget());
            }
        });

        //? Connect 'file open' dialog to the 'Open' Button
        let parent = self.window.clone();
        let playlist = self.playlist.clone();

        self.toolbar.open_button.connect_clicked(move |_| {
            let file = show_open_dialog(&parent);
            if let Some(file) = file {
                playlist.add(&file);
            }
        });

        //? Connect the 'remove selection' action to the 'remove' button
        let playlist = self.playlist.clone();
        self.toolbar.remove_button.connect_clicked(move |_| {
            playlist.remove_selection();
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

fn show_open_dialog(parent: &gtk::ApplicationWindow) -> Option<PathBuf> {
    let mut file = None;
    let dialog =
        FileChooserDialog::new(Some("Select a file"), Some(parent), FileChooserAction::Open);
    let filter = FileFilter::new();
    filter.add_mime_type("audio/mp3");
    filter.set_name(Some("MP3 Audio"));
    dialog.add_filter(&filter);
    dialog.add_button("Accept", gtk::ResponseType::Accept);
    dialog.add_button("Cancel", gtk::ResponseType::Cancel);

    let result = dialog.run();
    if result == gtk::ResponseType::Accept {
        file = dialog.get_filename();
    }
    unsafe {
        dialog.destroy();
    }
    file
}
