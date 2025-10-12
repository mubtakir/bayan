// src/runtime/dynamic_types.rs
//! Dynamic type system for AlBayan runtime
//!
//! This module implements the core dynamic types that enable runtime polymorphism
//! and dynamic data structures like lists, dictionaries, and objects.

use std::collections::HashMap;
use std::fmt;

/// Tag enum to identify the type stored in AlbayanValue
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlbayanValueTag {
    Int = 0,
    Float = 1,
    Bool = 2,
    String = 3,
    List = 4,
    Struct = 5,
    Tuple = 6,
    Enum = 7,
    Null = 8,
}

/// Union-like structure to hold different types of values at runtime
/// This is the core of AlBayan's dynamic type system
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AlbayanValue {
    pub tag: AlbayanValueTag,
    pub payload: AlbayanValuePayload,
}

/// The actual data payload for AlbayanValue
/// Uses a union-like approach with different data types
#[repr(C)]
#[derive(Clone, Copy)]
pub union AlbayanValuePayload {
    pub int_val: i64,
    pub float_val: f64,
    pub bool_val: bool,
    pub string_val: *mut AlbayanString,
    pub list_val: *mut AlbayanList,
    pub struct_val: *mut AlbayanStruct,
    pub tuple_val: *mut AlbayanTuple,
    pub enum_val: *mut AlbayanEnum,
}

impl fmt::Debug for AlbayanValuePayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AlbayanValuePayload {{ ... }}")
    }
}

/// Dynamic string type for AlBayan
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AlbayanString {
    pub data: *mut u8,
    pub len: usize,
    pub capacity: usize,
}

/// Dynamic list type for AlBayan - similar to Vec<AlbayanValue>
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AlbayanList {
    pub data: *mut AlbayanValue,
    pub len: usize,
    pub capacity: usize,
}

/// Dynamic struct type for AlBayan
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AlbayanStruct {
    pub fields: *mut HashMap<String, AlbayanValue>,
    pub type_name: *mut AlbayanString,
}

/// Dynamic tuple type for AlBayan
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AlbayanTuple {
    pub elements: *mut AlbayanValue,
    pub len: usize,
}

/// Dynamic enum type for AlBayan (Expert recommendation: tagged unions)
#[repr(C)]
#[derive(Debug, Clone)]
pub struct AlbayanEnum {
    pub variant_tag: u32,  // Which variant is active
    pub enum_name: *mut AlbayanString,  // Name of the enum type
    pub variant_name: *mut AlbayanString,  // Name of the active variant
    pub payload: *mut AlbayanValue,  // Optional data for the variant (null if no data)
}

impl AlbayanValue {
    /// Create a new integer value
    pub fn new_int(value: i64) -> Self {
        Self {
            tag: AlbayanValueTag::Int,
            payload: AlbayanValuePayload { int_val: value },
        }
    }

    /// Create a new float value
    pub fn new_float(value: f64) -> Self {
        Self {
            tag: AlbayanValueTag::Float,
            payload: AlbayanValuePayload { float_val: value },
        }
    }

    /// Create a new boolean value
    pub fn new_bool(value: bool) -> Self {
        Self {
            tag: AlbayanValueTag::Bool,
            payload: AlbayanValuePayload { bool_val: value },
        }
    }

    /// Create a new null value
    pub fn new_null() -> Self {
        Self {
            tag: AlbayanValueTag::Null,
            payload: AlbayanValuePayload { int_val: 0 },
        }
    }

    /// Create a new list value
    pub fn new_list(list: *mut AlbayanList) -> Self {
        Self {
            tag: AlbayanValueTag::List,
            payload: AlbayanValuePayload { list_val: list },
        }
    }

    /// Create a new enum value (Expert recommendation: tagged unions)
    pub fn new_enum(enum_val: *mut AlbayanEnum) -> Self {
        Self {
            tag: AlbayanValueTag::Enum,
            payload: AlbayanValuePayload { enum_val },
        }
    }

    /// Get the integer value (unsafe - caller must ensure tag is Int)
    pub unsafe fn as_int(&self) -> i64 {
        self.payload.int_val
    }

    /// Get the float value (unsafe - caller must ensure tag is Float)
    pub unsafe fn as_float(&self) -> f64 {
        self.payload.float_val
    }

    /// Get the boolean value (unsafe - caller must ensure tag is Bool)
    pub unsafe fn as_bool(&self) -> bool {
        self.payload.bool_val
    }

    /// Get the list pointer (unsafe - caller must ensure tag is List)
    pub unsafe fn as_list(&self) -> *mut AlbayanList {
        self.payload.list_val
    }

    /// Get the enum pointer (unsafe - caller must ensure tag is Enum)
    pub unsafe fn as_enum(&self) -> *mut AlbayanEnum {
        self.payload.enum_val
    }

    /// Check if this value is of a specific type
    pub fn is_type(&self, tag: AlbayanValueTag) -> bool {
        self.tag == tag
    }

    /// Get a string representation of the type
    pub fn type_name(&self) -> &'static str {
        match self.tag {
            AlbayanValueTag::Int => "int",
            AlbayanValueTag::Float => "float",
            AlbayanValueTag::Bool => "bool",
            AlbayanValueTag::String => "string",
            AlbayanValueTag::List => "list",
            AlbayanValueTag::Struct => "struct",
            AlbayanValueTag::Tuple => "tuple",
            AlbayanValueTag::Enum => "enum",
            AlbayanValueTag::Null => "null",
        }
    }
}

impl AlbayanList {
    /// Create a new empty list
    pub fn new() -> Box<Self> {
        Box::new(Self {
            data: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        })
    }

    /// Create a new list with initial capacity
    pub fn with_capacity(capacity: usize) -> Box<Self> {
        let layout = std::alloc::Layout::array::<AlbayanValue>(capacity).unwrap();
        let data = unsafe { std::alloc::alloc(layout) as *mut AlbayanValue };

        Box::new(Self {
            data,
            len: 0,
            capacity,
        })
    }

    /// Push a value to the end of the list
    pub fn push(&mut self, value: AlbayanValue) {
        if self.len >= self.capacity {
            self.grow();
        }

        unsafe {
            *self.data.add(self.len) = value;
        }
        self.len += 1;
    }

    /// Get a value at the specified index
    pub fn get(&self, index: usize) -> Option<&AlbayanValue> {
        if index < self.len {
            unsafe { Some(&*self.data.add(index)) }
        } else {
            None
        }
    }

    /// Get the length of the list
    pub fn len(&self) -> usize {
        self.len
    }

    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Grow the list capacity (double it or set to 4 if empty)
    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };

        let new_layout = std::alloc::Layout::array::<AlbayanValue>(new_capacity).unwrap();
        let new_data = unsafe { std::alloc::alloc(new_layout) as *mut AlbayanValue };

        if !self.data.is_null() && self.len > 0 {
            unsafe {
                std::ptr::copy_nonoverlapping(self.data, new_data, self.len);
                let old_layout = std::alloc::Layout::array::<AlbayanValue>(self.capacity).unwrap();
                std::alloc::dealloc(self.data as *mut u8, old_layout);
            }
        }

        self.data = new_data;
        self.capacity = new_capacity;
    }
}

impl Drop for AlbayanList {
    fn drop(&mut self) {
        if !self.data.is_null() && self.capacity > 0 {
            unsafe {
                let layout = std::alloc::Layout::array::<AlbayanValue>(self.capacity).unwrap();
                std::alloc::dealloc(self.data as *mut u8, layout);
            }
        }
    }
}

// Default implementations
impl Default for AlbayanList {
    fn default() -> Self {
        Self {
            data: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
}

// =============================================================================
// Runtime API Functions - C-compatible interface for LLVM IR
// =============================================================================

/// Create a new empty list
/// Returns a pointer to the newly created AlbayanList
#[no_mangle]
pub extern "C" fn albayan_rt_list_create() -> *mut AlbayanList {
    let list = AlbayanList::new();
    Box::into_raw(list)
}

/// Create a new list with initial capacity
/// Returns a pointer to the newly created AlbayanList
#[no_mangle]
pub extern "C" fn albayan_rt_list_create_with_capacity(capacity: usize) -> *mut AlbayanList {
    let list = AlbayanList::with_capacity(capacity);
    Box::into_raw(list)
}

/// Push a value to the end of the list
/// Returns 0 on success, -1 on error
#[no_mangle]
pub extern "C" fn albayan_rt_list_push(list_ptr: *mut AlbayanList, value: AlbayanValue) -> i32 {
    if list_ptr.is_null() {
        return -1;
    }

    unsafe {
        let list = &mut *list_ptr;
        list.push(value);
    }

    0
}

/// Get a value at the specified index
/// Returns the value if found, or a null value if index is out of bounds
#[no_mangle]
pub extern "C" fn albayan_rt_list_get(list_ptr: *const AlbayanList, index: usize) -> AlbayanValue {
    if list_ptr.is_null() {
        return AlbayanValue::new_null();
    }

    unsafe {
        let list = &*list_ptr;
        match list.get(index) {
            Some(value) => value.clone(),
            None => AlbayanValue::new_null(),
        }
    }
}

/// Get the length of the list
/// Returns the length, or 0 if the list pointer is null
#[no_mangle]
pub extern "C" fn albayan_rt_list_len(list_ptr: *const AlbayanList) -> usize {
    if list_ptr.is_null() {
        return 0;
    }

    unsafe {
        let list = &*list_ptr;
        list.len()
    }
}

/// Check if the list is empty
/// Returns 1 if empty, 0 if not empty, -1 if list pointer is null
#[no_mangle]
pub extern "C" fn albayan_rt_list_is_empty(list_ptr: *const AlbayanList) -> i32 {
    if list_ptr.is_null() {
        return -1;
    }

    unsafe {
        let list = &*list_ptr;
        if list.is_empty() { 1 } else { 0 }
    }
}

/// Free a list and all its resources
/// Should be called when the list is no longer needed
#[no_mangle]
pub extern "C" fn albayan_rt_list_free(list_ptr: *mut AlbayanList) {
    if !list_ptr.is_null() {
        unsafe {
            let _list = Box::from_raw(list_ptr);
            // Drop will be called automatically
        }
    }
}

/// Destroy a list and all its resources (Expert recommendation: Automatic destroy calls)
/// Alias for albayan_rt_list_free for consistency with borrow checker terminology
#[no_mangle]
pub extern "C" fn albayan_rt_list_destroy(list_ptr: *mut AlbayanList) {
    albayan_rt_list_free(list_ptr);
}

// =============================================================================
// AlbayanValue Helper Functions
// =============================================================================

/// Create a new integer AlbayanValue
#[no_mangle]
pub extern "C" fn albayan_rt_value_new_int(value: i64) -> AlbayanValue {
    AlbayanValue::new_int(value)
}

/// Create a new float AlbayanValue
#[no_mangle]
pub extern "C" fn albayan_rt_value_new_float(value: f64) -> AlbayanValue {
    AlbayanValue::new_float(value)
}

/// Create a new boolean AlbayanValue
#[no_mangle]
pub extern "C" fn albayan_rt_value_new_bool(value: bool) -> AlbayanValue {
    AlbayanValue::new_bool(value)
}

/// Create a new null AlbayanValue
#[no_mangle]
pub extern "C" fn albayan_rt_value_new_null() -> AlbayanValue {
    AlbayanValue::new_null()
}

/// Get the tag of an AlbayanValue
#[no_mangle]
pub extern "C" fn albayan_rt_value_get_tag(value: *const AlbayanValue) -> AlbayanValueTag {
    if value.is_null() {
        return AlbayanValueTag::Null;
    }

    unsafe { (*value).tag }
}

/// Extract integer value from AlbayanValue (unsafe - caller must check tag)
#[no_mangle]
pub extern "C" fn albayan_rt_value_as_int(value: *const AlbayanValue) -> i64 {
    if value.is_null() {
        return 0;
    }

    unsafe { (*value).as_int() }
}

/// Extract float value from AlbayanValue (unsafe - caller must check tag)
#[no_mangle]
pub extern "C" fn albayan_rt_value_as_float(value: *const AlbayanValue) -> f64 {
    if value.is_null() {
        return 0.0;
    }

    unsafe { (*value).as_float() }
}

/// Extract boolean value from AlbayanValue (unsafe - caller must check tag)
#[no_mangle]
pub extern "C" fn albayan_rt_value_as_bool(value: *const AlbayanValue) -> bool {
    if value.is_null() {
        return false;
    }

    unsafe { (*value).as_bool() }
}

/// Runtime panic function for type safety violations (Expert recommendation: Unboxing Safety)
#[no_mangle]
pub extern "C" fn albayan_rt_panic(message_ptr: *const u8, message_len: usize) -> ! {
    if message_ptr.is_null() {
        panic!("AlBayan Runtime Panic: Unknown error");
    }

    let message_slice = unsafe {
        std::slice::from_raw_parts(message_ptr, message_len)
    };

    let message = String::from_utf8_lossy(message_slice);
    panic!("AlBayan Runtime Panic: {}", message);
}
