// use gio::prelude::*;
use gtk::prelude::*;
const PLAY_STOCK: &str = "media-playback-start";
#[derive(Clone)]
pub struct MusicToolbar {
    toolbar: gtk::Toolbar,
    pub open_button: gtk::ToolButton,
    pub prev_button: gtk::ToolButton,
    pub play_button: gtk::ToolButton,
    pub stop_button: gtk::ToolButton,
    pub next_button: gtk::ToolButton,
    pub remove_button: gtk::ToolButton,
    pub quit_button: gtk::ToolButton,
}

fn create_toolbutton(icon_name: &str, label: &str) -> gtk::ToolButton {
    let icon = gtk::Image::from_icon_name(Some(icon_name), gtk::IconSize::Button);
    gtk::ToolButton::new(Some(&icon), Some(label))
}
impl MusicToolbar {
    pub fn new() -> Self {
        let toolbar = gtk::Toolbar::new();

        let open_button = create_toolbutton("document-open", "Open");
        toolbar.add(&open_button);

        toolbar.add(&gtk::SeparatorToolItem::new());

        let prev_button = create_toolbutton("media-skip-backward", "Previous");
        toolbar.add(&prev_button);

        let play_button = create_toolbutton(PLAY_STOCK, "Play");
        toolbar.add(&play_button);

        let stop_button = create_toolbutton("media-playback-stop", "Stop");
        toolbar.add(&stop_button);

        let next_button = create_toolbutton("media-skip-forward", "Next");
        toolbar.add(&next_button);

        toolbar.add(&gtk::SeparatorToolItem::new());

        let remove_button = create_toolbutton("list-remove", "Remove");
        toolbar.add(&remove_button);

        toolbar.add(&gtk::SeparatorToolItem::new());

        let quit_button = create_toolbutton("application-exit", "Quit");
        toolbar.add(&quit_button);

        MusicToolbar {
            toolbar,
            open_button,
            prev_button,
            play_button,
            stop_button,
            next_button,
            remove_button,
            quit_button,
        }
    }

    pub fn toolbar(&self) -> &gtk::Toolbar {
        &self.toolbar
    }
}
