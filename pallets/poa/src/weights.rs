use frame_support::weights::Weight;

pub trait WeightInfo {
    fn add_authority() -> Weight;
    fn remove_authority() -> Weight;
    fn set_invulnerables(_: u32) -> Weight;
    fn cancel_deferred_slash(_: u32) -> Weight;
    fn force_no_eras() -> Weight;
    fn force_new_era() -> Weight;
    fn force_new_era_always() -> Weight;
    fn set_history_depth(e: u32, ) -> Weight;
}
impl WeightInfo for () {
    fn add_authority() -> Weight {
        0
    }

    fn remove_authority() -> Weight {
        0
    }

    fn set_invulnerables(_: u32) -> Weight {
        0
    }

    fn cancel_deferred_slash(_: u32) -> Weight {
        0
    }

    fn force_no_eras() -> Weight {
        0
    }

    fn force_new_era() -> Weight {
        0
    }

    fn force_new_era_always() -> Weight {
        0
    }

    fn set_history_depth(_e: u32, ) -> Weight {
        0
    }
}
