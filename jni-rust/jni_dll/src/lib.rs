use jni::{JavaVM, JNIVersion};
use jni::InitArgsBuilder;
use jni::errors::Result;
use jni::strings::JNIStr;
use jni::signature::RuntimeMethodSignature;

use std::sync::Once;
static START: Once = Once::new();

fn attach() {
    let jvm = get_existing_vm().expect("no JVM running in this process");

    jvm.attach_current_thread(|env| -> Result<()> {
        let class = env.find_class(JNIStr::from_cstr(c"Hello").unwrap()).unwrap();

        let sig = RuntimeMethodSignature::from_str("()V")?;
        env.call_static_method(
            class,
            JNIStr::from_cstr(c"goal").unwrap(),
            sig.method_signature(),
            &[],
        )?;

        Ok(())
    }).expect("Something");

}

use jni::sys;
use std::ptr;
use std::panic;

fn get_existing_vm() -> Option<JavaVM> {
    unsafe {
        let mut raw_vm: *mut sys::JavaVM = ptr::null_mut();
        let mut count: jni::sys::jsize = 0;
        let res = sys::JNI_GetCreatedJavaVMs(&mut raw_vm, 1, &mut count);
        if res != sys::JNI_OK || count == 0 || raw_vm.is_null() {
            return None;
        }
        Some(JavaVM::from_raw(raw_vm))
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