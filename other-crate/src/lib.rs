pub trait WithVc {
    type Vc;
}

pub type Vc<T> = <T as WithVc>::Vc;

pub trait SomeTrait {
    fn func(&self);
}

impl WithVc for dyn SomeTrait {
    type Vc = ();
}
