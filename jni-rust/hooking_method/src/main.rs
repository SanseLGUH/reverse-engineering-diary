use jni::{JavaVM, JNIVersion};
use jni::InitArgsBuilder;
use jni::errors::Result;
use jni::strings::JNIStr;
use jni::signature::RuntimeMethodSignature;

fn run() -> Result<()> {
    let jvm_args = InitArgsBuilder::new()
        .version(JNIVersion::V9)
        .option("-Xcheck:jni")
        .option("-Djava.class.path=/home/sansel/java-test")
        .build()
        .unwrap();
    
    let jvm = JavaVM::new(jvm_args).unwrap();

    jvm.attach_current_thread(|env| {
        let class = env.find_class(JNIStr::from_cstr(c"Hello").unwrap())?;

        let sig = RuntimeMethodSignature::from_str("()V")?;

        env.call_static_method(
            class,
            JNIStr::from_cstr(c"goal").unwrap(),
            sig.method_signature(),
            &[],
        )?;
        Ok(())
    })
}

fn main() {
    run().unwrap();
}