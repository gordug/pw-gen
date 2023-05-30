pub const SPECIAL_CHARS: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
pub const NUMERICAL_CHARS: &str = "0123456789";
pub const LOWERCASE_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";
pub const UPPERCASE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const SIMILAR_CHARS: &str = "012568BGIOQDSZbgiloqdsz";
pub const AMBIGUOUS_SPECIAL_CHARS: &str = "{}[]()/\\'\"`~,;:.<>";
pub const DEFAULT_PASSWORD_LENGTH: usize = 16;
pub const DEFAULT_PASSWORD_OPTIONS: &str = "special, numbers, lowercase, uppercase";
pub const MIN_PASSWORD_LENGTH: usize = 8;