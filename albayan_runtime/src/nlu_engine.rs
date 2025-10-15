//! Natural Language Understanding Engine - محرك فهم اللغة الطبيعية
//!
//! تنفيذ الأولوية الرابعة من توجيهات الخبير:
//! "بناء محلل NLU الأولي: قم ببناء محلل بسيط جدًا داخل المترجم يمكنه فهم جمل مثل "أ فوق ب" وترجمتها إلى استدعاء std::knowledge::assert_above"

use std::collections::HashMap;
use std::ffi::{CStr, CString, c_char, c_int};
use std::ptr;

/// محرك فهم اللغة الطبيعية العالمي
static mut GLOBAL_NLU_ENGINE: Option<NLUEngine> = None;

/// نوع العلاقة المستخرجة من الجملة
#[derive(Debug, Clone, PartialEq)]
pub enum NLURelationType {
    /// علاقات مكانية
    Above,      // فوق
    Below,      // تحت
    LeftOf,     // يسار
    RightOf,    // يمين
    InFrontOf,  // أمام
    Behind,     // خلف
    Near,       // قريب
    Far,        // بعيد

    /// علاقات تفاعلية
    Eats,       // يأكل
    Touches,    // يلمس
    Hits,       // يضرب
    Loves,      // يحب
    Sees,       // يرى

    /// علاقة غير معروفة
    Unknown(String),
}

/// نتيجة تحليل الجملة
#[derive(Debug, Clone)]
pub struct NLUParseResult {
    /// الجملة الأصلية
    pub original_sentence: String,
    /// نوع العلاقة المكتشفة
    pub relation_type: NLURelationType,
    /// الفاعل
    pub subject: Option<String>,
    /// المفعول
    pub object: Option<String>,
    /// مستوى الثقة (0.0 إلى 1.0)
    pub confidence: f64,
    /// استدعاء API المقترح
    pub suggested_api_call: String,
    /// رسالة خطأ (إن وجدت)
    pub error_message: Option<String>,
}

/// محرك فهم اللغة الطبيعية
pub struct NLUEngine {
    /// قاموس الكلمات المفتاحية للعلاقات
    relation_keywords: HashMap<String, NLURelationType>,
    /// آخر نتيجة تحليل
    last_parse_result: Option<NLUParseResult>,
    /// إحصائيات الاستخدام
    total_parses: u64,
    successful_parses: u64,
}

impl NLUEngine {
    /// إنشاء محرك جديد
    pub fn new() -> Self {
        let mut relation_keywords = HashMap::new();

        // إضافة الكلمات المفتاحية للعلاقات المكانية
        relation_keywords.insert("فوق".to_string(), NLURelationType::Above);
        relation_keywords.insert("تحت".to_string(), NLURelationType::Below);
        relation_keywords.insert("يسار".to_string(), NLURelationType::LeftOf);
        relation_keywords.insert("يمين".to_string(), NLURelationType::RightOf);
        relation_keywords.insert("أمام".to_string(), NLURelationType::InFrontOf);
        relation_keywords.insert("خلف".to_string(), NLURelationType::Behind);
        relation_keywords.insert("قريب".to_string(), NLURelationType::Near);
        relation_keywords.insert("بعيد".to_string(), NLURelationType::Far);

        // إضافة الكلمات المفتاحية للعلاقات التفاعلية
        relation_keywords.insert("يأكل".to_string(), NLURelationType::Eats);
        relation_keywords.insert("يلمس".to_string(), NLURelationType::Touches);
        relation_keywords.insert("يضرب".to_string(), NLURelationType::Hits);
        relation_keywords.insert("يحب".to_string(), NLURelationType::Loves);
        relation_keywords.insert("يرى".to_string(), NLURelationType::Sees);

        Self {
            relation_keywords,
            last_parse_result: None,
            total_parses: 0,
            successful_parses: 0,
        }
    }

    /// تحليل جملة طبيعية
    pub fn parse_sentence(&mut self, sentence: &str) -> NLUParseResult {
        self.total_parses += 1;

        let words: Vec<&str> = sentence.trim().split_whitespace().collect();

        if words.is_empty() {
            return NLUParseResult {
                original_sentence: sentence.to_string(),
                relation_type: NLURelationType::Unknown("empty".to_string()),
                subject: None,
                object: None,
                confidence: 0.0,
                suggested_api_call: "// جملة فارغة".to_string(),
                error_message: Some("الجملة فارغة".to_string()),
            };
        }

        // تحليل النمط الأساسي: "أ علاقة ب"
        if words.len() == 3 {
            let subject = words[0].to_string();
            let relation_word = words[1];
            let object = words[2].to_string();

            if let Some(relation_type) = self.relation_keywords.get(relation_word) {
                let api_call = self.generate_api_call(relation_type, &subject, &object);
                let confidence = 0.9; // ثقة عالية للأنماط البسيطة

                self.successful_parses += 1;

                let result = NLUParseResult {
                    original_sentence: sentence.to_string(),
                    relation_type: relation_type.clone(),
                    subject: Some(subject),
                    object: Some(object),
                    confidence,
                    suggested_api_call: api_call,
                    error_message: None,
                };

                self.last_parse_result = Some(result.clone());
                return result;
            }
        }

        // إذا لم نتمكن من التحليل
        NLUParseResult {
            original_sentence: sentence.to_string(),
            relation_type: NLURelationType::Unknown("unrecognized_pattern".to_string()),
            subject: None,
            object: None,
            confidence: 0.0,
            suggested_api_call: format!("// لا يمكن تحليل: {}", sentence),
            error_message: Some("نمط غير معروف".to_string()),
        }
    }

    /// توليد استدعاء API مناسب
    fn generate_api_call(&self, relation: &NLURelationType, subject: &str, object: &str) -> String {
        match relation {
            NLURelationType::Above => {
                format!("std::knowledge::assert_above(&Object::new(\"{}\"), &Object::new(\"{}\"))", subject, object)
            },
            NLURelationType::Below => {
                format!("std::knowledge::assert_below(&Object::new(\"{}\"), &Object::new(\"{}\"))", subject, object)
            },
            NLURelationType::LeftOf => {
                format!("std::knowledge::assert_left_of(&Object::new(\"{}\"), &Object::new(\"{}\"))", subject, object)
            },
            NLURelationType::RightOf => {
                format!("std::knowledge::assert_right_of(&Object::new(\"{}\"), &Object::new(\"{}\"))", subject, object)
            },
            NLURelationType::Eats => {
                format!("std::knowledge::assert_eats(&Object::new(\"{}\"), &Object::new(\"{}\"))", subject, object)
            },
            NLURelationType::Touches => {
                format!("std::knowledge::assert_touches(&Object::new(\"{}\"), &Object::new(\"{}\"))", subject, object)
            },
            NLURelationType::Hits => {
                format!("std::knowledge::assert_hits(&Object::new(\"{}\"), &Object::new(\"{}\"))", subject, object)
            },
            NLURelationType::Loves => {
                format!("std::knowledge::assert_loves(&Object::new(\"{}\"), &Object::new(\"{}\"))", subject, object)
            },
            NLURelationType::Sees => {
                format!("std::knowledge::assert_sees(&Object::new(\"{}\"), &Object::new(\"{}\"))", subject, object)
            },
            _ => format!("// علاقة غير مدعومة: {} {} {}", subject, "?", object),
        }
    }

    /// الحصول على آخر نتيجة تحليل
    pub fn get_last_result(&self) -> Option<&NLUParseResult> {
        self.last_parse_result.as_ref()
    }

    /// الحصول على إحصائيات الاستخدام
    pub fn get_statistics(&self) -> (u64, u64, f64) {
        let success_rate = if self.total_parses > 0 {
            self.successful_parses as f64 / self.total_parses as f64
        } else {
            0.0
        };
        (self.total_parses, self.successful_parses, success_rate)
    }

    /// إضافة كلمة مفتاحية جديدة
    pub fn add_relation_keyword(&mut self, keyword: String, relation: NLURelationType) {
        self.relation_keywords.insert(keyword, relation);
    }
}

/// تهيئة محرك فهم اللغة الطبيعية
pub fn initialize_nlu_engine() {
    unsafe {
        GLOBAL_NLU_ENGINE = Some(NLUEngine::new());
    }
}

/// الحصول على مرجع للمحرك العالمي
fn get_nlu_engine() -> &'static mut NLUEngine {
    unsafe {
        GLOBAL_NLU_ENGINE.as_mut().expect("NLU Engine not initialized")
    }
}

/// دوال FFI للتكامل مع لغة البيان

/// تحليل جملة طبيعية
#[no_mangle]
pub extern "C" fn albayan_rt_parse_natural_language(sentence: *const c_char) -> c_int {
    if sentence.is_null() {
        return 0;
    }

    let c_str = unsafe { CStr::from_ptr(sentence) };
    let sentence_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let engine = get_nlu_engine();
    let result = engine.parse_sentence(sentence_str);

    if result.error_message.is_none() && result.confidence > 0.5 {
        1 // نجح
    } else {
        0 // فشل
    }
}

/// الحصول على مستوى الثقة لآخر تحليل
#[no_mangle]
pub extern "C" fn albayan_rt_get_parse_confidence() -> f64 {
    let engine = get_nlu_engine();
    if let Some(result) = engine.get_last_result() {
        result.confidence
    } else {
        0.0
    }
}

/// الحصول على استدعاء API المولد
#[no_mangle]
pub extern "C" fn albayan_rt_get_generated_api_call() -> *const c_char {
    let engine = get_nlu_engine();
    if let Some(result) = engine.get_last_result() {
        let c_string = CString::new(result.suggested_api_call.clone()).unwrap_or_default();
        c_string.into_raw()
    } else {
        ptr::null()
    }
}

/// الحصول على إحصائيات الاستخدام
#[no_mangle]
pub extern "C" fn albayan_rt_get_nlu_statistics() -> u64 {
    let engine = get_nlu_engine();
    let (total, successful, _) = engine.get_statistics();
    (total << 32) | successful
}

/// إضافة كلمة مفتاحية جديدة
#[no_mangle]
pub extern "C" fn albayan_rt_add_nlu_keyword(
    keyword: *const c_char,
    relation_type: c_int,
) -> c_int {
    if keyword.is_null() {
        return 0;
    }

    let c_str = unsafe { CStr::from_ptr(keyword) };
    let keyword_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return 0,
    };

    let relation = match relation_type {
        1 => NLURelationType::Above,
        2 => NLURelationType::Below,
        3 => NLURelationType::Eats,
        4 => NLURelationType::Touches,
        _ => NLURelationType::Unknown("custom".to_string()),
    };

    let engine = get_nlu_engine();
    engine.add_relation_keyword(keyword_str, relation);

    1 // نجح
}
