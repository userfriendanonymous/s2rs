

const SEPARATOR: &str = "/";

#[derive(Default)]
pub struct UrlPath {
    parts: Vec<String>,
}

impl From<UrlPath> for String {
    fn from(value: UrlPath) -> Self {
        value.into_vec().join(SEPARATOR)
    }
}

impl From<Vec<&str>> for UrlPath {
    fn from(value: Vec<&str>) -> Self {
        Self::new(value)
    }
}

impl From<&str> for UrlPath {
    fn from(value: &str) -> Self {
        Self {
            parts: value.split(SEPARATOR).map(|v| v.to_owned()).collect()
        }
    }
}

impl UrlPath {
    pub fn new(parts: Vec<&str>) -> Self {
        Self {
            parts: parts.into_iter().map(Self::make_valid).collect()
        }
    }

    pub fn from_str_trimmed(value: &str) -> Self {
        let mut parts = Vec::new();
        value.split(SEPARATOR).for_each(|v| {
            if !parts.is_empty() || !v.is_empty() {
                parts.push(v.to_owned())
            }
        });

        if let Some(item) = parts.last() {
            if item.is_empty() {
                parts.pop();
            }
        }

        Self {
            parts
        }
    }

    pub fn empty() -> Self {
        Self {
            parts: Vec::new()
        }
    }

    fn make_valid(part: &str) -> String {
        urlencoding::encode(part).into_owned()
    }

    pub fn push(&mut self, part: &str) {
        self.parts.push(Self::make_valid(part));
    }

    pub fn into_vec(self) -> Vec<String> {
        self.parts
    }

    pub fn merge(&mut self, other: Self) {
        self.parts.append(&mut other.into_vec());
    }

    pub fn merged(mut self, other: Self) -> Self {
        self.merge(other);
        self
    }
}

impl From<UrlPath> for Vec<String> {
    fn from(val: UrlPath) -> Self {
        val.into_vec() // there seems like a reason to impl Into instead of From, lol this field is private
    }
}