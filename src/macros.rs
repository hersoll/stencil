#[macro_export]
macro_rules! metadata {
    (
        map $struct_name:ident to $enum_name:ident {
            $(
                $subject:ident : ( $method_name:ident , $difficulty:expr $(, $weight:expr)? )
            ),* $(,)?
        }
    ) => {
        use crate::problems::{Difficulty, Problem, ProblemBuilder, Config};


        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $enum_name {
            $( $subject ),*
        }

        #[derive(Debug, Default)]
        pub struct $struct_name {
            config: Config<$enum_name>,
        }

        impl ProblemBuilder for $struct_name {
            type ProblemId = $enum_name;

            fn new() -> Self {
                Self {config: Config::default()}
            }

            fn config(&mut self) -> &mut Config<Self::ProblemId> {
                &mut self.config
            }

            fn read_config(&self) -> &Config<Self::ProblemId> {
                &self.config
            }

            fn problem_registry(&mut self) -> Vec<(Self::ProblemId, fn(&Self) -> Problem, u8, Difficulty)> {
                vec![
                    $(
                        (
                            <$enum_name>::$subject,
                            $struct_name::$method_name,
                            metadata!(@unwrap_weight $( $weight )?),
                            $difficulty
                        )
                    ),*
                ]
            }
        }
    };

    (@unwrap_weight) => { 1 };
    (@unwrap_weight $weight:expr) => { $weight };
}
