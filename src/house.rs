use color_eyre::eyre::Result;

#[derive(PartialEq, Debug, Default)]
pub enum LockableState {
    #[default]
    ClosedAndUnlocked,
    Open,
    Locked,
}

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

trait Lockable {
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

mod doors {

    use super::{LockableError, LockableState};

    #[derive(PartialEq, Debug, Default)]
    pub struct Door {
        state: LockableState,
    }

    impl Door {
        pub fn new(state: LockableState) -> Door {
            Door { state }
        }
    }

    impl super::Lockable for Door {
        fn can_open(&self) -> bool {
            matches!(self.state, LockableState::ClosedAndUnlocked)
        }

        fn can_close(&self) -> bool {
            self.is_open()
        }

        fn is_open(&self) -> bool {
            matches!(self.state, LockableState::Open)
        }
        fn is_closed(&self) -> bool {
            !self.is_open()
        }

        fn is_locked(&self) -> bool {
            matches!(self.state, LockableState::Locked)
        }

        fn close(&mut self) -> Result<(), super::LockableError> {
            match self.state {
                LockableState::Open => {
                    self.state = LockableState::ClosedAndUnlocked;
                    Ok(())
                }
                _ => Err(LockableError::AlreadyClosed),
            }
        }

        fn close_and_lock(&mut self) -> Result<(), LockableError> {
            self.close()?;
            self.lock()
        }

        fn lock(&mut self) -> Result<(), super::LockableError> {
            match self.state {
                LockableState::Open => Err(LockableError::Open),
                LockableState::Locked => Err(LockableError::AlreadyLocked),
                LockableState::ClosedAndUnlocked => {
                    self.state = LockableState::Locked;
                    Ok(())
                }
            }
        }

        fn open(&mut self) -> Result<(), super::LockableError> {
            match self.state {
                LockableState::Open => Err(LockableError::AlreadyOpen),
                LockableState::Locked => Err(LockableError::Locked),
                LockableState::ClosedAndUnlocked => {
                    self.state = LockableState::Open;
                    Ok(())
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {

        use crate::house::{Lockable, LockableError};

        use super::*;

        #[test]
        fn closed_by_default() {
            let door = Door::default();
            assert_eq!(door.is_closed(), true);
        }

        #[test]
        fn is_open() {
            let d1 = Door::new(LockableState::Open);
            let d2 = Door::new(LockableState::ClosedAndUnlocked);
            let d3 = Door::new(LockableState::Locked);
            assert_eq!(d1.is_open(), true);
            assert_eq!(d2.is_open(), false);
            assert_eq!(d3.is_open(), false);
        }

        #[test]
        fn is_closed() {
            let d1 = Door::new(LockableState::Open);
            let d2 = Door::new(LockableState::ClosedAndUnlocked);
            let d3 = Door::new(LockableState::Locked);
            assert_eq!(d1.is_closed(), false);
            assert_eq!(d2.is_closed(), true);
            assert_eq!(d3.is_closed(), true);
        }

        #[test]
        fn is_locked() {
            let d1 = Door::new(LockableState::Open);
            let d2 = Door::new(LockableState::ClosedAndUnlocked);
            let d3 = Door::new(LockableState::Locked);
            assert_eq!(d1.is_locked(), false);
            assert_eq!(d2.is_locked(), false);
            assert_eq!(d3.is_closed(), true);
        }

        #[test]
        fn can_open() {
            let d1 = Door::new(LockableState::Open);
            let d2 = Door::new(LockableState::ClosedAndUnlocked);
            let d3 = Door::new(LockableState::Locked);
            assert_eq!(d1.can_open(), false);
            assert_eq!(d2.can_open(), true);
            assert_eq!(d3.can_open(), false);
        }

        #[test]
        fn can_close() {
            let d1 = Door::new(LockableState::Open);
            let d2 = Door::new(LockableState::ClosedAndUnlocked);
            let d3 = Door::new(LockableState::Locked);
            assert_eq!(d1.can_close(), true);
            assert_eq!(d2.can_close(), false);
            assert_eq!(d3.can_close(), false);
        }

        #[test]
        fn try_to_close() {
            let mut d1 = Door::new(LockableState::Open);
            let mut d2 = Door::new(LockableState::ClosedAndUnlocked);
            let mut d3 = Door::new(LockableState::Locked);

            assert_eq!(d1.close().is_ok(), true);
            assert_eq!(d2.close().unwrap_err(), LockableError::AlreadyClosed);
            assert_eq!(d3.close().unwrap_err(), LockableError::AlreadyClosed);

            assert_eq!(d1.is_closed(), true);
            assert_eq!(d2.is_closed(), true);
            assert_eq!(d3.is_closed(), true);
        }

        #[test]
        fn try_to_open() {
            let mut d1 = Door::new(LockableState::Open);
            let mut d2 = Door::new(LockableState::ClosedAndUnlocked);
            let mut d3 = Door::new(LockableState::Locked);

            assert_eq!(d1.open().unwrap_err(), LockableError::AlreadyOpen);
            assert_eq!(d2.open().is_ok(), true);
            assert_eq!(d3.open().unwrap_err(), LockableError::Locked);

            assert_eq!(d1.is_open(), true);
            assert_eq!(d2.is_open(), true);
            assert_eq!(d3.is_open(), false);
        }

        #[test]
        fn try_to_lock() {
            let mut d1 = Door::new(LockableState::Open);
            let mut d2 = Door::new(LockableState::ClosedAndUnlocked);
            let mut d3 = Door::new(LockableState::Locked);

            assert_eq!(d1.lock().unwrap_err(), LockableError::Open);
            assert_eq!(d2.lock().is_ok(), true);
            assert_eq!(d3.lock().unwrap_err(), LockableError::AlreadyLocked);

            assert_eq!(d1.is_locked(), false);
            assert_eq!(d2.is_locked(), true);
            assert_eq!(d3.is_locked(), true);
        }

        #[test]
        fn try_to_close_and_lock() {
            let mut d1 = Door::new(LockableState::Open);
            let mut d2 = Door::new(LockableState::ClosedAndUnlocked);
            let mut d3 = Door::new(LockableState::Locked);

            assert_eq!(d1.close_and_lock().is_ok(), true);
            assert_eq!(d2.close_and_lock().is_ok(), true);
            assert_eq!(
                d3.close_and_lock().unwrap_err(),
                LockableError::AlreadyLocked
            );
        }

        #[test]
        fn can_be_created() {
            let d1 = Door::new(LockableState::Open);
            let d2 = Door::new(LockableState::ClosedAndUnlocked);
            let d3 = Door::new(LockableState::Locked);
            assert_eq!(
                d1,
                Door {
                    state: LockableState::Open
                }
            );
            assert_eq!(
                d2,
                Door {
                    state: LockableState::ClosedAndUnlocked
                }
            );
            assert_eq!(
                d3,
                Door {
                    state: LockableState::Locked
                }
            );
        }
    }
}
