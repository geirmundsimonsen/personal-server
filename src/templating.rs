use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

// hmmm .... it might be not much to gain by having a separate struct for Void / Text after all?
pub struct NormalElement {
    pub element_name: String,
    pub children: Vec<Element>,
    pub attributes: HashMap<String, String>
}

impl NormalElement {
    fn text(mut self, text: &str) -> NormalElement {
        self.children.push(Element::Text(TextElement { text: text.to_string() }));
        self
    }

    fn add_normal_element(mut self, normal_element: NormalElement) -> NormalElement {
        self.children.push(Element::Normal(normal_element));
        self
    }

    fn attr(mut self, attribute: &str, value: &str) -> NormalElement {
        self.attributes.insert(attribute.to_string(), value.to_string());
        self
    }
}

pub struct VoidElement {
    pub element_name: String
}

pub struct TextElement {
    pub text: String
}

pub enum Element {
    Normal(NormalElement),
    Void(VoidElement),
    Text(TextElement)
}

impl Display for NormalElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {     
        if let Err(err) = write!(f, "<{}", self.element_name) {
            return Result::Err(err)
        }

        for (attr, value) in &self.attributes {
            if let Err(err) = write!(f, " {}=\"{}\"", attr, value) {
                return Result::Err(err)
            }
        }

        if let Err(err) = write!(f, ">") {
            return Result::Err(err)
        }

        for element in &self.children {
            match element {
                Element::Normal(e) => {
                    if let Err(err) = write!(f, "{}", e) {
                        return Result::Err(err)
                    }
                }
                Element::Void(e) => {
                    if let Err(err) = write!(f, "{}", e) {
                        return Result::Err(err)
                    }
                }
                Element::Text(e) => {
                    if let Err(err) = write!(f, "{}", e) {
                        return Result::Err(err)
                    }
                }
            }
        }

        write!(f, "</{}>", self.element_name)
    }
}

impl Display for VoidElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {     
        write!(f, "<{} />", self.element_name)
    }
}

impl Display for TextElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {     
        write!(f, "{}", self.text)
    }
}

pub fn p() -> NormalElement { 
    NormalElement { element_name: "p".to_string(), children: vec![], attributes: HashMap::new() }
}

pub fn a() -> NormalElement { 
    NormalElement { element_name: "a".to_string(), children: vec![], attributes: HashMap::new() }
}

pub struct A {
    pub content: String,
    pub link: String,
}

impl Display for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "<a href=\"{}\">{}</a>", self.link, self.content)
    }
}

pub struct P {
    pub content: Vec<Box<dyn Display>>
}

impl Display for P {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let r = write!(f, "<p>");

        if r.is_err() {
            return r;
        }

        for node in &self.content {
            let r = write!(f, "{}", node);
            if r.is_err() {
                return r;
            }
        }

        write!(f, "</p>")
    }
}

#[cfg(test)]
mod tests {
    use crate::templating::{Element, NormalElement, VoidElement, TextElement, HashMap, a, p};

    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
         };
    );

    #[test]
    fn f() {
        let text = TextElement { text: "hey".to_string() };
        let mut p = NormalElement { element_name: "p".to_string(), children: vec![], attributes: HashMap::new() };

        assert_eq!(p.to_string(), "<p></p>".to_string());   
        p.children.push(Element::Text(text));
        assert_eq!(p.to_string(), "<p>hey</p>".to_string());
        p.attributes.insert("class".to_string(), "blue".to_string());
        assert_eq!(p.to_string(), r#"<p class="blue">hey</p>"#.to_string());
    }

    #[test]
    fn f2() {
        let paragraph_with_link = NormalElement { 
            element_name: "p".to_string(), 
            children: vec![Element::Normal(NormalElement {
                element_name: "a".to_string(),
                children: vec![Element::Text(TextElement { text: "DuckDuckGo".to_string() })],
                attributes: map!{ "href".to_string() => "https://www.duckduckgo.com".to_string() }
            })],
            attributes: HashMap::new()
        };
        assert_eq!(paragraph_with_link.to_string(), r#"<p><a href="https://www.duckduckgo.com">DuckDuckGo</a></p>"#.to_string());

        assert_eq!(
            p().add_normal_element(a().text("DuckDuckGo").attr("href", "https://www.duckduckgo.com")).to_string(),
            r#"<p><a href="https://www.duckduckgo.com">DuckDuckGo</a></p>"#
        )
    }
}