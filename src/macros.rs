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

//#################################
//#       FORMATTING MATH         #
//#################################

#[macro_export]
macro_rules! fmt_const {
    ($const:expr) => {{
        match $const {
            0 => String::new(),
            n => format!("{:+}", n),
        }
    }};
}

#[macro_export]
macro_rules! fmt_const_first {
    ($const:expr) => {{
        match $const {
            0 => String::new(),
            n => format!("{}", n),
        }
    }};
}

/// Adjusts the coefficient of an algebraic term, with a leading sign
///
/// If the coefficient is 0, the whole expression vanishes.
/// # Examples:
/// assert_eq!(fmt_term!(2, "x"), "+2x".to_string());
/// assert_eq!(fmt_term!(1, "x"), "+x".to_string());
/// assert_eq!(fmt_term!(-1, "x"), "-x".to_string());
/// assert_eq!(fmt_term!(0, "x"), String::new());
#[macro_export]
macro_rules! fmt_term {
    ($coeff:expr, $var:expr) => {{
        match $coeff {
            0 => String::new(),
            1 => format!("+{}", $var),
            -1 => format!("-{}", $var),
            n if n > 0 => format!("+{}{}", n, $var),
            n => format!("{}{}", n, $var),
        }
    }};
}

/// Adjusts the coefficient of an algebraic term, without leading +, as if it was the first term in
/// an expression.
///
/// If the coefficient is 0, the whole expression vanishes.
/// # Examples:
/// assert_eq!(fmt_term_first!(2, "x"), "2x".to_string());
/// assert_eq!(fmt_term_first!(1, "x"), "x".to_string());
/// assert_eq!(fmt_term_first!(-1, "x"), "-x".to_string());
/// assert_eq!(fmt_term_first!(0, "x"), String::new());
#[macro_export]
macro_rules! fmt_term_first {
    ($coeff:expr, $var:expr) => {{
        match $coeff {
            0 => String::new(),
            1 => format!("{}", $var),
            -1 => format!("-{}", $var),
            n => format!("{}{}", n, $var),
        }
    }};
}
