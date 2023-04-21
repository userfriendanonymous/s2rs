use std::collections::HashMap;

// region: Cookie
pub struct Cookie {
    pub value: String,
}

impl Cookie {
    #[cfg(feature = "cookie")]
    pub fn from_header(header: &str, find_name: &str) -> Option<Self> {
        let (_, value) = simple_cookie::parse_cookie_header_value(header.as_bytes())
        .find(|(name, _)| *name == find_name)?;
        Some(Self {
            value: String::from_utf8_lossy(value).to_string()
        })
    }
}

impl From<&str> for Cookie {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

impl From<Cookie> for String {
    fn from(value: Cookie) -> Self {
        value.value
    }
}
// endregion: Cookie

// region: Cookies
#[derive(Default)]
pub struct Cookies(HashMap<String, Cookie>);

impl Cookies {
    pub fn add(&mut self, name: impl Into<String>, cookie: impl Into<Cookie>) {
        self.0.insert(name.into(), cookie.into());
    }

    pub fn unwrap(self) -> HashMap<String, Cookie> {
        self.0
    }
}

impl From<Cookies> for String {
    fn from(value: Cookies) -> Self {
        let mut result = String::new();
        for (idx, (name, cookie)) in value.unwrap().into_iter().enumerate() {
            if idx != 0 {
                result.push(';');
            }
            result.push_str(&format!("{name}={}", Into::<String>::into(cookie)));
        }
        result
    }
}
// endregion: Cookies
