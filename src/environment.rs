use tetra::graphics::{self, Texture, Color, DrawParams};
use tetra::{Context};
use tetra::math::Vec2;

// Background class
#[derive(Debug)]
pub struct Background {
    image : Texture,
    x : f32,
    y : f32,
    width : f32,
    height : f32,
    scale : f32,
    color : Color,
}


impl Background {

    // Constructor

    #[allow(dead_code)]
    pub fn new(image:Texture, x:f32, y:f32, width:f32, height:f32, scale:f32) -> Self {
        Background { image: image, x: x, y: y, width: width, height: height, scale : scale, color : Color::WHITE}
    }

    // Getters

    #[allow(dead_code)]
    pub fn get_x(&self) -> f32 {
        return self.x;
    }

    #[allow(dead_code)]
    pub fn get_y(&self) -> f32 {
        return self.y;
    }

    #[allow(dead_code)]
    pub fn get_width(&self) -> f32 {
        return self.width;
    }

    #[allow(dead_code)]
    pub fn get_height(&self) -> f32 {
        return self.height;
    }

    // Setters

    #[allow(dead_code)]
    pub fn set_default_color(&mut self, color: Color) {
        self.color = color;
    }

    // Draw

    #[allow(dead_code)]
    pub fn draw(&self, ctx: &mut Context) {

        // Draw default background color
        graphics::clear(ctx, self.color);


        // To seemlessly scroll the image across the screen, the image is drawn 3 times side by side
        // The first image is the middle image - which starts by taking the whole screen
        // The post-image is the image immediately to the right of the first image
        // The pre-image is the image immediately to the left of the first image

        // Draw first image
        self.image.draw(ctx, DrawParams{
            position : Vec2::new(self.x, self.y),
            scale : Vec2::new(self.scale, self.scale),
            origin : Vec2::new(0.0, 0.0),
            rotation: 0.0,
            color: Color::WHITE
        });

        // Draw post-image
        self.image.draw(ctx, DrawParams{
            position : Vec2::new(self.x + self.scale*self.width, self.y),
            scale : Vec2::new(self.scale, self.scale),
            origin : Vec2::new(0.0, 0.0),
            rotation: 0.0,
            color: Color::WHITE
        });

        // Draw pre-image
        self.image.draw(ctx, DrawParams{
            position : Vec2::new(self.x - self.scale*self.width, self.y),
            scale : Vec2::new(self.scale, self.scale),
            origin : Vec2::new(0.0, 0.0),
            rotation: 0.0,
            color: Color::WHITE
        });

    }


    // Background scroll

    pub fn scroll(&mut self) {
        let screen_w = self.width*self.scale;
        let speed = -0.1;

        self.x += speed;
        if self.x <= screen_w*-1.0 { self.x = 0.0; }
        else if self.x >= screen_w { self.x = 0.0; }
    }

}


// Foreground class

pub struct Foreground {
    image : Texture,
    width : f32,
    height : f32,
    x : f32,
    y : f32,
    scale: f32,
}

impl Foreground {

    // Constructor

    #[allow(dead_code)]
    pub fn new(image:Texture, x:f32, y:f32, width:f32, height:f32, scale:f32) -> Self {
        Foreground { image: image, x: x, y: y, width: width, height: height, scale : scale}
    }

    // Getters

    #[allow(dead_code)]
    pub fn get_x(&self) -> f32 {
        return self.x;
    }

    #[allow(dead_code)]
    pub fn get_y(&self) -> f32 {
        return self.y;
    }

    #[allow(dead_code)]
    pub fn get_width(&self) -> f32 {
        return self.width;
    }

    #[allow(dead_code)]
    pub fn get_height(&self) -> f32 {
        return self.height;
    }
    
    // Draw

    #[allow(dead_code)]
    pub fn draw(&self, ctx: &mut Context) {

        // To seemlessly scroll the image across the screen, the image is drawn 3 times side by side
        // The first image is the middle image - which starts by taking the whole screen
        // The post-image is the image immediately to the right of the first image
        // The pre-image is the image immediately to the left of the first image

        // Draw first image
        self.image.draw(ctx, DrawParams{
            position : Vec2::new(self.x, self.y),
            scale : Vec2::new(self.scale, self.scale),
            origin : Vec2::new(0.0, 0.0),
            rotation: 0.0,
            color: Color::WHITE
        });

        // Draw post-image
        self.image.draw(ctx, DrawParams{
            position : Vec2::new(self.x + self.width*self.scale, self.y),
            scale : Vec2::new(self.scale, self.scale),
            origin : Vec2::new(0.0, 0.0),
            rotation: 0.0,
            color: Color::WHITE
        });

        // Draw pre-image
        self.image.draw(ctx, DrawParams{
            position : Vec2::new(self.x - self.width*self.scale, self.y),
            scale : Vec2::new(self.scale, self.scale),
            origin : Vec2::new(0.0, 0.0),
            rotation: 0.0,
            color: Color::WHITE
        });
    }
}