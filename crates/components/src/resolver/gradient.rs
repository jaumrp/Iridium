use std::vec;

use crate::{
    Component, Style,
    colors::Color,
    resolver::{Resolver, ResolverContext},
};

pub struct GradientTag {
    pub start: Color,
    pub end: Color,
}

impl Resolver for GradientTag {
    fn resolve(&self, ctx: &mut ResolverContext) -> usize {
        let close_tag = "</gradient>";

        if let Some(idx) = ctx.text[ctx.cursor..].find(close_tag) {
            let start = ctx.cursor;
            let end = start + idx;
            let inner_text = &ctx.text[start..end];

            let base = ctx.style_stack.last().cloned().unwrap_or_default();
            let gradient =
                create_gradient(inner_text, self.start, self.end, base, ctx.protocol_version);

            ctx.out.extend(gradient);
            return end + close_tag.len();
        }

        ctx.cursor
    }
}

fn create_gradient(
    text: &str,
    start: Color,
    end: Color,
    base: Style,
    protocol_version: i32,
) -> Vec<Component> {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    if len == 0 {
        return vec![];
    }

    if len == 1 || protocol_version < 735 {
        let mut part = Component::text(text).protocol(protocol_version);
        let mut style = base.clone();
        style.color = Some(start);
        part.style = style;
        return vec![part];
    }
    let mut components = Vec::with_capacity(len);

    let (r1, g1, b1) = (start.r as f32, start.g as f32, start.b as f32);
    let (r2, g2, b2) = (end.r as f32, end.g as f32, end.b as f32);

    for (i, ch) in chars.iter().enumerate() {
        let factor = i as f32 / (len - 1) as f32;

        let r = (r1 + (r2 - r1) * factor) as u8;
        let g = (g1 + (g2 - g1) * factor) as u8;
        let b = (b1 + (b2 - b1) * factor) as u8;

        let mut part = Component::text(ch.to_string());
        let mut style = base.clone();
        style.color = Some(Color { r, g, b });
        part.style = style;
        components.push(part);
    }

    components
}
