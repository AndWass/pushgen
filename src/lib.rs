mod begin_end;
mod stage;
mod stage_ext;
mod filter;
mod transform;
mod then;
mod take_skip;

pub use begin_end::*;
pub use stage::*;
pub use stage_ext::*;
pub use filter::Filter;
pub use transform::Transform;
pub use then::Then;
pub use take_skip::{Take, Skip};

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
