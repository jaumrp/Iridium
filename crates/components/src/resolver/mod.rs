use crate::{Component, Style, colors::Color, resolver::style::StyleTag};

pub mod gradient;
pub mod style;

pub enum Tag {
    Logic(Box<dyn Resolver>),
    Close,
    Reset,
    Invalid,
}
pub struct TagResolver;

impl TagResolver {
    pub fn resolve(content: &str) -> Tag {
        match content {
            tag if tag.starts_with('/') => Tag::Close,
            "reset" | "r" => Tag::Reset,
            tag if tag.starts_with("gradient") => {
                let parts: Vec<&str> = tag.split(':').collect();
                if parts.len() >= 3 {
                    let start = Color::from(parts[1]);
                    let end = Color::from(parts[2]);
                    if let (Ok(start), Ok(end)) = (start, end) {
                        return Tag::Logic(Box::new(gradient::GradientTag { start, end }));
                    }
                }
                Tag::Invalid
            }
            tag => {
                let style = Self::parse_style(tag);
                if style.is_some() {
                    return Tag::Logic(Box::new(StyleTag {
                        style: style.unwrap(),
                    }));
                }
                Tag::Invalid
            }
        }
    }

    fn parse_style(content: &str) -> Option<Style> {
        let content = content.trim();
        let mut style = Style::default();

        if let Ok(color) = Color::from(content) {
            style.color = Some(color);
            return Some(style);
        }

        let is_decoration: bool = match content {
            "bold" | "b" => {
                style.bold = Some(true);
                true
            }
            "italic" | "i" => {
                style.italic = Some(true);
                true
            }
            "underlined" | "underline" | "u" => {
                style.underlined = Some(true);
                true
            }
            "strikethrough" | "st" | "strike" | "s" => {
                style.strikethrough = Some(true);
                true
            }
            "obfuscated" | "obf" => {
                style.obfuscated = Some(true);
                true
            }

            _ => false,
        };
        if is_decoration { Some(style) } else { None }
    }
}

pub struct ResolverContext<'a> {
    pub text: &'a str,
    pub cursor: usize,
    pub style_stack: &'a mut Vec<Style>,
    pub out: &'a mut Vec<Component>,
    pub protocol_version: i32,
}

pub trait Resolver {
    fn resolve(&self, ctx: &mut ResolverContext) -> usize;
}
