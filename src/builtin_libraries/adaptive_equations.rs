// المعادلات المتكيفة - مكتبة مدمجة في لغة البيان
// Adaptive Equations - Built-in Library for AlBayan Language

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// عدد مركب للمعادلات المتقدمة
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ComplexNumber {
    pub real: f64,      // الجزء الحقيقي
    pub imaginary: f64, // الجزء التخيلي
}

/// مكون سيغمويد معمم
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralizedSigmoidComponent {
    pub alpha: f64,                    // معامل التضخيم
    pub k_complex: ComplexNumber,      // معامل مركب للأس
    pub x0: f64,                       // نقطة الإزاحة
    pub cutting_factor: f64,           // عامل القطع
    pub adaptation_rate: f64,          // معدل التكيف
}

/// مكون خط مستقيم
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearComponent {
    pub slope: f64,           // الميل
    pub intercept: f64,       // نقطة التقاطع
    pub weight: f64,          // الوزن في المعادلة
    pub adaptation_rate: f64, // معدل التكيف
}

/// معادلة الشكل العام
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralShapeEquation {
    pub sigmoid_components: Vec<GeneralizedSigmoidComponent>,
    pub linear_components: Vec<LinearComponent>,
    pub global_parameters: GlobalParameters,
    pub adaptation_history: Vec<AdaptationEvent>,
    pub performance_metrics: PerformanceMetrics,
}

/// المعاملات العامة
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalParameters {
    pub overall_scaling: f64,
    pub noise_tolerance: f64,
    pub convergence_threshold: f64,
    pub max_iterations: u32,
    pub learning_rate: f64,
}

/// حدث تكيف
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationEvent {
    pub timestamp: u64,
    pub input_data: Vec<f64>,
    pub target_output: f64,
    pub predicted_output: f64,
    pub error: f64,
    pub parameter_changes: HashMap<String, f64>,
}

/// مقاييس الأداء
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub mean_squared_error: f64,
    pub correlation_coefficient: f64,
    pub adaptation_speed: f64,
    pub stability_index: f64,
    pub complexity_score: f64,
}

/// نتيجة التقييم
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub output_value: f64,
    pub component_contributions: HashMap<String, f64>,
    pub confidence_score: f64,
    pub computational_cost: u64,
    pub gradient_info: Vec<f64>,
}

/// نوع التكيف
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdaptationType {
    Gradient,      // تكيف بالتدرج
    Evolutionary,  // تكيف تطوري
    Bayesian,      // تكيف بايزي
    Hybrid,        // تكيف مختلط
}

/// استراتيجية التحسين
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OptimizationStrategy {
    MinimizeError,     // تقليل الخطأ
    MaximizeAccuracy,  // تعظيم الدقة
    BalanceComplexity, // توازن التعقيد
    FastConvergence,   // تقارب سريع
}

impl ComplexNumber {
    pub fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }

    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imaginary * self.imaginary).sqrt()
    }

    pub fn phase(&self) -> f64 {
        self.imaginary.atan2(self.real)
    }

    pub fn multiply(&self, other: &ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }

    pub fn exp(&self) -> ComplexNumber {
        let exp_real = self.real.exp();
        ComplexNumber {
            real: exp_real * self.imaginary.cos(),
            imaginary: exp_real * self.imaginary.sin(),
        }
    }
}

impl GeneralizedSigmoidComponent {
    pub fn new(alpha: f64, k_complex: ComplexNumber, x0: f64) -> Self {
        Self {
            alpha,
            k_complex,
            x0,
            cutting_factor: 1.0,
            adaptation_rate: 0.01,
        }
    }

    /// تقييم مكون السيغمويد
    pub fn evaluate(&self, x: f64) -> f64 {
        let shifted_x = x - self.x0;

        // حساب الأس المركب
        let exponent = ComplexNumber::new(-self.k_complex.real * shifted_x, -self.k_complex.imaginary * shifted_x);
        let exp_result = exponent.exp();

        // السيغمويد المعمم
        let denominator = 1.0 + exp_result.magnitude();
        let sigmoid_value = self.alpha / denominator;

        // تطبيق عامل القطع
        sigmoid_value * self.cutting_factor
    }

    /// حساب التدرج للتكيف
    pub fn gradient(&self, x: f64, target: f64) -> HashMap<String, f64> {
        let current_output = self.evaluate(x);
        let error = target - current_output;

        let mut gradients = HashMap::new();

        // تدرج alpha
        let alpha_grad = error * (current_output / self.alpha);
        gradients.insert("alpha".to_string(), alpha_grad);

        // تدرج k_real
        let shifted_x = x - self.x0;
        let k_real_grad = error * current_output * (1.0 - current_output / self.alpha) * shifted_x;
        gradients.insert("k_real".to_string(), k_real_grad);

        // تدرج x0
        let x0_grad = -error * current_output * (1.0 - current_output / self.alpha) * self.k_complex.real;
        gradients.insert("x0".to_string(), x0_grad);

        gradients
    }

    /// تكيف المعاملات
    pub fn adapt(&mut self, gradients: &HashMap<String, f64>) {
        if let Some(alpha_grad) = gradients.get("alpha") {
            self.alpha += self.adaptation_rate * alpha_grad;
            self.alpha = self.alpha.max(0.1); // منع القيم السالبة
        }

        if let Some(k_real_grad) = gradients.get("k_real") {
            self.k_complex.real += self.adaptation_rate * k_real_grad;
        }

        if let Some(x0_grad) = gradients.get("x0") {
            self.x0 += self.adaptation_rate * x0_grad;
        }
    }
}

impl LinearComponent {
    pub fn new(slope: f64, intercept: f64, weight: f64) -> Self {
        Self {
            slope,
            intercept,
            weight,
            adaptation_rate: 0.01,
        }
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        self.weight * (self.slope * x + self.intercept)
    }

    pub fn gradient(&self, x: f64, target: f64) -> HashMap<String, f64> {
        let current_output = self.evaluate(x);
        let error = target - current_output;

        let mut gradients = HashMap::new();
        gradients.insert("slope".to_string(), error * self.weight * x);
        gradients.insert("intercept".to_string(), error * self.weight);
        gradients.insert("weight".to_string(), error * (self.slope * x + self.intercept));

        gradients
    }

    pub fn adapt(&mut self, gradients: &HashMap<String, f64>) {
        if let Some(slope_grad) = gradients.get("slope") {
            self.slope += self.adaptation_rate * slope_grad;
        }

        if let Some(intercept_grad) = gradients.get("intercept") {
            self.intercept += self.adaptation_rate * intercept_grad;
        }

        if let Some(weight_grad) = gradients.get("weight") {
            self.weight += self.adaptation_rate * weight_grad;
        }
    }
}

impl GeneralShapeEquation {
    /// إنشاء معادلة شكل عام جديدة
    pub fn new() -> Self {
        Self {
            sigmoid_components: Vec::new(),
            linear_components: Vec::new(),
            global_parameters: GlobalParameters::default(),
            adaptation_history: Vec::new(),
            performance_metrics: PerformanceMetrics::default(),
        }
    }

    /// إضافة مكون سيغمويد
    pub fn add_sigmoid_component(&mut self, alpha: f64, k_complex: ComplexNumber, x0: f64) -> &mut Self {
        self.sigmoid_components.push(GeneralizedSigmoidComponent::new(alpha, k_complex, x0));
        self
    }

    /// إضافة مكون خطي
    pub fn add_linear_component(&mut self, slope: f64, intercept: f64, weight: f64) -> &mut Self {
        self.linear_components.push(LinearComponent::new(slope, intercept, weight));
        self
    }

    /// تقييم المعادلة
    pub fn evaluate(&self, x: f64) -> EvaluationResult {
        let start_time = std::time::Instant::now();

        let mut total_output = 0.0;
        let mut component_contributions = HashMap::new();

        // تقييم مكونات السيغمويد
        for (i, sigmoid) in self.sigmoid_components.iter().enumerate() {
            let contribution = sigmoid.evaluate(x);
            total_output += contribution;
            component_contributions.insert(format!("sigmoid_{}", i), contribution);
        }

        // تقييم المكونات الخطية
        for (i, linear) in self.linear_components.iter().enumerate() {
            let contribution = linear.evaluate(x);
            total_output += contribution;
            component_contributions.insert(format!("linear_{}", i), contribution);
        }

        // تطبيق التحجيم العام
        total_output *= self.global_parameters.overall_scaling;

        let computational_cost = start_time.elapsed().as_nanos() as u64;

        EvaluationResult {
            output_value: total_output,
            component_contributions,
            confidence_score: self.calculate_confidence(),
            computational_cost,
            gradient_info: self.calculate_gradient_info(x),
        }
    }

    /// تدريب المعادلة على البيانات
    pub fn train(&mut self, training_data: Vec<(f64, f64)>, adaptation_type: AdaptationType) {
        for epoch in 0..self.global_parameters.max_iterations {
            let mut total_error = 0.0;

            for (input, target) in &training_data {
                let result = self.evaluate(*input);
                let error = target - result.output_value;
                total_error += error * error;

                // تكيف المكونات
                match adaptation_type {
                    AdaptationType::Gradient => self.gradient_adaptation(*input, *target),
                    AdaptationType::Evolutionary => self.evolutionary_adaptation(*input, *target),
                    AdaptationType::Bayesian => self.bayesian_adaptation(*input, *target),
                    AdaptationType::Hybrid => self.hybrid_adaptation(*input, *target),
                }

                // تسجيل حدث التكيف
                self.record_adaptation_event(*input, *target, result.output_value, error);
            }

            let mse = total_error / training_data.len() as f64;
            self.performance_metrics.mean_squared_error = mse;

            // فحص التقارب
            if mse < self.global_parameters.convergence_threshold {
                break;
            }
        }
    }

    /// تكيف بالتدرج
    fn gradient_adaptation(&mut self, input: f64, target: f64) {
        // تكيف مكونات السيغمويد
        for sigmoid in &mut self.sigmoid_components {
            let gradients = sigmoid.gradient(input, target);
            sigmoid.adapt(&gradients);
        }

        // تكيف المكونات الخطية
        for linear in &mut self.linear_components {
            let gradients = linear.gradient(input, target);
            linear.adapt(&gradients);
        }
    }

    /// تكيف تطوري (مبسط)
    fn evolutionary_adaptation(&mut self, input: f64, target: f64) {
        let current_error = (target - self.evaluate(input).output_value).abs();

        // طفرات عشوائية صغيرة
        for i in 0..self.sigmoid_components.len() {
            let mutation_strength = 0.01;
            let old_alpha = self.sigmoid_components[i].alpha;
            let old_k_real = self.sigmoid_components[i].k_complex.real;
            let old_x0 = self.sigmoid_components[i].x0;

            // تطبيق الطفرة
            self.sigmoid_components[i].alpha += (rand::random::<f64>() - 0.5) * mutation_strength;
            self.sigmoid_components[i].k_complex.real += (rand::random::<f64>() - 0.5) * mutation_strength;
            self.sigmoid_components[i].x0 += (rand::random::<f64>() - 0.5) * mutation_strength;

            // فحص إذا كانت الطفرة تحسن الأداء
            let new_error = (target - self.evaluate(input).output_value).abs();
            if new_error > current_error {
                // إلغاء الطفرة
                self.sigmoid_components[i].alpha = old_alpha;
                self.sigmoid_components[i].k_complex.real = old_k_real;
                self.sigmoid_components[i].x0 = old_x0;
            }
        }
    }

    /// تكيف بايزي (مبسط)
    fn bayesian_adaptation(&mut self, input: f64, target: f64) {
        // تحديث المعاملات بناءً على التوزيع البايزي
        let uncertainty = self.calculate_uncertainty();
        let adaptation_factor = 1.0 / (1.0 + uncertainty);

        for sigmoid in &mut self.sigmoid_components {
            sigmoid.adaptation_rate = self.global_parameters.learning_rate * adaptation_factor;
            let gradients = sigmoid.gradient(input, target);
            sigmoid.adapt(&gradients);
        }
    }

    /// تكيف مختلط
    fn hybrid_adaptation(&mut self, input: f64, target: f64) {
        // دمج التدرج والتطوري
        self.gradient_adaptation(input, target);

        if rand::random::<f64>() < 0.1 { // 10% احتمال للطفرة
            self.evolutionary_adaptation(input, target);
        }
    }

    // الدوال المساعدة
    fn calculate_confidence(&self) -> f64 {
        1.0 - self.performance_metrics.mean_squared_error
    }

    fn calculate_gradient_info(&self, x: f64) -> Vec<f64> {
        // حساب معلومات التدرج لكل مكون
        let mut gradients = Vec::new();

        for sigmoid in &self.sigmoid_components {
            let shifted_x = x - sigmoid.x0;
            let gradient_magnitude = sigmoid.alpha * sigmoid.k_complex.magnitude() * shifted_x.abs();
            gradients.push(gradient_magnitude);
        }

        for linear in &self.linear_components {
            gradients.push(linear.slope.abs());
        }

        gradients
    }

    fn calculate_uncertainty(&self) -> f64 {
        self.performance_metrics.mean_squared_error +
        (1.0 / (self.adaptation_history.len() as f64 + 1.0))
    }

    fn record_adaptation_event(&mut self, input: f64, target: f64, predicted: f64, error: f64) {
        let event = AdaptationEvent {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            input_data: vec![input],
            target_output: target,
            predicted_output: predicted,
            error,
            parameter_changes: HashMap::new(), // سيتم ملؤها في التطبيق الفعلي
        };

        self.adaptation_history.push(event);
    }
}

impl GlobalParameters {
    pub fn default() -> Self {
        Self {
            overall_scaling: 1.0,
            noise_tolerance: 0.01,
            convergence_threshold: 0.001,
            max_iterations: 1000,
            learning_rate: 0.01,
        }
    }
}

impl PerformanceMetrics {
    pub fn default() -> Self {
        Self {
            mean_squared_error: f64::INFINITY,
            correlation_coefficient: 0.0,
            adaptation_speed: 0.0,
            stability_index: 0.0,
            complexity_score: 0.0,
        }
    }
}

// استخدام rand للطفرات العشوائية
mod rand {
    pub fn random<T>() -> T
    where
        T: From<f64>
    {
        // مولد أرقام عشوائية مبسط
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let random_value = ((seed.wrapping_mul(1103515245).wrapping_add(12345)) >> 16) as f64 / 32768.0;
        T::from(random_value)
    }
}
