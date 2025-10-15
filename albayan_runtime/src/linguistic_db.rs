// linguistic_db.rs - قاعدة بيانات دلالات الحروف
//
// تطبيق رؤية الخبير حول "الحوسبة الاشتقاقية" و "علم أصل المعنى الحاسوبي"
// Computational Etymology/Semiotics - فهم "لماذا" الكلمات تعني ما تعنيه

use std::collections::HashMap;
use std::ffi::{CStr, CString, c_char, c_int};

/// دلالات الحرف الواحد - Character Semantics
#[derive(Debug, Clone)]
pub struct CharacterSemantics {
    /// المعاني الصوتية للحرف
    pub sound_meaning: Vec<String>,
    /// المعاني الشكلية للحرف
    pub shape_meaning: Vec<String>,
    /// طاقة الحرف (إيجابية/سلبية/محايدة)
    pub energy_type: String,
    /// ارتباط الحرف بعنصر طبيعي
    pub natural_element: String,
    /// قوة تأثير الحرف (0.0 - 1.0)
    pub influence_strength: f64,
    /// الخصائص الإضافية
    pub additional_properties: Vec<String>,
}

/// توقيع دلالي للكلمة - Semantic Signature
#[derive(Debug, Clone)]
pub struct WordSemanticSignature {
    /// الكلمة الأصلية
    pub word: String,
    /// اللغة
    pub language: String,
    /// التوقيع الدلالي المجمع
    pub semantic_signature: Vec<String>,
    /// قوة التوقيع الإجمالية
    pub signature_strength: f64,
    /// تحليل الطاقة الإجمالية
    pub energy_analysis: String,
    /// العنصر الطبيعي المهيمن
    pub dominant_element: String,
}

/// محلل الكلمات اللغوي - Linguistic Word Analyzer
#[derive(Debug)]
pub struct LinguisticWordAnalyzer {
    /// قاعدة بيانات الحروف العربية
    arabic_chars: HashMap<char, CharacterSemantics>,
    /// قاعدة بيانات الحروف الإنجليزية
    english_chars: HashMap<char, CharacterSemantics>,
    /// تاريخ التحليلات
    analysis_history: Vec<WordSemanticSignature>,
}

impl LinguisticWordAnalyzer {
    /// إنشاء محلل جديد مع قواعد البيانات المحملة
    pub fn new() -> Self {
        let mut analyzer = LinguisticWordAnalyzer {
            arabic_chars: HashMap::new(),
            english_chars: HashMap::new(),
            analysis_history: Vec::new(),
        };

        analyzer.load_arabic_character_database();
        analyzer.load_english_character_database();

        analyzer
    }

    /// تحميل قاعدة بيانات الحروف العربية
    /// مبنية على دراسات "فقه اللغة" و "علم الأصوات اللغوي"
    fn load_arabic_character_database(&mut self) {
        // حرف الألف - الوحدة والبداية والألوهية
        self.arabic_chars.insert('ا', CharacterSemantics {
            sound_meaning: vec![
                "الوحدة".to_string(),
                "البداية".to_string(),
                "الألوهية".to_string(),
                "الأصل".to_string(),
            ],
            shape_meaning: vec![
                "الاستقامة".to_string(),
                "الوقوف".to_string(),
                "العمود".to_string(),
            ],
            energy_type: "إيجابية قوية".to_string(),
            natural_element: "النور".to_string(),
            influence_strength: 1.0,
            additional_properties: vec!["حرف مقدس".to_string(), "أساس الكلام".to_string()],
        });

        // حرف الباء - البيت والاحتواء والبركة
        self.arabic_chars.insert('ب', CharacterSemantics {
            sound_meaning: vec![
                "البيت".to_string(),
                "الاحتواء".to_string(),
                "البركة".to_string(),
                "الحماية".to_string(),
            ],
            shape_meaning: vec![
                "الوعاء".to_string(),
                "الاحتضان".to_string(),
                "الانحناء".to_string(),
            ],
            energy_type: "إيجابية محايدة".to_string(),
            natural_element: "الأرض".to_string(),
            influence_strength: 0.8,
            additional_properties: vec!["حرف الحماية".to_string()],
        });

        // حرف التاء - التاج والعلو والتمام
        self.arabic_chars.insert('ت', CharacterSemantics {
            sound_meaning: vec![
                "التاج".to_string(),
                "العلو".to_string(),
                "التمام".to_string(),
                "الكمال".to_string(),
            ],
            shape_meaning: vec![
                "التاج".to_string(),
                "النقاط العلوية".to_string(),
                "الارتفاع".to_string(),
            ],
            energy_type: "إيجابية عالية".to_string(),
            natural_element: "الهواء".to_string(),
            influence_strength: 0.9,
            additional_properties: vec!["حرف الكمال".to_string()],
        });

        // حرف الثاء - الثبات والاستقرار
        self.arabic_chars.insert('ث', CharacterSemantics {
            sound_meaning: vec![
                "الثبات".to_string(),
                "الاستقرار".to_string(),
                "الاستمرارية".to_string(),
            ],
            shape_meaning: vec![
                "الثلاث نقاط".to_string(),
                "التوازن".to_string(),
                "الاستقرار".to_string(),
            ],
            energy_type: "محايدة مستقرة".to_string(),
            natural_element: "الأرض".to_string(),
            influence_strength: 0.7,
            additional_properties: vec!["حرف الثبات".to_string()],
        });

        // حرف الجيم - الجمع والاجتماع والجمال
        self.arabic_chars.insert('ج', CharacterSemantics {
            sound_meaning: vec![
                "الجمع".to_string(),
                "الاجتماع".to_string(),
                "الجمال".to_string(),
                "التجميع".to_string(),
            ],
            shape_meaning: vec![
                "شكل رأس الشجرة".to_string(),
                "التاج المنحني".to_string(),
                "الاحتضان".to_string(),
            ],
            energy_type: "إيجابية اجتماعية".to_string(),
            natural_element: "النبات".to_string(),
            influence_strength: 0.85,
            additional_properties: vec!["حرف الجمال".to_string(), "حرف الاجتماع".to_string()],
        });

        // حرف الحاء - الحياة والحيوية
        self.arabic_chars.insert('ح', CharacterSemantics {
            sound_meaning: vec![
                "الحياة".to_string(),
                "الحيوية".to_string(),
                "النشاط".to_string(),
                "الحركة".to_string(),
            ],
            shape_meaning: vec![
                "الانفتاح".to_string(),
                "التنفس".to_string(),
                "الدائرة المفتوحة".to_string(),
            ],
            energy_type: "إيجابية حيوية".to_string(),
            natural_element: "الهواء".to_string(),
            influence_strength: 0.9,
            additional_properties: vec!["حرف الحياة".to_string()],
        });

        // حرف الخاء - الخروج والخلاص
        self.arabic_chars.insert('خ', CharacterSemantics {
            sound_meaning: vec![
                "الخروج".to_string(),
                "الخلاص".to_string(),
                "التحرر".to_string(),
                "الانطلاق".to_string(),
            ],
            shape_meaning: vec![
                "النقطة العلوية".to_string(),
                "الارتفاع".to_string(),
                "التحرر".to_string(),
            ],
            energy_type: "محايدة متحررة".to_string(),
            natural_element: "الهواء".to_string(),
            influence_strength: 0.75,
            additional_properties: vec!["حرف التحرر".to_string()],
        });

        // حرف الدال - الدلالة والإرشاد
        self.arabic_chars.insert('د', CharacterSemantics {
            sound_meaning: vec![
                "الدلالة".to_string(),
                "الإرشاد".to_string(),
                "التوجيه".to_string(),
                "الهداية".to_string(),
            ],
            shape_meaning: vec![
                "الانحناء".to_string(),
                "التوجيه".to_string(),
                "الإشارة".to_string(),
            ],
            energy_type: "إيجابية هادية".to_string(),
            natural_element: "النور".to_string(),
            influence_strength: 0.8,
            additional_properties: vec!["حرف الهداية".to_string()],
        });

        // حرف الذال - الذل والخضوع
        self.arabic_chars.insert('ذ', CharacterSemantics {
            sound_meaning: vec![
                "الذل".to_string(),
                "الخضوع".to_string(),
                "التواضع".to_string(),
                "الانكسار".to_string(),
            ],
            shape_meaning: vec![
                "النقطة العلوية".to_string(),
                "الانحناء".to_string(),
                "التواضع".to_string(),
            ],
            energy_type: "سلبية متواضعة".to_string(),
            natural_element: "الأرض".to_string(),
            influence_strength: 0.6,
            additional_properties: vec!["حرف التواضع".to_string()],
        });

        // حرف الراء - الرحمة والرقة والتكرار والحركة
        self.arabic_chars.insert('ر', CharacterSemantics {
            sound_meaning: vec![
                "الرحمة".to_string(),
                "الرقة".to_string(),
                "التكرار".to_string(),
                "الحركة".to_string(),
                "الانسياب".to_string(),
                "التفرع".to_string(),
            ],
            shape_meaning: vec![
                "التفرع".to_string(),
                "الانسياب".to_string(),
                "الأغصان".to_string(),
            ],
            energy_type: "إيجابية متدفقة".to_string(),
            natural_element: "الماء".to_string(),
            influence_strength: 0.95,
            additional_properties: vec!["حرف الرحمة".to_string(), "حرف الحركة".to_string()],
        });

        // حرف الزاي - الزينة والجمال
        self.arabic_chars.insert('ز', CharacterSemantics {
            sound_meaning: vec![
                "الزينة".to_string(),
                "الجمال".to_string(),
                "البهاء".to_string(),
                "التألق".to_string(),
            ],
            shape_meaning: vec![
                "النقطة العلوية".to_string(),
                "التزيين".to_string(),
                "الإشراق".to_string(),
            ],
            energy_type: "إيجابية جمالية".to_string(),
            natural_element: "النور".to_string(),
            influence_strength: 0.8,
            additional_properties: vec!["حرف الجمال".to_string()],
        });

        // حرف السين - السلام والسكينة
        self.arabic_chars.insert('س', CharacterSemantics {
            sound_meaning: vec![
                "السلام".to_string(),
                "السكينة".to_string(),
                "الهدوء".to_string(),
                "الانسياب".to_string(),
            ],
            shape_meaning: vec![
                "الأسنان".to_string(),
                "التموج".to_string(),
                "الانسياب".to_string(),
            ],
            energy_type: "إيجابية هادئة".to_string(),
            natural_element: "الماء".to_string(),
            influence_strength: 0.75,
            additional_properties: vec!["حرف السلام".to_string()],
        });

        // حرف الشين - الشعاع والانتشار
        self.arabic_chars.insert('ش', CharacterSemantics {
            sound_meaning: vec![
                "الشعاع".to_string(),
                "الانتشار".to_string(),
                "الإشراق".to_string(),
                "التوزع".to_string(),
            ],
            shape_meaning: vec![
                "الثلاث نقاط".to_string(),
                "الانتشار".to_string(),
                "الأشعة".to_string(),
            ],
            energy_type: "إيجابية منتشرة".to_string(),
            natural_element: "النور".to_string(),
            influence_strength: 0.9,
            additional_properties: vec!["حرف الإشراق".to_string(), "حرف الانتشار".to_string()],
        });

        // حرف الصاد - الصدق والصلابة
        self.arabic_chars.insert('ص', CharacterSemantics {
            sound_meaning: vec![
                "الصدق".to_string(),
                "الصلابة".to_string(),
                "الوضوح".to_string(),
                "القوة".to_string(),
            ],
            shape_meaning: vec![
                "الاستقامة".to_string(),
                "القوة".to_string(),
                "الصلابة".to_string(),
            ],
            energy_type: "إيجابية قوية".to_string(),
            natural_element: "المعدن".to_string(),
            influence_strength: 0.85,
            additional_properties: vec!["حرف الصدق".to_string()],
        });

        // حرف الضاد - الضوء والوضوح
        self.arabic_chars.insert('ض', CharacterSemantics {
            sound_meaning: vec![
                "الضوء".to_string(),
                "الوضوح".to_string(),
                "الإضاءة".to_string(),
                "البيان".to_string(),
            ],
            shape_meaning: vec![
                "النقطة العلوية".to_string(),
                "الإضاءة".to_string(),
                "الوضوح".to_string(),
            ],
            energy_type: "إيجابية مضيئة".to_string(),
            natural_element: "النور".to_string(),
            influence_strength: 0.9,
            additional_properties: vec!["حرف الضوء".to_string(), "حرف العرب".to_string()],
        });

        // حرف الطاء - الطهارة والنظافة
        self.arabic_chars.insert('ط', CharacterSemantics {
            sound_meaning: vec![
                "الطهارة".to_string(),
                "النظافة".to_string(),
                "الصفاء".to_string(),
                "النقاء".to_string(),
            ],
            shape_meaning: vec![
                "الاستقامة".to_string(),
                "النظافة".to_string(),
                "الطهارة".to_string(),
            ],
            energy_type: "إيجابية طاهرة".to_string(),
            natural_element: "الماء".to_string(),
            influence_strength: 0.85,
            additional_properties: vec!["حرف الطهارة".to_string()],
        });

        // حرف الظاء - الظل والحماية
        self.arabic_chars.insert('ظ', CharacterSemantics {
            sound_meaning: vec![
                "الظل".to_string(),
                "الحماية".to_string(),
                "الستر".to_string(),
                "الراحة".to_string(),
            ],
            shape_meaning: vec![
                "النقطة العلوية".to_string(),
                "الظل".to_string(),
                "الحماية".to_string(),
            ],
            energy_type: "محايدة حامية".to_string(),
            natural_element: "الظل".to_string(),
            influence_strength: 0.7,
            additional_properties: vec!["حرف الحماية".to_string()],
        });

        // حرف العين - العين والرؤية والعلم
        self.arabic_chars.insert('ع', CharacterSemantics {
            sound_meaning: vec![
                "العين".to_string(),
                "الرؤية".to_string(),
                "العلم".to_string(),
                "المعرفة".to_string(),
            ],
            shape_meaning: vec![
                "العين".to_string(),
                "الدائرة".to_string(),
                "الرؤية".to_string(),
            ],
            energy_type: "إيجابية عالمة".to_string(),
            natural_element: "النور".to_string(),
            influence_strength: 0.95,
            additional_properties: vec!["حرف العلم".to_string(), "حرف الرؤية".to_string()],
        });

        // حرف الغين - الغيب والخفاء
        self.arabic_chars.insert('غ', CharacterSemantics {
            sound_meaning: vec![
                "الغيب".to_string(),
                "الخفاء".to_string(),
                "السر".to_string(),
                "الغموض".to_string(),
            ],
            shape_meaning: vec![
                "النقطة العلوية".to_string(),
                "الخفاء".to_string(),
                "الغموض".to_string(),
            ],
            energy_type: "محايدة غامضة".to_string(),
            natural_element: "الهواء".to_string(),
            influence_strength: 0.7,
            additional_properties: vec!["حرف الغيب".to_string()],
        });

        // حرف الفاء - الفم والكلام
        self.arabic_chars.insert('ف', CharacterSemantics {
            sound_meaning: vec![
                "الفم".to_string(),
                "الكلام".to_string(),
                "النطق".to_string(),
                "التعبير".to_string(),
            ],
            shape_meaning: vec![
                "النقطة العلوية".to_string(),
                "الفم".to_string(),
                "الكلام".to_string(),
            ],
            energy_type: "إيجابية ناطقة".to_string(),
            natural_element: "الهواء".to_string(),
            influence_strength: 0.8,
            additional_properties: vec!["حرف الكلام".to_string()],
        });

        // حرف القاف - القوة والشدة
        self.arabic_chars.insert('ق', CharacterSemantics {
            sound_meaning: vec![
                "القوة".to_string(),
                "الشدة".to_string(),
                "الصلابة".to_string(),
                "المتانة".to_string(),
            ],
            shape_meaning: vec![
                "النقطتان العلويتان".to_string(),
                "القوة".to_string(),
                "الصلابة".to_string(),
            ],
            energy_type: "إيجابية قوية".to_string(),
            natural_element: "المعدن".to_string(),
            influence_strength: 0.9,
            additional_properties: vec!["حرف القوة".to_string()],
        });

        // حرف الكاف - الكف واليد
        self.arabic_chars.insert('ك', CharacterSemantics {
            sound_meaning: vec![
                "الكف".to_string(),
                "اليد".to_string(),
                "الإمساك".to_string(),
                "القبض".to_string(),
            ],
            shape_meaning: vec![
                "شكل الكف".to_string(),
                "الإمساك".to_string(),
                "القبض".to_string(),
            ],
            energy_type: "محايدة ماسكة".to_string(),
            natural_element: "الأرض".to_string(),
            influence_strength: 0.75,
            additional_properties: vec!["حرف الإمساك".to_string()],
        });

        // حرف اللام - اللسان والكلام
        self.arabic_chars.insert('ل', CharacterSemantics {
            sound_meaning: vec![
                "اللسان".to_string(),
                "الكلام".to_string(),
                "البيان".to_string(),
                "التوضيح".to_string(),
            ],
            shape_meaning: vec![
                "الارتفاع".to_string(),
                "الإشارة".to_string(),
                "التوجيه".to_string(),
            ],
            energy_type: "إيجابية بيانية".to_string(),
            natural_element: "الهواء".to_string(),
            influence_strength: 0.9,
            additional_properties: vec!["حرف البيان".to_string(), "حرف الكلام".to_string()],
        });

        // حرف الميم - الماء والحياة
        self.arabic_chars.insert('م', CharacterSemantics {
            sound_meaning: vec![
                "الماء".to_string(),
                "الحياة".to_string(),
                "الأمومة".to_string(),
                "الرحم".to_string(),
            ],
            shape_meaning: vec![
                "الدائرة المغلقة".to_string(),
                "الاحتواء".to_string(),
                "الحماية".to_string(),
            ],
            energy_type: "إيجابية حيوية".to_string(),
            natural_element: "الماء".to_string(),
            influence_strength: 0.95,
            additional_properties: vec!["حرف الحياة".to_string(), "حرف الأمومة".to_string()],
        });

        // حرف النون - النور والإضاءة
        self.arabic_chars.insert('ن', CharacterSemantics {
            sound_meaning: vec![
                "النور".to_string(),
                "الإضاءة".to_string(),
                "الوضوح".to_string(),
                "الهداية".to_string(),
            ],
            shape_meaning: vec![
                "النقطة العلوية".to_string(),
                "الإضاءة".to_string(),
                "النور".to_string(),
            ],
            energy_type: "إيجابية مضيئة".to_string(),
            natural_element: "النور".to_string(),
            influence_strength: 0.9,
            additional_properties: vec!["حرف النور".to_string(), "حرف الهداية".to_string()],
        });

        // حرف الهاء - الهواء والتنفس
        self.arabic_chars.insert('ه', CharacterSemantics {
            sound_meaning: vec![
                "الهواء".to_string(),
                "التنفس".to_string(),
                "الحياة".to_string(),
                "النفس".to_string(),
            ],
            shape_meaning: vec![
                "الدائرة المفتوحة".to_string(),
                "التنفس".to_string(),
                "الانفتاح".to_string(),
            ],
            energy_type: "إيجابية حيوية".to_string(),
            natural_element: "الهواء".to_string(),
            influence_strength: 0.85,
            additional_properties: vec!["حرف التنفس".to_string(), "حرف الحياة".to_string()],
        });

        // حرف الواو - الواو والربط
        self.arabic_chars.insert('و', CharacterSemantics {
            sound_meaning: vec![
                "الواو".to_string(),
                "الربط".to_string(),
                "الوصل".to_string(),
                "التجميع".to_string(),
            ],
            shape_meaning: vec![
                "الدائرة".to_string(),
                "الربط".to_string(),
                "الوصل".to_string(),
            ],
            energy_type: "محايدة رابطة".to_string(),
            natural_element: "الهواء".to_string(),
            influence_strength: 0.7,
            additional_properties: vec!["حرف الربط".to_string(), "حرف الوصل".to_string()],
        });

        // حرف الياء - اليد والعمل
        self.arabic_chars.insert('ي', CharacterSemantics {
            sound_meaning: vec![
                "اليد".to_string(),
                "العمل".to_string(),
                "الفعل".to_string(),
                "الإنجاز".to_string(),
            ],
            shape_meaning: vec![
                "النقطتان السفليتان".to_string(),
                "اليد".to_string(),
                "العمل".to_string(),
            ],
            energy_type: "إيجابية عاملة".to_string(),
            natural_element: "الأرض".to_string(),
            influence_strength: 0.85,
            additional_properties: vec!["حرف العمل".to_string(), "حرف الفعل".to_string()],
        });

        // الحروف الإضافية

        // التاء المربوطة
        self.arabic_chars.insert('ة', CharacterSemantics {
            sound_meaning: vec![
                "التاج".to_string(),
                "الكمال".to_string(),
                "التمام".to_string(),
                "الجمال".to_string(),
            ],
            shape_meaning: vec![
                "التاج".to_string(),
                "النقطتان العلويتان".to_string(),
                "الكمال".to_string(),
            ],
            energy_type: "إيجابية مكملة".to_string(),
            natural_element: "الهواء".to_string(),
            influence_strength: 0.8,
            additional_properties: vec!["حرف الكمال".to_string(), "حرف التمام".to_string()],
        });

        // الألف المقصورة
        self.arabic_chars.insert('ى', CharacterSemantics {
            sound_meaning: vec![
                "الوحدة".to_string(),
                "الاستقامة".to_string(),
                "النهاية".to_string(),
                "الكمال".to_string(),
            ],
            shape_meaning: vec![
                "الاستقامة".to_string(),
                "النهاية".to_string(),
                "الكمال".to_string(),
            ],
            energy_type: "إيجابية مكملة".to_string(),
            natural_element: "النور".to_string(),
            influence_strength: 0.85,
            additional_properties: vec!["حرف النهاية".to_string()],
        });

        // الهمزة
        self.arabic_chars.insert('ء', CharacterSemantics {
            sound_meaning: vec![
                "البداية".to_string(),
                "الانطلاق".to_string(),
                "القوة".to_string(),
                "الشدة".to_string(),
            ],
            shape_meaning: vec![
                "الرأس".to_string(),
                "القمة".to_string(),
                "البداية".to_string(),
            ],
            energy_type: "إيجابية قوية".to_string(),
            natural_element: "النار".to_string(),
            influence_strength: 0.9,
            additional_properties: vec!["حرف البداية".to_string(), "حرف القوة".to_string()],
        });
    }

    /// تحميل قاعدة بيانات الحروف الإنجليزية (هيكل فارغ للخبير)
    fn load_english_character_database(&mut self) {
        // هيكل فارغ للخبير لملئه
        // سيتم ملؤه بناءً على توجيهات الخبير

        // مثال أولي لحرف T
        self.english_chars.insert('T', CharacterSemantics {
            sound_meaning: vec![
                "Tree structure".to_string(),
                "Vertical support".to_string(),
            ],
            shape_meaning: vec![
                "Tree trunk".to_string(),
                "Cross shape".to_string(),
                "Support beam".to_string(),
            ],
            energy_type: "Positive structural".to_string(),
            natural_element: "Wood".to_string(),
            influence_strength: 0.8,
            additional_properties: vec!["Structural letter".to_string()],
        });

        // مثال أولي لحرف R
        self.english_chars.insert('R', CharacterSemantics {
            sound_meaning: vec![
                "Rolling".to_string(),
                "Repetition".to_string(),
                "Vibration".to_string(),
            ],
            shape_meaning: vec![
                "Curved flow".to_string(),
                "Leg extension".to_string(),
            ],
            energy_type: "Dynamic flowing".to_string(),
            natural_element: "Air".to_string(),
            influence_strength: 0.85,
            additional_properties: vec!["Movement letter".to_string()],
        });

        // باقي الحروف الإنجليزية ستُملأ لاحقًا
    }

    /// تحليل كلمة وإنشاء توقيعها الدلالي
    pub fn analyze_word(&mut self, word: &str, language: &str) -> WordSemanticSignature {
        let mut semantic_signature = Vec::new();
        let mut total_strength = 0.0;
        let mut energy_types = Vec::new();
        let mut elements = Vec::new();

        let char_db = match language {
            "ar" | "arabic" => &self.arabic_chars,
            "en" | "english" => &self.english_chars,
            _ => &self.arabic_chars, // افتراضي
        };

        // تحليل كل حرف في الكلمة
        for ch in word.chars() {
            if let Some(semantics) = char_db.get(&ch) {
                // إضافة المعاني الصوتية والشكلية
                semantic_signature.extend(semantics.sound_meaning.clone());
                semantic_signature.extend(semantics.shape_meaning.clone());

                // تجميع القوة والطاقة والعناصر
                total_strength += semantics.influence_strength;
                energy_types.push(semantics.energy_type.clone());
                elements.push(semantics.natural_element.clone());
            }
        }

        // حساب القوة المتوسطة
        let average_strength = if word.chars().count() > 0 {
            total_strength / word.chars().count() as f64
        } else {
            0.0
        };

        // تحليل الطاقة المهيمنة
        let dominant_energy = self.analyze_dominant_energy(&energy_types);
        let dominant_element = self.analyze_dominant_element(&elements);

        let signature = WordSemanticSignature {
            word: word.to_string(),
            language: language.to_string(),
            semantic_signature,
            signature_strength: average_strength,
            energy_analysis: dominant_energy,
            dominant_element,
        };

        // حفظ في التاريخ
        self.analysis_history.push(signature.clone());

        signature
    }

    /// تحليل الطاقة المهيمنة
    fn analyze_dominant_energy(&self, energy_types: &[String]) -> String {
        // منطق بسيط لتحديد الطاقة المهيمنة
        let mut positive_count = 0;
        let mut negative_count = 0;
        let mut neutral_count = 0;

        for energy in energy_types {
            if energy.contains("إيجابية") || energy.contains("Positive") {
                positive_count += 1;
            } else if energy.contains("سلبية") || energy.contains("Negative") {
                negative_count += 1;
            } else {
                neutral_count += 1;
            }
        }

        if positive_count > negative_count && positive_count > neutral_count {
            "طاقة إيجابية مهيمنة".to_string()
        } else if negative_count > positive_count && negative_count > neutral_count {
            "طاقة سلبية مهيمنة".to_string()
        } else {
            "طاقة متوازنة".to_string()
        }
    }

    /// تحليل العنصر الطبيعي المهيمن
    fn analyze_dominant_element(&self, elements: &[String]) -> String {
        // منطق بسيط لتحديد العنصر المهيمن
        let mut element_counts = HashMap::new();

        for element in elements {
            *element_counts.entry(element.clone()).or_insert(0) += 1;
        }

        element_counts.into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(element, _)| element)
            .unwrap_or_else(|| "متنوع".to_string())
    }

    /// الحصول على معلومات حرف معين
    pub fn get_character_info(&self, ch: char, language: &str) -> Option<&CharacterSemantics> {
        match language {
            "ar" | "arabic" => self.arabic_chars.get(&ch),
            "en" | "english" => self.english_chars.get(&ch),
            _ => self.arabic_chars.get(&ch),
        }
    }

    /// الحصول على إحصائيات التحليل
    pub fn get_analysis_stats(&self) -> (usize, f64) {
        let total_analyses = self.analysis_history.len();
        let average_strength = if total_analyses > 0 {
            self.analysis_history.iter()
                .map(|sig| sig.signature_strength)
                .sum::<f64>() / total_analyses as f64
        } else {
            0.0
        };

        (total_analyses, average_strength)
    }
}

// متغير عام للمحلل اللغوي
use std::sync::{Mutex, OnceLock};

static GLOBAL_LINGUISTIC_ANALYZER: OnceLock<Mutex<LinguisticWordAnalyzer>> = OnceLock::new();

/// تهيئة المحلل اللغوي
pub fn initialize_linguistic_analyzer() {
    let analyzer = LinguisticWordAnalyzer::new();
    let _ = GLOBAL_LINGUISTIC_ANALYZER.set(Mutex::new(analyzer));
    println!("🔤 تم تهيئة محلل دلالات الحروف بنجاح!");
}

/// الحصول على مرجع للمحلل اللغوي
pub fn get_linguistic_analyzer() -> Option<std::sync::MutexGuard<'static, LinguisticWordAnalyzer>> {
    GLOBAL_LINGUISTIC_ANALYZER.get().map(|m| m.lock().unwrap())
}

// === دوال FFI للتكامل مع لغة البيان ===

/// تحليل كلمة وإرجاع نجاح العملية
#[no_mangle]
pub extern "C" fn albayan_rt_analyze_word_semantics(
    word: *const c_char,
    language: *const c_char,
) -> c_int {
    if word.is_null() || language.is_null() {
        return 0;
    }

    let word_str = match unsafe { CStr::from_ptr(word) }.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let lang_str = match unsafe { CStr::from_ptr(language) }.to_str() {
        Ok(s) => s,
        Err(_) => "ar",
    };

    if let Some(mut analyzer_guard) = get_linguistic_analyzer() {
        let _signature = analyzer_guard.analyze_word(word_str, lang_str);
        1 // نجح
    } else {
        0 // فشل
    }
}

/// الحصول على معلومات حرف معين
#[no_mangle]
pub extern "C" fn albayan_rt_get_character_semantics(
    character: c_char,
    language: *const c_char,
) -> *const c_char {
    let lang_str = if language.is_null() {
        "ar"
    } else {
        match unsafe { CStr::from_ptr(language) }.to_str() {
            Ok(s) => s,
            Err(_) => "ar",
        }
    };

    if let Some(analyzer_guard) = get_linguistic_analyzer() {
        let ch = character as u8 as char;
        if let Some(semantics) = analyzer_guard.get_character_info(ch, lang_str) {
            let info = format!("صوتي: {:?}, شكلي: {:?}",
                semantics.sound_meaning, semantics.shape_meaning);
            match CString::new(info) {
                Ok(c_string) => c_string.into_raw(),
                Err(_) => std::ptr::null(),
            }
        } else {
            std::ptr::null()
        }
    } else {
        std::ptr::null()
    }
}

/// الحصول على إحصائيات المحلل
#[no_mangle]
pub extern "C" fn albayan_rt_get_linguistic_stats() -> c_int {
    if let Some(analyzer_guard) = get_linguistic_analyzer() {
        let (total_analyses, _) = analyzer_guard.get_analysis_stats();
        total_analyses as c_int
    } else {
        0
    }
}

/// تحرير ذاكرة النص المُرجع
#[no_mangle]
pub extern "C" fn albayan_rt_free_linguistic_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
