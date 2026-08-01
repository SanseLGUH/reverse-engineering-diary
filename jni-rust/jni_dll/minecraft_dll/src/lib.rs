use jni::{
    JavaVM, JNIVersion, InitArgsBuilder, jni_str, 
    errors::Result, signature::{RuntimeMethodSignature, RuntimeFieldSignature, ReturnType, Primitive::Void}
};
use std::{thread, time::Duration, sync::Once};
static START: Once = Once::new();

fn attach() {
    let jvm = get_existing_vm().expect("no JVM running in this process");

    jvm.attach_current_thread(|env| -> Result<()> {

        // get minecraft 
        let mc_class = env.find_class(jni_str!("ave")).unwrap();
        
        let get_mc = env.get_static_method_id(
            &mc_class, jni_str!("A"), 
            RuntimeMethodSignature::from_str("()Lave;")?.method_signature()
        )?;

        let mc_instance = unsafe {
            env.call_static_method_unchecked(
                &mc_class,
                get_mc,
                ReturnType::Object,
                &[],
            )?
        };
        let mc_instance = mc_instance.l()?;

        // get player
        let player_field = env.get_field_id(
            mc_class, jni_str!("h"), 
            RuntimeFieldSignature::from_str("Lbew;")?.field_signature()
        )?;

        let player_object_field = unsafe {
            env.get_field_unchecked(&mc_instance, player_field, ReturnType::Object)?
        };
        let player_object_field = player_object_field.l()?;

        let player_class = env.get_object_class(&player_object_field)?;

        // jump method
        let jumpmethod = env.get_method_id(
            player_class, jni_str!("bF"), 
            RuntimeMethodSignature::from_str("()V")?.method_signature()
        )?;

        loop {
            unsafe { 
                env.call_method_unchecked(
                    &player_object_field, 
                    jumpmethod, ReturnType::Primitive(Void), &[] );
            }

            thread::sleep(Duration::from_secs(2));
        }

        Ok(())
    }).expect("Error getting env");
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
            START.call_once(|| { 
                setup_console();
                attach(); 
            });
        }
        _ => {}
    }
    true
}

use windows::Win32::System::Console::{AllocConsole, GetStdHandle, STD_OUTPUT_HANDLE, STD_ERROR_HANDLE};
use windows::Win32::Storage::FileSystem::{CreateFileW, FILE_GENERIC_WRITE, FILE_SHARE_WRITE, FILE_SHARE_READ, OPEN_EXISTING};
use windows::core::PCWSTR;
use windows::Win32::System::Console::SetStdHandle;

fn setup_console() {
    unsafe {
        let _ = AllocConsole();

        let conout: Vec<u16> = "CONOUT$\0".encode_utf16().collect();
        let handle = CreateFileW(
            PCWSTR(conout.as_ptr()),
            FILE_GENERIC_WRITE.0,
            FILE_SHARE_WRITE | FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            Default::default(),
            None,
        ).unwrap();

        let _ = SetStdHandle(STD_OUTPUT_HANDLE, handle);
        let _ = SetStdHandle(STD_ERROR_HANDLE, handle);
    }
}