use crate::utils::occupiable::OccupiableState;

#[derive(PartialEq, Debug, Default)]
pub struct Chair {
    pub occupation_state: OccupiableState,
}

impl Chair {
    pub fn new() -> Chair {
        Chair {
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::occupiable::Occupiable;

    #[test]
    fn vacant_by_default() {
        let c1 = Chair::default();
        assert_eq!(c1.occupation_state.is_vacant(), true);
        assert_eq!(c1.occupation_state.is_occupied(), false);
    }

    // #[test]
    // fn can_occupy() {
    //     let c1 = Chair::new();
    //     assert_eq!(c1.occupy.ok(), Chair {});
    // }

    // #[test]
    // fn can_vacate() {
    //     let c1 = Chair::new();
    //     assert_eq!(c1, Chair {});
    // }
}
