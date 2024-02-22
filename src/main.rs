use std::time::{Duration, Instant};

use ggez::{
    graphics::{Canvas, Color, DrawParam, Drawable, Mesh, Rect},
    *,
};
use mint::{Point2, Vector2};
use rand::{thread_rng, Rng};
fn main() {
    let mut state = State {
        dt: std::time::Duration::new(0, 0),
        snake: Snake::new(vec![[0., 0.]]),
        food: Food::new([300., 300.]),
        instant: Instant::now(),
        score: 0,
    };

    let mut c = conf::Conf::new();
    c.window_mode.width = 1600.0;
    c.window_mode.height = 1400.0;
    let (ctx, event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state);
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
struct State {
    dt: Duration,
    snake: Snake,
    food: Food,
    instant: std::time::Instant,
    score: i32,
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        let mut is_gameover = false;
        if self.instant.elapsed() > Duration::new(0, 200_000_000) {
            let mut prev_coordinates = self.snake.positions[0].clone();
            self.snake.prev_tail_coordinates =
                self.snake.positions[self.snake.positions.len() - 1].clone();

            //移动头
            match self.snake.direction {
                Direction::UP => self.snake.positions[0][1] -= 50.,

                Direction::DOWN => self.snake.positions[0][1] += 50.,

                Direction::LEFT => self.snake.positions[0][0] -= 50.,

                Direction::RIGHT => self.snake.positions[0][0] += 50.,
            }
            let new_head_coordinates = self.snake.positions[0].clone();

            //移动身体
            for body in self.snake.positions.iter_mut().skip(1) {
                if *body == new_head_coordinates {
                    is_gameover = true;
                    break;
                }
                let temp = body.clone();
                body[0] = prev_coordinates[0];
                body[1] = prev_coordinates[1];
                prev_coordinates = temp;
            }
            self.instant = Instant::now();
        }

        //判断是否吃到食物
        if self.snake.positions[0][0] == self.food.coordinate[0]
            && self.snake.positions[0][1] == self.food.coordinate[1]
        {
            self.score += 1;
            self.snake.positions.push(self.snake.prev_tail_coordinates);

            let mut rng = thread_rng();
            self.food.coordinate = [
                (rng.gen_range(0..32) * 50) as f32,
                (rng.gen_range(0..28) * 50) as f32,
            ];
        }

        //判断是否出界
        if self.snake.positions[0][0] >= 1600.
            || self.snake.positions[0][0] < 0.
            || self.snake.positions[0][1] >= 1400.
            || self.snake.positions[0][1] < 0.
        {
            is_gameover = true;
        }

        if is_gameover {
            self.snake.positions = vec![[0., 0.]];
            self.snake.direction = Direction::DOWN;
            self.score = 0;
            is_gameover = false;
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::from([0., 0., 0., 1.]));
        self.snake.draw(&mut canvas, ctx)?;
        self.food.draw(&mut canvas)?;
        let score_txt = graphics::Text::new(format!("Score: {}", self.score));
        canvas.draw(
            &score_txt,
            DrawParam::default()
                .color(Color::WHITE)
                .scale(Vector2 { x: 3.0, y: 3.0 }),
        );
        canvas.finish(ctx)?;
        Ok(())
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        match input.keycode {
            Some(input::keyboard::KeyCode::Right) => self.snake.direction = Direction::RIGHT,

            Some(input::keyboard::KeyCode::Left) => self.snake.direction = Direction::LEFT,

            Some(input::keyboard::KeyCode::Up) => self.snake.direction = Direction::UP,

            Some(input::keyboard::KeyCode::Down) => self.snake.direction = Direction::DOWN,

            _ => {}
        };

        Ok(())
    }
}

struct Food {
    coordinate: [f32; 2],
}
impl Food {
    fn new(coordinate: [f32; 2]) -> Self {
        Self { coordinate }
    }
    fn draw(&mut self, canvas: &mut Canvas) -> GameResult {
        canvas.draw(
            &graphics::Quad,
            DrawParam::default()
                .color(graphics::Color::RED)
                .scale([50., 50.])
                .dest(self.coordinate),
        );
        Ok(())
    }
}

struct Snake {
    positions: Vec<[f32; 2]>,
    prev_tail_coordinates: [f32; 2],
    direction: Direction,
}
impl Snake {
    fn new(positions: Vec<[f32; 2]>) -> Self {
        Snake {
            prev_tail_coordinates: positions[0].clone(),
            positions,
            direction: Direction::DOWN,
        }
    }
    fn draw(&mut self, canvas: &mut Canvas, ctx: &mut Context) -> GameResult {
        let head_point = Point2 {
            x: self.positions[0][0] + 25.,
            y: self.positions[0][1] + 25.,
        };
        let head = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            head_point,
            25.,
            1.,
            graphics::Color::GREEN,
        )?;
        head.draw(canvas, DrawParam::default());
        for body in self.positions.iter().skip(1) {
            canvas.draw(
                &graphics::Quad,
                DrawParam::default()
                    .color(graphics::Color::GREEN)
                    .scale([50., 50.])
                    .dest(*body),
            );
        }

        Ok(())
    }
}
