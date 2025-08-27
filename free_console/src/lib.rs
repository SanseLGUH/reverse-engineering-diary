use std::sync::Once;
static START: Once = Once::new();

fn attach() {
    println!("Debug3");
    unsafe { FreeConsole(); }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
extern "system" fn DllMain(call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            START.call_once(|| { attach(); });
        }
        _ => { println!("debug"); }
    }

    true
}


unsafe extern "system" {
    // fn AllocConsole();
    fn FreeConsole() -> bool;
}