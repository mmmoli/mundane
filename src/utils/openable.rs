#[derive(PartialEq, Debug)]
pub enum OpenableError {
    AlreadyClosed,
    AlreadyOpen,
    CannotOpen,
}

impl std::error::Error for OpenableError {}
impl std::fmt::Display for OpenableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            OpenableError::AlreadyClosed => write!(f, "This is already closed"),
            OpenableError::AlreadyOpen => write!(f, "This is already open"),
            OpenableError::CannotOpen => write!(f, "This cannot be opened."),
        }
    }
}

pub trait Openable {
    fn can_open(&self) -> bool;
    fn can_close(&self) -> bool;
    fn is_open(&self) -> bool;
    fn is_closed(&self) -> bool;
    fn open(&mut self) -> Result<(), OpenableError>;
    fn close(&mut self) -> Result<(), OpenableError>;
}

#[derive(PartialEq, Debug, Default)]
pub enum OpenableState {
    #[default]
    Closed,
    Open,
}

impl Openable for OpenableState {
    fn is_open(&self) -> bool {
        matches!(self, OpenableState::Open)
    }
    fn is_closed(&self) -> bool {
        matches!(self, OpenableState::Closed)
    }

    fn can_open(&self) -> bool {
        self.is_closed()
    }

    fn can_close(&self) -> bool {
        !self.can_open()
    }

    fn close(&mut self) -> Result<(), OpenableError> {
        match self {
            OpenableState::Open => {
                *self = OpenableState::Closed;
                Ok(())
            }
            _ => Err(OpenableError::AlreadyClosed),
        }
    }

    fn open(&mut self) -> Result<(), OpenableError> {
        match self {
            OpenableState::Open => Err(OpenableError::AlreadyOpen),
            OpenableState::Closed => {
                *self = OpenableState::Open;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn closed_by_default() {
        let d = OpenableState::default();
        assert_eq!(d.is_closed(), true);
    }

    #[test]
    fn is_open() {
        let w1 = OpenableState::Open;
        let w2 = OpenableState::Closed;

        assert_eq!(w1.is_open(), true);
        assert_eq!(w2.is_open(), false);
    }

    #[test]
    fn is_closed() {
        let w1 = OpenableState::Open;
        let w2 = OpenableState::Closed;

        assert_eq!(w1.is_closed(), false);
        assert_eq!(w2.is_closed(), true);
    }

    #[test]
    fn can_open() {
        let w1 = OpenableState::Open;
        let w2 = OpenableState::Closed;

        assert_eq!(w1.can_open(), false);
        assert_eq!(w2.can_open(), true);
    }

    #[test]
    fn can_close() {
        let w1 = OpenableState::Open;
        let w2 = OpenableState::Closed;

        assert_eq!(w1.can_close(), true);
        assert_eq!(w2.can_close(), false);
    }

    #[test]
    fn try_to_close() {
        let mut w1 = OpenableState::Open;
        let mut w2 = OpenableState::Closed;

        assert_eq!(w1.close().is_ok(), true);
        assert_eq!(w2.close().unwrap_err(), OpenableError::AlreadyClosed);

        assert_eq!(w1, OpenableState::Closed);
        assert_eq!(w2, OpenableState::Closed);
    }

    #[test]
    fn try_to_open() {
        let mut w1 = OpenableState::Open;
        let mut w2 = OpenableState::Closed;

        assert_eq!(w1.open().unwrap_err(), OpenableError::AlreadyOpen);
        assert_eq!(w2.open().is_ok(), true);

        assert_eq!(w1, OpenableState::Open);
        assert_eq!(w2, OpenableState::Open);
    }
}
