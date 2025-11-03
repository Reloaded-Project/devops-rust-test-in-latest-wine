/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// use wine_test_dummy::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// C-compatible add function for FFI.
#[no_mangle]
pub extern "C" fn add_c(a: i32, b: i32) -> i32 {
    add(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    mod wine_detection {
        use std::ffi::CStr;

        // Windows API types
        type HMODULE = *mut std::ffi::c_void;
        type FARPROC = *mut std::ffi::c_void;

        // Windows API functions for runtime resolution
        #[link(name = "kernel32")]
        extern "system" {
            fn LoadLibraryA(lpFileName: *const u8) -> HMODULE;
            fn GetProcAddress(hModule: HMODULE, lpProcName: *const u8) -> FARPROC;
        }

        // Function pointer type for wine_get_version
        type WineGetVersionFn = unsafe extern "C" fn() -> *const i8;

        /// Prints the Wine version if running under Wine, using runtime resolution.
        ///
        /// This function uses `println!` to output the Wine version. When running tests,
        /// you must pass `-- --show-output` to see this output:
        /// ```bash
        /// cargo test -- --show-output
        /// ```
        pub fn print_wine_version() {
            unsafe {
                // Load ntdll.dll at runtime
                let ntdll_name = b"ntdll.dll\0";
                let ntdll = LoadLibraryA(ntdll_name.as_ptr());
                
                if ntdll.is_null() {
                    println!("Failed to load ntdll.dll");
                    return;
                }

                // Try to get wine_get_version function pointer at runtime
                let func_name = b"wine_get_version\0";
                let wine_get_version_ptr = GetProcAddress(ntdll, func_name.as_ptr());

                if wine_get_version_ptr.is_null() {
                    println!("Not running under Wine (wine_get_version function not found in ntdll.dll)");
                    return;
                }

                // Cast the function pointer and call it
                let wine_get_version: WineGetVersionFn = std::mem::transmute(wine_get_version_ptr);
                let version_ptr = wine_get_version();
                
                if !version_ptr.is_null() {
                    if let Ok(version_cstr) = CStr::from_ptr(version_ptr).to_str() {
                        println!("Running under Wine version: {}", version_cstr);
                    } else {
                        println!("Wine version string is not valid UTF-8");
                    }
                } else {
                    println!("wine_get_version returned null pointer");
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    fn print_wine_version() {
        wine_detection::print_wine_version();
    }

    #[cfg(not(target_os = "windows"))]
    fn print_wine_version() {
        println!("Not running on Windows target");
    }

    #[test]
    fn test_add() {
        // NOTE: Wine version output uses println! and requires running tests with:
        // cargo test -- --show-output
        print_wine_version();
        assert_eq!(add(2, 2), 4);
    }
}
