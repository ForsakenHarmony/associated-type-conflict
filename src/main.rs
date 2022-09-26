use other_crate::{SomeTrait, WithVc};

struct SomeValueVc;

impl From<SomeValueVc> for <dyn SomeTrait as WithVc>::Vc {
    fn from(_: SomeValueVc) -> Self {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
