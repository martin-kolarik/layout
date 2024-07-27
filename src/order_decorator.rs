use crate::{Layout, Style};

pub struct OrderDecorator<'s, L, IL>
where
    IL: IntoIterator<Item = L>,
    L: Layout,
{
    index: usize,
    source: IL::IntoIter,
    len: usize,
    first: Option<&'s Style>,
    odd: Option<&'s Style>,
    even: Option<&'s Style>,
    last: Option<&'s Style>,
    third_one: Option<&'s Style>,
    third_two: Option<&'s Style>,
    third_three: Option<&'s Style>,
}

impl<'s, L, IL> OrderDecorator<'s, L, IL>
where
    IL: IntoIterator<Item = L>,
    L: Layout,
{
    pub fn new(source: IL, len: usize) -> Self {
        Self {
            index: 0,
            source: source.into_iter(),
            len,
            first: None,
            odd: None,
            even: None,
            last: None,
            third_one: None,
            third_two: None,
            third_three: None,
        }
    }

    pub fn first(mut self, style: &'s Style) -> Self {
        self.first = Some(style);
        self
    }

    pub fn last(mut self, style: &'s Style) -> Self {
        self.last = Some(style);
        self
    }

    pub fn odd(mut self, style: &'s Style) -> Self {
        self.odd = Some(style);
        self
    }

    pub fn even(mut self, style: &'s Style) -> Self {
        self.even = Some(style);
        self
    }

    pub fn first_third(mut self, style: &'s Style) -> Self {
        self.third_one = Some(style);
        self
    }

    pub fn second_third(mut self, style: &'s Style) -> Self {
        self.third_two = Some(style);
        self
    }

    pub fn third_third(mut self, style: &'s Style) -> Self {
        self.third_three = Some(style);
        self
    }
}

impl<'s, L, IL> Iterator for OrderDecorator<'s, L, IL>
where
    IL: IntoIterator<Item = L>,
    L: Layout,
{
    type Item = L;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            None
        } else {
            let mut current = self.source.next().unwrap();

            match self.index % 3 {
                0 => {
                    if let Some(style) = self.third_one {
                        current.override_style(style);
                    }
                }
                1 => {
                    if let Some(style) = self.third_two {
                        current.override_style(style);
                    }
                }
                2 => {
                    if let Some(style) = self.third_three {
                        current.override_style(style);
                    }
                }
                _ => unreachable!(),
            }

            match self.index % 2 {
                0 => {
                    if let Some(style) = self.even {
                        current.override_style(style);
                    }
                }
                1 => {
                    if let Some(style) = self.odd {
                        current.override_style(style);
                    }
                }
                _ => unreachable!(),
            }

            if self.index == 0 {
                if let Some(style) = self.first {
                    current.override_style(style);
                }
            }

            self.index += 1;

            if self.index == self.len {
                if let Some(style) = self.last {
                    current.override_style(style);
                }
            }

            Some(current)
        }
    }
}

pub trait OrderDecorate<'s, L, IL>
where
    IL: Iterator<Item = L>,
    L: Layout,
{
    fn decorate(self, len: usize) -> OrderDecorator<'s, L, IL>;
}

impl<'s, L, IL> OrderDecorate<'s, L, IL> for IL
where
    IL: Iterator<Item = L>,
    L: Layout,
{
    fn decorate(self, len: usize) -> OrderDecorator<'s, L, IL> {
        OrderDecorator::new(self, len)
    }
}
