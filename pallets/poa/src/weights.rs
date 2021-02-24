use frame_support::weights::Weight;

pub trait WeightInfo {
    fn add_authority() -> Weight;
    fn remove_authority() -> Weight;
}
impl WeightInfo for () {
    fn add_authority() -> u64 {
        0
    }

    fn remove_authority() -> u64 {
        0
    }
}
