use std::io;

use bindings::*;

pub struct JsonDictionaryMerger {
    merger: *mut root::keyvi_json_dictionary_merger,
}

unsafe impl Send for JsonDictionaryMerger {}

unsafe impl Sync for JsonDictionaryMerger {}

impl JsonDictionaryMerger {
    pub fn new() -> io::Result<JsonDictionaryMerger> {
        let ptr = unsafe { root::keyvi_create_json_dictionary_merger() };
        if ptr.is_null() {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "could not create json dictionary merger",
            ))
        } else {
            Ok(JsonDictionaryMerger { merger: ptr })
        }
    }

    pub fn add(&self, filename: &str) {
        unsafe {
            root::keyvi_json_dictionary_merger_add(
                self.merger,
                filename.as_ptr() as *const i8,
                filename.len() as u64,
            );
        }
    }

    pub fn merge(&self, filename: &str) {
        unsafe {
            root::keyvi_json_dictionary_merger_merge(
                self.merger,
                filename.as_ptr() as *const i8,
                filename.len() as u64,
            );
        };
    }
}

impl Drop for JsonDictionaryMerger {
    fn drop(&mut self) {
        unsafe {
            root::keyvi_json_dictionary_merger_destroy(self.merger);
        }
    }
}

pub struct CompletionDictionaryMerger {
    merger: *mut root::keyvi_completion_dictionary_merger,
}

unsafe impl Send for CompletionDictionaryMerger {}

unsafe impl Sync for CompletionDictionaryMerger {}

impl CompletionDictionaryMerger {
    pub fn new() -> io::Result<CompletionDictionaryMerger> {
        let ptr = unsafe { root::keyvi_create_completion_dictionary_merger() };
        if ptr.is_null() {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "could not create completion dictionary compiler",
            ))
        } else {
            Ok(CompletionDictionaryMerger { merger: ptr })
        }
    }

    pub fn add(&self, filename: &str) {
        unsafe {
            root::keyvi_completion_dictionary_merger_add(
                self.merger,
                filename.as_ptr() as *const i8,
                filename.len() as u64,
            );
        }
    }

    pub fn merge(&self, filename: &str) {
        unsafe {
            root::keyvi_completion_dictionary_merger_merge(
                self.merger,
                filename.as_ptr() as *const i8,
                filename.len() as u64,
            );
        };
    }
}

impl Drop for CompletionDictionaryMerger {
    fn drop(&mut self) {
        unsafe {
            root::keyvi_completion_dictionary_merger_destroy(self.merger);
        }
    }
}
