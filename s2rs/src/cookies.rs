use std::collections::HashMap;

// region: Cookie
pub struct Cookie {
    pub value: String,
}

impl From<&str> for Cookie {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

impl From<String> for Cookie {
    fn from(value: String) -> Self {
        Self {
            value,
        }
    }
}

impl From<Cookie> for String {
    fn from(value: Cookie) -> Self {
        value.value
    }
}
// endregion: Cookie

#[cfg(feature = "cookie")]
pub type CookiesFromHeaderError = basic_cookies::Error;

// region: Cookies
#[derive(Default)]
pub struct Cookies(HashMap<String, Cookie>);

impl Cookies {
    pub fn add(&mut self, name: impl Into<String>, cookie: impl Into<Cookie>) {
        self.0.insert(name.into(), cookie.into());
    }

    pub fn get(&self, name: &str) -> Option<&Cookie> {
        self.0.get(name)
    }

    pub fn unwrap(self) -> HashMap<String, Cookie> {
        self.0
    }

    #[cfg(feature = "cookie")]
    pub fn from_header(value: &str) -> Result<Self, CookiesFromHeaderError> {
        let parsed_cookies = basic_cookies::Cookie::parse(value)?;
        let mut cookies = Self::default();

        for cookie in parsed_cookies {
            cookies.add(cookie.get_name().to_owned(), cookie.get_value())
        }
        Ok(cookies)
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
