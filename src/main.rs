use tetra::graphics::{self, Color, Texture, DrawParams};
use tetra::{Context, ContextBuilder, State};
use tetra::input::{self, Key};
use tetra::math::Vec2;

// Context: a struct that holds the global "state" managed by the framework

// State: for the game loop
struct GameState {
    background : Texture,
    foreground : Texture,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let background = Texture::new(ctx, "./resources/sky.png")?; // background image
        let foreground = Texture::new(ctx, "./resources/ground.png")?; // foreground image

        Ok(GameState { background, foreground })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Background color of window
        graphics::clear(ctx, Color::rgb(1.0, 0.611, 0.318));

        // Draw background image
        self.background.draw(ctx, DrawParams{
            position : Vec2::new(0.0, 0.0),

            scale : Vec2::new(
                SCREEN_W / (self.background.width() as f32), 
                SCREEN_H / (self.background.height() as f32)),
                
            origin : Vec2::new(0.0, 0.0),
            rotation : 0.0,
            color : Color::WHITE
        });

        // Draw foreground image
        self.foreground.draw(ctx, DrawParams{
            position : Vec2::new(0.0, 0.0),

            scale : Vec2::new(
                SCREEN_W / (self.foreground.width() as f32),
                SCREEN_H / (self.foreground.height() as f32)),

            origin : Vec2::new(0.0, 0.0),
            rotation : 0.0,
            color : Color::WHITE
        });

        Ok(())
    }
}

const SCALE: f32 = 0.6;
const SCREEN_W: f32 = 1920.0*SCALE;
const SCREEN_H: f32 = 1080.0*SCALE;

fn main() -> tetra::Result {

    ContextBuilder::new("Infinito", SCREEN_W as i32, SCREEN_H as i32)
    .quit_on_escape(true)
    .build()?
    .run(GameState::new)
}
