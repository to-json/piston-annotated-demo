/*
 * This is a super simple example toy in Piston. It renders a square that can
 * be controlled by the arrow keys. This is to demonstrate getting to the
 * point where we can begin to focus on game development rather than fighting
 * with the libraries. This heavily commented version of the code seeks to
 * make jumping in and playing relatively easy.
 */

/* 
 * This is how we import external libraries in rust. Crates represent 
 * repositories of rust code that will compiled and linked when we run
 * `cargo build`. This set of crates gives us a nice baseline set of tools
 * for our first forays into game dev
 */
// graphics deals, predictably, with drawing to the screen
extern crate graphics;
// input provides structs and methods for keyboard and mouse handlers
extern crate input;
// piston is the core engine
extern crate piston;
/*
 * Piston attempts to modularize as much code as possible, including the
 * rendering engine. SDL2 is a well known and widely used set of libraries
 * for game development, typically used with C++. This crate lets us use SDL2
 * to deliver our game. Conceptually, with the way piston has been designed,
 * we should be able to swap this out underneath an existing game, making 
 * porting woes a thing of the past! Of course, nothing is actually this easy
 * in software, but we can dream.
 */
extern crate sdl2_window;
// SDL itself supports multiple renderers; here we elect to use OpenGL
extern crate opengl_graphics;
// This provides constants to help designate which version of OpenGL we use
extern crate shader_version;
// This provides our event loop
extern crate event;

/*
 * In many languages we can require a library and *BAM*, there's the whole
 * dang thing, right up in our global namespace. Rust does not default to
 * this sort of behaviour. Instead, when we'd like to use a particular 
 * struct, we call the aptly named Use keyword. This allows us to keep our
 * namespace clean. While crate declarations can be made once per project
 * use declarations are on a per-file/per-module basis (I think) so as to
 * ensure that all the code you need to know about is visible to you 
 * wherever you are in the code base.
 */

// The Sdl2Window is where we draw all our stuff
use sdl2_window::Sdl2Window;
// Gl is the way Sdl2Window draws on itself
use opengl_graphics::Gl;
// OpenGL_3_2 specifies the version of OpenGL we draw with
use shader_version::opengl::OpenGL_3_2;
// We use a refcell in main to wrap the OpenGL window because the event loop
// has several methods dependent on it. I'll write more about it down there.
use std::cell::RefCell;
// When we have a RenderEvent, it passes RenderArgs. We want this struct so
// that we can make functions that take one. Same idea for UpdateArgs
use piston::{
    RenderArgs,
    UpdateArgs,
};
// Button is an Enum that provides our Keyboard and Mouse event types.
// We're not handling Mouse events in this code.
use input::{
    Button,
    Keyboard
};
/*
 * Context is the in memory representation of our drawable area. I might call
 * this 'canvas' in future work to help my brain analogize it to HTML5
 * or Processing style canvases.
 * AddRectangle and AddColor give us tools to draw on the canvas; there are 
 * numerous others provided in Piston::graphics
 * Draw is the context method that actually does the work of drawing our
 * beautiful masterpieces.
 */
use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Draw,
};
/*
 * Events provides our event handlers.
 * Events itself provides the big enumerator of event types
 * RenderEvent happens once a frame and where one normally handles drawing
 * UpdateEvent also happens once per frame, and this is where we modify state
 * PressEvents happen when a key or mouse button is pressed.
 */
use event::{
    Events,
    Window,
    RenderEvent,
    UpdateEvent,
    PressEvent,
};

/*
 * Our App struct contains our game state. We define render and update
 * on it as methods for our convenience, though they could easily be
 * pure functions if we so chose.
 * The drawing backend gl was provided by the example code, and we use it
 * when we execute the draw method
 * expand is a simple variable to power the spacebar behaviour and to
 * demonstrate the update method
 * player is an additional struct to represent the current position of
 * the player in the game world.
 */

pub struct App {
    gl: Gl,       // OpenGL drawing backend.
    expand: f64, // Rotation for the square.
    player: Player
}
// Here's the player struct declaration. Incidentally, this shows one of the
// niceties of (most) compiled languages; the declaration being after the 
// first use doesn't matter.
pub struct Player {
    x: f64,
    y: f64
}
/*
 * Here's where we define methods on the App struct. Rust does not explicitly
 * support classes, but we can attach functions to structs as methods, such
 * that we can simulate them. We actually get more versatility than with 
 * class based OO because of Rust's traits system, and because we don't have 
 * classes, we dont have the wretched mess that is inheritance.
 */
impl App {
    // The render method is what we call to draw a frame. If I understand
    // correctly, the event loop ensures that this is done 60 times per
    // second.
    fn render<W: Window>(&mut self, _: &mut W, args: &RenderArgs) {
        // We create a local variable to represent our canvas
        let context = &Context::abs(args.width as f64, args.height as f64);
        /*
         * This part is kinda cool; The context struct carries a set of fluent
         * methods; that is, methods that return an object of the same type.
         * We're able to chain methods on the context to pull together a 
         * representation of what we want to put on the canvas. Then we can
         * call draw, passing in a mutable representation of the window to
         * actually draw on.
         */
        // In this one, we simply paint the whole thing grey. Because we have
        // no shapes, the color is assigned to the whole canvas.
        context.rgba(0.6,0.6,0.6,1.0).draw(&mut self.gl);

        /*
         * Here we build the rectangle that represents our player.
         * rect takes four arguments; the x and y coordinates of the top
         * left corner of the rectangle, followed by the width and height.
         * We're able to reference the Player struct we defined earlier for
         * the position, and the width and height are static values.
         * We modify these by the 'expand' attribute we tacked on to the state
         * above. We add it to the width and height to grow the box, and 
         * subtract half of it from the coordinates so that the expansion is 
         * evenly distributed rather than emitting exclusively right and down
         * from the shape. Finally, we assign it a color (red) and draw it to
         * the canvas.
         */
        context
            .rect((self.player.x - (self.expand / 2.0)), 
                  (self.player.y - (self.expand / 2.0)), 
                  (self.expand + 10.0), 
                  (self.expand + 10.0))
            .rgba(1.0, 0.0, 0.0,1.0)
            .draw(&mut self.gl);
    }
    
    // Here, we shrink the value of expand every frame if it's set, so as to
    // make the player square shrink back to normal.
    fn update<W: Window>(&mut self, _: &mut W, args: &UpdateArgs) {
        if self.expand > 0.0 { self.expand -= 1.0 };
    }
}
/*
 * Here is our keyhandler. We'd probably benefit in minor ways from having
 * this be a method on the app, but I wanted to experiment with passing
 * bare functions as arguments. Then it turned out that I needed a closure
 * to do partial application anyway, and I had to do partial application
 * in order to take the right set of arguments from the event handler.
 * C'est la vie.
 */
fn handleKey(key: Button, app: &mut App) { 
    match key {
        /*
         * Here are our movement controls. Because we use Cartesian 
         * coordinates to describe our world, we simply add a value
         * to the x or y coordinate to represent movement; x for lateral
         * movement, y for vertical movement.
         */
        Keyboard(input::keyboard::Up) => { app.player.y -= 10.0 }
        Keyboard(input::keyboard::Down) => { app.player.y += 10.0 }
        Keyboard(input::keyboard::Left) => { app.player.x -= 10.0 }
        Keyboard(input::keyboard::Right) => { app.player.x += 10.0 }
        // Space adds 10 to the expand value to give the impression of
        // a player 'power' or something. I mostly just wanted to do
        // something more than just movement.
        Keyboard(input::keyboard::Space) => { app.expand += 10.0 }
        /* Rust makes you match all possibilities and doesn't have nil.
         * Aww yiss.
         * When matching, _ is basically else.
         * Empty closure is a no-op.
         */
        _ => {}
    }
}

fn main() {
    /*
     * I love this bit because it's all lets and then a loop.
     * Let there be a window
     * Let there be a player
     * Let there be a world
     * Let there be a mutable container for the window
     * For now we LOOP
     */
    let window = Sdl2Window::new(
        OpenGL_3_2,
        piston::WindowSettings::default()
    );

    let mut player = Player { x: 50.0, y: 50.0 };
    let mut app = App { gl: Gl::new(OpenGL_3_2), expand: 0.0, player: player };

    let window = RefCell::new(window);
    /*
     * The event loop has several traits it depends upon, that are all 
     * implemented against the refcell type. If I understand correctly,
     * this is to facillitate the multiple backends approach referenced
     * earlier. The refcell type provides an interface to mutate it's 
     * contents. Even if they would normally be immutable! Ugh! But that's
     * an important trait for the display to have; it's hard to do IO 
     * without having effects on the world.
     *
     * Because we've wrapped window in a RefCell, we need to unpack that 
     * refcell to pass window to the app in the event handlers. Fortunately,
     * refcell provides borrow_mut and deref_mut to get us a mutable pointer
     * into it's contents.
     */
    for e in Events::new(&window) {
        // Was there a key pressed? Handle that!
        e.press(|key| handleKey(key, &mut app));
        // Is it time for a new frame? Render that!
        e.render(|r| app.render(window.borrow_mut().deref_mut(), r));
        // Did a frame just get rendered for this world? Update that!
        e.update(|u| app.update(window.borrow_mut().deref_mut(), u));
    }
}
