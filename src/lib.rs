#[macro_export]
macro_rules! impl_all {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident(Vec<$inner:ty>)
        $(where [$($where_clause:tt)*])?$(;)?
    ) => {
        def!(
            $(#[$meta])*
            $vis struct $name(Vec<$inner>)
            $(where [$($where_clause)*])?;
        );
        impl_self!($name, $inner);
        impl_extend!($name, $inner);
        impl_into_iter_own!($name, $inner);
        impl_into_iter_ref!($name, $inner);
    };
}

#[macro_export]
macro_rules! def {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident(Vec<$inner:ty>)
        $(where [$($where_clause:tt)*])?$(;)?
    ) => {
        #[derive(derive_more::Deref, derive_more::DerefMut, derive_more::From, derive_more::Into)]
        #[repr(transparent)]
        $(#[$meta])*
        $vis struct $name(Vec<$inner>)
        $(where $($where_clause)*)?;
    };
}

#[macro_export]
macro_rules! impl_self {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub fn new(inner: impl Into<Vec<$inner>>) -> Self {
                Self(inner.into())
            }

            /// Use this function instead of [`Default::default`] because `Default::default` requires `A: Default`, which might not be implemented
            pub fn empty() -> Self {
                Self(vec![])
            }

            pub fn push(&mut self, value: impl Into<$inner>) {
                self.0.push(value.into())
            }

            pub fn extend_from<T: Into<$inner>>(&mut self, iter: impl IntoIterator<Item = T>) {
                self.extend(iter.into_iter().map(T::into))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_extend {
    ($name:ident, $inner:ty) => {
        impl Extend<$inner> for $name {
            fn extend<I: IntoIterator<Item = $inner>>(&mut self, iter: I) {
                self.0.extend(iter);
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into_iter_own {
    ($name:ident, $inner:ty) => {
        impl IntoIterator for $name {
            type Item = $inner;
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into_iter_ref {
    ($name:ident, $inner:ty) => {
        impl<'a> IntoIterator for &'a $name {
            type Item = &'a $inner;
            type IntoIter = std::slice::Iter<'a, $inner>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from {
    ($name:ident, [$($value_source:ty),+]) => {
        $(
            impl_from!($name, $value_source);
        )+
    };
    ($name:ident, $value_source:ty) => {
        impl From<$value_source> for $name {
            fn from(source: $value_source) -> Self {
                Self(vec![source.into()])
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct SignUpAction {
        username: String,
        password: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct SendMessageAction {
        from: String,
        to: String,
        text: String,
    }

    #[derive(Serialize, Deserialize)]
    pub enum Action {
        SendMessage(SendMessageAction),
        SignUp(SignUpAction),
    }

    impl_all!(
        #[derive(Serialize, Deserialize)]
        pub struct ActionVec(Vec<Action>);
    );

    #[derive(derive_more::Deref, derive_more::DerefMut, derive_more::From, derive_more::Into)]
    pub struct ActionVecDeriveMore(Vec<Action>);
}
