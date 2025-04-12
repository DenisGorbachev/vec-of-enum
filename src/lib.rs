#[macro_export]
macro_rules! define {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident(Vec<$inner:ty>)
        $(where [$($where_clause:tt)*])?;
        $(variants = [$($variant:ty),+];)?
    ) => {
        define_struct!(
            $(#[$meta])*
            $vis struct $name(Vec<$inner>)
            $(where [$($where_clause)*])?;
        );
        impl_self!($name, $inner);
        impl_default!($name);
        impl_extend!($name, $inner);
        impl_into_iter_own!($name, $inner);
        impl_into_iter_ref!($name, $inner);
        impl_deref!($name, $inner);
        impl_deref_mut!($name, $inner);
        impl_from_vec!($name, $inner);
        impl_into_vec!($name, $inner);
        $(impl_from_value!($name, [$($variant),+]);)?
    };
}

#[macro_export]
macro_rules! define_struct {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident(Vec<$inner:ty>)
        $(where [$($where_clause:tt)*])?;
    ) => {
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
macro_rules! impl_default {
    ($name:ident) => {
        impl Default for $name {
            fn default() -> Self {
                Self(Default::default())
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
macro_rules! impl_deref {
    ($name:ident, $inner:ty) => {
        impl std::ops::Deref for $name {
            type Target = Vec<$inner>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref_mut {
    ($name:ident, $inner:ty) => {
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from_vec {
    ($name:ident, $inner:ty) => {
        impl From<Vec<$inner>> for $name {
            fn from(vec: Vec<$inner>) -> Self {
                Self(vec)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from_value {
    ($name:ident, [$($value_source:ty),+]) => {
        $(
            impl_from_value!($name, $value_source);
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

#[macro_export]
macro_rules! impl_into_vec {
    ($name:ident, $inner:ty) => {
        impl From<$name> for Vec<$inner> {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use derive_more::{Constructor, From};
    use serde::{Deserialize, Serialize};

    #[derive(Constructor, Serialize, Deserialize)]
    pub struct SignUpAction {
        username: String,
        password: String,
    }

    #[derive(Constructor, Serialize, Deserialize)]
    pub struct SendMessageAction {
        from: String,
        to: String,
        text: String,
    }

    #[derive(From, Serialize, Deserialize)]
    pub enum Action {
        SignUp(SignUpAction),
        SendMessage(SendMessageAction),
    }

    impl From<(&str, &str)> for Action {
        fn from((username, password): (&str, &str)) -> Self {
            Self::SignUp(SignUpAction::new(username.into(), password.into()))
        }
    }

    define!(
        #[derive(Serialize, Deserialize)]
        pub struct ActionVec(Vec<Action>);
    );

    define!(
        #[derive(Serialize, Deserialize)]
        pub struct ActionVecWithVariants(Vec<Action>);
        variants = [SignUpAction, SendMessageAction];
    );

    #[test]
    fn supports_from() {
        let mut actions = ActionVec::default();
        // no need to call Action::from
        actions.push(SignUpAction::new("alice".into(), "qwerty".into()));
        // even shorter if you have `impl From<(&str, &str)> for Action`
        actions.push(("alice", "qwerty"));
    }
}
