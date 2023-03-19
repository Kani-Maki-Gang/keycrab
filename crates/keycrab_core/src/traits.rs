use std::sync::Arc;

pub trait IntoArc {
    fn into_arc(self) -> Arc<Self>
    where
        Self: Sized,
    {
        Arc::new(self)
    }
}

impl<T> IntoArc for T {}
