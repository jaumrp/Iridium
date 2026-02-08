use crate::{
    Component, Style,
    colors::Color,
    get_protocol_version,
    resolver::{ResolverContext, Tag, TagResolver},
};

impl Component {
    pub fn modern_text_as_protocol(text: &str, protocol: i32) -> Component {
        let mut root = Component::text("").protocol(protocol);

        let mut style_stack = vec![Style::default()];
        let mut cursor = 0;

        let current_style =
            |stack: &Vec<Style>| -> Style { stack.last().cloned().unwrap_or_default() };

        let create_part = |content: &str, style: Style| -> Component {
            let mut part = Component::text(content);
            part.style = style;
            part.protocol = protocol;
            part
        };

        while let Some(offset) = text[cursor..].find('<') {
            let start = cursor + offset;
            if start > cursor {
                let content = &text[cursor..start];
                let part = create_part(content, current_style(&style_stack));
                root.extra.push(part);
            }

            if let Some(end_offset) = text[start..].find(">") {
                let end = start + end_offset;
                let tag_content = &text[start + 1..end];
                let tag = TagResolver::resolve(tag_content);

                match tag {
                    Tag::Logic(logic) => {
                        let mut ctx = ResolverContext {
                            text,
                            cursor: end + 1,
                            style_stack: &mut style_stack,
                            out: &mut root.extra,
                            protocol_version: protocol,
                        };
                        cursor = logic.resolve(&mut ctx);
                    }
                    Tag::Close => {
                        if style_stack.len() > 1 {
                            style_stack.pop();
                        }
                        cursor = end + 1;
                    }
                    Tag::Reset => {
                        style_stack.clear();
                        style_stack.push(Style::default());
                        cursor = end + 1;
                    }
                    Tag::Invalid => {
                        let part = create_part("<", current_style(&style_stack));
                        root.extra.push(part);
                        cursor = start + 1;
                    }
                }
            } else {
                let part = create_part("<", current_style(&style_stack));
                root.extra.push(part);
                cursor = start + 1;
            }
        }

        if cursor < text.len() {
            let content = &text[cursor..];
            let mut part = Component::text(content);
            part.style = current_style(&style_stack);
            root.extra.push(part);
        }

        root
    }

    pub fn modern_text(text: &str) -> Component {
        Self::modern_text_as_protocol(text, get_protocol_version())
    }
}

impl Component {
    pub fn legacy_text(text: &str) -> Component {
        let mut root = Component::text("");
        let mut current_style = Style::default();

        let mut iter = text.char_indices().peekable();
        let mut idx = 0;

        while let Some((i, c)) = iter.next() {
            if c == '&' || c == '§' {
                if let Some(&(_, next_char)) = iter.peek() {
                    let code = next_char.to_ascii_lowercase();

                    let is_color = Color::from_legacy_char(code).is_some();
                    let is_format = matches!(code, 'k'..='o' | 'r');

                    if is_color || is_format {
                        if i > idx {
                            let slice = &text[idx..i];
                            let mut part = Component::text(slice);
                            part.style = current_style.clone();
                            root.extra.push(part);
                        }

                        if let Some(color) = Color::from_legacy_char(code) {
                            current_style = Style::default();
                            current_style.color = Some(color);
                        } else {
                            match code {
                                'l' => current_style.bold = Some(true),
                                'm' => current_style.italic = Some(true),
                                'n' => current_style.underlined = Some(true),
                                'o' => current_style.strikethrough = Some(true),
                                'k' => current_style.obfuscated = Some(true),
                                'r' => current_style = Style::default(),
                                _ => {}
                            }
                        }
                        iter.next();
                        idx = i + c.len_utf8() + next_char.len_utf8();
                    }
                }
            }
        }

        if idx < text.len() {
            let mut part = Component::text(&text[idx..]);
            part.style = current_style;
            root.extra.push(part);
        }

        root
    }
}
