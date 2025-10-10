// وحدة الاستنباط - Shape Inference
// تحويل الصور إلى معادلات الشكل العام

use std::collections::HashMap;
use super::adaptive_equations::{GeneralShapeEquation, ComplexNumber, GeneralizedSigmoidComponent, LinearComponent};
use super::artistic_renderer::{BasicShape, ShapeProperty, RenderedImage, Pixel};
use super::thinking_core::{ThinkingCore, AnalysisResult, LayerType};
use super::expert_explorer::{ExpertExplorer, DecisionResult, OperationMode};

// ========== تحليل الصورة - Image Analysis ==========
#[derive(Debug, Clone)]
pub struct ImageFeatures {
    pub dominant_frequencies: Vec<f64>,
    pub color_distribution: ColorDistribution,
    pub geometric_properties: GeometricProperties,
    pub texture_analysis: TextureAnalysis,
    pub symmetry_measures: SymmetryMeasures,
}

#[derive(Debug, Clone)]
pub struct ColorDistribution {
    pub red_histogram: Vec<u32>,
    pub green_histogram: Vec<u32>,
    pub blue_histogram: Vec<u32>,
    pub dominant_color: (u8, u8, u8),
    pub color_variance: f64,
}

#[derive(Debug, Clone)]
pub struct GeometricProperties {
    pub center_of_mass: (f64, f64),
    pub bounding_box: (f64, f64, f64, f64), // x, y, width, height
    pub aspect_ratio: f64,
    pub area: f64,
    pub perimeter: f64,
    pub circularity: f64,
    pub rectangularity: f64,
}

#[derive(Debug, Clone)]
pub struct TextureAnalysis {
    pub roughness: f64,
    pub smoothness: f64,
    pub pattern_regularity: f64,
    pub edge_density: f64,
    pub gradient_magnitude: f64,
}

#[derive(Debug, Clone)]
pub struct SymmetryMeasures {
    pub horizontal_symmetry: f64,
    pub vertical_symmetry: f64,
    pub radial_symmetry: f64,
    pub rotational_symmetry: f64,
}

// ========== مُولد المعادلات - Equation Generator ==========
#[derive(Debug)]
pub struct EquationGenerator {
    pub sigmoid_templates: Vec<SigmoidTemplate>,
    pub linear_templates: Vec<LinearTemplate>,
    pub combination_rules: Vec<CombinationRule>,
}

#[derive(Debug, Clone)]
pub struct SigmoidTemplate {
    pub alpha_range: (f64, f64),
    pub k_real_range: (f64, f64),
    pub k_imag_range: (f64, f64),
    pub x0_range: (f64, f64),
    pub shape_type: BasicShape,
    pub confidence: f64,
}

#[derive(Debug, Clone)]
pub struct LinearTemplate {
    pub slope_range: (f64, f64),
    pub intercept_range: (f64, f64),
    pub weight_range: (f64, f64),
    pub shape_type: BasicShape,
    pub confidence: f64,
}

#[derive(Debug, Clone)]
pub struct CombinationRule {
    pub primary_component: ComponentType,
    pub secondary_components: Vec<ComponentType>,
    pub mixing_weights: Vec<f64>,
    pub applicability_score: f64,
}

#[derive(Debug, Clone)]
pub enum ComponentType {
    Sigmoid(SigmoidTemplate),
    Linear(LinearTemplate),
}

// ========== مُقيم الثقة - Confidence Evaluator ==========
#[derive(Debug)]
pub struct ConfidenceEvaluator {
    pub feature_weights: HashMap<String, f64>,
    pub shape_priors: HashMap<BasicShape, f64>,
    pub uncertainty_threshold: f64,
}

impl ConfidenceEvaluator {
    pub fn new() -> Self {
        let mut feature_weights = HashMap::new();
        feature_weights.insert("geometric_match".to_string(), 0.4);
        feature_weights.insert("color_consistency".to_string(), 0.2);
        feature_weights.insert("texture_similarity".to_string(), 0.2);
        feature_weights.insert("symmetry_alignment".to_string(), 0.2);

        let mut shape_priors = HashMap::new();
        shape_priors.insert(BasicShape::Circle, 0.9);
        shape_priors.insert(BasicShape::Square, 0.85);
        shape_priors.insert(BasicShape::Cat, 0.7);
        shape_priors.insert(BasicShape::Human, 0.75);
        shape_priors.insert(BasicShape::Tree, 0.8);

        Self {
            feature_weights,
            shape_priors,
            uncertainty_threshold: 0.6,
        }
    }

    pub fn evaluate_confidence(&self, features: &ImageFeatures, predicted_shape: BasicShape, equation: &GeneralShapeEquation) -> f64 {
        let geometric_score = self.evaluate_geometric_match(features, predicted_shape);
        let color_score = self.evaluate_color_consistency(features);
        let texture_score = self.evaluate_texture_similarity(features, predicted_shape);
        let symmetry_score = self.evaluate_symmetry_alignment(features, predicted_shape);

        let weighted_score =
            geometric_score * self.feature_weights["geometric_match"] +
            color_score * self.feature_weights["color_consistency"] +
            texture_score * self.feature_weights["texture_similarity"] +
            symmetry_score * self.feature_weights["symmetry_alignment"];

        let prior_score = self.shape_priors.get(&predicted_shape).unwrap_or(&0.5);

        // دمج النتيجة مع المعرفة المسبقة
        weighted_score * 0.8 + prior_score * 0.2
    }

    fn evaluate_geometric_match(&self, features: &ImageFeatures, shape: BasicShape) -> f64 {
        match shape {
            BasicShape::Circle => features.geometric_properties.circularity,
            BasicShape::Square => features.geometric_properties.rectangularity,
            BasicShape::Cat => {
                // القطة لها خصائص هندسية معقدة
                let asymmetry = 1.0 - features.symmetry_measures.horizontal_symmetry;
                let complexity = features.texture_analysis.edge_density;
                (asymmetry + complexity) / 2.0
            },
            BasicShape::Human => {
                // الإنسان له تماثل عمودي وتعقيد متوسط
                let vertical_sym = features.symmetry_measures.vertical_symmetry;
                let aspect_ratio_score = if features.geometric_properties.aspect_ratio > 1.5 { 1.0 } else { 0.5 };
                (vertical_sym + aspect_ratio_score) / 2.0
            },
            BasicShape::Tree => {
                // الشجرة لها تماثل عمودي وشكل عمودي
                let vertical_sym = features.symmetry_measures.vertical_symmetry;
                let vertical_aspect = if features.geometric_properties.aspect_ratio > 1.2 { 1.0 } else { 0.3 };
                (vertical_sym + vertical_aspect) / 2.0
            },
            _ => 0.5, // قيمة افتراضية للأشكال الأخرى
        }
    }

    fn evaluate_color_consistency(&self, features: &ImageFeatures) -> f64 {
        // تقييم اتساق الألوان
        1.0 - features.color_distribution.color_variance
    }

    fn evaluate_texture_similarity(&self, features: &ImageFeatures, shape: BasicShape) -> f64 {
        match shape {
            BasicShape::Cat => features.texture_analysis.roughness, // القطة لها فراء
            BasicShape::Human => features.texture_analysis.smoothness, // الإنسان له جلد ناعم
            BasicShape::Tree => features.texture_analysis.roughness, // الشجرة لها لحاء خشن
            BasicShape::Circle | BasicShape::Square => features.texture_analysis.smoothness, // الأشكال الهندسية ناعمة
            _ => 0.5,
        }
    }

    fn evaluate_symmetry_alignment(&self, features: &ImageFeatures, shape: BasicShape) -> f64 {
        match shape {
            BasicShape::Circle => features.symmetry_measures.radial_symmetry,
            BasicShape::Square => (features.symmetry_measures.horizontal_symmetry + features.symmetry_measures.vertical_symmetry) / 2.0,
            BasicShape::Human | BasicShape::Tree => features.symmetry_measures.vertical_symmetry,
            BasicShape::Cat => 1.0 - features.symmetry_measures.horizontal_symmetry, // القطة غير متماثلة
            _ => 0.5,
        }
    }
}

// ========== وحدة الاستنباط الرئيسية - Main Shape Inference ==========
#[derive(Debug)]
pub struct ShapeInference {
    pub thinking_core: ThinkingCore,
    pub expert_explorer: ExpertExplorer,
    pub equation_generator: EquationGenerator,
    pub confidence_evaluator: ConfidenceEvaluator,
    pub feature_extractor: FeatureExtractor,
}

#[derive(Debug)]
pub struct FeatureExtractor {
    pub edge_detection_threshold: f64,
    pub color_quantization_levels: u8,
    pub texture_window_size: u32,
}

impl FeatureExtractor {
    pub fn new() -> Self {
        Self {
            edge_detection_threshold: 0.1,
            color_quantization_levels: 16,
            texture_window_size: 5,
        }
    }

    pub fn extract_features(&self, image: &RenderedImage) -> ImageFeatures {
        let color_dist = self.analyze_color_distribution(image);
        let geometric_props = self.analyze_geometric_properties(image);
        let texture_analysis = self.analyze_texture(image);
        let symmetry_measures = self.analyze_symmetry(image);
        let dominant_frequencies = self.extract_frequency_domain(image);

        ImageFeatures {
            dominant_frequencies,
            color_distribution: color_dist,
            geometric_properties: geometric_props,
            texture_analysis,
            symmetry_measures,
        }
    }

    fn analyze_color_distribution(&self, image: &RenderedImage) -> ColorDistribution {
        let mut red_hist = vec![0u32; 256];
        let mut green_hist = vec![0u32; 256];
        let mut blue_hist = vec![0u32; 256];

        let mut total_red = 0u64;
        let mut total_green = 0u64;
        let mut total_blue = 0u64;
        let total_pixels = (image.width * image.height) as u64;

        for row in &image.pixels {
            for pixel in row {
                red_hist[pixel.red as usize] += 1;
                green_hist[pixel.green as usize] += 1;
                blue_hist[pixel.blue as usize] += 1;

                total_red += pixel.red as u64;
                total_green += pixel.green as u64;
                total_blue += pixel.blue as u64;
            }
        }

        let avg_red = (total_red / total_pixels) as u8;
        let avg_green = (total_green / total_pixels) as u8;
        let avg_blue = (total_blue / total_pixels) as u8;

        // حساب التباين
        let mut variance_sum = 0.0;
        for row in &image.pixels {
            for pixel in row {
                let r_diff = pixel.red as f64 - avg_red as f64;
                let g_diff = pixel.green as f64 - avg_green as f64;
                let b_diff = pixel.blue as f64 - avg_blue as f64;
                variance_sum += r_diff * r_diff + g_diff * g_diff + b_diff * b_diff;
            }
        }
        let color_variance = variance_sum / (total_pixels as f64 * 3.0 * 255.0 * 255.0);

        ColorDistribution {
            red_histogram: red_hist,
            green_histogram: green_hist,
            blue_histogram: blue_hist,
            dominant_color: (avg_red, avg_green, avg_blue),
            color_variance,
        }
    }

    fn analyze_geometric_properties(&self, image: &RenderedImage) -> GeometricProperties {
        // حساب مركز الكتلة
        let mut total_intensity = 0.0;
        let mut weighted_x = 0.0;
        let mut weighted_y = 0.0;
        let mut non_zero_pixels = 0;

        for (y, row) in image.pixels.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                let intensity = (pixel.red as f64 + pixel.green as f64 + pixel.blue as f64) / (3.0 * 255.0);
                if intensity > 0.1 {
                    total_intensity += intensity;
                    weighted_x += x as f64 * intensity;
                    weighted_y += y as f64 * intensity;
                    non_zero_pixels += 1;
                }
            }
        }

        let center_x = if total_intensity > 0.0 { weighted_x / total_intensity } else { image.width as f64 / 2.0 };
        let center_y = if total_intensity > 0.0 { weighted_y / total_intensity } else { image.height as f64 / 2.0 };

        // حساب الصندوق المحيط
        let mut min_x: f64 = image.width as f64;
        let mut max_x: f64 = 0.0;
        let mut min_y: f64 = image.height as f64;
        let mut max_y: f64 = 0.0;

        for (y, row) in image.pixels.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                let intensity = (pixel.red as f64 + pixel.green as f64 + pixel.blue as f64) / (3.0 * 255.0);
                if intensity > 0.1 {
                    min_x = min_x.min(x as f64);
                    max_x = max_x.max(x as f64);
                    min_y = min_y.min(y as f64);
                    max_y = max_y.max(y as f64);
                }
            }
        }

        let width = max_x - min_x;
        let height = max_y - min_y;
        let aspect_ratio = if height > 0.0 { width / height } else { 1.0 };
        let area = non_zero_pixels as f64;
        let perimeter = 2.0 * (width + height); // تقريب بسيط

        // حساب الدائرية والمستطيلية
        let circularity = if perimeter > 0.0 { 4.0 * std::f64::consts::PI * area / (perimeter * perimeter) } else { 0.0 };
        let rectangularity = if width * height > 0.0 { area / (width * height) } else { 0.0 };

        GeometricProperties {
            center_of_mass: (center_x, center_y),
            bounding_box: (min_x, min_y, width, height),
            aspect_ratio,
            area,
            perimeter,
            circularity,
            rectangularity,
        }
    }

    fn analyze_texture(&self, image: &RenderedImage) -> TextureAnalysis {
        let mut edge_count = 0;
        let mut gradient_sum = 0.0;
        let total_pixels = (image.width * image.height) as f64;

        for y in 1..(image.height - 1) {
            for x in 1..(image.width - 1) {
                let current = &image.pixels[y as usize][x as usize];
                let right = &image.pixels[y as usize][(x + 1) as usize];
                let down = &image.pixels[(y + 1) as usize][x as usize];

                let current_intensity = (current.red as f64 + current.green as f64 + current.blue as f64) / 3.0;
                let right_intensity = (right.red as f64 + right.green as f64 + right.blue as f64) / 3.0;
                let down_intensity = (down.red as f64 + down.green as f64 + down.blue as f64) / 3.0;

                let grad_x = (right_intensity - current_intensity).abs();
                let grad_y = (down_intensity - current_intensity).abs();
                let gradient_magnitude = (grad_x * grad_x + grad_y * grad_y).sqrt();

                gradient_sum += gradient_magnitude;

                if gradient_magnitude > self.edge_detection_threshold * 255.0 {
                    edge_count += 1;
                }
            }
        }

        let edge_density = edge_count as f64 / total_pixels;
        let avg_gradient = gradient_sum / total_pixels;
        let roughness = edge_density;
        let smoothness = 1.0 - roughness;
        let pattern_regularity = 1.0 - (avg_gradient / 255.0); // تقريب بسيط

        TextureAnalysis {
            roughness,
            smoothness,
            pattern_regularity,
            edge_density,
            gradient_magnitude: avg_gradient,
        }
    }

    fn analyze_symmetry(&self, image: &RenderedImage) -> SymmetryMeasures {
        let width = image.width as usize;
        let height = image.height as usize;

        // التماثل الأفقي
        let mut horizontal_diff = 0.0;
        for y in 0..height {
            for x in 0..(width / 2) {
                let left_pixel = &image.pixels[y][x];
                let right_pixel = &image.pixels[y][width - 1 - x];

                let left_intensity = (left_pixel.red as f64 + left_pixel.green as f64 + left_pixel.blue as f64) / 3.0;
                let right_intensity = (right_pixel.red as f64 + right_pixel.green as f64 + right_pixel.blue as f64) / 3.0;

                horizontal_diff += (left_intensity - right_intensity).abs();
            }
        }
        let horizontal_symmetry = 1.0 - (horizontal_diff / (255.0 * (width * height / 2) as f64));

        // التماثل العمودي
        let mut vertical_diff = 0.0;
        for y in 0..(height / 2) {
            for x in 0..width {
                let top_pixel = &image.pixels[y][x];
                let bottom_pixel = &image.pixels[height - 1 - y][x];

                let top_intensity = (top_pixel.red as f64 + top_pixel.green as f64 + top_pixel.blue as f64) / 3.0;
                let bottom_intensity = (bottom_pixel.red as f64 + bottom_pixel.green as f64 + bottom_pixel.blue as f64) / 3.0;

                vertical_diff += (top_intensity - bottom_intensity).abs();
            }
        }
        let vertical_symmetry = 1.0 - (vertical_diff / (255.0 * (width * height / 2) as f64));

        // التماثل الشعاعي (تقريب بسيط)
        let radial_symmetry = (horizontal_symmetry + vertical_symmetry) / 2.0;

        // التماثل الدوراني (تقريب بسيط)
        let rotational_symmetry = radial_symmetry;

        SymmetryMeasures {
            horizontal_symmetry: horizontal_symmetry.max(0.0).min(1.0),
            vertical_symmetry: vertical_symmetry.max(0.0).min(1.0),
            radial_symmetry: radial_symmetry.max(0.0).min(1.0),
            rotational_symmetry: rotational_symmetry.max(0.0).min(1.0),
        }
    }

    fn extract_frequency_domain(&self, image: &RenderedImage) -> Vec<f64> {
        // تحليل تردد مبسط - استخراج الترددات المهيمنة
        let mut frequencies = Vec::new();

        // تحليل الترددات الأفقية
        for y in 0..image.height {
            let mut row_intensity = Vec::new();
            for x in 0..image.width {
                let pixel = &image.pixels[y as usize][x as usize];
                let intensity = (pixel.red as f64 + pixel.green as f64 + pixel.blue as f64) / (3.0 * 255.0);
                row_intensity.push(intensity);
            }

            // حساب التردد المهيمن للصف (تقريب بسيط)
            let mut changes = 0;
            for i in 1..row_intensity.len() {
                if (row_intensity[i] - row_intensity[i-1]).abs() > 0.1 {
                    changes += 1;
                }
            }
            let frequency = changes as f64 / row_intensity.len() as f64;
            frequencies.push(frequency);
        }

        frequencies
    }
}

impl ShapeInference {
    pub fn new() -> Self {
        Self {
            thinking_core: ThinkingCore::new(),
            expert_explorer: ExpertExplorer::new(),
            equation_generator: EquationGenerator::new(),
            confidence_evaluator: ConfidenceEvaluator::new(),
            feature_extractor: FeatureExtractor::new(),
        }
    }

    pub fn image_to_equation(&mut self, image: &RenderedImage) -> InferenceResult {
        // استخراج الخصائص
        let features = self.feature_extractor.extract_features(image);

        // تحليل بالنواة التفكيرية
        let thinking_input = self.features_to_thinking_input(&features);
        let thinking_result = self.thinking_core.analyze(thinking_input);

        // قرار بنظام الخبير/المستكشف
        let decision_input = self.thinking_result_to_decision_input(&thinking_result);
        let decision_result = self.expert_explorer.make_decision(decision_input);

        // توليد المعادلة
        let predicted_shape = self.decision_to_shape(&decision_result);
        let equation = self.equation_generator.generate_equation(&features, predicted_shape);

        // تقييم الثقة
        let confidence = self.confidence_evaluator.evaluate_confidence(&features, predicted_shape, &equation);

        InferenceResult {
            predicted_shape,
            equation,
            confidence,
            features,
            thinking_analysis: thinking_result,
            decision_reasoning: decision_result,
        }
    }

    fn features_to_thinking_input(&self, features: &ImageFeatures) -> Vec<f64> {
        vec![
            features.geometric_properties.circularity,
            features.geometric_properties.rectangularity,
            features.geometric_properties.aspect_ratio,
            features.color_distribution.color_variance,
            features.texture_analysis.roughness,
            features.symmetry_measures.horizontal_symmetry,
            features.symmetry_measures.vertical_symmetry,
            features.geometric_properties.area / 10000.0, // تطبيع
        ]
    }

    fn thinking_result_to_decision_input(&self, thinking_result: &AnalysisResult) -> Vec<f64> {
        thinking_result.layer_contributions.values().cloned().collect()
    }

    fn decision_to_shape(&self, decision: &DecisionResult) -> BasicShape {
        // تحويل قرار الخبير/المستكشف إلى شكل أساسي
        let decision_value = decision.final_decision;

        if decision_value > 0.9 {
            BasicShape::Circle
        } else if decision_value > 0.8 {
            BasicShape::Square
        } else if decision_value > 0.7 {
            BasicShape::Human
        } else if decision_value > 0.6 {
            BasicShape::Cat
        } else if decision_value > 0.5 {
            BasicShape::Tree
        } else {
            BasicShape::Circle // افتراضي
        }
    }
}

impl EquationGenerator {
    pub fn new() -> Self {
        Self {
            sigmoid_templates: Self::create_sigmoid_templates(),
            linear_templates: Self::create_linear_templates(),
            combination_rules: Self::create_combination_rules(),
        }
    }

    fn create_sigmoid_templates() -> Vec<SigmoidTemplate> {
        vec![
            SigmoidTemplate {
                alpha_range: (50.0, 150.0),
                k_real_range: (1.0, 5.0),
                k_imag_range: (0.0, 2.0),
                x0_range: (-2.0, 2.0),
                shape_type: BasicShape::Cat,
                confidence: 0.8,
            },
            SigmoidTemplate {
                alpha_range: (80.0, 120.0),
                k_real_range: (2.0, 8.0),
                k_imag_range: (0.0, 0.5),
                x0_range: (-1.0, 1.0),
                shape_type: BasicShape::Circle,
                confidence: 0.9,
            },
        ]
    }

    fn create_linear_templates() -> Vec<LinearTemplate> {
        vec![
            LinearTemplate {
                slope_range: (0.5, 2.0),
                intercept_range: (-1.0, 1.0),
                weight_range: (0.3, 1.0),
                shape_type: BasicShape::Square,
                confidence: 0.85,
            },
        ]
    }

    fn create_combination_rules() -> Vec<CombinationRule> {
        vec![
            CombinationRule {
                primary_component: ComponentType::Sigmoid(SigmoidTemplate {
                    alpha_range: (80.0, 120.0),
                    k_real_range: (2.0, 8.0),
                    k_imag_range: (0.0, 0.5),
                    x0_range: (-1.0, 1.0),
                    shape_type: BasicShape::Circle,
                    confidence: 0.9,
                }),
                secondary_components: vec![],
                mixing_weights: vec![1.0],
                applicability_score: 0.9,
            },
        ]
    }

    pub fn generate_equation(&self, features: &ImageFeatures, shape: BasicShape) -> GeneralShapeEquation {
        let mut equation = GeneralShapeEquation::new();

        // اختيار القالب المناسب
        for template in &self.sigmoid_templates {
            if template.shape_type == shape {
                let alpha = self.interpolate_range(template.alpha_range, features.geometric_properties.area / 10000.0);
                let k_real = self.interpolate_range(template.k_real_range, features.geometric_properties.circularity);
                let k_imag = self.interpolate_range(template.k_imag_range, features.texture_analysis.roughness);
                let x0 = self.interpolate_range(template.x0_range, features.geometric_properties.center_of_mass.0 / 256.0 - 1.0);

                equation.add_sigmoid_component(alpha, ComplexNumber::new(k_real, k_imag), x0);
                break;
            }
        }

        // إضافة مكونات خطية إذا لزم الأمر
        for template in &self.linear_templates {
            if template.shape_type == shape {
                let slope = self.interpolate_range(template.slope_range, features.geometric_properties.aspect_ratio);
                let intercept = self.interpolate_range(template.intercept_range, features.geometric_properties.center_of_mass.1 / 256.0 - 1.0);
                let weight = self.interpolate_range(template.weight_range, features.texture_analysis.smoothness);

                equation.add_linear_component(slope, intercept, weight);
                break;
            }
        }

        equation
    }

    fn interpolate_range(&self, range: (f64, f64), factor: f64) -> f64 {
        let clamped_factor = factor.max(0.0).min(1.0);
        range.0 + (range.1 - range.0) * clamped_factor
    }
}

// ========== نتيجة الاستنباط - Inference Result ==========
#[derive(Debug)]
pub struct InferenceResult {
    pub predicted_shape: BasicShape,
    pub equation: GeneralShapeEquation,
    pub confidence: f64,
    pub features: ImageFeatures,
    pub thinking_analysis: AnalysisResult,
    pub decision_reasoning: DecisionResult,
}
