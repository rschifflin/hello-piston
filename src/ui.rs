use conrod::Ui as ConrodUi;
use bit_vec::BitVec;
use glium;
use glium_graphics::{GliumGraphics, GliumWindow, GlyphCache, Texture, TextureSettings};
use glium::texture::{RawImage2d, ClientFormat, UncompressedFloatFormat, MipmapsOption};
use glium::backend::Facade;
use glium::{Rect, Frame};
use conrod;
use screen;
use std::borrow::Cow;

pub const FONT_PATH: &'static str = "./assets/fonts/NotoSans-Regular.ttf";
pub const GLYPH_CACHE_WIDTH: u32 = 1024;
pub const GLYPH_CACHE_HEIGHT: u32 = 1024;

widget_ids! {
  pub struct Ids {
    master,
    left_col,
    middle_col,
    right_col,
    left_text,
    middle_text,
    right_text
  }
}

pub struct Ui {
  pub ui: ConrodUi,
  pub text_texture_cache: Texture,
  pub glyph_cache: conrod::text::GlyphCache,
  pub image_map: conrod::image::Map<glium::Texture2d>,
  pub renderer: conrod::backend::glium::Renderer,
  ids: Ids
}

impl Ui {
  pub fn cache_queued_glyphs(g: &mut GliumGraphics<Frame>, texture: &mut Texture, rect: conrod::text::rt::Rect<u32>, buf: &[u8]) {
    let ref mut inner = texture.0;
    let width = rect.width() * 8;
    let height = rect.height();
    let data_alpha = BitVec::from_bytes(buf)
      .iter()
      .map(|b| {
        if b { 255u8 }
        else { 0u8 }
      })
      .collect::<Vec<u8>>();

    println!("data_alpha: {}, {}", width * height, data_alpha.len());

    /*
    println!("Cache rect:");
    buf.chunks(width as usize).map(|x| println!("{}", x.iter().map(|x| format!("{:08b}", x)).collect::<String>())).count();
    */
    inner.main_level().write(
      Rect {
        left: rect.min.x,
        bottom: rect.min.y,
        width: width,
        height: height
      },
      RawImage2d {
        data: Cow::Owned(data_alpha),
        width: width,
        height: height,
        format: ClientFormat::U8
      }
    );
  }
  pub fn texture_from_image<T>(img: &T) -> &T { img }

  pub fn new(window: &mut GliumWindow) -> Ui {
    let mut ui = conrod::UiBuilder::new([screen::WIDTH as f64, screen::HEIGHT as f64]).build();
    let renderer = conrod::backend::glium::Renderer::new(window).unwrap();
    let (w, h) = window.get_context().get_framebuffer_dimensions();
    let text_texture_cache = {
      let gray_image = vec![(255u8); (w as usize * h as usize)];
      let texture_settings = TextureSettings::new();
      Texture::from_memory_alpha(window, &gray_image, w, h, &texture_settings).unwrap()
    };
    let glyph_cache = conrod::text::GlyphCache::new(w, h, 0.1, 0.1);
    let font_id = ui.fonts.insert_from_file(FONT_PATH).unwrap();
    let image_map = conrod::image::Map::new();
    ui.theme.font_id = Some(font_id);
    let ids = Ids::new(ui.widget_id_generator());

    Ui {
      ui: ui,
      text_texture_cache: text_texture_cache,
      glyph_cache: glyph_cache,
      image_map: image_map,
      renderer: renderer,
      ids: ids
    }
  }

  pub fn update(&mut self) {
    use conrod::{color, widget, Colorable, Positionable, Scalar, Sizeable, Widget};

    let ui = &mut self.ui.set_widgets();
    let ids = &self.ids;

    // Our `Canvas` tree, upon which we will place our text widgets.
    widget::Canvas::new().flow_right(&[
        (ids.left_col, widget::Canvas::new().color(color::BLUE)),
        (ids.middle_col, widget::Canvas::new().color(color::DARK_CHARCOAL)),
        (ids.right_col, widget::Canvas::new().color(color::CHARCOAL)),
    ]).set(ids.master, ui);

    /*
    const DEMO_TEXT: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
        Mauris aliquet porttitor tellus vel euismod. Integer lobortis volutpat bibendum. Nulla \
        finibus odio nec elit condimentum, rhoncus fermentum purus lacinia. Interdum et malesuada \
        fames ac ante ipsum primis in faucibus. Cras rhoncus nisi nec dolor bibendum pellentesque. \
        Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. \
        Quisque commodo nibh hendrerit nunc sollicitudin sodales. Cras vitae tempus ipsum. Nam \
        magna est, efficitur suscipit dolor eu, consectetur consectetur urna.";
*/
    const DEMO_TEXT: &'static str = "Hello world!";
    const PAD: Scalar = 20.0;

    widget::Text::new(DEMO_TEXT)
        .color(color::LIGHT_RED)
        .padded_w_of(ids.left_col, PAD)
        .mid_top_with_margin_on(ids.left_col, PAD)
        .align_text_left()
        .line_spacing(10.0)
        .set(ids.left_text, ui);

    /*
    widget::Text::new(DEMO_TEXT)
        .color(color::LIGHT_GREEN)
        .padded_w_of(ids.middle_col, PAD)
        .middle_of(ids.middle_col)
        .align_text_middle()
        .line_spacing(2.5)
        .set(ids.middle_text, ui);

    widget::Text::new(DEMO_TEXT)
        .color(color::LIGHT_BLUE)
        .padded_w_of(ids.right_col, PAD)
        .mid_bottom_with_margin_on(ids.right_col, PAD)
        .align_text_right()
        .line_spacing(5.0)
        .set(ids.right_text, ui);
    */
  }
}
