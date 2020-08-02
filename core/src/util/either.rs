#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn is_left(&self) -> bool {
        match self {
            Self::Left(l) => true,
            _ => false,
        }
    }

    pub fn is_right(&self) -> bool {
        match self {
            Self::Right(r) => true,
            _ => false,
        }
    }
}
