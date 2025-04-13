use iced::{
    Color, Point, Renderer, Size, Theme,
    mouse::Cursor,
    widget::canvas::{self, Path},
};

use crate::Pixel;

pub struct Preview(pub Vec<Pixel>, canvas::Cache);
impl Preview {
    pub fn new(pixels: Vec<Pixel>) -> Self {
        Self(pixels, canvas::Cache::default())
    }
    pub fn request_redraw(&mut self) {
        self.1.clear();
    }
}

impl<Message> canvas::Program<Message> for Preview {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        vec![self.1.draw(renderer, bounds.size(), |frame| {
            let background_path = Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(&background_path, Color::from_rgb8(0x10, 0x10, 0x10));
            for pixel in &self.0 {
                frame.fill_rectangle(
                    Point::new(
                        bounds.width / 2. + pixel.x * bounds.width / 2.,
                        bounds.height / 2. + pixel.y * bounds.height / 2.,
                    ),
                    Size::new(4., 4.),
                    Color::new(pixel.r, pixel.g, pixel.b, 1.),
                );
            }
        })]
    }
}
