use std::path::Path;

use sory_arg0::Arg0DispatchPaths;
use sory_arg0::Arg0PathEntryGuard;
use sory_arg0::arg0_dispatch;
use tempfile::TempDir;

pub struct TestBinaryDispatchGuard {
    _sory_home: TempDir,
    arg0: Arg0PathEntryGuard,
    _previous_sory_home: Option<std::ffi::OsString>,
}

impl TestBinaryDispatchGuard {
    pub fn paths(&self) -> &Arg0DispatchPaths {
        self.arg0.paths()
    }
}

pub enum TestBinaryDispatchMode {
    DispatchArg0Only,
    Skip,
    InstallAliases,
}

pub fn configure_test_binary_dispatch<F>(
    sory_home_prefix: &str,
    classify: F,
) -> Option<TestBinaryDispatchGuard>
where
    F: FnOnce(&str, Option<&str>) -> TestBinaryDispatchMode,
{
    let mut args = std::env::args_os();
    let argv0 = args.next().unwrap_or_default();
    let exe_name = Path::new(&argv0)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    let argv1 = args.next();
    match classify(exe_name, argv1.as_deref().and_then(|arg| arg.to_str())) {
        TestBinaryDispatchMode::DispatchArg0Only => {
            let _ = arg0_dispatch();
            None
        }
        TestBinaryDispatchMode::Skip => None,
        TestBinaryDispatchMode::InstallAliases => {
            let sory_home = match tempfile::Builder::new().prefix(sory_home_prefix).tempdir() {
                Ok(sory_home) => sory_home,
                Err(error) => panic!("failed to create test sory_HOME: {error}"),
            };
            let previous_sory_home = std::env::var_os("sory_HOME");
            // Safety: this runs from a test ctor before test threads begin.
            unsafe {
                std::env::set_var("sory_HOME", sory_home.path());
            }

            let arg0 = match arg0_dispatch() {
                Some(arg0) => arg0,
                None => panic!("failed to configure arg0 dispatch aliases for test binary"),
            };
            match previous_sory_home.as_ref() {
                Some(value) => unsafe {
                    std::env::set_var("sory_HOME", value);
                },
                None => unsafe {
                    std::env::remove_var("sory_HOME");
                },
            }

            Some(TestBinaryDispatchGuard {
                _sory_home: sory_home,
                arg0,
                _previous_sory_home: previous_sory_home,
            })
        }
    }
}
