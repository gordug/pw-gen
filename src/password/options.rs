#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub(crate) enum PasswordOptions {
    Special {
        required: bool,
    },
    Numbers{
        required: bool,
    },
    Lowercase{
        required: bool,
    },
    Uppercase{
        required: bool,
    },
    NotSequential,
    NotSimilar,
    NotAmbiguous,
}