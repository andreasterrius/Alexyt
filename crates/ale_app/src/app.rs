use std::slice::Windows;

use ale_opengl::old::opengl::SimpleRenderTasks;
use ale_opengl::{ale_opengl_clear_render, ale_opengl_clear_render_color};
use ale_resources::resources;
use ale_resources::resources::Resources;
use ale_ui::element::{Panel, RenderResources};
use ale_window::backend;
use ale_window::window::Window;
use ale_world::components::Renderable;
use ale_world::visitor::RenderableVisitor;
use ale_world::world::World;

use crate::engine::Engine;
use crate::visitor::WorldVisitor;
use crate::{AppError, DisplaySetting, FixedStep, init_term, WorldTick};

pub trait Genesis {
  fn register_components(&self, world: &mut World);

  fn init(&self, engine: &mut Engine, world: &mut World) -> Result<(), AppError>;
}

pub struct App {
  genesis: Box<dyn Genesis>,
}

impl App {
  pub fn new<T: Genesis + 'static>(init: T) -> App {
    App {
      genesis: Box::new(init),
    }
  }

  pub fn run(mut self) {
    self.run_app_loop().unwrap();
  }

  fn run_app_loop(&mut self) -> anyhow::Result<()> {
    init_term();

    let mut tick = WorldTick::FixedStep(FixedStep::new(0.01f32));
    let mut world = World::new();
    let mut engine = Engine::new()?;

    self.genesis.register_components(&mut world);
    self.genesis.init(&mut engine, &mut world)?;

    while engine.windows.len() > 0 {
      engine.windows.poll_inputs();

      tick.prepare_tick();
      let delta_time = tick.delta_time();

      while tick.should_tick() {
        tick.tick();
        world.fixed_tick(tick.delta_time());
      }

      world.tick(delta_time);
      self.render(&mut engine, &mut world);

      engine.windows.cleanup();
      println!("{}", engine.windows.len());
    }

    Ok(())
  }

  fn render(&mut self, engine: &mut Engine, world: &mut World) {
    let mut world_visitor = WorldVisitor::new();
    world.visit_renderables(&mut world_visitor);
    world.visit_cameras(&mut world_visitor);

    for window in &mut engine.windows.iter_mut() {
      if window.is_hidden {
        continue
      }
      window.make_current();

      ale_opengl_clear_render();
      window.swap_buffers();
    }
  }
}
