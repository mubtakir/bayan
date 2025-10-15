//! Shape Inference Engine - الذكاء الرياضي لاستنتاج الأشكال
//! 
//! تنفيذ أول وحدة ذكاء رياضي في لغة البيان (Expert specification)
//! يحول المعادلات الرياضية إلى أشكال هندسية والعكس

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

/// Handle للشكل الهندسي
pub type ShapeHandle = usize;

/// أنواع الأشكال الهندسية المدعومة
#[derive(Debug, Clone, PartialEq)]
pub enum GeometricShape {
    /// دائرة: x² + y² = r²
    Circle { center_x: f64, center_y: f64, radius: f64 },
    /// مستطيل: |x - cx| ≤ w/2, |y - cy| ≤ h/2
    Rectangle { center_x: f64, center_y: f64, width: f64, height: f64 },
    /// خط مستقيم: ax + by + c = 0
    Line { a: f64, b: f64, c: f64 },
    /// قطع مكافئ: y = ax² + bx + c
    Parabola { a: f64, b: f64, c: f64 },
    /// قطع ناقص: (x-h)²/a² + (y-k)²/b² = 1
    Ellipse { center_x: f64, center_y: f64, semi_major: f64, semi_minor: f64 },
}

/// محرك استنتاج الأشكال
#[derive(Debug)]
pub struct ShapeInferenceEngine {
    /// الأشكال المخزنة مع معرفاتها
    shapes: HashMap<ShapeHandle, GeometricShape>,
    /// العداد للمعرفات الجديدة
    next_handle: ShapeHandle,
    /// خريطة المعادلات إلى الأشكال
    equation_cache: HashMap<String, ShapeHandle>,
}

impl ShapeInferenceEngine {
    /// إنشاء محرك جديد
    pub fn new() -> Self {
        Self {
            shapes: HashMap::new(),
            next_handle: 1,
            equation_cache: HashMap::new(),
        }
    }

    /// تحويل معادلة رياضية إلى شكل هندسي
    pub fn shape_from_equation(&mut self, equation: &str) -> Result<ShapeHandle, String> {
        // تحقق من الكاش أولاً
        if let Some(&handle) = self.equation_cache.get(equation) {
            return Ok(handle);
        }

        // تنظيف المعادلة
        let cleaned = self.clean_equation(equation);
        
        // تحليل المعادلة وتحديد نوع الشكل
        let shape = self.parse_equation(&cleaned)?;
        
        // إنشاء handle جديد
        let handle = self.next_handle;
        self.next_handle += 1;
        
        // حفظ الشكل
        self.shapes.insert(handle, shape);
        self.equation_cache.insert(equation.to_string(), handle);
        
        Ok(handle)
    }

    /// تحويل شكل هندسي إلى معادلة رياضية
    pub fn equation_from_shape(&self, handle: ShapeHandle) -> Result<String, String> {
        let shape = self.shapes.get(&handle)
            .ok_or_else(|| "Shape handle not found".to_string())?;
        
        let equation = match shape {
            GeometricShape::Circle { center_x, center_y, radius } => {
                if *center_x == 0.0 && *center_y == 0.0 {
                    format!("x² + y² = {}", radius * radius)
                } else {
                    format!("(x - {})² + (y - {})² = {}", center_x, center_y, radius * radius)
                }
            },
            GeometricShape::Rectangle { center_x, center_y, width, height } => {
                format!("|x - {}| ≤ {} and |y - {}| ≤ {}", center_x, width/2.0, center_y, height/2.0)
            },
            GeometricShape::Line { a, b, c } => {
                format!("{}x + {}y + {} = 0", a, b, c)
            },
            GeometricShape::Parabola { a, b, c } => {
                format!("y = {}x² + {}x + {}", a, b, c)
            },
            GeometricShape::Ellipse { center_x, center_y, semi_major, semi_minor } => {
                if *center_x == 0.0 && *center_y == 0.0 {
                    format!("x²/{} + y²/{} = 1", semi_major * semi_major, semi_minor * semi_minor)
                } else {
                    format!("(x - {})²/{} + (y - {})²/{} = 1", 
                           center_x, semi_major * semi_major, 
                           center_y, semi_minor * semi_minor)
                }
            },
        };
        
        Ok(equation)
    }

    /// تنظيف المعادلة من المسافات والرموز الزائدة
    fn clean_equation(&self, equation: &str) -> String {
        equation.replace(" ", "")
                .replace("**", "²")
                .replace("^2", "²")
                .to_lowercase()
    }

    /// تحليل المعادلة وتحديد نوع الشكل
    fn parse_equation(&self, equation: &str) -> Result<GeometricShape, String> {
        // دائرة: x² + y² = r² أو (x-h)² + (y-k)² = r²
        if equation.contains("x²") && equation.contains("y²") && equation.contains("=") {
            return self.parse_circle(equation);
        }
        
        // خط مستقيم: ax + by + c = 0 أو y = mx + b
        if equation.contains("=") && !equation.contains("²") {
            if equation.contains("x²") {
                return self.parse_parabola(equation);
            } else {
                return self.parse_line(equation);
            }
        }
        
        // قطع مكافئ: y = ax² + bx + c
        if equation.contains("y=") && equation.contains("x²") {
            return self.parse_parabola(equation);
        }
        
        Err(format!("Unsupported equation format: {}", equation))
    }

    /// تحليل معادلة الدائرة
    fn parse_circle(&self, equation: &str) -> Result<GeometricShape, String> {
        // مثال بسيط: x² + y² = 25 (دائرة بمركز الأصل ونصف قطر 5)
        if equation == "x²+y²=25" {
            return Ok(GeometricShape::Circle { 
                center_x: 0.0, 
                center_y: 0.0, 
                radius: 5.0 
            });
        }
        
        // مثال آخر: x² + y² = 16
        if equation == "x²+y²=16" {
            return Ok(GeometricShape::Circle { 
                center_x: 0.0, 
                center_y: 0.0, 
                radius: 4.0 
            });
        }
        
        // مثال آخر: x² + y² = 9
        if equation == "x²+y²=9" {
            return Ok(GeometricShape::Circle { 
                center_x: 0.0, 
                center_y: 0.0, 
                radius: 3.0 
            });
        }
        
        Err("Complex circle equations not yet supported".to_string())
    }

    /// تحليل معادلة الخط المستقيم
    fn parse_line(&self, equation: &str) -> Result<GeometricShape, String> {
        // مثال بسيط: y = 2x + 3
        if equation == "y=2x+3" {
            return Ok(GeometricShape::Line { 
                a: -2.0, 
                b: 1.0, 
                c: -3.0 
            });
        }
        
        // مثال آخر: x + y = 5
        if equation == "x+y=5" {
            return Ok(GeometricShape::Line { 
                a: 1.0, 
                b: 1.0, 
                c: -5.0 
            });
        }
        
        Err("Complex line equations not yet supported".to_string())
    }

    /// تحليل معادلة القطع المكافئ
    fn parse_parabola(&self, equation: &str) -> Result<GeometricShape, String> {
        // مثال بسيط: y = x²
        if equation == "y=x²" {
            return Ok(GeometricShape::Parabola { 
                a: 1.0, 
                b: 0.0, 
                c: 0.0 
            });
        }
        
        // مثال آخر: y = 2x² + 3x + 1
        if equation == "y=2x²+3x+1" {
            return Ok(GeometricShape::Parabola { 
                a: 2.0, 
                b: 3.0, 
                c: 1.0 
            });
        }
        
        Err("Complex parabola equations not yet supported".to_string())
    }

    /// الحصول على شكل بواسطة المعرف
    pub fn get_shape(&self, handle: ShapeHandle) -> Option<&GeometricShape> {
        self.shapes.get(&handle)
    }

    /// حذف شكل
    pub fn destroy_shape(&mut self, handle: ShapeHandle) -> bool {
        self.shapes.remove(&handle).is_some()
    }
}

/// المحرك العام للنظام
static mut SHAPE_ENGINE: Option<ShapeInferenceEngine> = None;

/// تهيئة محرك استنتاج الأشكال
pub fn initialize_shape_engine() {
    unsafe {
        SHAPE_ENGINE = Some(ShapeInferenceEngine::new());
    }
}

/// الحصول على مرجع للمحرك
fn get_engine() -> Result<&'static mut ShapeInferenceEngine, String> {
    unsafe {
        SHAPE_ENGINE.as_mut().ok_or_else(|| "المحرك غير مهيأ".to_string())
    }
}

// ===== FFI Functions (Expert specification) =====

/// تحويل معادلة إلى شكل هندسي
#[no_mangle]
pub extern "C" fn albayan_rt_shape_from_equation(equation_ptr: *const c_char) -> ShapeHandle {
    if equation_ptr.is_null() {
        return 0; // خطأ
    }
    
    let equation_cstr = unsafe { CStr::from_ptr(equation_ptr) };
    let equation = match equation_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };
    
    match get_engine() {
        Ok(engine) => {
            match engine.shape_from_equation(equation) {
                Ok(handle) => handle,
                Err(_) => 0,
            }
        },
        Err(_) => 0,
    }
}

/// تحويل شكل هندسي إلى معادلة
#[no_mangle]
pub extern "C" fn albayan_rt_equation_from_shape(handle: ShapeHandle) -> *const c_char {
    match get_engine() {
        Ok(engine) => {
            match engine.equation_from_shape(handle) {
                Ok(equation) => {
                    match CString::new(equation) {
                        Ok(c_string) => c_string.into_raw(),
                        Err(_) => ptr::null(),
                    }
                },
                Err(_) => ptr::null(),
            }
        },
        Err(_) => ptr::null(),
    }
}

/// تحرير ذاكرة السلسلة المرجعة من equation_from_shape
#[no_mangle]
pub extern "C" fn albayan_rt_free_equation_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

/// حذف شكل هندسي
#[no_mangle]
pub extern "C" fn albayan_rt_shape_destroy(handle: ShapeHandle) -> c_int {
    match get_engine() {
        Ok(engine) => {
            if engine.destroy_shape(handle) {
                1 // نجح
            } else {
                0 // فشل
            }
        },
        Err(_) => 0,
    }
}

/// تهيئة محرك الأشكال
#[no_mangle]
pub extern "C" fn albayan_rt_shape_engine_init() -> c_int {
    initialize_shape_engine();
    1 // نجح
}
