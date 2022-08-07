#[derive(PartialEq, Debug)]
pub enum OccupiableError {
    AlreadyOccupied,
    AlreadyVacant,
}

impl std::error::Error for OccupiableError {}
impl std::fmt::Display for OccupiableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            OccupiableError::AlreadyOccupied => write!(f, "Already occupied"),
            OccupiableError::AlreadyVacant => write!(f, "Already vacant"),
        }
    }
}

pub trait Occupiable {
    fn can_occupy(&self) -> bool;
    fn can_vacate(&self) -> bool;
    fn is_occupied(&self) -> bool;
    fn is_vacant(&self) -> bool;
    fn occupy(&mut self) -> Result<(), OccupiableError>;
    fn vacate(&mut self) -> Result<(), OccupiableError>;
}

#[derive(PartialEq, Debug, Default)]
pub enum OccupiableState {
    #[default]
    Vacant,
    Occupied,
}

impl Occupiable for OccupiableState {
    fn can_occupy(&self) -> bool {
        matches!(&self, OccupiableState::Vacant)
    }

    fn can_vacate(&self) -> bool {
        !self.can_occupy()
    }

    fn is_occupied(&self) -> bool {
        matches!(&self, OccupiableState::Occupied)
    }

    fn is_vacant(&self) -> bool {
        !self.is_occupied()
    }

    fn occupy(&mut self) -> Result<(), OccupiableError> {
        match &self {
            OccupiableState::Occupied => Err(OccupiableError::AlreadyOccupied),
            OccupiableState::Vacant => {
                *self = OccupiableState::Occupied;
                Ok(())
            }
        }
    }

    fn vacate(&mut self) -> Result<(), OccupiableError> {
        match &self {
            OccupiableState::Vacant => Err(OccupiableError::AlreadyVacant),
            OccupiableState::Occupied => {
                *self = OccupiableState::Vacant;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn vacant_by_default() {
        let d = OccupiableState::default();
        assert_eq!(d.is_vacant(), true);
    }

    #[test]
    fn is_occupied() {
        let o1 = OccupiableState::Occupied;
        let o2 = OccupiableState::Vacant;
        assert_eq!(o1.is_occupied(), true);
        assert_eq!(o2.is_occupied(), false);
    }

    #[test]
    fn is_vacant() {
        let o1 = OccupiableState::Occupied;
        let o2 = OccupiableState::Vacant;
        assert_eq!(o1.is_vacant(), false);
        assert_eq!(o2.is_vacant(), true);
    }

    #[test]
    fn can_occupy() {
        let o1 = OccupiableState::Occupied;
        let o2 = OccupiableState::Vacant;
        assert_eq!(o1.can_occupy(), false);
        assert_eq!(o2.can_occupy(), true);
    }

    #[test]
    fn can_vacate() {
        let o1 = OccupiableState::Occupied;
        let o2 = OccupiableState::Vacant;
        assert_eq!(o1.can_vacate(), true);
        assert_eq!(o2.can_vacate(), false);
    }

    #[test]
    fn try_to_occupy() {
        let mut o1 = OccupiableState::Occupied;
        let mut o2 = OccupiableState::Vacant;

        assert_eq!(o1.occupy().unwrap_err(), OccupiableError::AlreadyOccupied);
        assert_eq!(o2.occupy().is_ok(), true);

        assert_eq!(o1, OccupiableState::Occupied);
        assert_eq!(o2, OccupiableState::Occupied);
    }

    #[test]
    fn try_to_vacate() {
        let mut o1 = OccupiableState::Occupied;
        let mut o2 = OccupiableState::Vacant;

        assert_eq!(o1.vacate().is_ok(), true);
        assert_eq!(o2.vacate().unwrap_err(), OccupiableError::AlreadyVacant);

        assert_eq!(o1, OccupiableState::Vacant);
        assert_eq!(o2, OccupiableState::Vacant);
    }
}
