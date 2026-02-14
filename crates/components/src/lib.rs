use std::ops::Add;

use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};

use crate::colors::Color;

pub mod colors;
pub mod parse;
pub mod resolver;

#[derive(Debug, Clone, PartialEq, Default, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Style {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Text {
        text: String,
    },
    Translatable {
        key: String,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        with: Vec<Component>,
    },
    Selector {
        key: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        with: Option<Box<Component>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Component {
    #[serde(flatten)]
    pub style: Style,
    #[serde(flatten)]
    pub content: Content,

    #[serde(rename = "extra", skip_serializing_if = "Vec::is_empty", default)]
    pub extra: Vec<Component>,

    #[serde(skip_serializing)]
    pub protocol: i32,
}

impl Component {
    pub fn new(content: Content) -> Self {
        Self {
            style: Style::default(),
            content,
            extra: Vec::new(),
            protocol: 754,
        }
    }

    pub fn text<S: Into<String>>(text: S) -> Self {
        Self::new(Content::Text { text: text.into() })
    }

    pub fn append<Child: Into<Component>>(mut self, child: Child) -> Component {
        let mut child = child.into();
        child.protocol = self.protocol;
        self.extra.push(child);
        self
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl<S: Into<String>> From<S> for Component {
    fn from(text: S) -> Self {
        Self::text(text)
    }
}

impl Add for Component {
    type Output = Component;

    fn add(self, other: Component) -> Self {
        self.append(other)
    }
}

impl Component {
    pub fn protocol(mut self, protocol: i32) -> Self {
        self.protocol = protocol;

        self.extra = self
            .extra
            .into_iter()
            .map(|e| e.protocol(protocol))
            .collect();

        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.style.color = Some(color);
        self
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.style.bold = Some(bold);
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.style.italic = Some(italic);
        self
    }

    pub fn underlined(mut self, underlined: bool) -> Self {
        self.style.underlined = Some(underlined);
        self
    }

    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.style.strikethrough = Some(strikethrough);
        self
    }

    pub fn obfuscated(mut self, obfuscated: bool) -> Self {
        self.style.obfuscated = Some(obfuscated);
        self
    }

    pub fn font(mut self, font: String) -> Self {
        self.style.font = Some(font);
        self
    }
}

macro_rules! serialize_optional_field {
    ($state:ident, $field_name:expr, $value:expr) => {
        if let Some(ref value) = $value {
            $state.serialize_field($field_name, value)?;
        }
    };
}

impl Serialize for Component {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let is_modern_protocol = self.protocol >= 735;

        let mut state = serializer.serialize_struct("Component", 1)?;

        match &self.content {
            Content::Text { text } => {
                state.serialize_field("text", text)?;
            }
            Content::Translatable { key, with } => {
                state.serialize_field("translate", key)?;
                if !with.is_empty() {
                    state.serialize_field("with", with)?;
                }
            }
            _ => {}
        }

        if let Some(color) = &self.style.color {
            if is_modern_protocol {
                state.serialize_field("color", &color.to_hex_string())?;
            } else {
                state.serialize_field("color", &color.to_legacy_name())?;
            }
        }

        serialize_optional_field!(state, "bold", self.style.bold);
        serialize_optional_field!(state, "italic", self.style.italic);
        serialize_optional_field!(state, "underlined", self.style.underlined);
        serialize_optional_field!(state, "strikethrough", self.style.strikethrough);
        serialize_optional_field!(state, "obfuscated", self.style.obfuscated);
        serialize_optional_field!(state, "font", self.style.font);

        if !self.extra.is_empty() {
            state.serialize_field("extra", &self.extra)?;
        }

        state.end()
    }
}

pub fn get_protocol_version() -> i32 {
    774
}

pub fn get_version_name() -> String {
    "Iridium 1.21.11".to_string()
}
