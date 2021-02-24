use frame_support::weights::Weight;

pub trait WeightInfo {
    fn add_authority() -> Weight;
    fn remove_authority() -> Weight;
}
