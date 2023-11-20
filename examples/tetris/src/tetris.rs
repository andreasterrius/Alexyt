use ale_data::channel::Sender;
use ale_data::entity::Entity;
use ale_data::indexmap::Id;
use ale_data::timer::{Recurrence, Timer};
use ale_data::wire_component;
use ale_input::Action::Press;
use ale_input::Input;
use ale_input::Key::{Left, Right};
use ale_math::color::Color;
use ale_math::Vector2;
use ale_opengl::renderer::task::{RenderTask, Sprite};
use ale_render::component::Renderable;
use ale_world::components::{Inputable, Spawnable, Tickable};
use ale_world::event::world::WorldCommand;
use ale_world::world::World;

use crate::template::{BlockTypeId, Templates};
use crate::tetris::Block::NotFilled;

const TICK_TIME: f32 = 2.0;
const HIDDEN_ROW_GRID_SIZE: usize = 4;
const ROW_GRID_SIZE: usize = 28;
const COLUMN_GRID_SIZE: usize = 10;
const BLOCK_SIZE: Vector2<usize> = Vector2::new(10, 10);

#[derive(Clone)]
pub enum Block {
  Filled(Color),
  NotFilled,
}

pub struct TetrisInfo {
  pub block_type: BlockTypeId,
  pub rotation_type: usize,
  pub position: Vector2<usize>,
}

pub struct GameCoordinator {
  pub id: Id<Entity>,
  pub templates: Templates,
  pub wc_sender: Sender<WorldCommand>,

  // Arena state
  pub arena: Vec<Vec<Block>>,
  pub selected: Option<TetrisInfo>,

  pub tetris_timer: Timer,
}

impl GameCoordinator {
  pub fn register_components(world: &mut World) {
    world.register_components(&[
      wire_component!(dyn Spawnable, GameCoordinator),
      wire_component!(dyn Tickable, GameCoordinator),
      wire_component!(dyn Inputable, GameCoordinator),
      wire_component!(dyn Renderable, GameCoordinator),
    ]);
  }

  pub fn new(wc_sender: Sender<WorldCommand>) -> GameCoordinator {
    let arena = vec![vec![NotFilled; COLUMN_GRID_SIZE]; ROW_GRID_SIZE + HIDDEN_ROW_GRID_SIZE];
    let mut templates = Templates::new();
    templates.add_all();

    GameCoordinator {
      id: Id::new(),
      templates,
      wc_sender,
      arena,
      selected: None,
      tetris_timer: Timer::new(TICK_TIME, Recurrence::Forever),
    }
  }
}

impl Tickable for GameCoordinator {
  fn fixed_tick(&mut self, delta_time: f32) {
    // do nothing
  }

  fn tick(&mut self, delta_time: f32) {
    //
    if self.selected.is_none() {
      let random = self.templates.random_one_piece();
      self.selected = Some(TetrisInfo {
        block_type: random.block_type,
        rotation_type: random.rotation_type,
        position: Vector2::new(COLUMN_GRID_SIZE / 2, 0),
      });
    }

    if self.tetris_timer.tick_and_check(delta_time) {
      match &mut self.selected {
        None => {}
        Some(tetris_info) => {
          let blocks = self
            .templates
            .blocks
            .get(&tetris_info.block_type)
            .expect("unexpected block type id")
            .get(tetris_info.rotation_type)
            .expect("unexpected rotation type");

          // check lowest block
          let mut lowest = tetris_info.position.y;
          for row in 0..blocks.len() {
            for column in 0..blocks[row].len() {
              // clean up the old blocks
              self.arena[tetris_info.position.y + column][tetris_info.position.x + row] = Block::NotFilled;

              if blocks[row][column] == 0 {
                continue;
              }
              lowest = usize::max(lowest, tetris_info.position.y + column);
            }
          }

          // place
          if (lowest + 1 < ROW_GRID_SIZE) {
            tetris_info.position.y += 1;

            // set new block as 1
          } else {
            // need to place block
          }

          for row in 0..blocks.len() {
            for column in 0..blocks[row].len() {
              if blocks[column][row] == 0 {
                continue;
              }
              self.arena[tetris_info.position.y + column][tetris_info.position.x + row] = Block::Filled(Color::red());
            }
          }

          // destroy lines
        }
      }
    }
  }
}

impl Spawnable for GameCoordinator {
  fn on_spawn(&mut self) {}

  fn on_kill(&mut self) {}

  fn id(&self) -> Id<Entity> {
    self.id
  }
}

impl Inputable for GameCoordinator {
  fn input(&mut self, input: Input) {
    match input {
      Input::Key(Left, _, Press, _) => {
        println!("left is pressed");
      }
      Input::Key(Right, _, Press, _) => {
        println!("right is pressed");
      }
      _ => {}
    }
  }
}

impl Renderable for GameCoordinator {
  fn get_render_tasks(&mut self) -> Vec<RenderTask> {
    let mut renderables = vec![];
    for (rowIndex, row) in self.arena.iter().enumerate() {
      for (columnIndex, block) in row.iter().enumerate() {
        match block {
          Block::Filled(color) => {
            renderables.push(RenderTask::Sprite(Sprite {
              texture_sprite: None,
              color: *color,
              position: Vector2::new((columnIndex * BLOCK_SIZE.x) as f32, (rowIndex * BLOCK_SIZE.y) as f32),
              size: Vector2::new(BLOCK_SIZE.x as f32, BLOCK_SIZE.y as f32),
            }));
          }
          NotFilled => {} //just skip
        }
      }
    }

    renderables
  }
}
