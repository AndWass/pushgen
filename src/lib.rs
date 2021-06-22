mod stage;
mod stage_ext;

pub mod structs;

pub use stage::{InputStage, InputOutputStage};
pub use stage_ext::StageExt;

pub fn begin<T>() -> structs::Begin<T> {
    structs::Begin::new()
}
