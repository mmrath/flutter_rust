
use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn hello() -> *const c_char {
    let string = match CString::new("RUST SAYS: Hello world!") {
        Ok(string) => string,
        Err(_) => panic!("Failed to create string"),
    };

    return string.as_ptr();
}

/// Expose the JNI interface for android below
#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::JNIEnv;

    use self::jni::objects::{JClass, JString, JObject, GlobalRef};

    // This is just a pointer. We'll be returning it from our function.
    // We can't return one of the objects with lifetime information because the
    // lifetime checker won't let us.
    use self::jni::sys::{jint, jlong, jstring};


    // This keeps rust from "mangling" the name and making it unique for this crate.
    #[no_mangle]
    // This turns off linter warnings because
    // the name doesn't conform to conventions.
    #[allow(non_snake_case)]
    pub extern "system" fn Java_com_yourcompany_flutterrust_MainActivity_hello(env: JNIEnv,
                                                 // this is the class that owns our
                                                 // static method. Not going to be
                                                 // used, but still needs to have
                                                 // an argument slot
                                                 class: JClass,
                                                 input: JString)
                                                 -> jstring {
        // First, we have to get the string out of java. Check out the `strings`
        // module for more info on how this works.
        let input: String =
            env.get_string(input).expect("Couldn't get java string!").into();

        // Then we have to create a new java string to return. Again, more info
        // in the `strings` module.
        let output = env.new_string(format!("Hello, {}!", input))
            .expect("Couldn't create java string!");

        // Finally, extract the raw pointer to return.
        output.into_inner()
    }

}

#[cfg(test)]
mod tests {
    use { hello };
    use std::ffi::CString;

    #[test]
    fn test_hello() {
        let expected = match CString::new("RUST SAYS: Hello world!") {
            Ok(string) => string.as_ptr(),
            Err(err) => panic!(err),
        };
        let actual = hello();
        assert_eq!(actual, expected);
    }
}
