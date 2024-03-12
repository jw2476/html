use std::fmt::Display;
use crate::*;

#[macro_export]
macro_rules! rem {
    ($value:expr) => { html::style::Length::Rem($value) }
}

#[macro_export]
macro_rules! px {
    ($value:expr) => { html::style::Length::Px($value) }
}

#[derive(Debug, Clone, Copy)]
pub enum Length {
    Rem(f32),
    Px(f32),
}

impl ToString for Length {
    fn to_string(&self) -> String {
        match self {
            Self::Rem(x) => format!("{x}rem"),
            Self::Px(x) => format!("{x}px"),
        }
    }
}

impl Default for Length {
    fn default() -> Self {
        Self::Rem(0.0)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Edges<T> {
    top: T,
    bottom: T,
    left: T,
    right: T,
}

impl<T: Copy> Edges<T> {
    pub fn top(mut self, top: T) -> Self {
        self.top = top;
        self 
    }

    pub fn bottom(mut self, bottom: T) -> Self {
        self.bottom = bottom;
        self 
    }

    pub fn left(mut self, left: T) -> Self {
        self.left = left;
        self 
    }

    pub fn right(mut self, right: T) -> Self {
        self.right = right;
        self 
    }

    pub fn all(value: T) -> Self {
        Self {
            top: value,
            bottom: value,
            left: value,
            right: value,
        }
    }
}

impl<T: ToString> ToString for Edges<T> {
    fn to_string(&self) -> String {
        format!(
            "{} {} {} {}",
            self.top.to_string(),
            self.right.to_string(),
            self.bottom.to_string(),
            self.left.to_string()
        )
    }
}

#[derive(Debug, Clone)]
pub enum Style {
    Padding(Edges<Length>),
    Margin(Edges<Length>),
    Border(Edges<Length>)
}

impl ToString for Style {
    fn to_string(&self) -> String {
        match self {
            Self::Padding(x) => format!("padding: {};", x.to_string()),
            Self::Margin(x) => format!("margin: {};", x.to_string()),
            Self::Border(x) => format!("border: {};", x.to_string()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Styles {
    styles: Vec<Style>
}

impl Display for Styles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.styles.iter().map(|style| format!("{}", style.to_string())).collect::<Vec<String>>().join(""))
    }
}

pub trait Styleable {
    fn style(self, style: Style) -> Self;
}

macro_rules! impl_styleable {
    ($($t:ty),*) => {
        $(
            impl Styleable for $t {
                fn style(mut self, style: Style) -> Self {
                    self.style.styles.push(style);
                    self
                }
            }
        )*
    }
}

impl_styleable!(Html, Body, Div, Ol, Ul, P, Input, Label, H1, H2, H3, H4, H5, H6, Li, Button, Image);
