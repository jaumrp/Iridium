use crate::{Style, resolver::Resolver};

pub struct StyleTag {
    pub style: Style,
}

macro_rules! style {
    ($old:expr, $new:expr) => {
        if let Some(c) = $new {
            $old = Some(c);
        }
    };
}

impl Resolver for StyleTag {
    fn resolve(&self, ctx: &mut super::ResolverContext) -> usize {
        let mut new_style = ctx.style_stack.last().cloned().unwrap_or_default();

        style!(new_style.color, self.style.color);
        style!(new_style.bold, self.style.bold);
        style!(new_style.italic, self.style.italic);
        style!(new_style.obfuscated, self.style.obfuscated);
        style!(new_style.strikethrough, self.style.strikethrough);
        style!(new_style.underlined, self.style.underlined);
        style!(new_style.font, self.style.font.clone());

        ctx.style_stack.push(new_style);

        ctx.cursor
    }
}
