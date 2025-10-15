//! Linguistic Intelligence Engine - محرك الذكاء اللغوي
//!
//! نظام تحليل الكلمات بناءً على دلالات الحروف الصوتية والبصرية
//! حدس مدروس لأكثر من 35 سنة - الكلمة تكشف دلالتها بنفسها

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

/// دلالة الحرف الصوتية
#[derive(Debug, Clone)]
pub struct PhoneticMeaning {
    /// المعنى الأساسي للصوت
    pub core_meaning: String,
    /// المعاني الثانوية
    pub secondary_meanings: Vec<String>,
    /// قوة الدلالة (0.0 إلى 1.0)
    pub strength: f64,
    /// السياقات المناسبة
    pub contexts: Vec<String>,
}

/// دلالة الحرف البصرية (الشكل)
#[derive(Debug, Clone)]
pub struct VisualMeaning {
    /// وصف الشكل
    pub shape_description: String,
    /// المعنى المستنبط من الشكل
    pub shape_meaning: String,
    /// الاتجاه والحركة
    pub direction_flow: String,
    /// التشابه مع أشكال طبيعية
    pub natural_resemblance: Vec<String>,
}

/// معلومات شاملة عن الحرف
#[derive(Debug, Clone)]
pub struct LetterIntelligence {
    /// الحرف نفسه
    pub letter: char,
    /// اللغة (عربي، إنجليزي، إلخ)
    pub language: String,
    /// الدلالة الصوتية
    pub phonetic_meaning: PhoneticMeaning,
    /// الدلالة البصرية
    pub visual_meaning: VisualMeaning,
    /// الموقع في الكلمة (بداية، وسط، نهاية)
    pub position_effects: HashMap<String, f64>,
    /// التفاعل مع حروف أخرى
    pub letter_interactions: HashMap<char, f64>,
}

/// قاعدة بيانات الذكاء اللغوي
#[derive(Debug)]
pub struct LinguisticIntelligenceDB {
    /// قاعدة بيانات الحروف العربية
    pub arabic_letters: HashMap<char, LetterIntelligence>,
    /// قاعدة بيانات الحروف الإنجليزية
    pub english_letters: HashMap<char, LetterIntelligence>,
    /// أنماط الكلمات المكتشفة
    pub word_patterns: HashMap<String, WordPattern>,
    /// إحصائيات التحليل
    pub analysis_stats: AnalysisStatistics,
}

/// نمط الكلمة المكتشف
#[derive(Debug, Clone)]
pub struct WordPattern {
    /// الكلمة
    pub word: String,
    /// المعنى المستنبط من الحروف
    pub inferred_meaning: String,
    /// قوة الاستنباط
    pub confidence: f64,
    /// الحروف المساهمة
    pub contributing_letters: Vec<char>,
    /// التفسير التفصيلي
    pub detailed_analysis: String,
}

/// إحصائيات التحليل
#[derive(Debug, Clone)]
pub struct AnalysisStatistics {
    /// عدد الكلمات المحللة
    pub words_analyzed: u64,
    /// معدل دقة التنبؤ
    pub prediction_accuracy: f64,
    /// أكثر الأنماط شيوعاً
    pub common_patterns: Vec<String>,
}

/// محرك الذكاء اللغوي
#[derive(Debug)]
pub struct LinguisticIntelligenceEngine {
    /// قاعدة البيانات
    pub database: LinguisticIntelligenceDB,
    /// خوارزميات التحليل
    pub analysis_algorithms: AnalysisAlgorithms,
    /// نظام التعلم التكيفي
    pub adaptive_learning: AdaptiveLearningSystem,
}

/// خوارزميات التحليل
#[derive(Debug)]
pub struct AnalysisAlgorithms {
    /// وزن الدلالة الصوتية
    pub phonetic_weight: f64,
    /// وزن الدلالة البصرية
    pub visual_weight: f64,
    /// وزن موقع الحرف
    pub position_weight: f64,
    /// وزن التفاعل بين الحروف
    pub interaction_weight: f64,
}

/// نظام التعلم التكيفي
#[derive(Debug)]
pub struct AdaptiveLearningSystem {
    /// الأنماط المتعلمة
    pub learned_patterns: Vec<LearnedPattern>,
    /// معدل التعلم
    pub learning_rate: f64,
    /// عتبة الثقة للقبول
    pub confidence_threshold: f64,
}

/// نمط متعلم
#[derive(Debug, Clone)]
pub struct LearnedPattern {
    /// النمط
    pub pattern: String,
    /// المعنى المرتبط
    pub associated_meaning: String,
    /// عدد مرات الظهور
    pub frequency: u32,
    /// مستوى الثقة
    pub confidence: f64,
}

impl LinguisticIntelligenceDB {
    /// إنشاء قاعدة بيانات جديدة مع الحروف العربية
    pub fn new() -> Self {
        let mut arabic_letters = HashMap::new();

        // إضافة الحروف العربية مع دلالاتها (حسب الحدس المدروس)

        // حرف الجيم - شكل رأس الشجرة
        arabic_letters.insert('ج', LetterIntelligence {
            letter: 'ج',
            language: "عربي".to_string(),
            phonetic_meaning: PhoneticMeaning {
                core_meaning: "التجمع والاجتماع".to_string(),
                secondary_meanings: vec!["القوة".to_string(), "الصلابة".to_string()],
                strength: 0.8,
                contexts: vec!["طبيعة".to_string(), "نبات".to_string()],
            },
            visual_meaning: VisualMeaning {
                shape_description: "شكل منحني يشبه رأس الشجرة".to_string(),
                shape_meaning: "التاج والقمة".to_string(),
                direction_flow: "من الأسفل إلى الأعلى".to_string(),
                natural_resemblance: vec!["رأس الشجرة".to_string(), "القبة".to_string()],
            },
            position_effects: {
                let mut effects = HashMap::new();
                effects.insert("بداية".to_string(), 0.9);
                effects.insert("وسط".to_string(), 0.6);
                effects.insert("نهاية".to_string(), 0.7);
                effects
            },
            letter_interactions: HashMap::new(),
        });

        // حرف الراء - التفرع والانسياب
        arabic_letters.insert('ر', LetterIntelligence {
            letter: 'ر',
            language: "عربي".to_string(),
            phonetic_meaning: PhoneticMeaning {
                core_meaning: "التفرع والانسياب".to_string(),
                secondary_meanings: vec!["الحركة".to_string(), "التدفق".to_string()],
                strength: 0.85,
                contexts: vec!["حركة".to_string(), "ماء".to_string()],
            },
            visual_meaning: VisualMeaning {
                shape_description: "خط منحني يوحي بالتدفق".to_string(),
                shape_meaning: "الانسياب والتفرع".to_string(),
                direction_flow: "انسيابي متدفق".to_string(),
                natural_resemblance: vec!["تفرع الأغصان".to_string(), "تدفق الماء".to_string()],
            },
            position_effects: {
                let mut effects = HashMap::new();
                effects.insert("بداية".to_string(), 0.7);
                effects.insert("وسط".to_string(), 0.9);
                effects.insert("نهاية".to_string(), 0.8);
                effects
            },
            letter_interactions: HashMap::new(),
        });

        // حرف الشين - التشتت والتشعب
        arabic_letters.insert('ش', LetterIntelligence {
            letter: 'ش',
            language: "عربي".to_string(),
            phonetic_meaning: PhoneticMeaning {
                core_meaning: "التشتت والتشعب".to_string(),
                secondary_meanings: vec!["الانتشار".to_string(), "التوزع".to_string()],
                strength: 0.8,
                contexts: vec!["انتشار".to_string(), "تشعب".to_string()],
            },
            visual_meaning: VisualMeaning {
                shape_description: "نقاط متعددة تشير للتشعب".to_string(),
                shape_meaning: "التشتت والانتشار".to_string(),
                direction_flow: "متعدد الاتجاهات".to_string(),
                natural_resemblance: vec!["أوراق الشجر".to_string(), "الأشعة".to_string()],
            },
            position_effects: {
                let mut effects = HashMap::new();
                effects.insert("بداية".to_string(), 0.8);
                effects.insert("وسط".to_string(), 0.7);
                effects.insert("نهاية".to_string(), 0.9);
                effects
            },
            letter_interactions: HashMap::new(),
        });

        // إضافة المزيد من الحروف العربية...
        // (سيتم إكمالها تدريجياً)

        Self {
            arabic_letters,
            english_letters: HashMap::new(), // جاهزة للملء
            word_patterns: HashMap::new(),
            analysis_stats: AnalysisStatistics {
                words_analyzed: 0,
                prediction_accuracy: 0.0,
                common_patterns: Vec::new(),
            },
        }
    }

    /// إضافة حرف إنجليزي جديد
    pub fn add_english_letter(&mut self, letter_info: LetterIntelligence) {
        self.english_letters.insert(letter_info.letter, letter_info);
    }

    /// تحليل كلمة وكشف دلالتها
    pub fn analyze_word(&mut self, word: &str, language: &str) -> Option<WordPattern> {
        let letters_db = match language {
            "عربي" => &self.arabic_letters,
            "english" => &self.english_letters,
            _ => return None,
        };

        let mut total_meaning_score = 0.0;
        let mut contributing_letters = Vec::new();
        let mut detailed_analysis = String::new();

        for (index, ch) in word.chars().enumerate() {
            if let Some(letter_info) = letters_db.get(&ch) {
                let position = if index == 0 {
                    "بداية"
                } else if index == word.len() - 1 {
                    "نهاية"
                } else {
                    "وسط"
                };

                let position_weight = letter_info.position_effects.get(position).unwrap_or(&0.5);
                let letter_contribution = letter_info.phonetic_meaning.strength * position_weight;

                total_meaning_score += letter_contribution;
                contributing_letters.push(ch);

                detailed_analysis.push_str(&format!(
                    "حرف '{}': {} (قوة: {:.2})\n",
                    ch,
                    letter_info.phonetic_meaning.core_meaning,
                    letter_contribution
                ));
            }
        }

        if !contributing_letters.is_empty() {
            let confidence = total_meaning_score / contributing_letters.len() as f64;
            let inferred_meaning = self.infer_word_meaning(word, &contributing_letters, letters_db);

            let pattern = WordPattern {
                word: word.to_string(),
                inferred_meaning,
                confidence,
                contributing_letters,
                detailed_analysis,
            };

            self.word_patterns.insert(word.to_string(), pattern.clone());
            self.analysis_stats.words_analyzed += 1;

            Some(pattern)
        } else {
            None
        }
    }

    /// استنباط معنى الكلمة من دلالات الحروف
    fn infer_word_meaning(
        &self,
        word: &str,
        letters: &[char],
        letters_db: &HashMap<char, LetterIntelligence>,
    ) -> String {
        let mut meanings = Vec::new();

        for &letter in letters {
            if let Some(letter_info) = letters_db.get(&letter) {
                meanings.push(letter_info.phonetic_meaning.core_meaning.clone());
            }
        }

        // دمج المعاني لتكوين معنى الكلمة
        if meanings.is_empty() {
            "غير محدد".to_string()
        } else {
            meanings.join(" + ")
        }
    }
}

/// مدير قاعدة البيانات العامة
static mut GLOBAL_LINGUISTIC_DB: Option<LinguisticIntelligenceDB> = None;

/// تهيئة قاعدة البيانات العامة
pub fn initialize_linguistic_intelligence() {
    unsafe {
        GLOBAL_LINGUISTIC_DB = Some(LinguisticIntelligenceDB::new());
    }
}

/// الحصول على مرجع لقاعدة البيانات العامة
pub fn get_linguistic_db() -> Option<&'static mut LinguisticIntelligenceDB> {
    unsafe { GLOBAL_LINGUISTIC_DB.as_mut() }
}

// ========== واجهة FFI للاستخدام من لغة البيان ==========

/// تهيئة محرك الذكاء اللغوي
#[no_mangle]
pub extern "C" fn albayan_rt_linguistic_initialize() -> c_int {
    initialize_linguistic_intelligence();
    1 // نجح
}

/// تحليل كلمة عربية
#[no_mangle]
pub extern "C" fn albayan_rt_analyze_arabic_word(word: *const c_char) -> c_int {
    if word.is_null() {
        return 0;
    }

    let word_str = match unsafe { CStr::from_ptr(word) }.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    if let Some(db) = get_linguistic_db() {
        if let Some(_pattern) = db.analyze_word(word_str, "عربي") {
            return 1; // تم التحليل بنجاح
        }
    }

    0 // فشل التحليل
}

/// إضافة حرف إنجليزي جديد
#[no_mangle]
pub extern "C" fn albayan_rt_add_english_letter(
    letter: c_char,
    phonetic_meaning: *const c_char,
    visual_meaning: *const c_char,
    strength: f64,
) -> c_int {
    if phonetic_meaning.is_null() || visual_meaning.is_null() {
        return 0;
    }

    let phonetic_str = match unsafe { CStr::from_ptr(phonetic_meaning) }.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let visual_str = match unsafe { CStr::from_ptr(visual_meaning) }.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    if let Some(db) = get_linguistic_db() {
        let letter_info = LetterIntelligence {
            letter: letter as u8 as char,
            language: "english".to_string(),
            phonetic_meaning: PhoneticMeaning {
                core_meaning: phonetic_str.to_string(),
                secondary_meanings: Vec::new(),
                strength,
                contexts: Vec::new(),
            },
            visual_meaning: VisualMeaning {
                shape_description: visual_str.to_string(),
                shape_meaning: visual_str.to_string(),
                direction_flow: "".to_string(),
                natural_resemblance: Vec::new(),
            },
            position_effects: HashMap::new(),
            letter_interactions: HashMap::new(),
        };

        db.add_english_letter(letter_info);
        return 1;
    }

    0
}

/// الحصول على معنى كلمة محللة
#[no_mangle]
pub extern "C" fn albayan_rt_get_word_meaning(
    word: *const c_char,
    language: *const c_char,
) -> *const c_char {
    if word.is_null() || language.is_null() {
        return std::ptr::null();
    }

    let word_str = match unsafe { CStr::from_ptr(word) }.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null(),
    };

    let lang_str = match unsafe { CStr::from_ptr(language) }.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null(),
    };

    if let Some(db) = get_linguistic_db() {
        if let Some(pattern) = db.word_patterns.get(word_str) {
            if let Ok(c_string) = CString::new(pattern.inferred_meaning.clone()) {
                return c_string.into_raw();
            }
        }
    }

    std::ptr::null()
}

/// الحصول على إحصائيات التحليل
#[no_mangle]
pub extern "C" fn albayan_rt_get_analysis_stats() -> u64 {
    if let Some(db) = get_linguistic_db() {
        return db.analysis_stats.words_analyzed;
    }
    0
}
