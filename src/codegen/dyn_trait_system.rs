// src/codegen/dyn_trait_system.rs
// نظام تعدد الأشكال الديناميكي - الأولوية القصوى للخبير
// Expert recommendation: "تعدد الأشكال الديناميكي (dyn Trait و V-Tables)"
// "هذا سيكمل قصة Traits ويجعل اللغة قوية ومرنة بشكل هائل"

use crate::semantic::{ResolvedType, AnnotatedTrait, AnnotatedImpl, AnnotatedFunction, SemanticError};
use std::collections::HashMap;
use anyhow::{Result, anyhow};

/// إدخال في V-Table يمثل طريقة في السمة
#[derive(Debug, Clone)]
pub struct VTableEntry {
    pub method_name: String,
    pub function_signature: String,
    pub implementation_type: String,
}

/// V-Table لتنفيذ سمة معينة
#[derive(Debug, Clone)]
pub struct VTable {
    pub trait_name: String,
    pub implementing_type: ResolvedType,
    pub entries: Vec<VTableEntry>,
    pub vtable_id: String,
}

/// مؤشر سمين لكائنات السمة
#[derive(Debug, Clone)]
pub struct FatPointer {
    pub data_ptr: String,      // مؤشر للبيانات الفعلية
    pub vtable_ptr: String,    // مؤشر لـ V-Table
    pub trait_name: String,    // اسم السمة
}

/// مدير V-Tables لوحدة الترجمة الكاملة
#[derive(Debug)]
pub struct VTableManager {
    /// V-Tables مخزنة بمفتاح (trait_name, implementing_type)
    pub vtables: HashMap<(String, String), VTable>,
    /// ترتيب طرق السمة للحصول على تخطيط V-Table متسق
    pub trait_method_order: HashMap<String, Vec<String>>,
    /// عداد لإنشاء معرفات V-Table فريدة
    vtable_counter: usize,
}

impl VTableManager {
    /// إنشاء مدير V-Table جديد
    pub fn new() -> Self {
        Self {
            vtables: HashMap::new(),
            trait_method_order: HashMap::new(),
            vtable_counter: 0,
        }
    }

    /// تسجيل طرق السمة للحصول على ترتيب متسق
    pub fn register_trait_methods(&mut self, trait_name: &str, method_names: Vec<String>) {
        self.trait_method_order.insert(trait_name.to_string(), method_names);
    }

    /// إنشاء V-Table لتنفيذ سمة
    pub fn create_vtable(
        &mut self,
        trait_name: &str,
        implementing_type: &ResolvedType,
        method_implementations: HashMap<String, String>,
    ) -> Result<&VTable, SemanticError> {
        let type_key = self.type_to_string(implementing_type);
        let vtable_key = (trait_name.to_string(), type_key.clone());

        // التحقق من وجود V-Table مسبقاً
        if self.vtables.contains_key(&vtable_key) {
            return Ok(self.vtables.get(&vtable_key).unwrap());
        }

        // الحصول على ترتيب طرق السمة
        let method_order = self.trait_method_order
            .get(trait_name)
            .ok_or_else(|| SemanticError::UndefinedType(trait_name.to_string()))?;

        // إنشاء إدخالات V-Table
        let mut entries = Vec::new();
        for method_name in method_order {
            let implementation = method_implementations
                .get(method_name as &str)
                .ok_or_else(|| SemanticError::UndefinedType(format!(
                    "Missing trait method {} for {} implementing {}",
                    method_name, type_key, trait_name
                )))?;

            entries.push(VTableEntry {
                method_name: method_name.clone(),
                function_signature: implementation.clone(),
                implementation_type: type_key.clone(),
            });
        }

        // إنشاء V-Table
        self.vtable_counter += 1;
        let vtable = VTable {
            trait_name: trait_name.to_string(),
            implementing_type: implementing_type.clone(),
            entries,
            vtable_id: format!("vtable_{}_{}", trait_name, self.vtable_counter),
        };

        // تخزين V-Table
        self.vtables.insert(vtable_key.clone(), vtable);
        Ok(self.vtables.get(&vtable_key).unwrap())
    }

    /// الحصول على V-Table لتنفيذ سمة معين
    pub fn get_vtable(&self, trait_name: &str, implementing_type: &ResolvedType) -> Option<&VTable> {
        let type_key = self.type_to_string(implementing_type);
        let vtable_key = (trait_name.to_string(), type_key);
        self.vtables.get(&vtable_key)
    }

    /// الحصول على فهرس طريقة في V-Table
    pub fn get_method_index(&self, trait_name: &str, method_name: &str) -> Option<usize> {
        self.trait_method_order
            .get(trait_name)?
            .iter()
            .position(|name| name == method_name)
    }

    /// تحويل ResolvedType إلى string للاستخدام كمفتاح
    pub fn type_to_string(&self, resolved_type: &ResolvedType) -> String {
        match resolved_type {
            ResolvedType::Int => "int".to_string(),
            ResolvedType::Float => "float".to_string(),
            ResolvedType::String => "string".to_string(),
            ResolvedType::Bool => "bool".to_string(),
            ResolvedType::Struct(name) => format!("struct_{}", name),
            ResolvedType::Enum(name) => format!("enum_{}", name),
            ResolvedType::Array(inner) => {
                format!("array_{}", self.type_to_string(inner))
            }
            ResolvedType::List(inner) => {
                format!("list_{}", self.type_to_string(inner))
            }
            ResolvedType::Reference(inner, is_mutable) => {
                let mutability = if *is_mutable { "mut" } else { "ref" };
                format!("{}_{}", mutability, self.type_to_string(inner))
            }
            ResolvedType::TraitObject(traits) => {
                format!("dyn_{}", traits.join("_"))
            }
            ResolvedType::Generic(name, _types) => format!("generic_{}", name),
            ResolvedType::Function(params, return_type) => {
                let param_types: Vec<String> = params.iter()
                    .map(|p| self.type_to_string(p))
                    .collect();
                format!("fn_{}_{}", param_types.join("_"), self.type_to_string(return_type))
            }
            // إضافة الحالات المفقودة
            ResolvedType::Char => "char".to_string(),
            ResolvedType::Null => "null".to_string(),
            ResolvedType::Unit => "unit".to_string(),
            ResolvedType::Tuple(types) => {
                let type_strings: Vec<String> = types.iter()
                    .map(|t| self.type_to_string(t))
                    .collect();
                format!("tuple_{}", type_strings.join("_"))
            }
            ResolvedType::GenericParam(name) => format!("generic_param_{}", name),
            // حالات أخرى
            _ => "unknown".to_string(),
        }
    }

    /// إنشاء كائن سمة من قيمة ملموسة
    pub fn create_trait_object(
        &self,
        concrete_value: &str,
        concrete_type: &ResolvedType,
        trait_name: &str,
    ) -> Result<FatPointer, SemanticError> {
        // الحصول على V-Table لهذا التنفيذ
        let vtable = self.get_vtable(trait_name, concrete_type)
            .ok_or_else(|| SemanticError::UndefinedType(format!(
                "V-Table not found for {} implementing {}",
                self.type_to_string(concrete_type),
                trait_name
            )))?;

        // إنشاء مؤشر سمين
        Ok(FatPointer {
            data_ptr: concrete_value.to_string(),
            vtable_ptr: vtable.vtable_id.clone(),
            trait_name: trait_name.to_string(),
        })
    }

    /// استدعاء طريقة على كائن سمة
    pub fn call_trait_method(
        &self,
        trait_object: &FatPointer,
        method_name: &str,
        args: &[String],
    ) -> Result<String, SemanticError> {
        // العثور على V-Table
        let vtable = self.vtables.values()
            .find(|v| v.vtable_id == trait_object.vtable_ptr)
            .ok_or_else(|| SemanticError::UndefinedType(format!(
                "V-Table {} not found", trait_object.vtable_ptr
            )))?;

        // العثور على الطريقة في V-Table
        let entry = vtable.entries.iter()
            .find(|e| e.method_name == method_name)
            .ok_or_else(|| SemanticError::UndefinedType(format!(
                "Missing trait method {} for {} implementing {}",
                method_name,
                self.type_to_string(&vtable.implementing_type),
                trait_object.trait_name
            )))?;

        // إنشاء استدعاء الطريقة
        let mut call_args = vec![trait_object.data_ptr.clone()];
        call_args.extend_from_slice(args);

        Ok(format!(
            "{}({})",
            entry.function_signature,
            call_args.join(", ")
        ))
    }

    /// الحصول على جميع V-Tables المسجلة
    pub fn get_all_vtables(&self) -> &HashMap<(String, String), VTable> {
        &self.vtables
    }

    /// طباعة معلومات V-Table للتشخيص
    pub fn print_vtable_info(&self, trait_name: &str, implementing_type: &ResolvedType) {
        if let Some(vtable) = self.get_vtable(trait_name, implementing_type) {
            println!("V-Table: {}", vtable.vtable_id);
            println!("Trait: {}", vtable.trait_name);
            println!("Type: {}", self.type_to_string(&vtable.implementing_type));
            println!("Methods:");
            for (i, entry) in vtable.entries.iter().enumerate() {
                println!("  [{}] {} -> {}", i, entry.method_name, entry.function_signature);
            }
        } else {
            println!("V-Table not found for {} implementing {}",
                self.type_to_string(implementing_type), trait_name);
        }
    }
}

/// أدوات مساعدة لكائنات السمة
pub struct TraitObjectUtils;

impl TraitObjectUtils {
    /// استخراج مؤشر البيانات من كائن سمة
    pub fn extract_data_ptr(trait_object: &FatPointer) -> &str {
        &trait_object.data_ptr
    }

    /// استخراج مؤشر V-Table من كائن سمة
    pub fn extract_vtable_ptr(trait_object: &FatPointer) -> &str {
        &trait_object.vtable_ptr
    }

    /// التحقق من أن كائن السمة يدعم سمة معينة
    pub fn supports_trait(trait_object: &FatPointer, trait_name: &str) -> bool {
        trait_object.trait_name == trait_name
    }

    /// إنشاء تمثيل نصي لكائن السمة
    pub fn to_string(trait_object: &FatPointer) -> String {
        format!(
            "TraitObject {{ data: {}, vtable: {}, trait: {} }}",
            trait_object.data_ptr,
            trait_object.vtable_ptr,
            trait_object.trait_name
        )
    }
}

/// مولد كود لنظام dyn Trait
pub struct DynTraitCodeGenerator {
    pub vtable_manager: VTableManager,
}

impl DynTraitCodeGenerator {
    /// إنشاء مولد كود جديد
    pub fn new() -> Self {
        Self {
            vtable_manager: VTableManager::new(),
        }
    }

    /// معالجة إعلان سمة
    pub fn process_trait_declaration(&mut self, trait_decl: &AnnotatedTrait) -> Result<()> {
        let method_names: Vec<String> = trait_decl.methods.iter()
            .map(|method| method.name.clone())
            .collect();

        self.vtable_manager.register_trait_methods(&trait_decl.name, method_names);
        Ok(())
    }

    /// معالجة تنفيذ سمة
    pub fn process_impl_declaration(
        &mut self,
        impl_decl: &AnnotatedImpl,
        trait_name: &str,
    ) -> Result<()> {
        // تحديد النوع المنفذ
        let implementing_type = ResolvedType::Struct(impl_decl.type_name.clone());

        // جمع تنفيذات الطرق
        let mut method_implementations = HashMap::new();
        for method in &impl_decl.methods {
            let function_name = format!("{}_{}_{}",
                impl_decl.type_name,
                trait_name,
                method.name
            );
            method_implementations.insert(method.name.clone(), function_name);
        }

        // إنشاء V-Table
        self.vtable_manager.create_vtable(
            trait_name,
            &implementing_type,
            method_implementations,
        )?;

        Ok(())
    }

    /// إنشاء كود لاستدعاء طريقة على كائن سمة
    pub fn generate_trait_method_call(
        &self,
        trait_object_expr: &str,
        method_name: &str,
        args: &[String],
    ) -> Result<String> {
        // هذا مبسط - في التنفيذ الحقيقي سنحتاج لتحليل trait_object_expr
        // للحصول على FatPointer الفعلي
        Ok(format!(
            "// Dynamic dispatch: {}.{}({})",
            trait_object_expr,
            method_name,
            args.join(", ")
        ))
    }
}
