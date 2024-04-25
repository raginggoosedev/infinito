use tetra::graphics::{Texture};
use tetra::{Context, ContextBuilder, State};
//use tetra::input::{self, Key};

mod environment;
use environment::{Background, Foreground};

// Context: a struct that holds the global "state" managed by the framework

// State: for the game loop
struct GameState {
    background : Background,
    foreground : Foreground,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let img = Texture::new(ctx, "./resources/sky.png")?; // background image
        let background = Background::new(img, 0.0, 0.0, 3840f32, 2160f32, SCREEN_W/3840f32); // image, x, y, width, height, scale

        let img = Texture::new(ctx, "./resources/ground.png")?; // foreground image
        let foreground = Foreground::new(img, 0.0, 0.0, 3840f32, 2160f32, SCREEN_W/3840f32); // image, x, y, width, height, scale

        Ok(GameState { background, foreground})
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {

        self.background.draw(ctx);
        self.background.scroll();

        self.foreground.draw(ctx);

        Ok(())
    }

}

const SCALE_FACTOR: f32 = 0.6;
const SCREEN_W: f32 = 1920.0*SCALE_FACTOR;
const SCREEN_H: f32 = 1080.0*SCALE_FACTOR;

fn main() -> tetra::Result {

    ContextBuilder::new("Infinito", SCREEN_W as i32, SCREEN_H as i32)
    .quit_on_escape(true)
    .build()?
    .run(GameState::new)
}
