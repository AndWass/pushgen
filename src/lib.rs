mod begin_end;
mod dedup;
mod filter;
mod stage;
mod stage_ext;
mod take_skip;
mod then;
mod transform;

pub use begin_end::*;
pub use dedup::Dedup;
pub use filter::Filter;
pub use stage::*;
pub use stage_ext::*;
pub use take_skip::{Skip, Take};
pub use then::Then;
pub use transform::Transform;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter() {
        let mut output = Vec::<i32>::new();
        let mut pipe = crate::Begin::<i32>::new()
            .filter(|x| x % 2 == 0)
            .end(|v| {
                output.push(v);
                true
            });

        for v in 0..10 {
            pipe.process(v);
        }

        println!("{:?}", output);
    }
}
