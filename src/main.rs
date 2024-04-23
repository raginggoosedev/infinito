use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

// Context: a struct that holds the global "state" managed by the framework

// State: for the game loop
struct GameState {}
impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Background color of window
        graphics::clear(ctx, Color::rgb(0.992, 0.784, 0.329));
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Infinito", 800, 600)
    .quit_on_escape(true)
    .build()?
    .run(|_| Ok(GameState {}))
}
