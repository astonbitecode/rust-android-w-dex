use android_activity::AndroidApp;
use j4rs::{self, Jvm};

pub fn enable_immersive(app: &AndroidApp) {
    j4rs::set_java_vm(app.vm_as_ptr().cast());
    let mut _jvm = Jvm::attach_thread();
}
