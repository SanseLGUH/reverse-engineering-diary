use jni::objects::{JClass, JObject};
use jni::{JNIEnv, JavaVM, JNIVersion, InitArgsBuilder};
use jni::errors::Result;
use jni::sys::jint;

pub extern "system" fn set_sneaking<'local>(mut env: JNIEnv<'local>, flag: bool) {
}

pub fn get_minecraft<'local>(env: &'local mut JNIEnv<'local>) -> Result<JObject<'local>> {
    let minecraft_class = env.find_class("ave")?;

    let method_id = env.get_static_method_id(&minecraft_class, "A", "()Lave;")?;

    unsafe {
        let result = env
        .call_static_method_unchecked(
            minecraft_class,
            method_id,
            jni::signature::ReturnType::Object,
            &[],
        )?
        .l()?;
        Ok(result)
    }

}

fn attach() -> Result<()> {
    let jvm_args = InitArgsBuilder::new()
          .version(JNIVersion::V6)
          .option("-Xcheck:jni")
          .build()
          .unwrap();

    let jvm = JavaVM::new(jvm_args).unwrap();

    let mut env = jvm.get_env().unwrap();

    get_minecraft(&mut env)?;

    println!("get minecraft success");

    Ok(())
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
extern "system" fn DllMain(call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            unsafe { 
                AllocConsole(); 
            }

            attach().unwrap();
        }
        _ => {}
    }

    true
}

unsafe extern "system" {
    fn AllocConsole() -> bool;
}

/*
thread '<unnamed>' panicked at src\lib.rs:41:37:
called `Result::unwrap()` on an `Err` value: NotFound(JavaLocatorError { description: "Could not find the jvm.dll library in any subdirectory of C:\\Program Files (x86)\\Common Files\\Oracle\\Java" })
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

thread '<unnamed>' panicked at library\core\src\panicking.rs:226:5:
panic in a function that cannot unwind
stack backtrace:
   0:     0x7ffe20119932 - std::backtrace_rs::backtrace::win64::trace
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\std\src\..\..\backtrace\src\backtrace\win64.rs:85
   1:     0x7ffe20119932 - std::backtrace_rs::backtrace::trace_unsynchronized
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2:     0x7ffe20119932 - std::sys::backtrace::_print_fmt
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\std\src\sys\backtrace.rs:66
   3:     0x7ffe20119932 - std::sys::backtrace::impl$0::print::impl$0::fmt
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\std\src\sys\backtrace.rs:39
   4:     0x7ffe2013379b - core::fmt::rt::Argument::fmt
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\core\src\fmt\rt.rs:181
   5:     0x7ffe2013379b - core::fmt::write
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\core\src\fmt\mod.rs:1446
   6:     0x7ffe201164b7 - std::io::default_write_fmt
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\std\src\io\mod.rs:639
   7:     0x7ffe201164b7 - std::io::Write::write_fmt<std::sys::stdio::windows::Stderr>
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\std\src\io\mod.rs:1914
   8:     0x7ffe20119775 - std::sys::backtrace::BacktraceLock::print
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\std\src\sys\backtrace.rs:42
   9:     0x7ffe2011d62c - std::panicking::default_hook::closure$0
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\std\src\panicking.rs:300
  10:     0x7ffe2011d3c2 - std::panicking::default_hook
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\std\src\panicking.rs:327
  11:     0x7ffe2011e20f - std::panicking::rust_panic_with_hook
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\std\src\panicking.rs:833
  12:     0x7ffe2011df72 - std::panicking::begin_panic_handler::closure$0
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\std\src\panicking.rs:699
  13:     0x7ffe2011a6ff - std::sys::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic_handler::closure_env$0,never$>
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\std\src\sys\backtrace.rs:168
  14:     0x7ffe2011dbae - std::panicking::begin_panic_handler
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\std\src\panicking.rs:697
  15:     0x7ffe20138fa5 - core::panicking::panic_nounwind_fmt
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\core\src\intrinsics\mod.rs:3196
  16:     0x7ffe20139053 - core::panicking::panic_nounwind
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\core\src\panicking.rs:226
  17:     0x7ffe20139177 - core::panicking::panic_cannot_unwind
                               at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library\core\src\panicking.rs:331
  18:     0x7ffe200cf606 - jni_rust::DllMain
                               at C:\Users\SanseL\Desktop\jni-rust\src\lib.rs:54
  19:     0x7ffe8a9a0430 - _CxxFrameHandler3
  20:     0x7ffe8a99342d - is_exception_typeof
  21:     0x7ffea99b1456 - RtlCaptureContext2
  22:     0x7ffe200cf57a - jni_rust::DllMain
                               at C:\Users\SanseL\Desktop\jni-rust\src\lib.rs:63
  23:     0x7ffe20137672 - dllmain_dispatch
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\dll_dllmain.cpp:281
  24:     0x7ffea9929a1d - RtlActivateActivationContextUnsafeFast
  25:     0x7ffea997c1e7 - LdrGetProcedureAddressEx
  26:     0x7ffea997bf7a - LdrGetProcedureAddressEx
  27:     0x7ffea994d937 - RtlSwitchedVVI
  28:     0x7ffea992fbae - RtlGetFullPathName_UstrEx
  29:     0x7ffea99273e4 - RtlDosPathNameToNtPathName_U
  30:     0x7ffea9926af4 - LdrLoadDll
  31:     0x7ffea725ae12 - LoadLibraryExW
  32:     0x7ffea9717034 - BaseThreadInitThunk
  33:     0x7ffea9962651 - RtlUserThreadStart
thread caused non-unwinding panic. aborting.
*/