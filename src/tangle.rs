use crate::style::*;

pub trait Tangled: Styleable + Sized {
    fn padding(self, padding: Edges<Length>) -> Self {
        self.style(Style::Padding(padding))
    } 

    fn border(self, border: Edges<Length>) -> Self {
        self.style(Style::Border(border))
    }

    fn margin(self, margin: Edges<Length>) -> Self {
        self.style(Style::Margin(margin))
    }
}

impl<T: Styleable> Tangled for T {}
