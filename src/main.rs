#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 1280/16;
const SCREEN_HEIGHT: i32 = 720/16;
const FRAME_DURATION: f32 = 16.66;

enum GameMode {
    Menu,
    Playing,
    End,
}


struct State {
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    mode: GameMode,
    score: i32,
}

impl State {
    fn new() -> Self {
        Self {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::Menu,
            score: 0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.mode = GameMode::Playing,
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        ctx.set_active_console(1);
        ctx.cls();
        self.player.render(ctx);
        ctx.set_active_console(0);

        ctx.print(0, 0, "Press SPACE to flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x);

        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(2, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    gap_size: i32,
}

impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            gap_size: i32::max(2, 20 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.gap_size / 2;

        // Draw the top half of the obstacle
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, GRAY, BLACK, 179);
        }

        // Draw the bottom half of the obstacle
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, GRAY, BLACK, 179/*to_cp437('|')*/);
        }
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.gap_size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < (self.gap_y - half_size);
        let player_below_gap = player.y > (self.gap_y + half_size);

        does_x_match && (player_above_gap || player_below_gap)
    }
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
    frames : [u16; 6],
    current_frame: usize ,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
            frames :  [ 64, 1, 2, 3, 2, 1 ],
            current_frame : 0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        //ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
        ctx.set_fancy(PointF::new(2.0, self.y as f32), 1, Degrees::new(0.0), PointF::new(2.0, 2.0), WHITE, NAVY, self.frames[self.current_frame]);
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 1.0 {
            self.velocity += 0.5;
        }

        self.x += 1;
        self.y += self.velocity as i32; // Explicitly cast self.velocity to i32

        if self.y < 0 {
            self.y = 0;
        }

        self.current_frame = (self.current_frame + 1) % 6; 
    }

    fn flap(&mut self) {
        self.velocity = -4.0;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "C:\\Users\\muhar\\Desktop\\RustGames\\flappy\\assets\\flappy32.png")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT,"C:\\Users\\muhar\\Desktop\\RustGames\\flappy\\assets\\flappy32.png")
        //.with_fullscreen(true)
        .with_font("C:\\Users\\muhar\\Desktop\\RustGames\\flappy\\assets\\flappy32.png", 32, 32)
        .with_title("Flappy Dragon")
        .with_tile_dimensions(16, 16)
        .build()?;


    main_loop(context, State::new())
}
