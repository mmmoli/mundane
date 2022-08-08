#[derive(PartialEq, Debug)]
pub enum LockableError {
    AlreadyLocked,
    AlreadyUnlocked,
    Locked,
}

impl std::error::Error for LockableError {}
impl std::fmt::Display for LockableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            LockableError::AlreadyLocked => write!(f, "This is already locked"),
            LockableError::AlreadyUnlocked => write!(f, "This is already unlocked"),
            LockableError::Locked => write!(f, "This is locked"),
        }
    }
}

pub trait Lockable {
    fn is_locked(&self) -> bool;
    fn is_unlocked(&self) -> bool;
    fn can_lock(&self) -> bool;
    fn can_unlock(&self) -> bool;
    fn lock(&mut self) -> Result<(), LockableError>;
    fn unlock(&mut self) -> Result<(), LockableError>;
}

#[derive(PartialEq, Debug, Default)]
pub enum LockableState {
    #[default]
    Unlocked,
    Locked,
}

impl Lockable for LockableState {
    fn is_locked(&self) -> bool {
        matches!(self, LockableState::Locked)
    }

    fn is_unlocked(&self) -> bool {
        !self.is_locked()
    }

    fn can_lock(&self) -> bool {
        matches!(self, LockableState::Unlocked)
    }

    fn can_unlock(&self) -> bool {
        !self.can_lock()
    }

    fn lock(&mut self) -> Result<(), LockableError> {
        match self {
            LockableState::Unlocked => {
                *self = LockableState::Locked;
                Ok(())
            }
            _ => Err(LockableError::AlreadyLocked),
        }
    }

    fn unlock(&mut self) -> Result<(), LockableError> {
        match self {
            LockableState::Locked => {
                *self = LockableState::Unlocked;
                Ok(())
            }
            _ => Err(LockableError::AlreadyUnlocked),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn unlocked_by_default() {
        let d = LockableState::default();
        assert_eq!(d, LockableState::Unlocked);
    }

    #[test]
    fn is_locked() {
        let w1 = LockableState::Locked;
        let w2 = LockableState::Unlocked;

        assert_eq!(w1.is_locked(), true);
        assert_eq!(w2.is_locked(), false);
    }

    #[test]
    fn is_unlocked() {
        let w1 = LockableState::Locked;
        let w2 = LockableState::Unlocked;

        assert_eq!(w1.is_unlocked(), false);
        assert_eq!(w2.is_unlocked(), true);
    }

    #[test]
    fn can_lock() {
        let w1 = LockableState::Locked;
        let w2 = LockableState::Unlocked;
        assert_eq!(w1.can_lock(), false);
        assert_eq!(w2.can_lock(), true);
    }

    #[test]
    fn can_unlock() {
        let w1 = LockableState::Locked;
        let w2 = LockableState::Unlocked;
        assert_eq!(w1.can_unlock(), true);
        assert_eq!(w2.can_unlock(), false);
    }

    #[test]
    fn try_to_lock() {
        let mut w1 = LockableState::Locked;
        let mut w2 = LockableState::Unlocked;

        assert_eq!(w1.lock().unwrap_err(), LockableError::AlreadyLocked);
        assert_eq!(w2.lock().is_ok(), true);

        assert_eq!(w1, LockableState::Locked);
        assert_eq!(w2, LockableState::Locked);
    }

    #[test]
    fn try_to_unlock() {
        let mut w1 = LockableState::Locked;
        let mut w2 = LockableState::Unlocked;

        assert_eq!(w1.unlock().is_ok(), true);
        assert_eq!(w2.unlock().unwrap_err(), LockableError::AlreadyUnlocked);

        assert_eq!(w1, LockableState::Unlocked);
        assert_eq!(w2, LockableState::Unlocked);
    }
}
