use std::io;

use bindings::*;

pub struct JsonDictionaryCompiler {
    compiler: *mut root::keyvi_json_dictionary_compiler,
}

unsafe impl Send for JsonDictionaryCompiler {}

unsafe impl Sync for JsonDictionaryCompiler {}

impl JsonDictionaryCompiler {
    pub fn new() -> io::Result<JsonDictionaryCompiler> {
        let ptr = unsafe { root::keyvi_create_json_dictionary_compiler() };
        if ptr.is_null() {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "could not create json dictionary compiler",
            ))
        } else {
            Ok(JsonDictionaryCompiler { compiler: ptr })
        }
    }

    pub fn add(&self, key: &str, value: &str) {
        unsafe {
            root::keyvi_json_dictionary_compiler_add(
                self.compiler,
                key.as_ptr() as *const i8,
                key.len() as u64,
                value.as_ptr() as *const i8,
                value.len() as u64,
            );
        }
    }

    pub fn compile(&self) {
        unsafe {
            root::keyvi_json_dictionary_compiler_compile(self.compiler);
        }
    }

    pub fn write_to_file(&self, filename: &str) {
        unsafe {
            root::keyvi_json_dictionary_compiler_write_to_file(
                self.compiler,
                filename.as_ptr() as *const i8,
                filename.len() as u64,
            );
        };
    }
}

impl Drop for JsonDictionaryCompiler {
    fn drop(&mut self) {
        unsafe {
            root::keyvi_json_dictionary_compiler_destroy(self.compiler);
        }
    }
}

pub struct CompletionDictionaryCompiler {
    compiler: *mut root::keyvi_completion_dictionary_compiler,
}

unsafe impl Send for CompletionDictionaryCompiler {}

unsafe impl Sync for CompletionDictionaryCompiler {}

impl CompletionDictionaryCompiler {
    pub fn new() -> io::Result<CompletionDictionaryCompiler> {
        let ptr = unsafe { root::keyvi_create_completion_dictionary_compiler() };
        if ptr.is_null() {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "could not create completion dictionary compiler",
            ))
        } else {
            Ok(CompletionDictionaryCompiler { compiler: ptr })
        }
    }

    pub fn add(&self, key: &str, value: u64) {
        unsafe {
            root::keyvi_completion_dictionary_compiler_add(
                self.compiler,
                key.as_ptr() as *const i8,
                key.len() as u64,
                value,
            );
        }
    }

    pub fn compile(&self) {
        unsafe {
            root::keyvi_completion_dictionary_compiler_compile(self.compiler);
        }
    }

    pub fn write_to_file(&self, filename: &str) {
        unsafe {
            root::keyvi_completion_dictionary_compiler_write_to_file(
                self.compiler,
                filename.as_ptr() as *const i8,
                filename.len() as u64,
            );
        };
    }
}

impl Drop for CompletionDictionaryCompiler {
    fn drop(&mut self) {
        unsafe {
            root::keyvi_completion_dictionary_compiler_destroy(self.compiler);
        }
    }
}
