// المكتبات المدمجة في لغة البيان
// Built-in Libraries for AlBayan Language

pub mod thinking_core;
pub mod expert_explorer;
pub mod adaptive_equations;
pub mod artistic_renderer;
pub mod shape_inference;

// إعادة تصدير الهياكل الرئيسية للوصول السهل
pub use thinking_core::{
    ThinkingCore, ThinkingLayer, LayerType, AnalysisResult,
    AnalysisDomain, ProcessingMode, GlobalThinkingState
};

pub use expert_explorer::{
    ExpertExplorer, OperationMode, ExpertiseDomain, DecisionResult,
    DecisionRecord, ExplorationPath, RiskAssessment
};

pub use adaptive_equations::{
    GeneralShapeEquation, GeneralizedSigmoidComponent, LinearComponent,
    ComplexNumber, AdaptationType, OptimizationStrategy, EvaluationResult
};

pub use artistic_renderer::{
    ArtisticRenderer, MotherEquation, BasicShape, ShapeProperty,
    RenderedImage, RenderingEngine, PropertyTransform
};

pub use shape_inference::{
    ShapeInference, InferenceResult, ImageFeatures, FeatureExtractor,
    ConfidenceEvaluator, EquationGenerator
};

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// مدير المكتبات المدمجة
#[derive(Debug, Clone)]
pub struct BuiltinLibraryManager {
    pub thinking_cores: HashMap<String, ThinkingCore>,
    pub expert_explorers: HashMap<String, ExpertExplorer>,
    pub equations: HashMap<String, GeneralShapeEquation>,
    pub global_config: GlobalLibraryConfig,
}

/// إعدادات المكتبات العامة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalLibraryConfig {
    pub enable_thinking_core: bool,
    pub enable_expert_explorer: bool,
    pub enable_adaptive_equations: bool,
    pub default_precision: f64,
    pub max_memory_usage: usize,
    pub performance_monitoring: bool,
}

/// نتيجة عملية مدمجة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinOperationResult {
    pub operation_type: String,
    pub success: bool,
    pub result_data: serde_json::Value,
    pub execution_time: u64,
    pub memory_used: usize,
    pub error_message: Option<String>,
}

/// إحصائيات الأداء
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    pub total_operations: u64,
    pub successful_operations: u64,
    pub average_execution_time: f64,
    pub peak_memory_usage: usize,
    pub error_rate: f64,
}

impl BuiltinLibraryManager {
    /// إنشاء مدير مكتبات جديد
    pub fn new() -> Self {
        Self {
            thinking_cores: HashMap::new(),
            expert_explorers: HashMap::new(),
            equations: HashMap::new(),
            global_config: GlobalLibraryConfig::default(),
        }
    }

    /// إنشاء نواة تفكيرية جديدة
    pub fn create_thinking_core(&mut self, name: String) -> Result<(), String> {
        if self.thinking_cores.contains_key(&name) {
            return Err(format!("Thinking core '{}' already exists", name));
        }

        let thinking_core = ThinkingCore::new();
        self.thinking_cores.insert(name, thinking_core);
        Ok(())
    }

    /// إنشاء نظام خبير/مستكشف جديد
    pub fn create_expert_explorer(&mut self, name: String) -> Result<(), String> {
        if self.expert_explorers.contains_key(&name) {
            return Err(format!("Expert explorer '{}' already exists", name));
        }

        let expert_explorer = ExpertExplorer::new();
        self.expert_explorers.insert(name, expert_explorer);
        Ok(())
    }

    /// إنشاء معادلة تكيفية جديدة
    pub fn create_adaptive_equation(&mut self, name: String) -> Result<(), String> {
        if self.equations.contains_key(&name) {
            return Err(format!("Equation '{}' already exists", name));
        }

        let equation = GeneralShapeEquation::new();
        self.equations.insert(name, equation);
        Ok(())
    }

    /// ربط نواة تفكيرية مع نظام خبير/مستكشف
    pub fn connect_thinking_to_explorer(&mut self, thinking_name: &str, explorer_name: &str) -> Result<(), String> {
        let thinking_core = self.thinking_cores.get(thinking_name)
            .ok_or_else(|| format!("Thinking core '{}' not found", thinking_name))?
            .clone();

        let explorer = self.expert_explorers.get_mut(explorer_name)
            .ok_or_else(|| format!("Expert explorer '{}' not found", explorer_name))?;

        explorer.thinking_core = Some(thinking_core);
        Ok(())
    }

    /// تحليل ذكي شامل
    pub fn comprehensive_analysis(&mut self,
                                thinking_name: &str,
                                explorer_name: &str,
                                input_data: Vec<f64>) -> Result<BuiltinOperationResult, String> {
        let start_time = std::time::Instant::now();

        // تحليل بالنواة التفكيرية
        let thinking_result = {
            let thinking_core = self.thinking_cores.get_mut(thinking_name)
                .ok_or_else(|| format!("Thinking core '{}' not found", thinking_name))?;
            thinking_core.analyze(input_data.clone())
        };

        // قرار بنظام الخبير/المستكشف
        let decision_result = {
            let explorer = self.expert_explorers.get_mut(explorer_name)
                .ok_or_else(|| format!("Expert explorer '{}' not found", explorer_name))?;
            explorer.make_decision(input_data)
        };

        let execution_time = start_time.elapsed().as_millis() as u64;

        // دمج النتائج
        let combined_result = serde_json::json!({
            "thinking_analysis": {
                "final_output": thinking_result.final_output,
                "confidence_score": thinking_result.confidence_score,
                "processing_time": thinking_result.processing_time,
                "dominant_patterns": thinking_result.dominant_patterns
            },
            "decision_result": {
                "final_decision": decision_result.final_decision,
                "chosen_mode": decision_result.chosen_mode,
                "confidence_level": decision_result.confidence_level,
                "recommendation": decision_result.recommendation
            }
        });

        Ok(BuiltinOperationResult {
            operation_type: "comprehensive_analysis".to_string(),
            success: true,
            result_data: combined_result,
            execution_time,
            memory_used: 0, // سيتم حسابها في التطبيق الفعلي
            error_message: None,
        })
    }

    /// تدريب معادلة تكيفية
    pub fn train_equation(&mut self,
                         equation_name: &str,
                         training_data: Vec<(f64, f64)>,
                         adaptation_type: AdaptationType) -> Result<BuiltinOperationResult, String> {
        let start_time = std::time::Instant::now();

        let equation = self.equations.get_mut(equation_name)
            .ok_or_else(|| format!("Equation '{}' not found", equation_name))?;

        equation.train(training_data, adaptation_type);

        let execution_time = start_time.elapsed().as_millis() as u64;

        let result_data = serde_json::json!({
            "equation_name": equation_name,
            "training_completed": true,
            "performance_metrics": equation.performance_metrics
        });

        Ok(BuiltinOperationResult {
            operation_type: "equation_training".to_string(),
            success: true,
            result_data,
            execution_time,
            memory_used: 0,
            error_message: None,
        })
    }

    /// تقييم معادلة
    pub fn evaluate_equation(&mut self,
                            equation_name: &str,
                            input_value: f64) -> Result<BuiltinOperationResult, String> {
        let start_time = std::time::Instant::now();

        let equation = self.equations.get(equation_name)
            .ok_or_else(|| format!("Equation '{}' not found", equation_name))?;

        let evaluation_result = equation.evaluate(input_value);

        let execution_time = start_time.elapsed().as_millis() as u64;

        let result_data = serde_json::json!({
            "input_value": input_value,
            "output_value": evaluation_result.output_value,
            "confidence_score": evaluation_result.confidence_score,
            "component_contributions": evaluation_result.component_contributions,
            "computational_cost": evaluation_result.computational_cost
        });

        Ok(BuiltinOperationResult {
            operation_type: "equation_evaluation".to_string(),
            success: true,
            result_data,
            execution_time,
            memory_used: 0,
            error_message: None,
        })
    }

    /// إحصائيات الأداء العامة
    pub fn get_performance_stats(&self) -> PerformanceStats {
        PerformanceStats {
            total_operations: 0, // سيتم حسابها من السجلات
            successful_operations: 0,
            average_execution_time: 0.0,
            peak_memory_usage: 0,
            error_rate: 0.0,
        }
    }

    /// تنظيف الذاكرة
    pub fn cleanup_memory(&mut self) {
        // تنظيف سجلات التاريخ القديمة
        for thinking_core in self.thinking_cores.values_mut() {
            for database in thinking_core.databases.values_mut() {
                if database.learning_history.len() > 1000 {
                    database.learning_history.drain(0..500); // الاحتفاظ بآخر 500 سجل
                }
            }
        }

        for explorer in self.expert_explorers.values_mut() {
            if explorer.decision_history.len() > 1000 {
                explorer.decision_history.drain(0..500);
            }
        }

        for equation in self.equations.values_mut() {
            if equation.adaptation_history.len() > 1000 {
                equation.adaptation_history.drain(0..500);
            }
        }
    }

    /// حفظ حالة المكتبات
    pub fn save_state(&self, file_path: &str) -> Result<(), String> {
        // سيتم تطبيقها لاحقاً - حفظ الحالة في ملف
        Ok(())
    }

    /// تحميل حالة المكتبات
    pub fn load_state(&mut self, file_path: &str) -> Result<(), String> {
        // سيتم تطبيقها لاحقاً - تحميل الحالة من ملف
        Ok(())
    }
}

impl GlobalLibraryConfig {
    pub fn default() -> Self {
        Self {
            enable_thinking_core: true,
            enable_expert_explorer: true,
            enable_adaptive_equations: true,
            default_precision: 1e-6,
            max_memory_usage: 1024 * 1024 * 100, // 100 MB
            performance_monitoring: true,
        }
    }
}

/// دوال مساعدة للوصول السريع
pub mod quick_access {
    use super::*;

    /// إنشاء نواة تفكيرية سريعة
    pub fn create_quick_thinking_core() -> ThinkingCore {
        ThinkingCore::new()
    }

    /// إنشاء خبير/مستكشف سريع
    pub fn create_quick_expert_explorer() -> ExpertExplorer {
        ExpertExplorer::new()
    }

    /// إنشاء معادلة تكيفية سريعة
    pub fn create_quick_adaptive_equation() -> GeneralShapeEquation {
        let mut equation = GeneralShapeEquation::new();

        // إضافة مكونات افتراضية
        equation.add_sigmoid_component(
            1.0,
            ComplexNumber::new(1.0, 0.0),
            0.0
        );
        equation.add_linear_component(1.0, 0.0, 0.5);

        equation
    }

    /// تحليل سريع
    pub fn quick_analysis(input_data: Vec<f64>) -> (AnalysisResult, DecisionResult) {
        let mut thinking_core = create_quick_thinking_core();
        let mut expert_explorer = create_quick_expert_explorer();

        let analysis = thinking_core.analyze(input_data.clone());
        let decision = expert_explorer.make_decision(input_data);

        (analysis, decision)
    }
}

/// ماكرو لإنشاء مكتبات مدمجة بسهولة
#[macro_export]
macro_rules! create_builtin_system {
    ($name:expr) => {
        {
            let mut manager = BuiltinLibraryManager::new();
            manager.create_thinking_core(format!("{}_thinking", $name)).unwrap();
            manager.create_expert_explorer(format!("{}_explorer", $name)).unwrap();
            manager.create_adaptive_equation(format!("{}_equation", $name)).unwrap();
            manager.connect_thinking_to_explorer(
                &format!("{}_thinking", $name),
                &format!("{}_explorer", $name)
            ).unwrap();
            manager
        }
    };
}
