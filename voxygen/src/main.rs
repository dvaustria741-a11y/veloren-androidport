#![deny(unsafe_code)]
#![recursion_limit = "2048"]

#[cfg(all(
    target_os = "windows",
    not(feature = "tracy-memory"),
    not(feature = "hot-egui"),
    not(feature = "hot-anim"),
))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// Allow profiling allocations with Tracy
#[cfg_attr(feature = "tracy-memory", global_allocator)]
#[cfg(feature = "tracy-memory")]
static GLOBAL: common_base::tracy_client::ProfiledAllocator<std::alloc::System> =
    common_base::tracy_client::ProfiledAllocator::new(std::alloc::System, 128);

// Desktop-only entry point. All of the actual bootstrap logic (settings,
// audio, window, global state, main loop) lives in
// `veloren_voxygen::bootstrap::bootstrap_and_run`, which is shared with the
// Android entry point in `veloren_voxygen::android::android_main`. Splitting
// it this way means this file only ever needs to know how to get a
// `cli::Args` on this platform (argv, here) — everything else is identical.
fn main() {
    use clap::Parser;
    let args = veloren_voxygen::cli::Args::parse();
    veloren_voxygen::bootstrap::bootstrap_and_run(args);
}
