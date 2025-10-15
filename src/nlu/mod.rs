//! Natural Language Understanding Module - وحدة فهم اللغة الطبيعية
//!
//! محلل بسيط لفهم جمل مثل "أ فوق ب" وترجمتها إلى استدعاءات API
//! تنفيذ الأولوية الرابعة من توجيهات الخبير

use std::collections::HashMap;

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

    /// علاقة غير معروفة
    Unknown(String),
}

/// عنصر مستخرج من الجملة
#[derive(Debug, Clone)]
pub struct ParsedElement {
    /// النص الأصلي
    pub text: String,
    /// النوع (فاعل، مفعول، فعل)
    pub element_type: ElementType,
    /// الموقع في الجملة
    pub position: usize,
}

/// نوع العنصر في الجملة
#[derive(Debug, Clone, PartialEq)]
pub enum ElementType {
    Subject,    // فاعل
    Predicate,  // فعل/علاقة
    Object,     // مفعول
    Unknown,    // غير معروف
}

/// نتيجة تحليل الجملة
#[derive(Debug, Clone)]
pub struct ParseResult {
    /// الجملة الأصلية
    pub original_sentence: String,
    /// العناصر المستخرجة
    pub elements: Vec<ParsedElement>,
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
}

/// محلل اللغة الطبيعية الأولي
pub struct SimpleNLUParser {
    /// قاموس الكلمات المفتاحية للعلاقات
    relation_keywords: HashMap<String, NLURelationType>,
    /// أنماط الجمل المعروفة
    sentence_patterns: Vec<SentencePattern>,
}

/// نمط جملة
#[derive(Debug, Clone)]
pub struct SentencePattern {
    /// النمط (مثل: "X فوق Y")
    pub pattern: String,
    /// نوع العلاقة
    pub relation: NLURelationType,
    /// استدعاء API المقابل
    pub api_call_template: String,
    /// مستوى الثقة
    pub confidence: f64,
}

impl SimpleNLUParser {
    /// إنشاء محلل جديد
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

        // إنشاء أنماط الجمل الأساسية
        let sentence_patterns = vec![
            SentencePattern {
                pattern: "X فوق Y".to_string(),
                relation: NLURelationType::Above,
                api_call_template: "std::knowledge::assert_above(&{subject}, &{object})".to_string(),
                confidence: 0.9,
            },
            SentencePattern {
                pattern: "X تحت Y".to_string(),
                relation: NLURelationType::Below,
                api_call_template: "std::knowledge::assert_below(&{subject}, &{object})".to_string(),
                confidence: 0.9,
            },
            SentencePattern {
                pattern: "X يأكل Y".to_string(),
                relation: NLURelationType::Eats,
                api_call_template: "std::knowledge::assert_eats(&{subject}, &{object})".to_string(),
                confidence: 0.8,
            },
        ];

        Self {
            relation_keywords,
            sentence_patterns,
        }
    }

    /// تحليل جملة بسيطة
    pub fn parse_sentence(&self, sentence: &str) -> ParseResult {
        let words: Vec<&str> = sentence.split_whitespace().collect();
        let mut elements = Vec::new();
        let mut relation_type = NLURelationType::Unknown("".to_string());
        let mut subject = None;
        let mut object = None;
        let mut confidence = 0.0;

        // تحليل الكلمات وتحديد العناصر
        for (i, word) in words.iter().enumerate() {
            let element = ParsedElement {
                text: word.to_string(),
                element_type: ElementType::Unknown,
                position: i,
            };
            elements.push(element);
        }

        // البحث عن العلاقات المعروفة
        for word in &words {
            if let Some(rel) = self.relation_keywords.get(*word) {
                relation_type = rel.clone();
                confidence = 0.8;
                break;
            }
        }

        // تحديد الفاعل والمفعول للأنماط البسيطة
        if words.len() == 3 {
            // نمط: "أ فوق ب"
            subject = Some(words[0].to_string());
            object = Some(words[2].to_string());

            // تحديث أنواع العناصر
            if elements.len() >= 3 {
                elements[0].element_type = ElementType::Subject;
                elements[1].element_type = ElementType::Predicate;
                elements[2].element_type = ElementType::Object;
            }

            confidence = 0.9;
        }

        // توليد استدعاء API
        let suggested_api_call = self.generate_api_call(&relation_type, &subject, &object);

        ParseResult {
            original_sentence: sentence.to_string(),
            elements,
            relation_type,
            subject,
            object,
            confidence,
            suggested_api_call,
        }
    }

    /// توليد استدعاء API مناسب
    fn generate_api_call(&self, relation: &NLURelationType, subject: &Option<String>, object: &Option<String>) -> String {
        match (relation, subject, object) {
            (NLURelationType::Above, Some(subj), Some(obj)) => {
                format!("std::knowledge::assert_above(&Object::new(\"{}\"), &Object::new(\"{}\"))", subj, obj)
            },
            (NLURelationType::Below, Some(subj), Some(obj)) => {
                format!("std::knowledge::assert_below(&Object::new(\"{}\"), &Object::new(\"{}\"))", subj, obj)
            },
            (NLURelationType::Eats, Some(subj), Some(obj)) => {
                format!("std::knowledge::assert_eats(&Object::new(\"{}\"), &Object::new(\"{}\"))", subj, obj)
            },
            _ => "// لا يمكن توليد استدعاء API لهذه الجملة".to_string(),
        }
    }

    /// إضافة نمط جملة جديد
    pub fn add_sentence_pattern(&mut self, pattern: SentencePattern) {
        self.sentence_patterns.push(pattern);
    }

    /// إضافة كلمة مفتاحية جديدة
    pub fn add_relation_keyword(&mut self, keyword: String, relation: NLURelationType) {
        self.relation_keywords.insert(keyword, relation);
    }

    /// تحليل متقدم باستخدام الأنماط
    pub fn parse_with_patterns(&self, sentence: &str) -> ParseResult {
        let basic_result = self.parse_sentence(sentence);

        // محاولة مطابقة الأنماط المعروفة
        for pattern in &self.sentence_patterns {
            if self.matches_pattern(sentence, &pattern.pattern) {
                let mut enhanced_result = basic_result.clone();
                enhanced_result.relation_type = pattern.relation.clone();
                enhanced_result.confidence = pattern.confidence;
                enhanced_result.suggested_api_call = self.apply_pattern_template(
                    &pattern.api_call_template,
                    &enhanced_result.subject,
                    &enhanced_result.object,
                );
                return enhanced_result;
            }
        }

        basic_result
    }

    /// فحص مطابقة النمط
    fn matches_pattern(&self, sentence: &str, pattern: &str) -> bool {
        // تنفيذ بسيط لمطابقة الأنماط
        let sentence_words: Vec<&str> = sentence.split_whitespace().collect();
        let pattern_words: Vec<&str> = pattern.split_whitespace().collect();

        if sentence_words.len() != pattern_words.len() {
            return false;
        }

        for (i, pattern_word) in pattern_words.iter().enumerate() {
            if *pattern_word != "X" && *pattern_word != "Y" {
                if sentence_words.get(i) != Some(pattern_word) {
                    return false;
                }
            }
        }

        true
    }

    /// تطبيق قالب النمط
    fn apply_pattern_template(&self, template: &str, subject: &Option<String>, object: &Option<String>) -> String {
        let mut result = template.to_string();

        if let Some(subj) = subject {
            result = result.replace("{subject}", &format!("Object::new(\"{}\")", subj));
        }

        if let Some(obj) = object {
            result = result.replace("{object}", &format!("Object::new(\"{}\")", obj));
        }

        result
    }
}

/// دالة مساعدة لتحليل جملة بسيطة
pub fn parse_simple_natural_language(sentence: &str) -> ParseResult {
    let parser = SimpleNLUParser::new();
    parser.parse_with_patterns(sentence)
}

/// دالة لتحويل نتيجة التحليل إلى كود البيان
pub fn generate_albayan_code(parse_result: &ParseResult) -> String {
    format!(
        "// تحليل الجملة: {}\n// مستوى الثقة: {:.2}\n{}\n",
        parse_result.original_sentence,
        parse_result.confidence,
        parse_result.suggested_api_call
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_parsing() {
        let parser = SimpleNLUParser::new();
        let result = parser.parse_sentence("أ فوق ب");

        assert_eq!(result.subject, Some("أ".to_string()));
        assert_eq!(result.object, Some("ب".to_string()));
        assert_eq!(result.relation_type, NLURelationType::Above);
        assert!(result.confidence > 0.5);
    }

    #[test]
    fn test_eating_relation() {
        let parser = SimpleNLUParser::new();
        let result = parser.parse_sentence("قط يأكل سمك");

        assert_eq!(result.subject, Some("قط".to_string()));
        assert_eq!(result.object, Some("سمك".to_string()));
        assert_eq!(result.relation_type, NLURelationType::Eats);
    }

    #[test]
    fn test_api_generation() {
        let result = parse_simple_natural_language("كتاب فوق طاولة");
        let code = generate_albayan_code(&result);

        assert!(code.contains("assert_above"));
        assert!(code.contains("كتاب"));
        assert!(code.contains("طاولة"));
    }
}
