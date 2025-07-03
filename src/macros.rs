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
