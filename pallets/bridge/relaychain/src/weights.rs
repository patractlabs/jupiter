use frame_support::weights::Weight;

pub trait WeightInfo {
    fn submit_ksm_extrinsic() -> Weight;
}
