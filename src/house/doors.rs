use crate::utils::{
    lockable::{Lockable, LockableError, LockableState},
    openable::{Openable, OpenableError, OpenableState},
};

#[derive(PartialEq, Debug)]
pub struct Door {
    open_state: OpenableState,
    lock_state: LockableState,
}

impl Openable for Door {
    fn can_open(&self) -> bool {
        self.open_state.can_open()
    }

    fn can_close(&self) -> bool {
        self.open_state.can_close()
    }

    fn is_open(&self) -> bool {
        self.open_state.is_open()
    }

    fn is_closed(&self) -> bool {
        self.open_state.is_closed()
    }

    fn open(&mut self) -> Result<(), OpenableError> {
        match self.unlock() {
            Ok(()) => self.open_state.open(),
            Err(_) => Err(OpenableError::CannotOpen),
        }
    }

    fn close(&mut self) -> Result<(), OpenableError> {
        self.open_state.close()
    }
}

impl Lockable for Door {
    fn is_locked(&self) -> bool {
        self.lock_state.is_locked()
    }

    fn is_unlocked(&self) -> bool {
        self.lock_state.is_unlocked()
    }

    fn can_lock(&self) -> bool {
        self.lock_state.can_lock()
    }

    fn can_unlock(&self) -> bool {
        self.lock_state.can_unlock()
    }

    fn lock(&mut self) -> Result<(), LockableError> {
        let _ = self.open_state.close();
        self.lock_state.lock()
    }

    fn unlock(&mut self) -> Result<(), LockableError> {
        self.lock_state.unlock()
    }
}

impl Door {
    pub fn builder() -> DoorBuilder {
        DoorBuilder::default()
    }
}

#[derive(Default)]
pub struct DoorBuilder {}

impl DoorBuilder {
    pub fn new() -> DoorBuilder {
        DoorBuilder {}
    }
}
