pub struct Executor;

impl sc_executor::NativeExecutionDispatch for Executor {
    // type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;
    #[cfg(feature = "runtime-benchmarks")]
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;
    /// Otherwise we only use the default Substrate host functions.
    #[cfg(not(feature = "runtime-benchmarks"))]
    type ExtendHostFunctions = ();

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        jupiter_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        jupiter_runtime::native_version()
    }
}
