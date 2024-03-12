#![feature(concat_idents)]

pub mod style;
pub mod tangle;
pub mod htmx;

use style::Styles;

pub trait Renderable {
    fn render(&self) -> String;
}

macro_rules! property {
    ($i:ident) => {
        stringify!($i).replace("_", "-")
    } 
}

macro_rules! parent {
    ($name:ident, $cname:ident, $tag:ident, children: [$( $child:ident ),*], required: {$( $required:ident: $required_ty:ty ),*}, optional: {$( $optional:ident: $optional_ty:ty ),*}) => {
        #[derive(Debug)]
        pub struct $name {
            pub children: Children<$cname>,
            $(
                pub $optional: $optional_ty,
            )*
            $(
                pub $required: $required_ty,
            )*
        }

        impl Renderable for $name {
            fn render(&self) -> String {
                let mut properties: Vec<String> = vec![String::new()]; // Empty string is for starting space
                $(
                    let $required = format!("{}", self.$required);
                    if !$required.is_empty() {
                        properties.push(format!("{}=\"{}\"", property!($required), $required));
                    }
                )*
                $(
                    let $optional = format!("{}", self.$optional);
                    if !$optional.is_empty() {
                        properties.push(format!("{}=\"{}\"", property!($optional), $optional));
                    }
                )*
                let properties = properties.join(" ");
                
                let inner = self.children.render();
                let inner = inner.split("\n").map(|x| format!("  {x}")).collect::<Vec<String>>().join("\n");

                format!("<{0}{1}>\n{2}\n</{0}>", stringify!($tag), properties, inner)
            }
        }

        #[derive(Debug)]
        pub enum $cname {
            $(
                $child($child),   
            )*
        }

        impl Renderable for $cname {
            fn render(&self) -> String {
                match self {
                    $(
                        Self::$child(x) => x.render(),
                    )*    
                }
            }
        }

        $(
            impl From<$child> for Children<$cname> {
                fn from(value: $child) -> Self {
                    Self(vec![$cname::$child(value)])
                }
            }
        )*

        pub fn $tag<T: Into<Children<$cname>>>($($required: $required_ty,)* children: T) -> $name {
            $name {
                $(
                    $required,
                )*
                $(
                    $optional: Default::default(),
                )*
                children: children.into()
            }
        }
    }
}

macro_rules! content {
    ($name:ident, $tag:ident, required: {$( $required:ident: $required_ty:ty ),*}, optional: {$( $optional:ident: $optional_ty:ty ),*}) => {
        #[derive(Debug)]
        pub struct $name {
            pub text: String,
            $(
                pub $optional: $optional_ty,
            )*
            $(
                pub $required: $required_ty,
            )*
        }

        impl Renderable for $name {
            fn render(&self) -> String {
                let mut properties: Vec<String> = vec![String::new()]; // Empty string is for starting space
                $(
                    let $required = format!("{}", self.$required);
                    if !$required.is_empty() {
                        properties.push(format!("{}=\"{}\"", property!($required), $required));
                    }
                )*
                $(
                    let $optional = format!("{}", self.$optional);
                    if !$optional.is_empty() {
                        properties.push(format!("{}=\"{}\"", property!($optional), $optional));
                    }
                )*
                let properties = properties.join(" ");
                
                format!("<{0}{1}>{2}</{0}>", stringify!($tag), properties, self.text)
            }
        }

        pub fn $tag($($required: $required_ty,)* text: &str) -> $name {
            $name {
                $(
                    $required,
                )*
                $(
                    $optional: Default::default(),
                )*
                text: text.to_string()
            }
        }
    }
}

macro_rules! self_closing {
    ($name:ident, $tag:ident, required: {$( $required:ident: $required_ty:ty ),*}, optional: {$( $optional:ident: $optional_ty:ty ),*}) => {
        #[derive(Debug)]
        pub struct $name {
            $(
                pub $optional: $optional_ty,
            )*
            $(
                pub $required: $required_ty,
            )*
        }

        impl Renderable for $name {
            fn render(&self) -> String {
                let mut properties: Vec<String> = vec![String::new()]; // Empty string is for starting space
                $(
                    let $required = format!("{}", self.$required);
                    if !$required.is_empty() {
                        properties.push(format!("{}=\"{}\"", property!($required), $required));
                    }
                )*
                $(
                    let $optional = format!("{}", self.$optional);
                    if !$optional.is_empty() {
                        properties.push(format!("{}=\"{}\"", property!($optional), $optional));
                    }
                )*
                let properties = properties.join(" ");
                
                format!("<{0}{1}></{0}>", stringify!($tag), properties)
            }
        }

        pub fn $tag($($required: $required_ty,)*) -> $name {
            $name {
                $(
                    $required,
                )*
                $(
                    $optional: Default::default(),
                )*
            }
        }
    }
}

#[derive(Debug)]
pub struct Children<T>(Vec<T>);

impl<T: Renderable> Renderable for Children<T> {
    fn render(&self) -> String {
        self.0
            .iter()
            .map(Renderable::render)
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl<T, A: Into<Children<T>>> From<Vec<A>> for Children<T> {
    fn from(value: Vec<A>) -> Self {
        Self(value.into_iter().flat_map(|x| x.into().0).collect())
    }
}

impl<T, A: Into<Children<T>>> From<(A,)> for Children<T> {
    fn from(value: (A,)) -> Self {
        value.0.into()
    }
}

impl<T, A: Into<Children<T>>, B: Into<Children<T>>> From<(A, B)> for Children<T> {
    fn from(value: (A, B)) -> Self {
        let mut data = value.0.into().0;
        data.append(&mut value.1.into().0);
        Self(data)
    }
}

impl<T, A: Into<Children<T>>, B: Into<Children<T>>, C: Into<Children<T>>> From<(A, B, C)>
    for Children<T>
{
    fn from(value: (A, B, C)) -> Self {
        let mut data = value.0.into().0;
        data.append(&mut value.1.into().0);
        data.append(&mut value.2.into().0);
        Self(data)
    }
}

macro_rules! base {
    () => {
            }
}

parent!(Html, HtmlChild, html, children: [Head, Body], required: {}, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String });
parent!(Head, HeadChild, head, children: [Title, Script], required: {}, optional: {});
parent!(Body, BodyChild, body, children: [Div, P, Input, Label, Image, Script], required: {}, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String });
parent!(Div, DivChild, div, children: [Div, P, Input, Label, Ol, Ul, Image], required: {},  optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String});
parent!(Ol, OlChild, ol, children: [Li], required: {}, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String});
parent!(Ul, UlChild, ul, children: [Li], required: {}, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String});

content!(Title, title, required: {}, optional: {});
content!(P, p, required: {}, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String});
content!(Input, input, required: {}, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String});
content!(Label, label, required: {}, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String});
content!(H1, h1, required: {}, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String});
content!(H2, h2, required: {}, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String});
content!(H3, h3, required: {}, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String});
content!(H4, h4, required: {}, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String});
content!(H5, h5, required: {}, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String});
content!(H6, h6, required: {}, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String});
content!(Li, li, required: {}, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String});
content!(Button, button, required: {}, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String});

self_closing!(Image, image, required: { src: String, alt: String }, optional: { style: Styles, hx_get: String, hx_post: String, hx_put: String, hx_patch: String, hx_delete: String, hx_trigger: String, hx_swap: String, hx_target: String});
self_closing!(Script, script, required: { src: String }, optional: {});

#[derive(Debug)]
pub struct Raw {
    pub text: String
}

impl Renderable for Raw {
    fn render(&self) -> String {
        self.text.clone()
    }
}

pub fn raw(text: &str) -> Raw {
    Raw { text: text.to_string() }
}
