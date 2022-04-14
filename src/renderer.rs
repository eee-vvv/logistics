use std::path::Path;

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use specs::prelude::*;

use crate::components::*;

// Type alias for the data needed by the renderer
pub type SystemData<'a> = (ReadStorage<'a, Position>, ReadStorage<'a, Sprite>);

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    data: SystemData,
) -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();

    let texture_creator = canvas.texture_creator();

    let (width, height) = canvas.output_size()?;

    let background_texture = texture_creator.load_texture("assets/background.png")?;
    canvas.copy(&background_texture, None, Rect::new(0, 0, width, height))?;

    // title rendering
    let font_path = Path::new("assets/mudclub.otf");
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font(font_path, 50).unwrap();
    let surface = font
        .render("logistics")
        .blended(Color::RGBA(255, 87, 0, 255))
        .unwrap();
    let font_texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    canvas.copy(
        &font_texture,
        None,
        Rect::from_center(Point::new(200, 200), 200, 100),
    )?;


    for (pos, sprite) in (&data.0, &data.1).join() {
        let current_frame = sprite.region;

        // Treat the center of the screen as the (0, 0) coordinate
        let screen_position = pos.0 + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(
            screen_position,
            current_frame.width(),
            current_frame.height(),
        );
        canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
    }

    canvas.present();

    Ok(())
}
