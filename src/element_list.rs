use crate::{
    fmt::{ElementFmt, Result},
    Element,
};

pub trait ElementList {
    fn render<'a, 'b>(&self, fmt: ElementFmt<'a, 'b>) -> Result<ElementFmt<'a, 'b>>;
}

impl<E: Element> ElementList for [E] {
    #[inline]
    fn render<'a, 'b>(&self, fmt: ElementFmt<'a, 'b>) -> Result<ElementFmt<'a, 'b>> {
        self.iter().try_fold(fmt, ElementFmt::render_child)
    }
}

impl<E: Element, const N: usize> ElementList for [E; N] {
    #[inline]
    fn render<'a, 'b>(&self, fmt: ElementFmt<'a, 'b>) -> Result<ElementFmt<'a, 'b>> {
        self.iter().try_fold(fmt, ElementFmt::render_child)
    }
}

impl<E: Element> ElementList for Vec<E> {
    #[inline]
    fn render<'a, 'b>(&self, fmt: ElementFmt<'a, 'b>) -> Result<ElementFmt<'a, 'b>> {
        self.iter().try_fold(fmt, ElementFmt::render_child)
    }
}

impl<E: Element> ElementList for E {
    #[inline]
    fn render<'a, 'b>(&self, fmt: ElementFmt<'a, 'b>) -> Result<ElementFmt<'a, 'b>> {
        fmt.render_child(self)
    }
}

macro_rules! tuples {
    ($T0:ident $(, $T:ident)*) => {
        tuples!($($T),*);
        tuple!($T0 $(, $T)*);
    };
    () => { tuple!(); };
}
macro_rules! tuple {
    ($($T:ident),*) => {
        impl<$($T),*> ElementList for ($($T,)*)
        where
            $($T: ElementList,)*
        {
            #[inline]
            fn render<'a, 'b>(&self, fmt: ElementFmt<'a, 'b>) -> Result<ElementFmt<'a, 'b>> {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                $(
                    let fmt = $T.render(fmt)?;
                )*
                Ok(fmt)
            }
        }
    };
}
tuples!(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11);
