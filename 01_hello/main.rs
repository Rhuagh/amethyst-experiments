extern crate amethyst;
extern crate remawin;
extern crate glutin;
extern crate time;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use amethyst::{Application, State, Trans};
use amethyst::asset_manager::AssetManager;
use amethyst::ecs::World;
use amethyst::gfx_device::DisplayConfig;
use amethyst::renderer::Pipeline;
use amethyst::config::Config;
use amethyst::WindowEvent;

use input::{InputContext, Action};

mod input;
mod input_mapper;

struct Hello;

impl State for Hello {
    fn on_start(&mut self, _ : &mut World, _ : &mut AssetManager, pipe : &mut Pipeline) {
        println!("begin");
        use amethyst::renderer::Layer;
        use amethyst::renderer::pass::Clear;

        let clear_layer = Layer::new("main", vec![Clear::new([0.0, 0.0, 0.0, 1.0])]);
        pipe.layers = vec![clear_layer];
    }

    fn handle_events(&mut self,
                     events: &[WindowEvent],
                     world: &mut World,
                     _: &mut AssetManager,
                     _: &mut Pipeline)
                     -> Trans {
        let mut input = world.write_resource::<input_mapper::AmethystEventMapper<Action, InputContext>>();
        let mapped_events = input.process(&events.to_vec());
        if mapped_events.len() > 0 {
            println!("{:?}", mapped_events);
        }
        for me in mapped_events {
            match me {
                remawin::Event::Controller(remawin::ControllerEvent::Action(Action::Exit, _)) |
                remawin::Event::Window(remawin::WindowEvent::Close) => {
                    return Trans::Quit;
                },
                _ => ()
            }
        }
        Trans::None
    }
}

fn main() {
    let path = format!("{}/01_hello/resources/config.yml",
                       env!("CARGO_MANIFEST_DIR"));
    let input_path = format!("{}/01_hello/resources/bindings.ron",
                       env!("CARGO_MANIFEST_DIR"));
    let cfg = DisplayConfig::load(path);
    let dim = cfg.dimensions.unwrap().clone();
    let mut game = Application::build(Hello, cfg).done();
    let mut event_mapper = input_mapper::AmethystEventMapper::<Action, InputContext>::new(
        (dim.0 as f64, dim.1 as f64));
    event_mapper.remapper_mut()
        .with_bindings_file(&input_path)
        .activate_context(&InputContext::Default, 1);
    game.world_mut().add_resource(event_mapper);
    game.run();
}
