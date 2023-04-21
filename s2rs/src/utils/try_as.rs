
pub trait TryAs<T, E> {
    fn try_as(&self) -> Result<T, E>;
}