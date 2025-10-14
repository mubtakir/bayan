// src/runtime/shape_inference.rs
// محرك استنتاج الأشكال - أول محرك في وقت التشغيل
// Expert recommendation: "بناء أول محرك في وقت التشغيل - ShapeInferenceEngine"

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// معرف مقبض الشكل
pub type ShapeHandle = usize;

/// محرك استنتاج الأشكال
/// Expert insight: "أبسط وحدة للبدء - معادلة الشكل محلولة لديك"
pub struct ShapeInferenceEngine {
    /// قاعدة بيانات الأشكال المعروفة
    known_shapes: HashMap<String, KnownShape>,

    /// مولد المقابض
    next_handle: ShapeHandle,

    /// مخزن الأشكال النشطة
    active_shapes: HashMap<ShapeHandle, GeometricShape>,

    /// محلل المعادلات
    equation_analyzer: EquationAnalyzer,

    /// مطابق الأنماط
    pattern_matcher: PatternMatcher,
}

/// شكل معروف في قاعدة البيانات
#[derive(Debug, Clone)]
pub struct KnownShape {
    /// اسم الشكل
    pub name: String,

    /// نوع الشكل
    pub shape_type: ShapeType,

    /// المعادلات المرتبطة
    pub equations: Vec<String>,

    /// الأنماط المميزة
    pub patterns: Vec<EquationPattern>,

    /// مستوى الثقة في التعرف
    pub confidence: f64,
}

/// شكل هندسي
#[derive(Debug, Clone)]
pub struct GeometricShape {
    /// نوع الشكل
    pub shape_type: ShapeType,

    /// معاملات الشكل
    pub parameters: Vec<f64>,

    /// المعادلات المرتبطة
    pub equations: Vec<String>,

    /// الخصائص المحسوبة
    pub properties: ShapeProperties,

    /// مستوى الثقة
    pub confidence: f64,
}

/// أنواع الأشكال الهندسية
#[derive(Debug, Clone, PartialEq)]
pub enum ShapeType {
    Circle,
    Square,
    Rectangle,
    Triangle,
    Ellipse,
    Parabola,
    Hyperbola,
    Line,
    Unknown,
}

/// خصائص الشكل
#[derive(Debug, Clone)]
pub struct ShapeProperties {
    /// المساحة
    pub area: f64,

    /// المحيط
    pub perimeter: f64,

    /// مركز الشكل
    pub center: (f64, f64),

    /// نصف القطر (للدوائر)
    pub radius: Option<f64>,

    /// الأبعاد (العرض، الارتفاع)
    pub dimensions: Option<(f64, f64)>,
}

/// محلل المعادلات
pub struct EquationAnalyzer {
    /// أنماط المعادلات المعروفة
    equation_patterns: Vec<EquationPattern>,
}

/// نمط معادلة
#[derive(Debug, Clone)]
pub struct EquationPattern {
    /// النمط (regex-like)
    pub pattern: String,

    /// نوع الشكل المرتبط
    pub shape_type: ShapeType,

    /// دالة استخراج المعاملات
    pub parameter_extractor: String,

    /// مستوى الثقة
    pub confidence: f64,
}

/// مطابق الأنماط
pub struct PatternMatcher {
    /// قواعد المطابقة
    matching_rules: Vec<MatchingRule>,
}

/// قاعدة مطابقة
#[derive(Debug, Clone)]
pub struct MatchingRule {
    /// الشرط
    pub condition: String,

    /// النتيجة
    pub result: ShapeType,

    /// الثقة
    pub confidence: f64,
}

impl ShapeInferenceEngine {
    /// إنشاء محرك جديد
    pub fn new() -> Self {
        let mut engine = ShapeInferenceEngine {
            known_shapes: HashMap::new(),
            next_handle: 1,
            active_shapes: HashMap::new(),
            equation_analyzer: EquationAnalyzer::new(),
            pattern_matcher: PatternMatcher::new(),
        };

        // تهيئة الأشكال المعروفة
        engine.initialize_known_shapes();

        engine
    }

    /// تهيئة الأشكال المعروفة
    fn initialize_known_shapes(&mut self) {
        // دائرة
        self.known_shapes.insert("circle".to_string(), KnownShape {
            name: "دائرة".to_string(),
            shape_type: ShapeType::Circle,
            equations: vec![
                "x² + y² = r²".to_string(),
                "(x-h)² + (y-k)² = r²".to_string(),
                "x² + y² = c".to_string(),
            ],
            patterns: vec![
                EquationPattern {
                    pattern: r"x\^?2\s*\+\s*y\^?2\s*=\s*(\d+)".to_string(),
                    shape_type: ShapeType::Circle,
                    parameter_extractor: "radius_from_constant".to_string(),
                    confidence: 0.95,
                },
            ],
            confidence: 0.95,
        });

        // قطع مكافئ
        self.known_shapes.insert("parabola".to_string(), KnownShape {
            name: "قطع مكافئ".to_string(),
            shape_type: ShapeType::Parabola,
            equations: vec![
                "y = x²".to_string(),
                "y = ax² + bx + c".to_string(),
                "x = y²".to_string(),
            ],
            patterns: vec![
                EquationPattern {
                    pattern: r"y\s*=\s*x\^?2".to_string(),
                    shape_type: ShapeType::Parabola,
                    parameter_extractor: "parabola_basic".to_string(),
                    confidence: 0.9,
                },
            ],
            confidence: 0.9,
        });

        // قطع ناقص
        self.known_shapes.insert("ellipse".to_string(), KnownShape {
            name: "قطع ناقص".to_string(),
            shape_type: ShapeType::Ellipse,
            equations: vec![
                "x²/a² + y²/b² = 1".to_string(),
                "(x-h)²/a² + (y-k)²/b² = 1".to_string(),
            ],
            patterns: vec![
                EquationPattern {
                    pattern: r"x\^?2/(\d+)\s*\+\s*y\^?2/(\d+)\s*=\s*1".to_string(),
                    shape_type: ShapeType::Ellipse,
                    parameter_extractor: "ellipse_standard".to_string(),
                    confidence: 0.9,
                },
            ],
            confidence: 0.9,
        });

        // خط مستقيم
        self.known_shapes.insert("line".to_string(), KnownShape {
            name: "خط مستقيم".to_string(),
            shape_type: ShapeType::Line,
            equations: vec![
                "y = mx + b".to_string(),
                "ax + by + c = 0".to_string(),
                "y = x".to_string(),
            ],
            patterns: vec![
                EquationPattern {
                    pattern: r"y\s*=\s*([+-]?\d*\.?\d*)x\s*([+-]\s*\d+\.?\d*)?".to_string(),
                    shape_type: ShapeType::Line,
                    parameter_extractor: "line_slope_intercept".to_string(),
                    confidence: 0.85,
                },
            ],
            confidence: 0.85,
        });
    }

    /// تحويل معادلة إلى شكل
    /// Expert recommendation: "albayan_rt_shape_from_equation"
    pub fn equation_to_shape(&mut self, equation: &str) -> Result<ShapeHandle, String> {
        // تنظيف المعادلة
        let cleaned_equation = self.clean_equation(equation);

        // تحليل المعادلة
        let analysis_result = self.equation_analyzer.analyze(&cleaned_equation)?;

        // مطابقة النمط
        let shape_type = self.pattern_matcher.match_pattern(&cleaned_equation, &analysis_result)?;

        // استخراج المعاملات
        let parameters = self.extract_parameters(&cleaned_equation, &shape_type)?;

        // حساب الخصائص
        let properties = self.calculate_properties(&shape_type, &parameters)?;

        // إنشاء الشكل
        let shape = GeometricShape {
            shape_type: shape_type.clone(),
            parameters,
            equations: vec![cleaned_equation.clone()],
            properties,
            confidence: 0.8, // سيتم حسابها بدقة أكبر
        };

        // إنشاء مقبض
        let handle = self.next_handle;
        self.next_handle += 1;

        // حفظ الشكل
        self.active_shapes.insert(handle, shape);

        Ok(handle)
    }

    /// تحويل شكل إلى معادلة
    /// Expert recommendation: "albayan_rt_equation_from_shape"
    pub fn shape_to_equation(&self, handle: ShapeHandle) -> Result<String, String> {
        let shape = self.active_shapes.get(&handle)
            .ok_or_else(|| "مقبض الشكل غير صالح".to_string())?;

        match shape.shape_type {
            ShapeType::Circle => {
                if shape.parameters.len() >= 3 {
                    let h = shape.parameters[0];
                    let k = shape.parameters[1];
                    let r = shape.parameters[2];

                    if h == 0.0 && k == 0.0 {
                        Ok(format!("x² + y² = {}", r * r))
                    } else {
                        Ok(format!("(x - {})² + (y - {})² = {}", h, k, r * r))
                    }
                } else {
                    Err("معاملات الدائرة غير كافية".to_string())
                }
            },

            ShapeType::Parabola => {
                if shape.parameters.len() >= 3 {
                    let a = shape.parameters[0];
                    let b = shape.parameters[1];
                    let c = shape.parameters[2];

                    if b == 0.0 && c == 0.0 {
                        if a == 1.0 {
                            Ok("y = x²".to_string())
                        } else {
                            Ok(format!("y = {}x²", a))
                        }
                    } else {
                        Ok(format!("y = {}x² + {}x + {}", a, b, c))
                    }
                } else {
                    Err("معاملات القطع المكافئ غير كافية".to_string())
                }
            },

            ShapeType::Line => {
                if shape.parameters.len() >= 2 {
                    let m = shape.parameters[0]; // الميل
                    let b = shape.parameters[1]; // نقطة التقاطع مع y

                    if b == 0.0 {
                        if m == 1.0 {
                            Ok("y = x".to_string())
                        } else {
                            Ok(format!("y = {}x", m))
                        }
                    } else {
                        Ok(format!("y = {}x + {}", m, b))
                    }
                } else {
                    Err("معاملات الخط غير كافية".to_string())
                }
            },

            ShapeType::Ellipse => {
                if shape.parameters.len() >= 4 {
                    let h = shape.parameters[0];
                    let k = shape.parameters[1];
                    let a = shape.parameters[2];
                    let b = shape.parameters[3];

                    if h == 0.0 && k == 0.0 {
                        Ok(format!("x²/{} + y²/{} = 1", a * a, b * b))
                    } else {
                        Ok(format!("(x - {})²/{} + (y - {})²/{} = 1", h, a * a, k, b * b))
                    }
                } else {
                    Err("معاملات القطع الناقص غير كافية".to_string())
                }
            },

            _ => Err(format!("نوع الشكل {:?} غير مدعوم حالياً", shape.shape_type)),
        }
    }

    /// الحصول على معلومات الشكل
    pub fn get_shape_info(&self, handle: ShapeHandle) -> Result<&GeometricShape, String> {
        self.active_shapes.get(&handle)
            .ok_or_else(|| "مقبض الشكل غير صالح".to_string())
    }

    /// حذف شكل
    pub fn remove_shape(&mut self, handle: ShapeHandle) -> Result<(), String> {
        self.active_shapes.remove(&handle)
            .ok_or_else(|| "مقبض الشكل غير صالح".to_string())
            .map(|_| ())
    }

    /// تنظيف المعادلة
    fn clean_equation(&self, equation: &str) -> String {
        equation
            .replace(" ", "")
            .replace("²", "^2")
            .replace("³", "^3")
            .to_lowercase()
    }

    /// استخراج المعاملات
    fn extract_parameters(&self, equation: &str, shape_type: &ShapeType) -> Result<Vec<f64>, String> {
        match shape_type {
            ShapeType::Circle => self.extract_circle_parameters(equation),
            ShapeType::Parabola => self.extract_parabola_parameters(equation),
            ShapeType::Line => self.extract_line_parameters(equation),
            ShapeType::Ellipse => self.extract_ellipse_parameters(equation),
            _ => Ok(vec![]),
        }
    }

    /// استخراج معاملات الدائرة
    fn extract_circle_parameters(&self, equation: &str) -> Result<Vec<f64>, String> {
        // نمط بسيط: x² + y² = r²
        if equation.contains("x^2+y^2=") {
            let parts: Vec<&str> = equation.split('=').collect();
            if parts.len() == 2 {
                let r_squared: f64 = parts[1].parse()
                    .map_err(|_| "لا يمكن تحليل نصف القطر".to_string())?;
                let r = r_squared.sqrt();
                return Ok(vec![0.0, 0.0, r]); // h=0, k=0, r
            }
        }

        // يمكن إضافة أنماط أكثر تعقيداً هنا
        Ok(vec![0.0, 0.0, 1.0]) // قيم افتراضية
    }

    /// استخراج معاملات القطع المكافئ
    fn extract_parabola_parameters(&self, equation: &str) -> Result<Vec<f64>, String> {
        // نمط بسيط: y = x²
        if equation == "y=x^2" {
            return Ok(vec![1.0, 0.0, 0.0]); // a=1, b=0, c=0
        }

        // يمكن إضافة أنماط أكثر تعقيداً هنا
        Ok(vec![1.0, 0.0, 0.0]) // قيم افتراضية
    }

    /// استخراج معاملات الخط
    fn extract_line_parameters(&self, equation: &str) -> Result<Vec<f64>, String> {
        // نمط بسيط: y = mx + b
        if equation.contains("y=") && equation.contains("x") {
            // تحليل بسيط - يمكن تحسينه
            return Ok(vec![1.0, 0.0]); // m=1, b=0
        }

        Ok(vec![1.0, 0.0]) // قيم افتراضية
    }

    /// استخراج معاملات القطع الناقص
    fn extract_ellipse_parameters(&self, equation: &str) -> Result<Vec<f64>, String> {
        // نمط بسيط: x²/a² + y²/b² = 1
        Ok(vec![0.0, 0.0, 1.0, 1.0]) // h=0, k=0, a=1, b=1
    }

    /// حساب خصائص الشكل
    fn calculate_properties(&self, shape_type: &ShapeType, parameters: &[f64]) -> Result<ShapeProperties, String> {
        match shape_type {
            ShapeType::Circle => {
                if parameters.len() >= 3 {
                    let h = parameters[0];
                    let k = parameters[1];
                    let r = parameters[2];

                    Ok(ShapeProperties {
                        area: std::f64::consts::PI * r * r,
                        perimeter: 2.0 * std::f64::consts::PI * r,
                        center: (h, k),
                        radius: Some(r),
                        dimensions: None,
                    })
                } else {
                    Err("معاملات الدائرة غير كافية لحساب الخصائص".to_string())
                }
            },

            ShapeType::Line => {
                Ok(ShapeProperties {
                    area: 0.0, // الخط ليس له مساحة
                    perimeter: f64::INFINITY, // الخط لا نهائي
                    center: (0.0, 0.0),
                    radius: None,
                    dimensions: None,
                })
            },

            _ => {
                // خصائص افتراضية للأشكال الأخرى
                Ok(ShapeProperties {
                    area: 0.0,
                    perimeter: 0.0,
                    center: (0.0, 0.0),
                    radius: None,
                    dimensions: None,
                })
            },
        }
    }
}

impl EquationAnalyzer {
    pub fn new() -> Self {
        EquationAnalyzer {
            equation_patterns: vec![],
        }
    }

    pub fn analyze(&self, equation: &str) -> Result<AnalysisResult, String> {
        // تحليل بسيط للمعادلة
        Ok(AnalysisResult {
            equation_type: "unknown".to_string(),
            variables: vec!["x".to_string(), "y".to_string()],
            degree: 2,
            coefficients: vec![],
        })
    }
}

#[derive(Debug)]
pub struct AnalysisResult {
    pub equation_type: String,
    pub variables: Vec<String>,
    pub degree: i32,
    pub coefficients: Vec<f64>,
}

impl PatternMatcher {
    pub fn new() -> Self {
        PatternMatcher {
            matching_rules: vec![],
        }
    }

    pub fn match_pattern(&self, equation: &str, _analysis: &AnalysisResult) -> Result<ShapeType, String> {
        // مطابقة أنماط بسيطة
        if equation.contains("x^2+y^2=") {
            return Ok(ShapeType::Circle);
        }

        if equation.contains("y=x^2") || equation.contains("y=") && equation.contains("x^2") {
            return Ok(ShapeType::Parabola);
        }

        if equation.contains("y=") && equation.contains("x") && !equation.contains("^") {
            return Ok(ShapeType::Line);
        }

        if equation.contains("/") && equation.contains("x^2") && equation.contains("y^2") {
            return Ok(ShapeType::Ellipse);
        }

        Ok(ShapeType::Unknown)
    }
}

/// المتغير العام للمحرك
static mut SHAPE_ENGINE: Option<ShapeInferenceEngine> = None;

/// تهيئة المحرك
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

// ==================== دوال FFI ====================
// Expert recommendation: "مع دالة FFI واحدة"

/// تحويل معادلة إلى شكل
/// Expert recommendation: "albayan_rt_shape_from_equation(equation_str: *const c_char) -> ShapeHandle"
#[no_mangle]
pub extern "C" fn albayan_rt_shape_from_equation(equation_str: *const c_char) -> ShapeHandle {
    if equation_str.is_null() {
        return 0; // مقبض خطأ
    }

    let equation = unsafe {
        match CStr::from_ptr(equation_str).to_str() {
            Ok(s) => s,
            Err(_) => return 0,
        }
    };

    match get_engine() {
        Ok(engine) => {
            match engine.equation_to_shape(equation) {
                Ok(handle) => handle,
                Err(_) => 0,
            }
        },
        Err(_) => 0,
    }
}

/// تحويل شكل إلى معادلة
/// Expert recommendation: "albayan_rt_equation_from_shape(shape_handle: ShapeHandle) -> *const c_char"
#[no_mangle]
pub extern "C" fn albayan_rt_equation_from_shape(shape_handle: ShapeHandle) -> *const c_char {
    if shape_handle == 0 {
        return ptr::null();
    }

    match get_engine() {
        Ok(engine) => {
            match engine.shape_to_equation(shape_handle) {
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

/// الحصول على معلومات الشكل
#[no_mangle]
pub extern "C" fn albayan_rt_get_shape_info(shape_handle: ShapeHandle) -> *const c_char {
    if shape_handle == 0 {
        return ptr::null();
    }

    match get_engine() {
        Ok(engine) => {
            match engine.get_shape_info(shape_handle) {
                Ok(shape) => {
                    let info = format!(
                        "{{\"type\":\"{:?}\",\"confidence\":{},\"area\":{},\"perimeter\":{}}}",
                        shape.shape_type,
                        shape.confidence,
                        shape.properties.area,
                        shape.properties.perimeter
                    );

                    match CString::new(info) {
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

/// حذف شكل وتحرير الذاكرة
#[no_mangle]
pub extern "C" fn albayan_rt_remove_shape(shape_handle: ShapeHandle) -> i32 {
    if shape_handle == 0 {
        return -1; // خطأ
    }

    match get_engine() {
        Ok(engine) => {
            match engine.remove_shape(shape_handle) {
                Ok(_) => 0, // نجح
                Err(_) => -1, // فشل
            }
        },
        Err(_) => -1,
    }
}

/// تهيئة محرك استنتاج الأشكال
#[no_mangle]
pub extern "C" fn albayan_rt_init_shape_engine() -> i32 {
    initialize_shape_engine();
    0 // نجح
}

/// تحرير ذاكرة النص المُرجع
#[no_mangle]
pub extern "C" fn albayan_rt_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

/// الحصول على إحصائيات المحرك
#[no_mangle]
pub extern "C" fn albayan_rt_get_engine_stats() -> *const c_char {
    match get_engine() {
        Ok(engine) => {
            let stats = format!(
                "{{\"active_shapes\":{},\"known_shapes\":{}}}",
                engine.active_shapes.len(),
                engine.known_shapes.len()
            );

            match CString::new(stats) {
                Ok(c_string) => c_string.into_raw(),
                Err(_) => ptr::null(),
            }
        },
        Err(_) => ptr::null(),
    }
}
