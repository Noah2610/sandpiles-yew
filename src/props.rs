//! Common prop types.

pub mod prelude {
    pub use super::Size;
    pub use super::ToClass;
}

pub trait ToClass {
    fn to_class(&self) -> String;
}

#[derive(Clone, PartialEq)]
pub enum Size {
    Xs,
    Sm,
    Base,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Xl4,
    Xl5,
    Xl6,
}

impl Default for Size {
    fn default() -> Self {
        Size::Lg
    }
}

impl ToClass for Size {
    fn to_class(&self) -> String {
        match self {
            Self::Xs => "text-xs".into(),
            Self::Sm => "text-sm".into(),
            Self::Base => "text-base".into(),
            Self::Lg => "text-lg".into(),
            Self::Xl => "text-xl".into(),
            Self::Xl2 => "text-2xl".into(),
            Self::Xl3 => "text-3xl".into(),
            Self::Xl4 => "text-4xl".into(),
            Self::Xl5 => "text-5xl".into(),
            Self::Xl6 => "text-6xl".into(),
        }
    }
}
