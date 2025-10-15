//! Knowledge Engine - محرك المعرفة الدلالية
//!
//! هذا هو قلب النظام الدلالي للبيان - يمثل "المعنى" نفسه
//! ويحول الرؤية الفلسفية إلى تنفيذ هندسي ملموس

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

/// معرف فريد للكائن في النظام الدلالي
pub type ObjectID = u64;

/// مقبض للكائن يُستخدم في واجهة FFI
pub type ObjectHandle = u64;

/// قيمة ديناميكية يمكن أن تحمل أنواع مختلفة من البيانات
#[derive(Debug, Clone)]
pub enum DynamicValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Coordinates(f64, f64, f64), // x, y, z
}

/// تصنيف الكائن حسب طبيعته
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectCategory {
    Unknown,    // مجهول - للكائنات الجديدة
    Real,       // حقيقي - كائنات فيزيائية
    Abstract,   // مجرد - مفاهيم غير ملموسة
    Metaphor,   // مجازي - استعارات ومجازات
}

/// معادلة الشكل للكائن (تمثيل رياضي للشكل)
#[derive(Debug, Clone)]
pub struct ShapeEquation {
    /// معاملات المعادلة
    pub coefficients: Vec<f64>,
    /// نوع المعادلة (سيغمويد، خطية، إلخ)
    pub equation_type: String,
    /// معاملات مركبة (حقيقي + تخيلي)
    pub complex_coefficients: Vec<(f64, f64)>,
}

/// كائن في النظام الدلالي - التمثيل الأساسي لـ "الشيء"
#[derive(Debug, Clone)]
pub struct AlBayanObject {
    /// معرف فريد
    pub id: ObjectID,
    /// اسم الكائن
    pub name: String,
    /// خصائص الكائن (مرن، صلب، لون، إلخ)
    pub properties: HashMap<String, DynamicValue>,
    /// موقع الكائن في الفضاء ثلاثي الأبعاد
    pub location: (f64, f64, f64), // x, y, z
    /// معادلة الشكل الرياضية
    pub shape_equation: Option<ShapeEquation>,
    /// تصنيف الكائن
    pub category: ObjectCategory,
    /// طابع زمني للإنشاء
    pub created_at: u64,
    /// طابع زمني للتحديث الأخير
    pub updated_at: u64,
}

impl AlBayanObject {
    /// إنشاء كائن جديد
    pub fn new(id: ObjectID, name: String) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id,
            name,
            properties: HashMap::new(),
            location: (0.0, 0.0, 0.0),
            shape_equation: None,
            category: ObjectCategory::Unknown,
            created_at: now,
            updated_at: now,
        }
    }

    /// تحديث خاصية للكائن
    pub fn set_property(&mut self, key: String, value: DynamicValue) {
        self.properties.insert(key, value);
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// الحصول على خاصية
    pub fn get_property(&self, key: &str) -> Option<&DynamicValue> {
        self.properties.get(key)
    }

    /// تحديث موقع الكائن
    pub fn set_location(&mut self, x: f64, y: f64, z: f64) {
        self.location = (x, y, z);
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// تحديث تصنيف الكائن
    pub fn set_category(&mut self, category: ObjectCategory) {
        self.category = category;
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

/// أنواع العلاقات الدلالية الأساسية
#[derive(Debug, Clone, PartialEq)]
pub enum RelationType {
    // العلاقات المكانية
    Above,      // فوق
    Below,      // تحت
    LeftOf,     // يسار
    RightOf,    // يمين
    InFrontOf,  // أمام
    Behind,     // خلف
    Near,       // قريب
    Far,        // بعيد
    Between,    // بين

    // العلاقات التفاعلية
    Eats,       // يأكل
    Touches,    // يلمس
    Hits,       // يضرب
    Scratches,  // يخدش

    // العلاقات الحالية
    Contains,   // يحتوي
    PartOf,     // جزء من
    SimilarTo,  // مشابه لـ

    // علاقة مخصصة
    Custom(String),
}

/// علاقة دلالية بين كائنين أو أكثر
#[derive(Debug, Clone)]
pub struct SemanticRelation {
    /// نوع العلاقة
    pub relation_type: RelationType,
    /// الكائن الأول (الفاعل عادة)
    pub subject_id: ObjectID,
    /// الكائن الثاني (المفعول عادة)
    pub object_id: ObjectID,
    /// كائن ثالث اختياري (للعلاقات مثل "بين")
    pub third_object_id: Option<ObjectID>,
    /// قوة العلاقة (0.0 إلى 1.0)
    pub strength: f64,
    /// طابع زمني للإنشاء
    pub created_at: u64,
}

impl SemanticRelation {
    /// إنشاء علاقة جديدة
    pub fn new(
        relation_type: RelationType,
        subject_id: ObjectID,
        object_id: ObjectID,
        strength: f64,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            relation_type,
            subject_id,
            object_id,
            third_object_id: None,
            strength,
            created_at: now,
        }
    }

    /// إنشاء علاقة ثلاثية (مثل "بين")
    pub fn new_ternary(
        relation_type: RelationType,
        subject_id: ObjectID,
        object_id: ObjectID,
        third_object_id: ObjectID,
        strength: f64,
    ) -> Self {
        let mut relation = Self::new(relation_type, subject_id, object_id, strength);
        relation.third_object_id = Some(third_object_id);
        relation
    }
}

/// قاعدة المعرفة الدلالية - قلب النظام
#[derive(Debug)]
pub struct SemanticKB {
    /// الكائنات المخزنة
    objects: HashMap<ObjectID, AlBayanObject>,
    /// العلاقات المخزنة
    relations: Vec<SemanticRelation>,
    /// عداد لتوليد معرفات فريدة
    next_id: ObjectID,
    /// فهرس للبحث السريع بالاسم
    name_index: HashMap<String, ObjectID>,
}

impl SemanticKB {
    /// إنشاء قاعدة معرفة جديدة
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            relations: Vec::new(),
            next_id: 1,
            name_index: HashMap::new(),
        }
    }

    /// إنشاء كائن جديد
    pub fn create_object(&mut self, name: String) -> ObjectID {
        let id = self.next_id;
        self.next_id += 1;

        let object = AlBayanObject::new(id, name.clone());
        self.objects.insert(id, object);
        self.name_index.insert(name, id);

        id
    }

    /// الحصول على كائن بالمعرف
    pub fn get_object(&self, id: ObjectID) -> Option<&AlBayanObject> {
        self.objects.get(&id)
    }

    /// الحصول على كائن قابل للتعديل
    pub fn get_object_mut(&mut self, id: ObjectID) -> Option<&mut AlBayanObject> {
        self.objects.get_mut(&id)
    }

    /// البحث عن كائن بالاسم
    pub fn find_object_by_name(&self, name: &str) -> Option<ObjectID> {
        self.name_index.get(name).copied()
    }

    /// إضافة علاقة جديدة
    pub fn assert_relation(&mut self, relation: SemanticRelation) {
        self.relations.push(relation);
    }

    /// البحث عن علاقات تتضمن كائن معين
    pub fn find_relations_with_object(&self, object_id: ObjectID) -> Vec<&SemanticRelation> {
        self.relations.iter()
            .filter(|r| r.subject_id == object_id || r.object_id == object_id ||
                       r.third_object_id == Some(object_id))
            .collect()
    }

    /// البحث عن علاقة محددة بين كائنين
    pub fn find_relation(
        &self,
        relation_type: &RelationType,
        subject_id: ObjectID,
        object_id: ObjectID,
    ) -> Option<&SemanticRelation> {
        self.relations.iter()
            .find(|r| r.relation_type == *relation_type &&
                     r.subject_id == subject_id &&
                     r.object_id == object_id)
    }

    /// تطبيق قواعد الاستدلال التلقائي
    pub fn apply_inference_rules(&mut self) {
        // قاعدة: إذا كان أ فوق ب، فإن ب تحت أ
        let above_relations: Vec<_> = self.relations.iter()
            .filter(|r| r.relation_type == RelationType::Above)
            .cloned()
            .collect();

        for relation in above_relations {
            // تحقق من عدم وجود العلاقة العكسية
            if self.find_relation(&RelationType::Below, relation.object_id, relation.subject_id).is_none() {
                let inverse_relation = SemanticRelation::new(
                    RelationType::Below,
                    relation.object_id,
                    relation.subject_id,
                    relation.strength,
                );
                self.relations.push(inverse_relation);
            }
        }

        // يمكن إضافة المزيد من قواعد الاستدلال هنا
    }
}

/// مدير قاعدة المعرفة العامة
static mut GLOBAL_SEMANTIC_KB: Option<SemanticKB> = None;

/// تهيئة قاعدة المعرفة العامة
pub fn initialize_semantic_kb() {
    unsafe {
        GLOBAL_SEMANTIC_KB = Some(SemanticKB::new());
    }
}

/// الحصول على مرجع لقاعدة المعرفة العامة
pub fn get_semantic_kb() -> Option<&'static mut SemanticKB> {
    unsafe { GLOBAL_SEMANTIC_KB.as_mut() }
}

//
// واجهة FFI للتفاعل مع محرك المعرفة من لغة البيان
//

/// تهيئة محرك المعرفة
#[no_mangle]
pub extern "C" fn albayan_rt_kb_initialize() -> c_int {
    initialize_semantic_kb();
    1 // نجح
}

/// إنشاء كائن جديد
#[no_mangle]
pub extern "C" fn albayan_rt_kb_create_object(name: *const c_char) -> ObjectHandle {
    if name.is_null() {
        return 0; // فشل
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return 0,
        }
    };

    if let Some(kb) = get_semantic_kb() {
        kb.create_object(name_str)
    } else {
        0 // فشل
    }
}

/// تعيين خاصية نصية لكائن
#[no_mangle]
pub extern "C" fn albayan_rt_kb_set_string_property(
    handle: ObjectHandle,
    property: *const c_char,
    value: *const c_char,
) -> c_int {
    if property.is_null() || value.is_null() {
        return 0;
    }

    let property_str = unsafe {
        match CStr::from_ptr(property).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return 0,
        }
    };

    let value_str = unsafe {
        match CStr::from_ptr(value).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return 0,
        }
    };

    if let Some(kb) = get_semantic_kb() {
        if let Some(object) = kb.get_object_mut(handle) {
            object.set_property(property_str, DynamicValue::String(value_str));
            return 1;
        }
    }

    0 // فشل
}

/// تعيين خاصية رقمية لكائن
#[no_mangle]
pub extern "C" fn albayan_rt_kb_set_float_property(
    handle: ObjectHandle,
    property: *const c_char,
    value: f64,
) -> c_int {
    if property.is_null() {
        return 0;
    }

    let property_str = unsafe {
        match CStr::from_ptr(property).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return 0,
        }
    };

    if let Some(kb) = get_semantic_kb() {
        if let Some(object) = kb.get_object_mut(handle) {
            object.set_property(property_str, DynamicValue::Float(value));
            return 1;
        }
    }

    0 // فشل
}

/// تعيين موقع كائن
#[no_mangle]
pub extern "C" fn albayan_rt_kb_set_location(
    handle: ObjectHandle,
    x: f64,
    y: f64,
    z: f64,
) -> c_int {
    if let Some(kb) = get_semantic_kb() {
        if let Some(object) = kb.get_object_mut(handle) {
            object.set_location(x, y, z);
            return 1;
        }
    }

    0 // فشل
}

/// تأكيد علاقة "فوق" بين كائنين
#[no_mangle]
pub extern "C" fn albayan_rt_kb_assert_above(
    subject: ObjectHandle,
    object: ObjectHandle,
    strength: f64,
) -> c_int {
    if let Some(kb) = get_semantic_kb() {
        let relation = SemanticRelation::new(
            RelationType::Above,
            subject,
            object,
            strength,
        );
        kb.assert_relation(relation);
        kb.apply_inference_rules(); // تطبيق قواعد الاستدلال
        return 1;
    }

    0 // فشل
}

/// تأكيد علاقة "يأكل" بين كائنين
#[no_mangle]
pub extern "C" fn albayan_rt_kb_assert_eats(
    eater: ObjectHandle,
    eaten: ObjectHandle,
    strength: f64,
) -> c_int {
    if let Some(kb) = get_semantic_kb() {
        let relation = SemanticRelation::new(
            RelationType::Eats,
            eater,
            eaten,
            strength,
        );
        kb.assert_relation(relation);

        // تطبيق تأثيرات الأكل
        if let Some(eater_obj) = kb.get_object_mut(eater) {
            eater_obj.set_property("حالة".to_string(), DynamicValue::String("شبعان".to_string()));
        }

        return 1;
    }

    0 // فشل
}

/// البحث عن كائن بالاسم
#[no_mangle]
pub extern "C" fn albayan_rt_kb_find_object_by_name(name: *const c_char) -> ObjectHandle {
    if name.is_null() {
        return 0;
    }

    let name_str = unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return 0,
        }
    };

    if let Some(kb) = get_semantic_kb() {
        kb.find_object_by_name(name_str).unwrap_or(0)
    } else {
        0
    }
}

/// التحقق من وجود علاقة بين كائنين
#[no_mangle]
pub extern "C" fn albayan_rt_kb_check_relation(
    relation_type: c_int,
    subject: ObjectHandle,
    object: ObjectHandle,
) -> c_int {
    let rel_type = match relation_type {
        0 => RelationType::Above,
        1 => RelationType::Below,
        2 => RelationType::LeftOf,
        3 => RelationType::RightOf,
        4 => RelationType::Eats,
        _ => return 0,
    };

    if let Some(kb) = get_semantic_kb() {
        if kb.find_relation(&rel_type, subject, object).is_some() {
            1
        } else {
            0
        }
    } else {
        0
    }
}
