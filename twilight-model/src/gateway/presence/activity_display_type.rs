use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum ActivityDisplayType {
    Name,
    State,
    Details,
    Unknown(u8),
}

impl From<u8> for ActivityDisplayType {
    fn from(value: u8) -> Self {
        match value {
            0 => ActivityDisplayType::Name,
            1 => ActivityDisplayType::State,
            2 => ActivityDisplayType::Details,
            unknown => ActivityDisplayType::Unknown(unknown),
        }
    }
}

impl From<ActivityDisplayType> for u8 {
    fn from(value: ActivityDisplayType) -> Self {
        match value {
            ActivityDisplayType::Name => 0,
            ActivityDisplayType::State => 1,
            ActivityDisplayType::Details => 2,
            ActivityDisplayType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ActivityDisplayType;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&ActivityDisplayType::Name, &[Token::U8(0)]);
        serde_test::assert_tokens(&ActivityDisplayType::State, &[Token::U8(1)]);
        serde_test::assert_tokens(&ActivityDisplayType::Details, &[Token::U8(2)]);
        serde_test::assert_tokens(&ActivityDisplayType::Unknown(99), &[Token::U8(99)]);
    }
}
