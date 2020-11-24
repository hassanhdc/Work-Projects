extern crate cairo;

use cairo::{Context, Format, ImageSurface};

fn main() {
    let mut surface = ImageSurface::create(Format::ARgb32, 600, 600).unwrap();

    let ctx = Context::new(&surface);
    ctx.set_source_rgb(1., 1., 0.);
    ctx.paint();

    ctx.set_source_rgb(1., 0., 0.);
    ctx.select_font_face(
        "Purisa",
        cairo::FontSlant::Normal,
        cairo::FontWeight::Normal,
    );

    ctx.set_font_size(18.);

    ctx.move_to(400., 590.);
    ctx.show_text("What is love");

    // ctx.rectangle(20., 20., 100., 100.);
    // ctx.fill();

    // ctx.set_source_rgb(0.6, 0.6, 0.6);
    // ctx.rectangle(150., 20., 100., 100.);
    // ctx.fill();

    // ctx.paint();

    use std::fs::File;

    let mut file = File::create("output.png").unwrap();
    // surface.write_to_png(&mut file).unwrap();

    drop(ctx);

    let stride = surface.get_stride();

    let mut cloned_surface = surface.clone();
    drop(surface);

    let data = cloned_surface.get_data().unwrap();
    let data = data.to_vec();
    let data = &data[1368000..];

    let mut dummy_data = vec![0; (600 * 4 * 30)];
    dummy_data.copy_from_slice(data);
    println!("{:?}", &stride);

    let other =
        cairo::ImageSurface::create_for_data(dummy_data, Format::ARgb32, 600, 30, stride).unwrap();
    other.write_to_png(&mut file).unwrap();
}
