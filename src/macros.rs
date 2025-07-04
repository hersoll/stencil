#[macro_export]
macro_rules! collect_into {
    (
        $name:ident {
            $($const_name:ident = $value:expr),+ $(,)?
        }
    ) => {
        pub struct $name;

        impl $name {
            $(pub const $const_name: &'static crate::problems::ProblemType = &$value;)+

            const ALL: &'static [&'static crate::problems::ProblemType] = &[
                $(Self::$const_name),+
            ];
        }

        impl crate::problems::ProblemArea for $name {
            fn get_problem_types() -> &'static [&'static crate::problems::ProblemType] {
                &Self::ALL
            }
        }
    };
}

// Macro to define the error struct
#[macro_export]
macro_rules! error {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name {
            message: String,
        }

        impl $name {
            pub fn new(message: impl Into<String>) -> Self {
                Self {
                    message: message.into(),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.message)
            }
        }

        impl std::error::Error for $name {}
    };
}

// Macro to construct an error of the given type
#[macro_export]
macro_rules! throw {
    ($err_ty:ident, $($arg:tt)*) => {
        $err_ty::new(format!($($arg)*))
    };
}
