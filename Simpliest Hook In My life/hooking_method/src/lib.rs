use std::sync::Once;
static START: Once = Once::new();

const PTR: usize = 0x7FF714081120;

fn attach() {
    let function: extern "system" fn(i32, bool) = unsafe { std::mem::transmute(PTR) };
    
    unsafe { 
        function(727, true);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
extern "system" fn DllMain(call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            START.call_once(|| { attach(); });
        }
        _ => {}
    }

    true
}