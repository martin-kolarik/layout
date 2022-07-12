use crate::{
    font::TextPosition,
    position::{Offset, Size},
    styled, Error, Layout, MeasureContext, Position, RenderContext, Style,
};

enum InnerText {
    Input(String),
    Layout(TextPosition),
}

pub struct Text {
    mark: Option<&'static str>,
    offset: Offset,
    size: Size,
    style: Style,
    text: InnerText,
}

impl Text {
    pub fn mark(mut self, mark: &'static str) -> Self {
        self.mark = Some(mark);
        self
    }

    pub fn new(text: impl Into<String>) -> Self {
        Self {
            mark: None,
            offset: Offset::zero(),
            size: Size::content(),
            style: Style::default(),
            text: InnerText::Input(text.into()),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Position for Text {
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

styled!(Text);

impl Layout for Text {
    fn measure(&mut self, ctx: &mut dyn MeasureContext, _: Size) -> Result<(), Error> {
        if let InnerText::Input(text) = &mut self.text {
            let style = self.style.merge(ctx.style());
            let font = match style.font() {
                Some(font) => font,
                None => return Ok(()),
            };

            let text = ctx.typeset(&style, text, None)?;
            let font_size = font.size();
            self.size = Size::fixed_depth(
                text.width * font_size,
                text.height * font_size,
                text.depth * font_size,
            );
            self.text = InnerText::Layout(text);
        }
        Ok(())
    }

    fn render(&self, ctx: &mut dyn RenderContext) -> Result<(), Error> {
        if let InnerText::Layout(text) = &self.text {
            ctx.text(self.offset_ref(), &self.style, text, false);
            ctx.debug_frame(self.offset_ref(), self.size_ref());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        position::{Offset, Quad, Size},
        unit::Em,
        DefaultFactory, Error, Factory, Features, GlyphPosition, Layout, MeasureContext,
        RenderContext, Stroke, Style, TextPosition,
    };

    static STYLE: Style = Style::new();

    impl MeasureContext for usize {
        fn style(&self) -> &Style {
            &STYLE
        }

        fn typeset(
            &mut self,
            _: &Style,
            _: &str,
            _: Option<&Features>,
        ) -> Result<TextPosition, Error> {
            Ok(TextPosition {
                width: Em(30.0),
                height: Em(10.0),
                depth: Em(2.0),
                positions: vec![GlyphPosition::new(1, Em(30.0), Em(0.0), Em(0.0), Em(0.0))],
            })
        }
    }

    impl RenderContext for usize {
        fn new_page(&mut self) {
            todo!()
        }

        fn new_page_size(&mut self, _: Quad, _: Size) {
            todo!()
        }

        fn debug_frame(&self, _: &Offset, _: &Size) {
            todo!()
        }

        fn line(&mut self, _: &Offset, _: &Offset, _: &Stroke) {
            todo!()
        }

        fn text(&mut self, _: &Offset, _: &Style, _: &TextPosition, _: bool) {
            todo!()
        }
    }

    #[test]
    fn text_in_hbox() {
        let t1 = DefaultFactory::text("a");
        let t2 = DefaultFactory::text("b");
        let t3 = DefaultFactory::text("c");

        let mut hbox = DefaultFactory::hbox().child(t1).child(t2).child(t3);

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
        assert_eq!(0, t1.offset_ref().x().0);
        assert_eq!(0, t1.offset_ref().y().0);

        let t2 = children.next().unwrap();
        assert_eq!(105833, t2.offset_ref().x().0);
        assert_eq!(0, t2.offset_ref().y().0);

        let t3 = children.next().unwrap();
        assert_eq!(2 * 105833, t3.offset_ref().x().0);
        assert_eq!(0, t3.offset_ref().y().0);
    }

    #[test]
    fn text_in_hbox_in_hbox() {
        let t1 = DefaultFactory::text("a").mark("t1");
        let t2 = DefaultFactory::text("b").mark("t2");
        let t3 = DefaultFactory::text("c").mark("t3");

        let mut hbox = DefaultFactory::hbox().mark("h1").child(
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
        assert_eq!(0, t1.offset_ref().x().0);
        assert_eq!(0, t1.offset_ref().y().0);

        let t2 = children.next().unwrap();
        assert_eq!(105833, t2.offset_ref().x().0);
        assert_eq!(0, t2.offset_ref().y().0);

        let t3 = children.next().unwrap();
        assert_eq!(2 * 105833, t3.offset_ref().x().0);
        assert_eq!(0, t3.offset_ref().y().0);
    }

    #[test]
    fn text_in_vbox() {
        let t1 = DefaultFactory::text("a");
        let t2 = DefaultFactory::text("b");
        let t3 = DefaultFactory::text("c");

        let mut vbox = DefaultFactory::vbox().child(t1).child(t2).child(t3);

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
        assert_eq!(0, t1.offset_ref().x().0);
        assert_eq!(0, t1.offset_ref().y().0);

        let t2 = children.next().unwrap();
        assert_eq!(0, t2.offset_ref().x().0);
        assert_eq!(35278, t2.offset_ref().y().0);

        let t3 = children.next().unwrap();
        assert_eq!(0, t3.offset_ref().x().0);
        assert_eq!(2 * 35278, t3.offset_ref().y().0);
    }

    #[test]
    fn text_in_vbox_in_vbox() {
        let t1 = DefaultFactory::text("a");
        let t2 = DefaultFactory::text("b");
        let t3 = DefaultFactory::text("c");

        let mut vbox =
            DefaultFactory::vbox().child(DefaultFactory::vbox().child(t1).child(t2).child(t3));

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
        assert_eq!(0, t1.offset_ref().x().0);
        assert_eq!(0, t1.offset_ref().y().0);

        let t2 = children.next().unwrap();
        assert_eq!(0, t2.offset_ref().x().0);
        assert_eq!(35278, t2.offset_ref().y().0);

        let t3 = children.next().unwrap();
        assert_eq!(0, t3.offset_ref().x().0);
        assert_eq!(2 * 35278, t3.offset_ref().y().0);
    }
}
