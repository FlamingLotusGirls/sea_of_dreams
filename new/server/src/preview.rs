use iced::{
    Color, Point, Renderer, Size, Theme,
    mouse::Cursor,
    widget::canvas::{self, Path},
};

use crate::Elder;

pub struct Preview(pub Vec<Elder>, canvas::Cache);
impl Preview {
    pub fn new(elders: Vec<Elder>) -> Self {
        Self(elders, canvas::Cache::default())
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
            for elder in &self.0 {
                frame.fill_rectangle(
                    Point::new(
                        bounds.width / 2. + elder.crane_light.x * bounds.width / 2.,
                        bounds.height / 2. + elder.crane_light.y * bounds.height / 2.,
                    ),
                    Size::new(4., 4.),
                    Color::new(
                        elder.crane_light.r,
                        elder.crane_light.g,
                        elder.crane_light.b,
                        1.,
                    ),
                );
                frame.fill_rectangle(
                    Point::new(
                        bounds.width / 2. + elder.poofer_both.x * bounds.width / 2.,
                        bounds.height / 2. + elder.poofer_both.y * bounds.height / 2.,
                    ),
                    Size::new(6., 6.),
                    if elder.poofer_both.on || elder.poofer_wide.on || elder.poofer_narrow.on {
                        Color::from_rgba8(235, 225, 52, 1.)
                    } else {
                        Color::new(0., 0., 0., 1.)
                    },
                );
            }
        })]
    }
}
