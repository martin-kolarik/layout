use std::sync::Arc;

use rtext::RefMap;

use crate::{
    Axis, Error, Layout, MeasureContext, Position, RenderContext, Style, Styled,
    font::TextPosition,
    position::{Offset, Size},
    unit::FillPerMille,
};

enum InnerText {
    Input(String),
    Layout(TextPosition),
}

pub struct Text {
    mark: Option<&'static str>,
    offset: Offset,
    size: Size,
    style: Arc<Style>,
    text: InnerText,
}

impl Text {
    pub fn mark(mut self, mark: &'static str) -> Self {
        self.mark = Some(mark);
        self
    }

    pub fn new(text: impl ToString) -> Self {
        Self {
            mark: None,
            offset: Offset::zero(),
            size: Size::content(),
            style: Style::new(),
            text: InnerText::Input(text.to_string()),
        }
    }

    pub fn style(mut self, style: impl Into<Arc<Style>>) -> Self {
        self.set_style(style.into());
        self
    }

    pub fn add_style(mut self, style: impl Into<Arc<Style>>) -> Self {
        self.set_style(style.into().merge(&self.style));
        self
    }
}

impl Position for Text {
    fn element(&self) -> &str {
        "Text"
    }

    fn mark(&self) -> &str {
        self.mark.unwrap_or_default()
    }

    fn offset_ref(&self) -> &Offset {
        &self.offset
    }

    fn offset_mut(&mut self) -> &mut Offset {
        &mut self.offset
    }

    fn size_ref(&self) -> &Size {
        &self.size
    }

    fn size_mut(&mut self) -> &mut Size {
        &mut self.size
    }
}

impl Styled for Text {
    fn style_ref(&self) -> &Style {
        self.style.as_ref()
    }

    fn set_style(&mut self, style: Arc<Style>) {
        self.size.apply_style(Axis::Horizontal, &style);
        self.style = style;
    }
}

impl Layout for Text {
    fn measure(&mut self, ctx: &mut dyn MeasureContext, _: Size) -> Result<(), Error> {
        if let InnerText::Input(text) = &mut self.text {
            if text.is_empty() {
                return Ok(());
            }

            let style = self.style.inherit(ctx.style());
            let font = style.font();
            if font.name().is_none() || font.size().is_none() {
                return Ok(());
            }

            let text = ctx.typeset(&style, text)?;
            let font_size = font.size().unwrap();
            let font_scaling = font.scaling().ref_map(FillPerMille::scaling).unwrap_or(1.0);
            self.size = Size::fixed_depth(
                text.width * font_size * font_scaling,
                text.height * font_size,
                text.depth * font_size,
            );
            self.text = InnerText::Layout(text);
        }
        Ok(())
    }

    fn render(&self, ctx: &mut dyn RenderContext) -> Result<(), Error> {
        if let InnerText::Layout(text) = &self.text {
            if !text.positions.is_empty() {
                ctx.text(self.offset_ref(), &self.style, text, false);
                ctx.debug_frame(self.offset_ref(), self.size_ref());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        DefaultFactory, Factory, Layout, Style,
        position::{Offset, Size},
    };

    #[test]
    fn text_in_hbox() {
        let t1 = DefaultFactory::text("a");
        let t2 = DefaultFactory::text("b");
        let t3 = DefaultFactory::text("c");

        let mut hbox = DefaultFactory::hbox()
            .style(Style::new_default())
            .child(t1)
            .child(t2)
            .child(t3);

        hbox.measure(&mut 0_usize, Size::fixed(1000000, 2000000))
            .unwrap();
        hbox.lay_out(
            &mut 0_usize,
            Offset::new(0, 0),
            Size::fixed(1000000, 2000000),
        )
        .unwrap();

        let mut children = hbox.iter();

        let t1 = children.next().unwrap();
        assert_eq!(0, t1.offset_ref().x.0);
        assert_eq!(0, t1.offset_ref().y.0);

        let t2 = children.next().unwrap();
        assert_eq!(105833, t2.offset_ref().x.0);
        assert_eq!(0, t2.offset_ref().y.0);

        let t3 = children.next().unwrap();
        assert_eq!(2 * 105833, t3.offset_ref().x.0);
        assert_eq!(0, t3.offset_ref().y.0);
    }

    #[test]
    fn text_in_hbox_in_hbox() {
        let t1 = DefaultFactory::text("a").mark("t1");
        let t2 = DefaultFactory::text("b").mark("t2");
        let t3 = DefaultFactory::text("c").mark("t3");

        let mut hbox = DefaultFactory::hbox()
            .style(Style::new_default())
            .mark("h1")
            .child(
                DefaultFactory::hbox()
                    .mark("h2")
                    .child(t1)
                    .child(t2)
                    .child(t3),
            );

        hbox.measure(&mut 0_usize, Size::fixed(1000000, 2000000))
            .unwrap();
        hbox.lay_out(
            &mut 0_usize,
            Offset::new(0, 0),
            Size::fixed(1000000, 2000000),
        )
        .unwrap();

        let mut children = hbox.iter();
        let mut children = children.next().unwrap().iter();

        let t1 = children.next().unwrap();
        assert_eq!(0, t1.offset_ref().x.0);
        assert_eq!(0, t1.offset_ref().y.0);

        let t2 = children.next().unwrap();
        assert_eq!(105833, t2.offset_ref().x.0);
        assert_eq!(0, t2.offset_ref().y.0);

        let t3 = children.next().unwrap();
        assert_eq!(2 * 105833, t3.offset_ref().x.0);
        assert_eq!(0, t3.offset_ref().y.0);
    }

    #[test]
    fn text_in_vbox() {
        let t1 = DefaultFactory::text("a");
        let t2 = DefaultFactory::text("b");
        let t3 = DefaultFactory::text("c");

        let mut vbox = DefaultFactory::vbox()
            .style(Style::new_default())
            .child(t1)
            .child(t2)
            .child(t3);

        vbox.measure(&mut 0_usize, Size::fixed(1000000, 2000000))
            .unwrap();
        vbox.lay_out(
            &mut 0_usize,
            Offset::new(0, 0),
            Size::fixed(1000000, 2000000),
        )
        .unwrap();

        let mut children = vbox.iter();

        let t1 = children.next().unwrap();
        assert_eq!(0, t1.offset_ref().x.0);
        assert_eq!(0, t1.offset_ref().y.0);

        let t2 = children.next().unwrap();
        assert_eq!(0, t2.offset_ref().x.0);
        assert_eq!(35278, t2.offset_ref().y.0);

        let t3 = children.next().unwrap();
        assert_eq!(0, t3.offset_ref().x.0);
        assert_eq!(2 * 35278, t3.offset_ref().y.0);
    }

    #[test]
    fn text_in_vbox_in_vbox() {
        let t1 = DefaultFactory::text("a");
        let t2 = DefaultFactory::text("b");
        let t3 = DefaultFactory::text("c");

        let mut vbox = DefaultFactory::vbox()
            .style(Style::new_default())
            .child(DefaultFactory::vbox().child(t1).child(t2).child(t3));

        vbox.measure(&mut 0_usize, Size::fixed(1000000, 2000000))
            .unwrap();
        vbox.lay_out(
            &mut 0_usize,
            Offset::new(0, 0),
            Size::fixed(1000000, 2000000),
        )
        .unwrap();

        let mut children = vbox.iter();
        let mut children = children.next().unwrap().iter();

        let t1 = children.next().unwrap();
        assert_eq!(0, t1.offset_ref().x.0);
        assert_eq!(0, t1.offset_ref().y.0);

        let t2 = children.next().unwrap();
        assert_eq!(0, t2.offset_ref().x.0);
        assert_eq!(35278, t2.offset_ref().y.0);

        let t3 = children.next().unwrap();
        assert_eq!(0, t3.offset_ref().x.0);
        assert_eq!(2 * 35278, t3.offset_ref().y.0);
    }
}
