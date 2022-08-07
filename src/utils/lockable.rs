#[derive(PartialEq, Debug)]
pub enum LockableError {
    Locked,
    Open,
    AlreadyClosed,
    AlreadyLocked,
    AlreadyOpen,
}

impl std::error::Error for LockableError {}
impl std::fmt::Display for LockableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            LockableError::AlreadyClosed => write!(f, "This is already closed"),
            LockableError::AlreadyLocked => write!(f, "This is already locked"),
            LockableError::AlreadyOpen => write!(f, "This is already open"),
            LockableError::Locked => write!(f, "This can't be opened because it's locked"),
            LockableError::Open => write!(f, "This can't be locked because it's open."),
        }
    }
}

pub trait Lockable {
    fn can_close(&self) -> bool;
    fn can_open(&self) -> bool;
    fn close_and_lock(&mut self) -> Result<(), LockableError>;
    fn close(&mut self) -> Result<(), LockableError>;
    fn is_closed(&self) -> bool;
    fn is_locked(&self) -> bool;
    fn is_open(&self) -> bool;
    fn lock(&mut self) -> Result<(), LockableError>;
    fn open(&mut self) -> Result<(), LockableError>;
}

#[derive(PartialEq, Debug, Default)]
pub enum LockableState {
    #[default]
    ClosedAndUnlocked,
    Open,
    Locked,
}

impl Lockable for LockableState {
    fn is_open(&self) -> bool {
        matches!(self, LockableState::Open)
    }

    fn is_closed(&self) -> bool {
        !self.is_open()
    }

    fn can_open(&self) -> bool {
        matches!(self, LockableState::ClosedAndUnlocked)
    }

    fn can_close(&self) -> bool {
        self.is_open()
    }

    fn is_locked(&self) -> bool {
        matches!(self, LockableState::Locked)
    }

    fn close(&mut self) -> Result<(), LockableError> {
        match self {
            LockableState::Open => {
                *self = LockableState::ClosedAndUnlocked;
                Ok(())
            }
            _ => Err(LockableError::AlreadyClosed),
        }
    }

    fn close_and_lock(&mut self) -> Result<(), LockableError> {
        let _ = self.close();
        self.lock()
    }

    fn lock(&mut self) -> Result<(), LockableError> {
        match self {
            LockableState::Open => Err(LockableError::Open),
            LockableState::Locked => Err(LockableError::AlreadyLocked),
            LockableState::ClosedAndUnlocked => {
                *self = LockableState::Locked;
                Ok(())
            }
        }
    }

    fn open(&mut self) -> Result<(), LockableError> {
        match self {
            LockableState::Open => Err(LockableError::AlreadyOpen),
            LockableState::Locked => Err(LockableError::Locked),
            LockableState::ClosedAndUnlocked => {
                *self = LockableState::Open;
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
        let d = LockableState::default();
        assert_eq!(d.is_closed(), true);
    }

    #[test]
    fn is_open() {
        let w1 = LockableState::Open;
        let w2 = LockableState::ClosedAndUnlocked;
        let w3 = LockableState::Locked;
        assert_eq!(w1.is_open(), true);
        assert_eq!(w2.is_open(), false);
        assert_eq!(w3.is_open(), false);
    }

    #[test]
    fn is_closed() {
        let w1 = LockableState::Open;
        let w2 = LockableState::ClosedAndUnlocked;
        let w3 = LockableState::Locked;
        assert_eq!(w1.is_closed(), false);
        assert_eq!(w2.is_closed(), true);
        assert_eq!(w3.is_closed(), true);
    }

    #[test]
    fn is_locked() {
        let w1 = LockableState::Open;
        let w2 = LockableState::ClosedAndUnlocked;
        let w3 = LockableState::Locked;
        assert_eq!(w1.is_locked(), false);
        assert_eq!(w2.is_locked(), false);
        assert_eq!(w3.is_closed(), true);
    }

    #[test]
    fn can_open() {
        let w1 = LockableState::Open;
        let w2 = LockableState::ClosedAndUnlocked;
        let w3 = LockableState::Locked;
        assert_eq!(w1.can_open(), false);
        assert_eq!(w2.can_open(), true);
        assert_eq!(w3.can_open(), false);
    }

    #[test]
    fn can_close() {
        let w1 = LockableState::Open;
        let w2 = LockableState::ClosedAndUnlocked;
        let w3 = LockableState::Locked;
        assert_eq!(w1.can_close(), true);
        assert_eq!(w2.can_close(), false);
        assert_eq!(w3.can_close(), false);
    }

    #[test]
    fn try_to_close() {
        let mut w1 = LockableState::Open;
        let mut w2 = LockableState::ClosedAndUnlocked;
        let mut w3 = LockableState::Locked;

        assert_eq!(w1.close().is_ok(), true);
        assert_eq!(w2.close().unwrap_err(), LockableError::AlreadyClosed);
        assert_eq!(w3.close().unwrap_err(), LockableError::AlreadyClosed);

        assert_eq!(w1.is_closed(), true);
        assert_eq!(w2.is_closed(), true);
        assert_eq!(w3.is_closed(), true);
    }

    #[test]
    fn try_to_open() {
        let mut w1 = LockableState::Open;
        let mut w2 = LockableState::ClosedAndUnlocked;
        let mut w3 = LockableState::Locked;

        assert_eq!(w1.open().unwrap_err(), LockableError::AlreadyOpen);
        assert_eq!(w2.open().is_ok(), true);
        assert_eq!(w3.open().unwrap_err(), LockableError::Locked);

        assert_eq!(w1.is_open(), true);
        assert_eq!(w2.is_open(), true);
        assert_eq!(w3.is_open(), false);
    }

    #[test]
    fn try_to_lock() {
        let mut w1 = LockableState::Open;
        let mut w2 = LockableState::ClosedAndUnlocked;
        let mut w3 = LockableState::Locked;

        assert_eq!(w1.lock().unwrap_err(), LockableError::Open);
        assert_eq!(w2.lock().is_ok(), true);
        assert_eq!(w3.lock().unwrap_err(), LockableError::AlreadyLocked);

        assert_eq!(w1.is_locked(), false);
        assert_eq!(w2.is_locked(), true);
        assert_eq!(w3.is_locked(), true);
    }

    #[test]
    fn try_to_close_and_lock() {
        let mut w1 = LockableState::Open;
        let mut w2 = LockableState::ClosedAndUnlocked;
        let mut w3 = LockableState::Locked;

        assert_eq!(w1.close_and_lock().is_ok(), true);
        assert_eq!(w2.close_and_lock().is_ok(), true);
        assert_eq!(
            w3.close_and_lock().unwrap_err(),
            LockableError::AlreadyLocked
        );
    }
}
