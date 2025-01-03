#[derive(Copy, Clone, Debug)]
pub struct Cons<H, T> {
    pub head: H,
    pub tail: T,
}

#[derive(Copy, Clone, Debug)]
pub struct Nil;

#[macro_export]
macro_rules! hlist {
    () => { $crate::hlist::Nil };
    ($a:expr $(,$($rest:tt)*)?) => {
        $crate::hlist::Cons {
            head: $a,
            tail: $crate::hlist::hlist!{$($($rest)*)?},
        }
    };
}

#[macro_export]
macro_rules! HList {
    () => { $crate::hlist::Nil };
    ($a:expr $(,$($rest:tt)*)?) => {
        $crate::hlist::Cons<$a, $crate::hlist::HList!($($($rest)*)?)>
    };
}

#[macro_export]
macro_rules! hlist_pat {
    () => { $crate::hlist::Nil };
    ($a:pat $(,$($rest:tt)*)?) => {
        $crate::hlist::Cons {
            head: $a,
            tail: $crate::hlist::hlist_pat!{$($($rest)*)?},
        }
    };
}

pub trait Map<T> {
    fn map(self) -> T;
}

impl<H, T, HH, TT> Map<Cons<HH, TT>> for Cons<H, T>
where
    H: Map<HH>,
    T: Map<TT>,
{
    fn map(self) -> Cons<HH, TT> {
        Cons {
            head: self.head.map(),
            tail: self.tail.map(),
        }
    }
}

/// ... So the mapped list and the mapping list must have the same length/shape.
impl Map<Nil> for Nil {
    fn map(self) -> Nil {
        Nil
    }
}

pub trait TypeMap<T> {
    type Result;
}
pub type TypeMapped<This, Op> = <This as TypeMap<Op>>::Result;

impl<H, T, HH, TT> TypeMap<Cons<HH, TT>> for Cons<H, T>
where
    H: TypeMap<HH>,
    T: TypeMap<TT>,
{
    type Result = Cons<TypeMapped<H, HH>, TypeMapped<T, TT>>;
}

/// ... So the mapped list and the mapping list must have the same length.
impl TypeMap<Nil> for Nil {
    type Result = Nil;
}
