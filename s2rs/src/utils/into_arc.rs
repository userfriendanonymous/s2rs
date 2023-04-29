use std::sync::Arc;

pub trait IntoArc<T> {
    fn into_arc(self) -> Arc<T>;
}

impl IntoArc<String> for String {
    fn into_arc(self) -> Arc<String> {
        Arc::new(self)
    }
}

impl IntoArc<String> for &str {
    fn into_arc(self) -> Arc<String> {
        Arc::new(self.to_owned())
    }
}


impl IntoArc<String> for &String {
    fn into_arc(self) -> Arc<String> {
        Arc::new(self.to_owned())
    }
}

impl IntoArc<String> for Arc<String> {
    fn into_arc(self) -> Arc<String> {
        self
    }
}