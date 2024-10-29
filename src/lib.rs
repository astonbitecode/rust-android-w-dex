use android_activity::AndroidApp;
use core::ptr;
use jni_sys::{jclass, JNIEnv, JavaVM};
use std::{ffi::CString, os::raw::c_void};

#[no_mangle]
fn android_main(app: AndroidApp) {
    test(&app);
}

fn test(app: &AndroidApp) {
    unsafe {
        let java_vm: *mut JavaVM = app.vm_as_ptr().cast();
        let mut env: *mut JNIEnv = ptr::null_mut();
        ((**java_vm).v1_4.AttachCurrentThread)(
            java_vm,
            (&mut env as *mut *mut JNIEnv) as *mut *mut c_void,
            ptr::null_mut(),
        );

        let fc = (**env).v1_6.FindClass;
        let cs = CString::new("java/lang/String".as_bytes()).unwrap();
        let cstr = cs.into_raw();
        let class: jclass = (fc)(env, cstr);
        let found = class != ptr::null_mut();
        println!("================found java/lang/String: {found}");

        let fc = (**env).v1_6.FindClass;
        let cs = CString::new("my/test/Calculator".as_bytes()).unwrap();
        let cstr = cs.into_raw();
        let class: jclass = (fc)(env, cstr);
        let found = class != ptr::null_mut();
        println!("================found my/test/Calculator: {found}");
    }
}
