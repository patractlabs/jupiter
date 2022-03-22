pub struct Executor;

impl sc_executor::NativeExecutionDispatch for Executor {
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        jupiter_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        jupiter_runtime::native_version()
    }
}
