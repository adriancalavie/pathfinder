use druid::RenderContext;
use druid::kurbo::Circle;
use druid::kurbo::Line;
use druid::Color;
use druid::Point;
use druid::Widget;

use alg::utils::Map;
use algorithms as alg;

pub struct MapWidget {
    pub map: Map,
}

impl MapWidget {
    pub fn from_map(map: Map) -> Self {
        MapWidget { map }
    }
}

impl Widget<String> for MapWidget {
    fn event(
        &mut self,
        _ctx: &mut druid::EventCtx,
        _event: &druid::Event,
        _data: &mut String,
        _env: &druid::Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut druid::LifeCycleCtx,
        _event: &druid::LifeCycle,
        _data: &String,
        _env: &druid::Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut druid::UpdateCtx,
        _old_data: &String,
        _data: &String,
        _env: &druid::Env,
    ) {
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &String,
        _env: &druid::Env,
    ) -> druid::Size {
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        //
        // bx.max() returns the maximum size of the widget. Be careful
        // using this, since always make sure the widget is bounded.
        // If bx.max() is used in a scrolling widget things will probably
        // not work correctly.
        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = ctx.window().get_size();

            bc.constrain(size)
        }
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &String, _env: &druid::Env) {
        let size = ctx.size();

        self.map.iter().for_each(|node| {
            let circle = Circle::new(
                Point::new(
                    alg::utils::map(node.point.x, 0., size.width - 100.),
                    alg::utils::map(node.point.y, 0., size.height - 100.),
                ),
                20.,
            );
            ctx.fill(circle, &Color::GRAY);

            node.neighbors.iter().for_each(|neighbor_point| {
                let line = Line::new(
                    circle.center,
                    Point::new(
                        alg::utils::map(neighbor_point.x, 0., size.width - 100.),
                        alg::utils::map(neighbor_point.y, 0., size.height - 100.),
                    ),
                );
                ctx.stroke(line, &Color::GRAY, 1.);
            });
        });
    }
}
