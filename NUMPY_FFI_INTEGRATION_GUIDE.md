# 🔗 دليل دمج NumPy عبر FFI في لغة البيان
# NumPy FFI Integration Guide for AlBayan Language

## 🎯 الهدف

دمج NumPy الأصلي في AlBayan Language للحصول على:
- ✅ أداء عالي جداً
- ✅ توافقية كاملة مع NumPy
- ✅ دعم العمليات المعقدة
- ✅ سهولة الاستخدام

---

## 📋 المرحلة 1: إعداد FFI (أسبوع 1)

### 1.1 تثبيت NumPy

```bash
# على Linux/Mac
pip install numpy

# التحقق من المسار
python3 -c "import numpy; print(numpy.get_include())"
```

### 1.2 إعداد Cargo.toml

```toml
[dependencies]
# FFI bindings
numpy = "0.20"
pyo3 = { version = "0.20", features = ["extension-module"] }

# C bindings
libc = "0.2"
```

### 1.3 إنشاء ملف FFI

```rust
// src/ffi/numpy_bindings.rs

#[link(name = "numpy")]
extern "C" {
    // إنشاء مصفوفة
    pub fn PyArray_SimpleNew(
        nd: i32,
        dims: *const i64,
        typenum: i32,
    ) -> *mut PyObject;
    
    // الوصول للبيانات
    pub fn PyArray_DATA(arr: *mut PyObject) -> *mut f64;
    
    // الحصول على الشكل
    pub fn PyArray_DIMS(arr: *mut PyObject) -> *mut i64;
    
    // ضرب المصفوفات
    pub fn PyArray_MatMul(
        a: *mut PyObject,
        b: *mut PyObject,
    ) -> *mut PyObject;
    
    // تحرير الذاكرة
    pub fn Py_DECREF(obj: *mut PyObject);
}
```

---

## 📋 المرحلة 2: تغليف الدوال (أسبوع 2)

### 2.1 تغليف العمليات الأساسية

```rust
// src/ffi/numpy_wrapper.rs

use crate::ffi::numpy_bindings::*;

pub struct NumpyArray {
    ptr: *mut PyObject,
    shape: Vec<i64>,
}

impl NumpyArray {
    pub fn new(shape: Vec<i64>) -> Self {
        unsafe {
            let ptr = PyArray_SimpleNew(
                shape.len() as i32,
                shape.as_ptr(),
                NPY_FLOAT64,
            );
            NumpyArray { ptr, shape }
        }
    }
    
    pub fn matmul(&self, other: &NumpyArray) -> NumpyArray {
        unsafe {
            let result_ptr = PyArray_MatMul(self.ptr, other.ptr);
            let dims = PyArray_DIMS(result_ptr);
            let shape = std::slice::from_raw_parts(*dims, 2).to_vec();
            NumpyArray {
                ptr: result_ptr,
                shape,
            }
        }
    }
    
    pub fn get_data(&self) -> *mut f64 {
        unsafe { PyArray_DATA(self.ptr) }
    }
}

impl Drop for NumpyArray {
    fn drop(&mut self) {
        unsafe {
            Py_DECREF(self.ptr);
        }
    }
}
```

### 2.2 واجهة AlBayan

```albayan
// std/math/numpy_bridge.ab

pub struct NumpyArray {
    ptr: i64,  // مؤشر للمصفوفة
    shape: Vec<i32>,
}

pub fn numpy_create(shape: Vec<i32>) -> NumpyArray {
    // استدعاء الدالة الأصلية
    let ptr = _numpy_create(shape.clone());
    NumpyArray { ptr, shape }
}

pub fn numpy_matmul(a: NumpyArray, b: NumpyArray) -> NumpyArray {
    let result_ptr = _numpy_matmul(a.ptr, b.ptr);
    let shape = vec![a.shape[0], b.shape[1]];
    NumpyArray { ptr: result_ptr, shape }
}

pub fn numpy_dot(a: NumpyArray, b: NumpyArray) -> f64 {
    _numpy_dot(a.ptr, b.ptr)
}

pub fn numpy_transpose(a: NumpyArray) -> NumpyArray {
    let result_ptr = _numpy_transpose(a.ptr);
    let mut new_shape = a.shape.clone();
    new_shape.reverse();
    NumpyArray { ptr: result_ptr, shape: new_shape }
}

// دوال FFI الخارجية
extern "C" {
    fn _numpy_create(shape: Vec<i32>) -> i64;
    fn _numpy_matmul(a: i64, b: i64) -> i64;
    fn _numpy_dot(a: i64, b: i64) -> f64;
    fn _numpy_transpose(a: i64) -> i64;
}
```

---

## 📋 المرحلة 3: العمليات المتقدمة (أسبوع 3)

### 3.1 العمليات الخطية

```albayan
// std/math/numpy_linalg.ab

pub fn numpy_solve(a: NumpyArray, b: NumpyArray) -> NumpyArray {
    // حل النظام الخطي Ax = b
    let result_ptr = _numpy_solve(a.ptr, b.ptr);
    NumpyArray { ptr: result_ptr, shape: b.shape }
}

pub fn numpy_inv(a: NumpyArray) -> NumpyArray {
    // حساب المعكوس
    let result_ptr = _numpy_inv(a.ptr);
    NumpyArray { ptr: result_ptr, shape: a.shape }
}

pub fn numpy_det(a: NumpyArray) -> f64 {
    // حساب المحدد
    _numpy_det(a.ptr)
}

pub fn numpy_eig(a: NumpyArray) -> (NumpyArray, NumpyArray) {
    // حساب القيم والمتجهات الذاتية
    let (eigenvalues_ptr, eigenvectors_ptr) = _numpy_eig(a.ptr);
    (
        NumpyArray { ptr: eigenvalues_ptr, shape: vec![a.shape[0]] },
        NumpyArray { ptr: eigenvectors_ptr, shape: a.shape }
    )
}

pub fn numpy_svd(a: NumpyArray) -> (NumpyArray, NumpyArray, NumpyArray) {
    // تحليل SVD
    let (u_ptr, s_ptr, vt_ptr) = _numpy_svd(a.ptr);
    (
        NumpyArray { ptr: u_ptr, shape: vec![a.shape[0], a.shape[0]] },
        NumpyArray { ptr: s_ptr, shape: vec![std::cmp::min(a.shape[0], a.shape[1])] },
        NumpyArray { ptr: vt_ptr, shape: vec![a.shape[1], a.shape[1]] }
    )
}

extern "C" {
    fn _numpy_solve(a: i64, b: i64) -> i64;
    fn _numpy_inv(a: i64) -> i64;
    fn _numpy_det(a: i64) -> f64;
    fn _numpy_eig(a: i64) -> (i64, i64);
    fn _numpy_svd(a: i64) -> (i64, i64, i64);
}
```

### 3.2 العمليات الإحصائية

```albayan
// std/math/numpy_stats.ab

pub fn numpy_mean(a: NumpyArray, axis: i32) -> NumpyArray {
    let result_ptr = _numpy_mean(a.ptr, axis);
    NumpyArray { ptr: result_ptr, shape: vec![] }
}

pub fn numpy_std(a: NumpyArray, axis: i32) -> NumpyArray {
    let result_ptr = _numpy_std(a.ptr, axis);
    NumpyArray { ptr: result_ptr, shape: vec![] }
}

pub fn numpy_var(a: NumpyArray, axis: i32) -> NumpyArray {
    let result_ptr = _numpy_var(a.ptr, axis);
    NumpyArray { ptr: result_ptr, shape: vec![] }
}

pub fn numpy_cov(a: NumpyArray) -> NumpyArray {
    let result_ptr = _numpy_cov(a.ptr);
    NumpyArray { ptr: result_ptr, shape: vec![a.shape[1], a.shape[1]] }
}

extern "C" {
    fn _numpy_mean(a: i64, axis: i32) -> i64;
    fn _numpy_std(a: i64, axis: i32) -> i64;
    fn _numpy_var(a: i64, axis: i32) -> i64;
    fn _numpy_cov(a: i64) -> i64;
}
```

---

## 📋 المرحلة 4: الاختبارات والتحسينات (أسبوع 4)

### 4.1 اختبارات الأداء

```albayan
// tests/numpy_performance_tests.ab

fn test_matmul_performance() {
    let size = 1000;
    let a = numpy_create([size, size]);
    let b = numpy_create([size, size]);
    
    let start = time::now();
    let result = numpy_matmul(a, b);
    let elapsed = time::now() - start;
    
    assert(elapsed < 100);  // يجب أن يكون أقل من 100ms
}

fn test_solve_performance() {
    let size = 500;
    let a = numpy_create([size, size]);
    let b = numpy_create([size, 1]);
    
    let start = time::now();
    let x = numpy_solve(a, b);
    let elapsed = time::now() - start;
    
    assert(elapsed < 500);  // يجب أن يكون أقل من 500ms
}
```

### 4.2 اختبارات الدقة

```albayan
fn test_matmul_accuracy() {
    let a = numpy_create([3, 3]);
    let b = numpy_create([3, 3]);
    
    // ملء البيانات
    fill_matrix(a, [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
    fill_matrix(b, [[9.0, 8.0, 7.0], [6.0, 5.0, 4.0], [3.0, 2.0, 1.0]]);
    
    let result = numpy_matmul(a, b);
    
    // التحقق من النتائج
    assert_approx_equal(get_element(result, 0, 0), 30.0);
    assert_approx_equal(get_element(result, 1, 1), 93.0);
}
```

---

## 🎯 الفوائد

| الميزة | الفائدة |
|--------|--------|
| **الأداء** | 10-100x أسرع من Python |
| **التوافقية** | توافق كامل مع NumPy |
| **المرونة** | دعم كامل للعربية |
| **الموثوقية** | مكتبة مثبتة وموثوقة |

---

## 🚀 الخطوات التالية

1. ✅ إعداد FFI
2. ✅ تغليف الدوال الأساسية
3. ✅ تطوير العمليات المتقدمة
4. ✅ الاختبارات والتحسينات
5. ✅ التوثيق الشامل

---

**Status**: ✅ **READY FOR NUMPY FFI INTEGRATION**
**Estimated Duration**: 4 أسابيع
**Expected Outcome**: دمج كامل لـ NumPy في AlBayan Language

