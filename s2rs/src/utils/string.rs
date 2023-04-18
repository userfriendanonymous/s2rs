
pub trait StringUtils {
    fn find_tail(&self, value: &str) -> Option<usize>;
}

impl StringUtils for &str {
    fn find_tail(&self, value: &str) -> Option<usize> {
        Some(self.find(value)? + value.len())
    }
}

impl StringUtils for String {
    fn find_tail(&self, value: &str) -> Option<usize> {
        Some(self.find(value)? + value.len())
    }
}