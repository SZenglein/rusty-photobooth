extern crate ggez;
extern crate ggez_goodies;

extern crate gphoto2_sys as gphoto;


mod camera_layer;

use ggez::*;

use ggez_goodies::scene::{Scene, SceneSwitch, SceneStack};

use ggez::event::Event;
use ggez::event::Event::*;
use ggez::graphics::Drawable;

use gphoto::{Camera, GPContext, CameraFile};


struct IdleScene {
    next_state: bool,
}

impl IdleScene {
    fn new() -> IdleScene {
        IdleScene {
            next_state: false,
        }
    }
}

impl Scene<SharedState, Event> for IdleScene {
    fn update(&mut self, gameworld: &mut SharedState) -> SceneSwitch<SharedState, Event> {
        if self.next_state {
            SceneSwitch::Push(Box::new(PreviewScene::new()))
        } else {
            SceneSwitch::None
        }
    }

    fn draw(&mut self, gameworld: &mut SharedState, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx);
        graphics::circle(ctx,
                         graphics::DrawMode::Fill,
                         graphics::Point2::new(50.0, 380.0),
                         100.0,
                         2.0)?;

        let text = graphics::Text::new(ctx, "IdleScene", &graphics::Font::default_font().unwrap()).unwrap();

        text.draw(ctx, graphics::Point2::new(30.0, 180.0), 0.0);

        graphics::present(ctx);
        Ok(())
    }

    fn input(&mut self, gameworld: &mut SharedState, event: Event, started: bool) {
        match event {
            MouseButtonUp {
                mouse_btn, x, y, ..
            } => {
                self.next_state = true;
            },
            _ => {},
        }
    }

    fn name(&self) -> &str {
        "IdleScene"
    }
}

struct PreviewScene {
    camera: *mut Camera,
    context: *mut GPContext,
    camera_file: *mut CameraFile,
}

impl PreviewScene {
    fn new() -> PreviewScene {
        let (camera, context) = camera_layer::initialize();
        let camera_file = camera_layer::new_camera_file();
        PreviewScene{
            camera,
            camera_file,
            context,
        }
    }
}

impl Scene<SharedState, Event> for PreviewScene {
    fn update(&mut self, gameworld: &mut SharedState) -> SceneSwitch<SharedState, Event> {
        SceneSwitch::None
    }

    fn draw(&mut self, gameworld: &mut SharedState, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx);
        graphics::circle(ctx,
                         graphics::DrawMode::Fill,
                         graphics::Point2::new(50.0, 380.0),
                         100.0,
                         2.0)?;

        let text = graphics::Text::new(ctx, "PreviewScene", &graphics::Font::default_font().unwrap()).unwrap();

        text.draw(ctx, graphics::Point2::new(30.0, 180.0), 0.0);

        let gphoto_image = camera_layer::get_preview_image(self.camera, self.context, self.camera_file);

        let image = graphics::Image::from_rgba8(ctx, gphoto_image.width() as u16, gphoto_image.height() as u16, &gphoto_image.into_buffer()).unwrap();

        image.draw(ctx, graphics::Point2::new(200.0, 200.0), 0.0);

        graphics::present(ctx);
        Ok(())
    }

    fn input(&mut self, gameworld: &mut SharedState, event: Event, started: bool) {

    }

    fn name(&self) -> &str {
        "PreviewScene"
    }
}


struct CountdownScene {

}

struct CaptureScene {

}

// State shared between all scenes, such as asset cache
struct SharedState {

}

impl SharedState {
    fn new() -> SharedState{
        SharedState{

        }
    }
}

// Make type shortcuts for the scene types we are interested in
type MySceneSwitch = SceneSwitch<SharedState, Event>;
type MySceneStack = SceneStack<SharedState, Event>;
type MyScene = Scene<SharedState, Event>;


pub fn main() {
    let mut file = std::fs::File::open("conf.toml").unwrap();
    let c = conf::Conf::from_toml_file(&mut file).unwrap();

    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();

    let mut scene_stack = MySceneStack::new(ctx, SharedState::new());

    scene_stack.push(Box::new(IdleScene::new()));

//    graphics::set_fullscreen(ctx, true);

    custom_events_loop(ctx, &mut scene_stack).unwrap();
}



fn custom_events_loop(ctx: &mut Context, scene_stack: &mut MySceneStack) -> GameResult<()> {
    let mut event_pump = ctx.sdl_context.event_pump()?;

    let mut continuing = true;
    while continuing {
        ctx.timer_context.tick();

        for event in event_pump.poll_iter() {
            ctx.process_event(&event);

            // forward event to scenestack
            scene_stack.input(event.clone(), true);

            match event {
                Quit { .. } => {
                    continuing = false;
                    // println!("Quit event: {:?}", t);
                }
                KeyDown {
                    keycode,
                    keymod,
                    repeat,
                    ..
                } => {
                    if let Some(key) = keycode {
                        match key {
                            ggez::event::Keycode::Escape => continuing = false,
                            _ => {},
                        }
//                        state.key_down_event(ctx, key, keymod, repeat)
                    }
                }
                KeyUp {
                    keycode,
                    keymod,
                    repeat,
                    ..
                } => {
                    if let Some(key) = keycode {
//                        state.key_up_event(ctx, key, keymod, repeat)
                    }
                }
                TextEditing {
                    text,
                    start,
                    length,
                    ..
                } => {
//                    state.text_editing_event(ctx, text, start, length)
                },
                TextInput { text, .. } => {
//                    state.text_input_event(ctx, text)
                },
                MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
//                    state.mouse_button_down_event(ctx, mouse_btn, x, y)
                },
                MouseButtonUp {
                    mouse_btn, x, y, ..
                } => {
//                    state.mouse_button_up_event(ctx, mouse_btn, x, y)
                },
                MouseMotion {
                    mousestate,
                    x,
                    y,
                    xrel,
                    yrel,
                    ..
                } => {
//                    state.mouse_motion_event(ctx, mousestate, x, y, xrel, yrel);
                }
                _ => {}
            }
        }
//        state.update(ctx)?;
//        state.draw(ctx)?;
        scene_stack.update();
        scene_stack.draw(ctx);
    }

    Ok(())
}

