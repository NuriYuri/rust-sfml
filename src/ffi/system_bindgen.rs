// Generated by rust-sfml-bindgen
// https://github.com/crumblingstatue/rust-sfml-bindgen

extern "C" {

// Clock.cpp
pub fn sfClock_new() -> *mut sfClock;
pub fn sfClock_delete(clock: *mut sfClock);
pub fn sfClock_getElapsedTime(clock: *const sfClock) -> i64;
pub fn sfClock_restart(clock: *mut sfClock) -> i64;
// InputStream.cpp
pub fn sfInputStream_new(read: sfInputStreamReadFunc, seek: sfInputStreamSeekFunc, tell: sfInputStreamTellFunc, getSize: sfInputStreamGetSizeFunc, userData: *mut c_void) -> *mut sfInputStream;
pub fn sfInputStream_destroy(stream: *mut sfInputStream);
// SfStdString.cpp
pub fn sfStdString_getLength(s: *const sfStdString) -> usize;
pub fn sfStdString_getData(s: *const sfStdString) -> *const c_char;
pub fn sfStdString_destroy(s: *mut sfStdString);
// SfStdVector.cpp
pub fn sfStdStringVector_destroy(vec: *mut sfStdStringVector);
pub fn sfStdStringVector_getLength(vec: *const sfStdStringVector) -> usize;
pub fn sfStdStringVector_index(vec: *const sfStdStringVector, index: usize) -> *const sfStdString;
// SfString.cpp
pub fn sfString_getLength(string: *const sfString) -> usize;
pub fn sfString_getData(string: *const sfString) -> *const u32;
pub fn sfString_delete(string: *mut sfString);
// Sleep.cpp
pub fn sfSleep(duration_ms: i64);

}