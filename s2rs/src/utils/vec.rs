
pub trait VecUtils<T> {
    fn try_map<U, E>(self) -> Result<Vec<U>, E> where T: TryInto<U, Error = E>;
}

impl<T> VecUtils<T> for Vec<T> {
    fn try_map<U, E>(self) -> Result<Vec<U>, E> where T: TryInto<U, Error = E> {
        let mut result = Vec::new();
        for item in self {
            result.push(item.try_into()?)
        }
        Ok(result)
    }
}