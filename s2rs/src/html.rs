use html_parser::{Element, Dom};

// region: ElementUtils
pub trait ElementUtils {
    fn child_by_class(&self, name: impl Into<String>) -> Option<&Element>;
    fn child_by_name(&self, name: &str) -> Option<&Element>;
    fn child_by_attribute(&self, name: &str, value: &str) -> Option<&Element>;
    fn child_by_id(&self, name: &str) -> Option<&Element>;
    fn get_attribute(&self, name: &str) -> Option<String>;
    fn get_text(&self) -> Option<&str>;
}

impl ElementUtils for Element {
    fn child_by_class(&self, name: impl Into<String>) -> Option<&Element> {
        let name: String = name.into();
        for child in &self.children {
            if let Some(element) = child.element() {
                if element.classes.contains(&Into::<String>::into(&name)) {
                    // dbg!(format!("[PASS] child_by_class: {}", &name));
                    return Some(element)
                }
            }
        }
        // dbg!(format!("[FAILED] child_by_class: {}", &name));
        None
    }

    fn child_by_name(&self, name: &str) -> Option<&Element> {
        for child in &self.children {
            if let Some(element) = child.element() {
                if element.name == name {
                    // dbg!(format!("[PASS] child_by_name: {}", name));
                    return Some(element)
                }
            }
        }
        // dbg!(format!("[FAILED] child_by_name: {}", name));
        None
    }

    fn child_by_id(&self, name: &str) -> Option<&Element> {
        for child in &self.children {
            if let Some(element) = child.element() {
                if let Some(id) = &element.id {
                    if id == name {
                        // dbg!(format!("[PASS] child_by_id: {}", name));
                        return Some(element)
                    }
                }
            }
        }
        // dbg!(format!("[FAILED] child_by_id: {}", name));
        None
    }

    fn child_by_attribute(&self, name: &str, expected: &str) -> Option<&Element> {
        for child in &self.children {
            if let Some(element) = child.element() {
                if let Some(ref got) = element.get_attribute(name) {
                    if got == expected {
                        // dbg!(format!("[PASS] child_by_attr: {}", name));
                        return Some(element)
                    }
                }
            }
        }
        // dbg!(format!("[FAILED] child_by_attr: {}", name));
        None
    }

    fn get_attribute(&self, name: &str) -> Option<String> {
        self.attributes.get(name)?.clone()
    }

    fn get_text(&self) -> Option<&str> {
        self.children.first()?.text()
    }
}
// endregion: ElementUtils

// region: DomUtils
pub trait DomUtils {
    fn child_by_name(&self, name: &str) -> Option<&Element>;
}

impl DomUtils for Dom {
    fn child_by_name(&self, name: &str) -> Option<&Element> {
        for child in &self.children {
            if let Some(element) = child.element() {
                if element.name == name {
                    return Some(element)
                }
            }
        }
        None
    }
}
// endregion: DomUtils
