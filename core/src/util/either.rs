//! structure and implementations for the type that represents one of the two types

/// structure that represents one of the two types
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Either<L, R> {
    /// one type
    Left(L),
    /// another type
    Right(R),
}

impl<L, R> Either<L, R> {
    /// check self is type of Left
    pub fn is_left(&self) -> bool {
        match self {
            Self::Left(_) => true,
            _ => false,
        }
    }

    /// check self is type of Right
    pub fn is_right(&self) -> bool {
        match self {
            Self::Right(_) => true,
            _ => false,
        }
    }
}
