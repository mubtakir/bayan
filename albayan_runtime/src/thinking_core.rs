//! ThinkingCore - محلل الكلمات الأولي
//! تنفيذ الأولوية الثالثة: بناء محلل الكلمات الأولي
//! Expert recommendation: Priority 3 - Build Initial Word Analyzer

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use crate::linguistic_db::{get_linguistic_analyzer, WordSemanticSignature};

/// نواة التفكير - محلل الكلمات الدلالي الثوري
/// ThinkingCore - Revolutionary Semantic Word Analyzer
#[derive(Debug, Clone)]
pub struct ThinkingCore {
    /// تاريخ التحليلات المنجزة
    analysis_history: Vec<ThinkingWordAnalysisResult>,
    /// إحصائيات الأداء
    performance_stats: PerformanceStats,
    /// إعدادات التحليل
    analysis_settings: AnalysisSettings,
}

/// نتيجة تحليل الكلمة - نواة التفكير
/// Word Analysis Result - ThinkingCore
#[derive(Debug, Clone)]
pub struct ThinkingWordAnalysisResult {
    /// الكلمة المحللة
    pub word: String,
    /// اللغة
    pub language: String,
    /// التوقيع الدلالي
    pub semantic_signature: Vec<String>,
    /// قوة التوقيع (0.0 - 1.0)
    pub signature_strength: f64,
    /// تحليل الطاقة
    pub energy_analysis: String,
    /// العنصر المهيمن
    pub dominant_element: String,
    /// دلالات الحروف الفردية
    pub character_semantics: Vec<CharacterAnalysis>,
    /// الوقت المستغرق في التحليل (ميكروثانية)
    pub analysis_time_microseconds: u64,
}

/// تحليل الحرف الفردي
/// Individual Character Analysis
#[derive(Debug, Clone)]
pub struct CharacterAnalysis {
    /// الحرف
    pub character: char,
    /// دلالات الصوت
    pub sound_meanings: Vec<String>,
    /// دلالات الشكل
    pub shape_meanings: Vec<String>,
    /// نوع الطاقة
    pub energy_type: String,
    /// العنصر الطبيعي
    pub natural_element: String,
    /// قوة التأثير
    pub influence_strength: f64,
}

/// إحصائيات الأداء
/// Performance Statistics
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    /// إجمالي التحليلات
    pub total_analyses: u64,
    /// التحليلات الناجحة
    pub successful_analyses: u64,
    /// متوسط وقت التحليل (ميكروثانية)
    pub average_analysis_time: f64,
    /// أسرع تحليل (ميكروثانية)
    pub fastest_analysis: u64,
    /// أبطأ تحليل (ميكروثانية)
    pub slowest_analysis: u64,
}

/// إعدادات التحليل
/// Analysis Settings
#[derive(Debug, Clone)]
pub struct AnalysisSettings {
    /// تفعيل التحليل العميق
    pub deep_analysis_enabled: bool,
    /// تفعيل تحليل الطاقة
    pub energy_analysis_enabled: bool,
    /// تفعيل تحليل العناصر
    pub element_analysis_enabled: bool,
    /// الحد الأدنى لقوة التوقيع
    pub minimum_signature_strength: f64,
}

impl Default for AnalysisSettings {
    fn default() -> Self {
        Self {
            deep_analysis_enabled: true,
            energy_analysis_enabled: true,
            element_analysis_enabled: true,
            minimum_signature_strength: 0.1,
        }
    }
}

impl Default for PerformanceStats {
    fn default() -> Self {
        Self {
            total_analyses: 0,
            successful_analyses: 0,
            average_analysis_time: 0.0,
            fastest_analysis: u64::MAX,
            slowest_analysis: 0,
        }
    }
}

impl ThinkingCore {
    /// إنشاء نواة تفكير جديدة
    /// Create new ThinkingCore instance
    pub fn new() -> Self {
        Self {
            analysis_history: Vec::new(),
            performance_stats: PerformanceStats::default(),
            analysis_settings: AnalysisSettings::default(),
        }
    }

    /// تحليل كلمة - الوظيفة الأساسية
    /// Analyze word - Core function
    pub fn analyze_word(&mut self, word: &str, language: &str) -> ThinkingWordAnalysisResult {
        let start_time = std::time::Instant::now();

        // تحديث الإحصائيات
        self.performance_stats.total_analyses += 1;

        // الحصول على التحليل من قاعدة البيانات اللغوية
        let semantic_signature = if let Some(mut analyzer) = get_linguistic_analyzer() {
            analyzer.analyze_word(word, language)
        } else {
            // إنشاء توقيع افتراضي في حالة عدم توفر المحلل
            WordSemanticSignature {
                word: word.to_string(),
                language: language.to_string(),
                semantic_signature: vec!["غير محلل".to_string()],
                signature_strength: 0.0,
                energy_analysis: "غير متاح".to_string(),
                dominant_element: "غير محدد".to_string(),
            }
        };

        // تحليل الحروف الفردية
        let character_semantics = self.analyze_individual_characters(word, language);

        // حساب الوقت المستغرق
        let analysis_time = start_time.elapsed().as_micros() as u64;

        // إنشاء نتيجة التحليل
        let result = ThinkingWordAnalysisResult {
            word: word.to_string(),
            language: language.to_string(),
            semantic_signature: semantic_signature.semantic_signature,
            signature_strength: semantic_signature.signature_strength,
            energy_analysis: semantic_signature.energy_analysis,
            dominant_element: semantic_signature.dominant_element,
            character_semantics,
            analysis_time_microseconds: analysis_time,
        };

        // تحديث الإحصائيات
        self.update_performance_stats(analysis_time, true);

        // حفظ في التاريخ
        self.analysis_history.push(result.clone());

        result
    }

    /// تحليل الحروف الفردية
    /// Analyze individual characters
    fn analyze_individual_characters(&self, word: &str, language: &str) -> Vec<CharacterAnalysis> {
        let mut character_analyses = Vec::new();

        if let Some(analyzer) = get_linguistic_analyzer() {
            for ch in word.chars() {
                if let Some(char_semantics) = analyzer.get_character_info(ch, language) {
                    character_analyses.push(CharacterAnalysis {
                        character: ch,
                        sound_meanings: char_semantics.sound_meaning.clone(),
                        shape_meanings: char_semantics.shape_meaning.clone(),
                        energy_type: char_semantics.energy_type.clone(),
                        natural_element: char_semantics.natural_element.clone(),
                        influence_strength: char_semantics.influence_strength,
                    });
                } else {
                    // حرف غير معروف
                    character_analyses.push(CharacterAnalysis {
                        character: ch,
                        sound_meanings: vec!["غير معروف".to_string()],
                        shape_meanings: vec!["غير معروف".to_string()],
                        energy_type: "غير محدد".to_string(),
                        natural_element: "غير محدد".to_string(),
                        influence_strength: 0.0,
                    });
                }
            }
        }

        character_analyses
    }

    /// تحديث إحصائيات الأداء
    /// Update performance statistics
    fn update_performance_stats(&mut self, analysis_time: u64, success: bool) {
        if success {
            self.performance_stats.successful_analyses += 1;
        }

        // تحديث أسرع وأبطأ تحليل
        if analysis_time < self.performance_stats.fastest_analysis {
            self.performance_stats.fastest_analysis = analysis_time;
        }
        if analysis_time > self.performance_stats.slowest_analysis {
            self.performance_stats.slowest_analysis = analysis_time;
        }

        // تحديث متوسط الوقت
        let total_time = self.performance_stats.average_analysis_time * (self.performance_stats.total_analyses - 1) as f64;
        self.performance_stats.average_analysis_time = (total_time + analysis_time as f64) / self.performance_stats.total_analyses as f64;
    }

    /// الحصول على إحصائيات الأداء
    /// Get performance statistics
    pub fn get_performance_stats(&self) -> &PerformanceStats {
        &self.performance_stats
    }

    /// الحصول على تاريخ التحليلات
    /// Get analysis history
    pub fn get_analysis_history(&self) -> &[ThinkingWordAnalysisResult] {
        &self.analysis_history
    }

    /// تحديث إعدادات التحليل
    /// Update analysis settings
    pub fn update_settings(&mut self, settings: AnalysisSettings) {
        self.analysis_settings = settings;
    }

    /// مقارنة التوقيعات الدلالية
    /// Compare semantic signatures
    pub fn compare_semantic_signatures(&self, word1: &str, word2: &str, language: &str) -> f64 {
        let analysis1 = self.analyze_word_readonly(word1, language);
        let analysis2 = self.analyze_word_readonly(word2, language);

        // حساب التشابه بناءً على التوقيعات الدلالية
        let common_semantics = analysis1.semantic_signature.iter()
            .filter(|s1| analysis2.semantic_signature.contains(s1))
            .count();

        let total_semantics = analysis1.semantic_signature.len() + analysis2.semantic_signature.len();

        if total_semantics > 0 {
            (2.0 * common_semantics as f64) / total_semantics as f64
        } else {
            0.0
        }
    }

    /// تحليل للقراءة فقط (لا يؤثر على الإحصائيات)
    /// Read-only analysis (doesn't affect statistics)
    fn analyze_word_readonly(&self, word: &str, language: &str) -> ThinkingWordAnalysisResult {
        // نسخة مبسطة للقراءة فقط
        if let Some(mut analyzer) = get_linguistic_analyzer() {
            let semantic_signature = analyzer.analyze_word(word, language);
            ThinkingWordAnalysisResult {
                word: word.to_string(),
                language: language.to_string(),
                semantic_signature: semantic_signature.semantic_signature,
                signature_strength: semantic_signature.signature_strength,
                energy_analysis: semantic_signature.energy_analysis,
                dominant_element: semantic_signature.dominant_element,
                character_semantics: Vec::new(), // تبسيط للقراءة فقط
                analysis_time_microseconds: 0,
            }
        } else {
            ThinkingWordAnalysisResult {
                word: word.to_string(),
                language: language.to_string(),
                semantic_signature: vec!["غير محلل".to_string()],
                signature_strength: 0.0,
                energy_analysis: "غير متاح".to_string(),
                dominant_element: "غير محدد".to_string(),
                character_semantics: Vec::new(),
                analysis_time_microseconds: 0,
            }
        }
    }
}

/// الحالة العامة لنواة التفكير
/// Global ThinkingCore state
static GLOBAL_THINKING_CORE: OnceLock<Mutex<ThinkingCore>> = OnceLock::new();

/// تهيئة نواة التفكير العامة
/// Initialize global ThinkingCore
pub fn initialize_thinking_core() {
    let thinking_core = ThinkingCore::new();
    let _ = GLOBAL_THINKING_CORE.set(Mutex::new(thinking_core));
    println!("🧠 تم تهيئة نواة التفكير بنجاح!");
}

/// الحصول على نواة التفكير العامة
/// Get global ThinkingCore
fn get_thinking_core() -> Option<std::sync::MutexGuard<'static, ThinkingCore>> {
    GLOBAL_THINKING_CORE.get().map(|m| m.lock().unwrap())
}

/// تحليل كلمة باستخدام نواة التفكير العامة
/// Analyze word using global ThinkingCore
pub fn analyze_word_global(word: &str, language: &str) -> Option<ThinkingWordAnalysisResult> {
    get_thinking_core().map(|mut core| core.analyze_word(word, language))
}

/// الحصول على إحصائيات الأداء العامة
/// Get global performance statistics
pub fn get_global_performance_stats() -> Option<PerformanceStats> {
    get_thinking_core().map(|core| core.get_performance_stats().clone())
}

// FFI Functions for AlBayan Language Integration
use std::ffi::{CStr, CString, c_char, c_int};

/// تحليل كلمة - واجهة FFI
/// Analyze word - FFI interface
#[no_mangle]
pub extern "C" fn albayan_rt_analyze_word_thinking(
    word: *const c_char,
    language: *const c_char,
) -> c_int {
    if word.is_null() || language.is_null() {
        return 0; // فشل
    }

    let word_str = match unsafe { CStr::from_ptr(word) }.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let lang_str = match unsafe { CStr::from_ptr(language) }.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    match analyze_word_global(word_str, lang_str) {
        Some(_) => 1, // نجح
        None => 0,    // فشل
    }
}

/// الحصول على إحصائيات الأداء - واجهة FFI
/// Get performance statistics - FFI interface
#[no_mangle]
pub extern "C" fn albayan_rt_get_thinking_performance_stats() -> c_int {
    match get_global_performance_stats() {
        Some(stats) => stats.total_analyses as c_int,
        None => -1,
    }
}

/// مقارنة دلالات الكلمات - واجهة FFI
/// Compare word semantics - FFI interface
#[no_mangle]
pub extern "C" fn albayan_rt_compare_word_semantics(
    word1: *const c_char,
    word2: *const c_char,
    language: *const c_char,
) -> f64 {
    if word1.is_null() || word2.is_null() || language.is_null() {
        return 0.0;
    }

    let word1_str = match unsafe { CStr::from_ptr(word1) }.to_str() {
        Ok(s) => s,
        Err(_) => return 0.0,
    };

    let word2_str = match unsafe { CStr::from_ptr(word2) }.to_str() {
        Ok(s) => s,
        Err(_) => return 0.0,
    };

    let lang_str = match unsafe { CStr::from_ptr(language) }.to_str() {
        Ok(s) => s,
        Err(_) => return 0.0,
    };

    if let Some(mut core) = get_thinking_core() {
        core.compare_semantic_signatures(word1_str, word2_str, lang_str)
    } else {
        0.0
    }
}
