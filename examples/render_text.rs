use macroquad::prelude::*;

use macroquad_text::Fonts;

// Include Fonts
const NOTO_SANS: &[u8] = include_bytes!("../assets/fonts/NotoSans-Regular.ttf");
const NOTO_SANS_JP: &[u8] = include_bytes!("../assets/fonts/NotoSansJP-Regular.otf");

// Window config for macroquad
fn window_conf() -> Conf {
  Conf {
    window_title: "Rendering Text Example".to_owned(),
    window_width: 330,
    window_height: 267,
    high_dpi: true,
    window_resizable: true,
    ..Default::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  // Start by creating a fonts instance to handle all your fonts
  let mut fonts = Fonts::default();

  // Load fonts, the order you load fonts is the order it uses for lookups
  fonts.load_font_from_bytes("Noto Sans", NOTO_SANS).unwrap();
  fonts.load_font_from_bytes("Noto Sans JP", NOTO_SANS_JP).unwrap();

  loop {
    // Draw text
    fonts.draw_text("Nice", 20.0, 0.0, 69.5, Color::from([1.0; 4]));
    fonts.draw_text("良い", 20.0, 89.0, 69.5, Color::from([1.0; 4]));
    fonts.draw_text("Nice 良い", 20.0, 178.0, 69., Color::from([1.0; 4]));

    next_frame().await;
  }
}
