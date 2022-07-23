use ale_camera::CameraRenderInfo;
use ale_opengl::renderer;
use ale_world::components::{Camera, Renderable, Tick};
use ale_world::visitor;
use ale_world::visitor::{CameraVisitor, RenderableVisitor, Visitor, VisitorMut};

pub struct WorldVisitor {
  pub render_task: Vec<renderer::task::Task>,
  pub camera_render_info: Vec<CameraRenderInfo>,
}

impl WorldVisitor {
  pub fn new() -> WorldVisitor {
    WorldVisitor { render_task: vec![], camera_render_info: vec![] }
  }

  pub fn take(self) -> (Vec<renderer::task::Task>, Vec<CameraRenderInfo>) {
    (self.render_task, self.camera_render_info)
  }
}

impl RenderableVisitor for WorldVisitor {
  fn visit(&mut self, renderable: &mut dyn Renderable) {
    self.render_task.push(renderable.get_render_task());
  }
}

impl CameraVisitor for WorldVisitor {
  fn visit(&mut self, camera: &mut dyn Camera) {
    self.camera_render_info.push(camera.get_camera_info());
  }
}

pub struct TickVisitor {
  pub delta_time: f32,
}

impl VisitorMut<'_, dyn Tick> for TickVisitor {
  fn visit(&mut self, component: &mut (dyn Tick + 'static)) {
    component.tick(self.delta_time)
  }
}

pub struct FixedTickVisitor {
  pub delta_time: f32,
}

impl VisitorMut<'_, dyn Tick> for FixedTickVisitor {
  fn visit(&mut self, component: &mut (dyn Tick + 'static)) {
    component.fixed_tick(self.delta_time)
  }
}
