use std::path::Path;

use gdk_pixbuf::{InterpType, Pixbuf, PixbufLoader, PixbufLoaderExt};
use gtk::prelude::*;

use id3::Tag;

const THUMBNAIL_COL: u32 = 0;
const TITLE_COL: u32 = 1;
const ARTIST_COL: u32 = 2;
const ALBUM_COL: u32 = 3;
const GENRE_COL: u32 = 4;
const YEAR_COL: u32 = 5;
const TRACK_COL: u32 = 6;
const PATH_COL: u32 = 7;
const PIXBUF_COL: u32 = 8;
const IMAGE_SIZE: i32 = 256;
const THUMBNAIL_SIZE: i32 = 64;
const INTERP_HYPER: InterpType = InterpType::Hyper;

pub struct Playlist {
    model: gtk::ListStore,
    treeview: gtk::TreeView,
}

#[derive(PartialEq)]
enum Visibility {
    Invisible,
    Visible,
}
use self::Visibility::*;

impl Playlist {
    pub fn new() -> Self {
        let model = gtk::ListStore::new(&[
            Pixbuf::static_type(),
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
            glib::Type::String,
            Pixbuf::static_type(),
        ]);
        let treeview = gtk::TreeView::with_model(&model);
        treeview.set_hexpand(true);
        treeview.set_vexpand(true);

        Self::create_columns(&treeview);

        Playlist { model, treeview }
    }
    fn create_columns(treeview: &gtk::TreeView) {
        Self::add_pixbuf_column(treeview, THUMBNAIL_COL as i32, Visible);
        Self::add_text_column(treeview, "Title", TITLE_COL as i32);
        Self::add_text_column(treeview, "Artist", ARTIST_COL as i32);
        Self::add_text_column(treeview, "Album", ALBUM_COL as i32);
        Self::add_text_column(treeview, "Genre", GENRE_COL as i32);
        Self::add_text_column(treeview, "Year", YEAR_COL as i32);
        Self::add_text_column(treeview, "Track", TRACK_COL as i32);
        Self::add_pixbuf_column(treeview, PIXBUF_COL as i32, Invisible);
    }
    fn add_text_column(treeview: &gtk::TreeView, title: &str, column: i32) {
        let view_column = gtk::TreeViewColumn::new();
        view_column.set_title(title);
        let cell = gtk::CellRendererText::new();
        view_column.set_expand(true);
        view_column.pack_start(&cell, true);
        view_column.add_attribute(&cell, "text", column);
        treeview.append_column(&view_column);
    }
    fn add_pixbuf_column(treeview: &gtk::TreeView, column: i32, visibility: Visibility) {
        let view_column = gtk::TreeViewColumn::new();
        if visibility == Visible {
            let cell = gtk::CellRendererPixbuf::new();
            view_column.pack_start(&cell, true);
            view_column.add_attribute(&cell, "pixbuf", column);
        }
        treeview.append_column(&view_column);
    }
    fn set_pixbuf(&self, row: &gtk::TreeIter, tag: &Tag) {
        if let Some(picture) = tag.pictures().next() {
            let pixbuf_loader = PixbufLoader::new();
            pixbuf_loader.set_size(IMAGE_SIZE, IMAGE_SIZE);
            pixbuf_loader.write(&picture.data).unwrap();
            if let Some(pixbuf) = pixbuf_loader.get_pixbuf() {
                let thumbnail = pixbuf
                    .scale_simple(THUMBNAIL_SIZE, THUMBNAIL_SIZE, INTERP_HYPER)
                    .unwrap();
                self.model
                    .set_value(row, THUMBNAIL_COL, &thumbnail.to_value());
                self.model.set_value(row, PIXBUF_COL, &pixbuf.to_value());
            }
            pixbuf_loader.close().unwrap();
        }
    }
    pub fn view(&self) -> &gtk::TreeView {
        &self.treeview
    }
    pub fn add(&self, path: &Path) {
        let filename = path
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        let row = self.model.append();

        if let Ok(tag) = Tag::read_from_path(path) {
            let title = tag.title().unwrap_or(filename);
            let artist = tag.artist().unwrap_or("no artist");
            let album = tag.album().unwrap_or("no album");
            let genre = tag.genre().unwrap_or("no genre");
            let year = tag
                .year()
                .map(|year| year.to_string())
                .unwrap_or(("no year").to_string());
            let track = tag
                .track()
                .map(|track| track.to_string())
                .unwrap_or(("??").to_string());
            let total_tracks = tag
                .total_tracks()
                .map(|total_tracks| total_tracks.to_string())
                .unwrap_or(("??").to_string());
            let track_value = format!("{} / {}", track, total_tracks);

            self.set_pixbuf(&row, &tag);

            self.model.set_value(&row, TITLE_COL, &title.to_value());
            self.model.set_value(&row, ARTIST_COL, &artist.to_value());
            self.model.set_value(&row, ALBUM_COL, &album.to_value());
            self.model.set_value(&row, GENRE_COL, &genre.to_value());
            self.model.set_value(&row, YEAR_COL, &year.to_value());
            self.model.set_value(&row, TRACK_COL, &track.to_value());
        } else {
            self.model.set_value(&row, TITLE_COL, &filename.to_value());
        }
        let path = path.to_str().unwrap_or_default();
        self.model.set_value(&row, PATH_COL, &path.to_value());
    }

    pub fn remove_selection(&self) {
        let selection = self.treeview.get_selection();
        if let Some((_, iter)) = selection.get_selected() {
            self.model.remove(&iter);
        }
    }

    pub fn pixbuf(&self) -> Option<Pixbuf> {
        let selection = self.treeview.get_selection();

        if let Some((_, iter)) = selection.get_selected() {
            let value = self.model.get_value(&iter, PIXBUF_COL as i32);
            return value.get::<Pixbuf>().unwrap();
        }
        None
    }
}
