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
        arabic_letters.insert(
            'ج',
            LetterIntelligence {
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
            },
        );

        // حرف الراء - التفرع والانسياب
        arabic_letters.insert(
            'ر',
            LetterIntelligence {
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
            },
        );

        // حرف الشين - التشتت والتشعب
        arabic_letters.insert(
            'ش',
            LetterIntelligence {
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
            },
        );

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
                    ch, letter_info.phonetic_meaning.core_meaning, letter_contribution
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

// ========== Letter Semantics (سيماء الحروف) ==========
// تصميم مرن قائم على ميزات مسماة قابلة للتوسعة لاحقاً
use std::sync::{OnceLock, RwLock};

#[derive(Debug, Clone, Default)]
pub struct LetterSemanticEntry {
    pub features: HashMap<String, f32>, // مثال مفاتيح: "shape:enclosure", "sound:friction", "polarity:pull"
}

static LETTER_SEM_DB: OnceLock<RwLock<HashMap<char, LetterSemanticEntry>>> = OnceLock::new();

/// تهيئة افتراضية وفق القواعد المقدّمة (قابلة للتوسعة)
pub fn ls_init_default() {
    let mut db: HashMap<char, LetterSemanticEntry> = HashMap::new();

    // سين: زحف/احتكاك + سور/التفاف
    db.insert(
        'س',
        LetterSemanticEntry {
            features: HashMap::from([
                ("sound:friction".to_string(), 0.9),
                ("sound:crawl".to_string(), 0.8),
                ("shape:wall".to_string(), 0.6),
            ]),
        },
    );

    // ص/ض/ظ: إطار/بيضاوي وصيوان أذن
    for ch in ['ص', 'ض', 'ظ'] {
        db.insert(
            ch,
            LetterSemanticEntry {
                features: HashMap::from([
                    ("shape:oval".to_string(), 0.8),
                    ("shape:enclosure".to_string(), 0.7),
                ]),
            },
        );
    }

    // ق: دقة/استدقاق + بعد
    db.insert(
        'ق',
        LetterSemanticEntry {
            features: HashMap::from([
                ("polarity:precision".to_string(), 0.85),
                ("concept:distance".to_string(), 0.6),
            ]),
        },
    );

    // ل: سحب/لمّ/التفاف
    db.insert(
        'ل',
        LetterSemanticEntry {
            features: HashMap::from([
                ("polarity:pull".to_string(), 0.9),
                ("shape:wrap".to_string(), 0.5),
            ]),
        },
    );

    // م: شفوي/فم/ضمّ
    db.insert(
        'م',
        LetterSemanticEntry {
            features: HashMap::from([
                ("place:labial".to_string(), 0.9),
                ("concept:mouth".to_string(), 0.8),
                ("polarity:contain".to_string(), 0.6),
            ]),
        },
    );

    // ب: تشبّع/انتقال
    db.insert(
        'ب',
        LetterSemanticEntry {
            features: HashMap::from([
                ("concept:saturation".to_string(), 0.75),
                ("concept:transfer".to_string(), 0.75),
            ]),
        },
    );

    // خ: خير/خبث (قطبية)، نتركها كقطبية عامة
    db.insert(
        'خ',
        LetterSemanticEntry {
            features: HashMap::from([
                ("polarity:good".to_string(), 0.5),
                ("polarity:bad".to_string(), 0.5),
            ]),
        },
    );

    // ش: انتشار/تشعّب
    db.insert(
        'ش',
        LetterSemanticEntry {
            features: HashMap::from([
                ("concept:spread".to_string(), 0.9),
                ("concept:branch".to_string(), 0.7),
            ]),
        },
    );

    // ر: تدفق/انسياب
    db.insert(
        'ر',
        LetterSemanticEntry {
            features: HashMap::from([
                ("sound:flow".to_string(), 0.9),
                ("concept:branch".to_string(), 0.5),
            ]),
        },
    );

    // الإنجليزية: s (التواء/تموّج)، o (عجلة/اندفاع), p/b/d (كأدوات/مجرفة)
    db.insert(
        's',
        LetterSemanticEntry {
            features: HashMap::from([
                ("shape:snake".to_string(), 0.6),
                ("sound:friction".to_string(), 0.7),
                ("concept:wavy".to_string(), 0.6),
            ]),
        },
    );
    db.insert(
        'o',
        LetterSemanticEntry {
            features: HashMap::from([
                ("shape:circle".to_string(), 0.9),
                ("concept:roll".to_string(), 0.8),
                ("concept:impulse".to_string(), 0.6),
            ]),
        },
    );
    for ch in ['p', 'b', 'd'] {
        db.insert(
            ch,
            LetterSemanticEntry {
                features: HashMap::from([
                    ("shape:tool".to_string(), 0.6),
                    ("concept:scoop".to_string(), 0.6),
                ]),
            },
        );
    }

    // إضافات وفق القائمة الجديدة المقدّمة:
    db.insert(
        'ء',
        LetterSemanticEntry {
            features: HashMap::from([
                ("concept:surprise".to_string(), 0.8),
                ("sound:alarm".to_string(), 0.7),
                ("place:guttural".to_string(), 0.7),
                ("psych:inner".to_string(), 0.6),
            ]),
        },
    );

    db.insert(
        'آ',
        LetterSemanticEntry {
            features: HashMap::from([
                ("concept:elevation".to_string(), 0.8),
                ("concept:compassion".to_string(), 0.7),
                ("concept:aggrandize".to_string(), 0.6),
            ]),
        },
    );

    db.insert(
        'و',
        LetterSemanticEntry {
            features: HashMap::from([
                ("concept:wonder".to_string(), 0.7),
                ("concept:attack".to_string(), 0.6),
                ("concept:ambush".to_string(), 0.6),
                ("place:labial".to_string(), 0.7),
                ("reality:concrete".to_string(), 0.6),
            ]),
        },
    );

    db.insert(
        'ي',
        LetterSemanticEntry {
            features: HashMap::from([
                ("psych:grief".to_string(), 0.8),
                ("psych:regret".to_string(), 0.7),
            ]),
        },
    );

    db.insert(
        'ج',
        LetterSemanticEntry {
            features: HashMap::from([
                ("concept:collect".to_string(), 0.8),
                ("concept:consolation".to_string(), 0.6),
                ("concept:grandeur".to_string(), 0.5),
            ]),
        },
    );

    db.insert(
        'د',
        LetterSemanticEntry {
            features: HashMap::from([
                ("concept:start_end".to_string(), 0.7),
                ("concept:stability".to_string(), 0.7),
                ("concept:gate".to_string(), 0.6),
                ("concept:opening".to_string(), 0.6),
            ]),
        },
    );

    db.insert(
        'ه',
        LetterSemanticEntry {
            features: HashMap::from([
                ("concept:effort".to_string(), 0.7),
                ("concept:result".to_string(), 0.6),
                ("concept:fruit".to_string(), 0.5),
                ("place:guttural".to_string(), 0.6),
                ("psych:inner".to_string(), 0.5),
            ]),
        },
    );

    db.insert(
        'ز',
        LetterSemanticEntry {
            features: HashMap::from([
                ("concept:slip".to_string(), 0.7),
                ("concept:supply".to_string(), 0.6),
            ]),
        },
    );

    // ح: صوت المشقة/العطش/التودد
    db.insert(
        'ح',
        LetterSemanticEntry {
            features: HashMap::from([
                ("sound:hard_effort".to_string(), 0.8),
                ("concept:thirst".to_string(), 0.7),
                ("concept:affection".to_string(), 0.5),
                ("place:guttural".to_string(), 0.7),
                ("psych:inner".to_string(), 0.6),
            ]),
        },
    );

    db.insert(
        'ط',
        LetterSemanticEntry {
            features: HashMap::from([
                ("concept:knock".to_string(), 0.8),
                ("concept:ask_permission".to_string(), 0.6),
                ("concept:escape".to_string(), 0.6),
                ("concept:soar".to_string(), 0.6),
            ]),
        },
    );

    db.insert(
        'ك',
        LetterSemanticEntry {
            features: HashMap::from([("concept:give".to_string(), 0.8)]),
        },
    );

    // ن: أنين/استقرار/ظهور/رمزية
    db.insert(
        'ن',
        LetterSemanticEntry {
            features: HashMap::from([
                ("sound:moan".to_string(), 0.7),
                ("concept:stability".to_string(), 0.7),
                ("concept:emergence".to_string(), 0.6),
                ("concept:placeholder".to_string(), 0.6),
            ]),
        },
    );

    // تعزيز ميزات ص
    if let Some(e) = db.get_mut(&'ص') {
        e.features.insert("sound:harder_than_s".to_string(), 0.7);
        e.features.insert("concept:listen".to_string(), 0.6);
        e.features.insert("concept:watch".to_string(), 0.6);
    }

    db.insert(
        'غ',
        LetterSemanticEntry {
            features: HashMap::from([
                ("sound:anger".to_string(), 0.8),
                ("concept:boil".to_string(), 0.7),
                ("concept:obscure".to_string(), 0.6),
                ("place:guttural".to_string(), 0.7),
                ("psych:inner".to_string(), 0.6),
            ]),
        },
    );

    db.insert(
        'ف',
        LetterSemanticEntry {
            features: HashMap::from([
                ("concept:gap".to_string(), 0.8),
                ("sound:explosion".to_string(), 0.6),
                ("place:labial".to_string(), 0.7),
                ("reality:concrete".to_string(), 0.6),
            ]),
        },
    );

    // ق موجودة، نُبقيها كما هي

    db.insert(
        'ذ',
        LetterSemanticEntry {
            features: HashMap::from([("sound:pleasure".to_string(), 0.8)]),
        },
    );

    // تعزيز ميزات ر
    if let Some(e) = db.get_mut(&'ر') {
        e.features.insert("concept:repeat".to_string(), 0.6);
    }

    db.insert(
        'ت',
        LetterSemanticEntry {
            features: HashMap::from([("concept:build".to_string(), 0.8)]),
        },
    );

    db.insert(
        'ث',
        LetterSemanticEntry {
            features: HashMap::from([
                ("concept:scatter_random".to_string(), 0.8),
                ("sound:stutter".to_string(), 0.6),
            ]),
        },
    );

    // ش موجودة

    db.insert(
        'ع',
        LetterSemanticEntry {
            features: HashMap::from([
                ("concept:push".to_string(), 0.8),
                ("concept:uproot".to_string(), 0.7),
                ("place:guttural".to_string(), 0.7),
                ("psych:inner".to_string(), 0.6),
            ]),
        },
    );

    // تعزيز ميزات خ (إضافة الخرق/الاختراق)
    if let Some(e) = db.get_mut(&'خ') {
        e.features.insert("sound:tear_pierce".to_string(), 0.7);
        e.features.insert("concept:perforate".to_string(), 0.6);
    }

    // تعزيز ميزات م (الرضا/الكتم)
    if let Some(e) = db.get_mut(&'م') {
        e.features.insert("concept:contentment".to_string(), 0.6);
        e.features.insert("concept:hide".to_string(), 0.5);
    }

    // تعزيز ميزات ب (الدك)
    if let Some(e) = db.get_mut(&'ب') {
        e.features.insert("concept:ramming".to_string(), 0.5);
    }

    // توليد الأضداد تلقائياً: لكل سمة، أنشئ anti:<name> بنفس الوزن
    for entry in db.values_mut() {
        let anti: Vec<(String, f32)> = entry
            .features
            .iter()
            .map(|(k, v)| (format!("anti:{}", k), *v))
            .collect();
        for (k, v) in anti {
            entry.features.entry(k).or_insert(v);
        }
    }

    let _ = LETTER_SEM_DB.set(RwLock::new(db));
}

/// إضافة/تحديث حرف بميزات مسماة (مرن لتوسعة لاحقة)
pub fn ls_register_letter(ch: char, features: Vec<(impl Into<String>, f32)>) {
    let lock = LETTER_SEM_DB.get_or_init(|| RwLock::new(HashMap::new()));
    let mut map = lock.write().expect("LETTER_SEM_DB poisoned");
    let entry = map.entry(ch).or_insert_with(LetterSemanticEntry::default);
    for (k, v) in features {
        entry.features.insert(k.into(), v);
    }
}

/// إنتاج متجه دلالي للكلمة + تقرير تفسير مساهمة كل حرف
pub fn ls_word_vector(word: &str) -> (HashMap<String, f32>, Vec<(char, HashMap<String, f32>)>) {
    let lock = LETTER_SEM_DB.get_or_init(|| RwLock::new(HashMap::new()));
    let db = lock.read().expect("LETTER_SEM_DB poisoned");
    let mut acc: HashMap<String, f32> = HashMap::new();
    let mut explain: Vec<(char, HashMap<String, f32>)> = Vec::new();

    let chars: Vec<char> = word.chars().collect();
    for (i, ch) in chars.iter().enumerate() {
        if let Some(entry) = db.get(ch) {
            let mut contrib = HashMap::new();
            // وزن موضعي بسيط: أول/وسط/آخر
            let w = if i == 0 {
                1.0
            } else if i == chars.len() - 1 {
                0.9
            } else {
                0.8
            } as f32;
            for (k, v) in &entry.features {
                let val = acc.get(k).cloned().unwrap_or(0.0) + v * w;
                acc.insert(k.clone(), val);
                contrib.insert(k.clone(), v * w);
            }
            explain.push((*ch, contrib));
        }
    }
    (acc, explain)
}

#[cfg(test)]
mod tests_letter_semantics {
    use super::*;

    #[test]
    fn test_word_vector_qalam() {
        ls_init_default();
        let (vec, explain) = ls_word_vector("قلم");
        // إشارات أساسية مستخلصة من وصفك: ق=دقة/بعد، ل=سحب، م=فم/ضم
        assert!(vec.get("polarity:precision").copied().unwrap_or(0.0) > 0.5);
        assert!(vec.get("polarity:pull").copied().unwrap_or(0.0) > 0.5);
        assert!(vec.get("concept:mouth").copied().unwrap_or(0.0) > 0.5);
        // تقرير تفسيري يحتوي على 3 أحرف على الأقل
        assert!(explain.len() >= 3);
    }
}
