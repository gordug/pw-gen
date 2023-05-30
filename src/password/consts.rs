pub(crate) const SPECIAL_CHARS: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
pub(crate) const NUMERICAL_CHARS: &str = "0123456789";
pub(crate) const LOWERCASE_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";
pub(crate) const UPPERCASE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub(crate) const SIMILAR_CHARS: &str = "012568BGIOQDSZbgiloqdsz";
pub(crate) const AMBIGUOUS_SPECIAL_CHARS: &str = "{}[]()/\\'\"`~,;:.<>";
pub(crate) const DEFAULT_PASSWORD_LENGTH: usize = 16;
pub(crate) const DEFAULT_PASSWORD_OPTIONS: &str = "special, numbers, lowercase, uppercase";
pub(crate) const MIN_PASSWORD_LENGTH: usize = 8;