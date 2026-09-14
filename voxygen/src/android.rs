//! Android entry point.
//!
//! This is Stage 3 of the Android port: gives Android something to actually
//! launch. It does three things:
//!   1. Stashes the `AndroidApp` winit hands us (it's not global state, so
//!      `Window::new` — which doesn't know about Android specifics — needs
//!      a way to get at it; see `build_event_loop` below).
//!   2. Builds a winit `EventLoop` bound to that `AndroidApp`. This has to
//!      happen before any window is created, and is the one place Android
//!      genuinely differs from desktop at the entry-point level (see
//!      `Window::new` in `window.rs` for the other half of this).
//!   3. Calls the same `bootstrap::bootstrap_and_run` that desktop's
//!      `main.rs` calls, with a default `cli::Args` (there's no argv on
//!      Android).
//!
//! Deliberately NOT covered yet: touch input, lifecycle (suspend/resume,
//! surface loss on backgrounding), or asset loading from the APK — those
//! are later stages. Right now the goal is just "winit gets far enough to
//! open a window", which is itself the next thing to verify on-device.

use std::sync::OnceLock;

use clap::Parser;
use winit::{
    event_loop::EventLoopBuilder,
    platform::android::{EventLoopBuilderExtAndroid, activity::AndroidApp},
};

use crate::window::EventLoop;

static ANDROID_APP: OnceLock<AndroidApp> = OnceLock::new();

/// Build the winit event loop for Android. Panics if called before
/// `android_main` has stashed the `AndroidApp` (i.e. if something tries to
/// create a `Window` before the app has actually started) or if called more
/// than once (winit only allows one event loop per process).
pub(crate) fn build_event_loop() -> EventLoop {
    let app = ANDROID_APP
        .get()
        .expect("build_event_loop called before android_main initialized AndroidApp")
        .clone();

    EventLoopBuilder::default()
        .with_android_app(app)
        .build()
        .expect("failed to build Android event loop")
}

#[expect(unsafe_code)]
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    if ANDROID_APP.set(app).is_err() {
        panic!("android_main called more than once");
    }

    // No argv on Android — Args' fields all have defaults (see cli.rs), so
    // parsing an empty argument list just gives us those defaults.
    let args = crate::cli::Args::parse_from(["veloren"]);

    crate::bootstrap::bootstrap_and_run(args);
}
