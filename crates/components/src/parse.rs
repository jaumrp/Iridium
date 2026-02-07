use crate::{Component, Style, colors::Color};

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

    pub fn modern_text(text: &str) -> Component {
        let mut root = Component::text("");
        let mut current_style = Style::default();

        let mut cursor = 0;

        while let Some(start_tag) = text[cursor..].find("<") {
            let start = cursor + start_tag;

            if let Some(end_tag) = text[start..].find(">") {
                let end = start + end_tag;
                let tag = &text[start + 1..end];

                let mut new_style = current_style.clone();
                let mut valid_tag = true;
                match Color::from(tag) {
                    Ok(color) => {
                        new_style.color = Some(color);
                    }
                    Err(_) => {
                        match tag {
                            "bold" => new_style.bold = Some(true),
                            "reset" => new_style = Style::default(),
                            "italic" => new_style.italic = Some(true),
                            "underlined" => new_style.underlined = Some(true),
                            "strike" => new_style.strikethrough = Some(true),
                            "obfuscated" => new_style.obfuscated = Some(true),
                            s if s.starts_with("/") => new_style = Style::default(),
                            _ => valid_tag = false,
                        };
                    }
                }
                if valid_tag {
                    if start > cursor {
                        let slice = &text[cursor..start];
                        let mut part = Component::text(slice);
                        part.style = current_style;
                        root.extra.push(part);
                    }
                    current_style = new_style;
                    cursor = end + 1;
                    continue;
                }
            }
            break;
        }

        if cursor < text.len() {
            let mut part = Component::text(&text[cursor..]);
            part.style = current_style;
            root.extra.push(part);
        }

        root
    }
}
