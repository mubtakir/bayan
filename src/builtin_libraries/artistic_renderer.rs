// وحدة الرسم الفنية - Artistic Renderer
// تحويل المعادلات إلى صور والعكس

use std::collections::HashMap;
use super::adaptive_equations::{GeneralShapeEquation, ComplexNumber, GeneralizedSigmoidComponent, LinearComponent};

// ========== المعادلة الأم - Mother Equation ==========
#[derive(Debug, Clone)]
pub struct MotherEquation {
    pub universal_constant: f64,
    pub zero_duality_factor: ComplexNumber,
    pub perpendicularity_matrix: [[f64; 2]; 2],
    pub filament_threads: Vec<FilamentThread>,
}

#[derive(Debug, Clone)]
pub struct FilamentThread {
    pub thread_id: u32,
    pub vibration_frequency: ComplexNumber,
    pub energy_level: f64,
    pub connection_strength: f64,
}

impl MotherEquation {
    pub fn new() -> Self {
        Self {
            universal_constant: 1.618033988749, // النسبة الذهبية
            zero_duality_factor: ComplexNumber::new(0.0, 1.0), // i
            perpendicularity_matrix: [[1.0, 0.0], [0.0, 1.0]], // مصفوفة الوحدة
            filament_threads: vec![
                FilamentThread {
                    thread_id: 1,
                    vibration_frequency: ComplexNumber::new(1.0, 0.0),
                    energy_level: 1.0,
                    connection_strength: 1.0,
                },
            ],
        }
    }

    pub fn generate_base_equation(&self) -> GeneralShapeEquation {
        let mut equation = GeneralShapeEquation::new();

        // إضافة مكون سيغمويد أساسي من المعادلة الأم
        let base_sigmoid = GeneralizedSigmoidComponent {
            alpha: self.universal_constant * 100.0,
            k_complex: self.zero_duality_factor.clone(),
            x0: 0.0,
            cutting_factor: 1.0,
            adaptation_rate: 0.1,
        };

        equation.sigmoid_components.push(base_sigmoid);
        equation
    }
}

// ========== الأشكال الأساسية - Basic Shapes ==========
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BasicShape {
    // كائنات حية
    Cat,           // قطة
    Dog,           // كلب
    Human,         // إنسان
    Lion,          // أسد
    Bird,          // طائر
    Fish,          // سمكة

    // نباتات
    Tree,          // شجرة
    Flower,        // زهرة
    Grass,         // عشب
    Leaf,          // ورقة نبات

    // أشكال هندسية
    Circle,        // دائرة
    Square,        // مربع
    Triangle,      // مثلث
    Rectangle,     // مستطيل
    Pentagon,      // خماسي
    Hexagon,       // سداسي

    // أدوات ومواد
    Paper,         // ورقة بيضاء
    Blackboard,    // لوح أسود
    Pen,           // قلم
    Book,          // كتاب
    Table,         // طاولة
    Chair,         // كرسي

    // عناصر طبيعية
    Mountain,      // جبل
    River,         // نهر
    Cloud,         // سحابة
    Sun,           // شمس
    Moon,          // قمر
    Star,          // نجمة
}

// ========== خصائص الأشكال - Shape Properties ==========
#[derive(Debug, Clone)]
pub enum ShapeProperty {
    // الموضع والحركة
    Position(String),      // "standing", "sitting", "sleeping", "flying"
    Orientation(f64),      // زاوية الدوران بالدرجات
    Movement(String),      // "static", "walking", "running", "swimming"

    // الألوان والمظهر
    Color(String),         // "black", "white", "red", "green", "blue"
    Brightness(f64),       // 0.0 إلى 1.0
    Transparency(f64),     // 0.0 (شفاف) إلى 1.0 (معتم)
    Texture(String),       // "smooth", "rough", "furry", "metallic"

    // الحجم والشكل
    Size(f64),            // 0.5, 1.0, 2.0
    Width(f64),           // العرض
    Height(f64),          // الارتفاع
    Depth(f64),           // العمق

    // خصائص متقدمة
    Expression(String),    // "happy", "sad", "angry", "neutral"
    Style(String),        // "realistic", "cartoon", "abstract"
    Age(String),          // "young", "adult", "old"
    Material(String),     // "wood", "metal", "plastic", "fabric"
}

// ========== تحويل الخصائص - Property Transform ==========
#[derive(Debug, Clone)]
pub struct PropertyTransform {
    pub property_type: String,
    pub transform_function: TransformFunction,
    pub intensity: f64,
}

#[derive(Debug, Clone)]
pub enum TransformFunction {
    Linear(f64, f64),           // y = mx + b
    Sigmoid(f64, f64, f64),     // alpha, k, x0
    Exponential(f64, f64),      // a * e^(bx)
    Sinusoidal(f64, f64, f64),  // amplitude, frequency, phase
    Custom(Vec<f64>),           // معاملات مخصصة
}

// ========== محرك الرسم - Rendering Engine ==========
#[derive(Debug)]
pub struct RenderingEngine {
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub resolution: f64,
    pub color_depth: u8,
    pub anti_aliasing: bool,
}

impl RenderingEngine {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            canvas_width: width,
            canvas_height: height,
            resolution: 1.0,
            color_depth: 24,
            anti_aliasing: true,
        }
    }

    pub fn render_equation(&self, equation: &GeneralShapeEquation) -> RenderedImage {
        let mut pixels = vec![vec![Pixel::default(); self.canvas_width as usize]; self.canvas_height as usize];

        // رسم كل نقطة في الصورة
        for y in 0..self.canvas_height {
            for x in 0..self.canvas_width {
                let normalized_x = (x as f64 / self.canvas_width as f64) * 10.0 - 5.0;
                let normalized_y = (y as f64 / self.canvas_height as f64) * 10.0 - 5.0;

                let evaluation_result = equation.evaluate(normalized_x);
                let intensity = self.calculate_intensity(normalized_y, evaluation_result.output_value);

                pixels[y as usize][x as usize] = Pixel {
                    red: (intensity * 255.0) as u8,
                    green: (intensity * 255.0) as u8,
                    blue: (intensity * 255.0) as u8,
                    alpha: 255,
                };
            }
        }

        RenderedImage {
            width: self.canvas_width,
            height: self.canvas_height,
            pixels,
        }
    }

    fn calculate_intensity(&self, y: f64, equation_value: f64) -> f64 {
        let distance = (y - equation_value).abs();
        let max_distance = 1.0;

        if distance <= max_distance {
            1.0 - (distance / max_distance)
        } else {
            0.0
        }
    }
}

// ========== الصورة المُرسمة - Rendered Image ==========
#[derive(Debug, Clone)]
pub struct RenderedImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Pixel>>,
}

#[derive(Debug, Clone, Default)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

// ========== وحدة الرسم الفنية الرئيسية - Main Artistic Renderer ==========
#[derive(Debug)]
pub struct ArtisticRenderer {
    pub mother_equation: MotherEquation,
    pub shape_database: HashMap<BasicShape, GeneralShapeEquation>,
    pub property_transforms: HashMap<String, PropertyTransform>,
    pub rendering_engine: RenderingEngine,
    pub shape_cache: HashMap<String, RenderedImage>,
}

impl ArtisticRenderer {
    pub fn new() -> Self {
        let mut renderer = Self {
            mother_equation: MotherEquation::new(),
            shape_database: HashMap::new(),
            property_transforms: HashMap::new(),
            rendering_engine: RenderingEngine::new(512, 512),
            shape_cache: HashMap::new(),
        };

        renderer.initialize_basic_shapes();
        renderer.initialize_property_transforms();
        renderer
    }

    fn initialize_basic_shapes(&mut self) {
        // إنشاء الأشكال الأساسية من المعادلة الأم المستوحاة من ashkal.md
        let base_equation = self.mother_equation.generate_base_equation();

        // ========== كائنات حية ==========

        // قطة - شكل منحني ناعم مع انحناءات طبيعية
        let mut cat_equation = base_equation.clone();
        // جسم القطة - منحنى سيغمويدي ناعم
        cat_equation.add_sigmoid_component(80.0, ComplexNumber::new(3.0, 0.8), 0.0);
        // رأس القطة - انحناء أصغر
        cat_equation.add_sigmoid_component(40.0, ComplexNumber::new(5.0, 0.3), -1.5);
        // ذيل القطة - منحنى خطي
        cat_equation.add_linear_component(0.4, 0.2, 0.6);
        self.shape_database.insert(BasicShape::Cat, cat_equation);

        // كلب - شكل أكثر استقامة من القطة
        let mut dog_equation = base_equation.clone();
        dog_equation.add_sigmoid_component(90.0, ComplexNumber::new(2.5, 0.4), 0.0);
        dog_equation.add_sigmoid_component(35.0, ComplexNumber::new(4.0, 0.2), -1.2);
        dog_equation.add_linear_component(0.3, 0.1, 0.7);
        self.shape_database.insert(BasicShape::Dog, dog_equation);

        // إنسان - شكل متوازن ومتناسق
        let mut human_equation = base_equation.clone();
        // الجذع - خط مستقيم أساسي
        human_equation.add_linear_component(0.8, 0.0, 1.0);
        // الرأس - انحناء ناعم
        human_equation.add_sigmoid_component(60.0, ComplexNumber::new(6.0, 0.1), -2.0);
        // الأطراف - انحناءات طفيفة
        human_equation.add_sigmoid_component(30.0, ComplexNumber::new(2.0, 0.3), 1.5);
        self.shape_database.insert(BasicShape::Human, human_equation);

        // أسد - شكل قوي ومهيب
        let mut lion_equation = base_equation.clone();
        lion_equation.add_sigmoid_component(110.0, ComplexNumber::new(2.0, 0.6), 0.0);
        lion_equation.add_sigmoid_component(70.0, ComplexNumber::new(4.5, 0.4), -1.8);
        lion_equation.add_linear_component(0.5, 0.3, 0.8);
        self.shape_database.insert(BasicShape::Lion, lion_equation);

        // طائر - شكل انسيابي
        let mut bird_equation = base_equation.clone();
        bird_equation.add_sigmoid_component(50.0, ComplexNumber::new(7.0, 1.2), 0.0);
        bird_equation.add_sigmoid_component(25.0, ComplexNumber::new(3.0, 0.8), -0.8);
        bird_equation.add_linear_component(0.2, 0.4, 0.5);
        self.shape_database.insert(BasicShape::Bird, bird_equation);

        // سمكة - شكل مائي انسيابي
        let mut fish_equation = base_equation.clone();
        fish_equation.add_sigmoid_component(70.0, ComplexNumber::new(4.0, 1.5), 0.0);
        fish_equation.add_sigmoid_component(20.0, ComplexNumber::new(8.0, 0.5), 1.2);
        fish_equation.add_linear_component(0.1, 0.3, 0.4);
        self.shape_database.insert(BasicShape::Fish, fish_equation);

        // ========== نباتات ==========

        // شجرة - شكل عمودي متفرع
        let mut tree_equation = base_equation.clone();
        // الجذع - خط مستقيم قوي
        tree_equation.add_linear_component(1.2, 0.0, 1.5);
        // التاج - انحناءات متعددة للأوراق
        tree_equation.add_sigmoid_component(80.0, ComplexNumber::new(2.0, 0.7), -2.5);
        tree_equation.add_sigmoid_component(60.0, ComplexNumber::new(3.5, 0.4), -2.0);
        tree_equation.add_sigmoid_component(40.0, ComplexNumber::new(4.0, 0.6), -1.5);
        self.shape_database.insert(BasicShape::Tree, tree_equation);

        // زهرة - شكل دائري متناظر
        let mut flower_equation = base_equation.clone();
        flower_equation.add_sigmoid_component(60.0, ComplexNumber::new(8.0, 2.0), 0.0);
        flower_equation.add_sigmoid_component(30.0, ComplexNumber::new(12.0, 1.5), 0.0);
        flower_equation.add_linear_component(0.1, 0.0, 0.3);
        self.shape_database.insert(BasicShape::Flower, flower_equation);

        // عشب - شكل خطي بسيط
        let mut grass_equation = base_equation.clone();
        grass_equation.add_linear_component(0.8, 0.1, 0.2);
        grass_equation.add_sigmoid_component(15.0, ComplexNumber::new(1.5, 0.2), 0.5);
        self.shape_database.insert(BasicShape::Grass, grass_equation);

        // ورقة - شكل بيضاوي
        let mut leaf_equation = base_equation.clone();
        leaf_equation.add_sigmoid_component(45.0, ComplexNumber::new(6.0, 1.0), 0.0);
        leaf_equation.add_linear_component(0.3, 0.2, 0.4);
        self.shape_database.insert(BasicShape::Leaf, leaf_equation);

        // ========== أشكال هندسية ==========

        // دائرة - شكل هندسي مثالي (مستوحى من النظريات الرياضية في ashkal.md)
        let mut circle_equation = base_equation.clone();
        // دائرة مثالية باستخدام سيغمويد عالي التردد
        circle_equation.add_sigmoid_component(100.0, ComplexNumber::new(10.0, 3.14159), 0.0);
        circle_equation.add_sigmoid_component(-100.0, ComplexNumber::new(10.0, 3.14159), 3.14159);
        self.shape_database.insert(BasicShape::Circle, circle_equation);

        // مربع - شكل هندسي حاد
        let mut square_equation = base_equation.clone();
        // أضلاع المربع باستخدام انتقالات حادة
        square_equation.add_linear_component(1.0, 0.0, 1.0);
        square_equation.add_linear_component(-1.0, 2.0, 0.5);
        square_equation.add_sigmoid_component(50.0, ComplexNumber::new(20.0, 0.0), 1.0);
        square_equation.add_sigmoid_component(-50.0, ComplexNumber::new(20.0, 0.0), -1.0);
        self.shape_database.insert(BasicShape::Square, square_equation);

        // مثلث - شكل هندسي مدبب
        let mut triangle_equation = base_equation.clone();
        triangle_equation.add_linear_component(1.5, 0.0, 0.0);
        triangle_equation.add_linear_component(-0.75, 1.0, 0.5);
        triangle_equation.add_linear_component(-0.75, -1.0, 0.5);
        self.shape_database.insert(BasicShape::Triangle, triangle_equation);

        // مستطيل - شكل هندسي ممدود
        let mut rectangle_equation = base_equation.clone();
        rectangle_equation.add_linear_component(0.8, 0.0, 1.2);
        rectangle_equation.add_linear_component(-0.8, 3.0, 0.4);
        rectangle_equation.add_sigmoid_component(40.0, ComplexNumber::new(15.0, 0.0), 1.5);
        self.shape_database.insert(BasicShape::Rectangle, rectangle_equation);

        // خماسي - شكل هندسي معقد
        let mut pentagon_equation = base_equation.clone();
        pentagon_equation.add_sigmoid_component(80.0, ComplexNumber::new(5.0, 1.256), 0.0);
        pentagon_equation.add_sigmoid_component(60.0, ComplexNumber::new(5.0, 2.512), 0.628);
        pentagon_equation.add_linear_component(0.2, 0.0, 0.8);
        self.shape_database.insert(BasicShape::Pentagon, pentagon_equation);

        // سداسي - شكل هندسي متناظر
        let mut hexagon_equation = base_equation.clone();
        hexagon_equation.add_sigmoid_component(90.0, ComplexNumber::new(6.0, 1.047), 0.0);
        hexagon_equation.add_sigmoid_component(70.0, ComplexNumber::new(6.0, 2.094), 0.524);
        hexagon_equation.add_linear_component(0.15, 0.0, 0.85);
        self.shape_database.insert(BasicShape::Hexagon, hexagon_equation);

        // ========== أدوات ومواد ==========

        // ورقة - سطح مستو
        let mut paper_equation = base_equation.clone();
        paper_equation.add_linear_component(0.1, 0.0, 0.95);
        paper_equation.add_sigmoid_component(5.0, ComplexNumber::new(1.0, 0.1), 0.0);
        self.shape_database.insert(BasicShape::Paper, paper_equation);

        // لوح أسود - سطح مستو داكن
        let mut blackboard_equation = base_equation.clone();
        blackboard_equation.add_linear_component(0.05, 0.0, 0.1);
        blackboard_equation.add_sigmoid_component(2.0, ComplexNumber::new(0.5, 0.0), 0.0);
        self.shape_database.insert(BasicShape::Blackboard, blackboard_equation);

        // قلم - شكل أسطواني
        let mut pen_equation = base_equation.clone();
        pen_equation.add_linear_component(0.8, 0.0, 0.1);
        pen_equation.add_sigmoid_component(20.0, ComplexNumber::new(8.0, 0.0), 0.0);
        pen_equation.add_sigmoid_component(10.0, ComplexNumber::new(12.0, 0.0), 2.0);
        self.shape_database.insert(BasicShape::Pen, pen_equation);

        // كتاب - شكل مستطيلي سميك
        let mut book_equation = base_equation.clone();
        book_equation.add_linear_component(0.6, 0.0, 0.8);
        book_equation.add_linear_component(-0.3, 2.5, 0.4);
        book_equation.add_sigmoid_component(15.0, ComplexNumber::new(3.0, 0.0), 1.0);
        self.shape_database.insert(BasicShape::Book, book_equation);

        // طاولة - شكل أفقي مع أرجل
        let mut table_equation = base_equation.clone();
        table_equation.add_linear_component(0.2, 0.0, 1.0);
        table_equation.add_sigmoid_component(30.0, ComplexNumber::new(2.0, 0.0), -1.5);
        table_equation.add_sigmoid_component(30.0, ComplexNumber::new(2.0, 0.0), 1.5);
        self.shape_database.insert(BasicShape::Table, table_equation);

        // كرسي - شكل عمودي مع مقعد
        let mut chair_equation = base_equation.clone();
        chair_equation.add_linear_component(0.7, 0.0, 0.8);
        chair_equation.add_sigmoid_component(40.0, ComplexNumber::new(3.0, 0.0), 0.0);
        chair_equation.add_linear_component(0.3, 0.0, 0.3);
        self.shape_database.insert(BasicShape::Chair, chair_equation);

        // ========== عناصر طبيعية ==========

        // جبل - شكل مثلثي طبيعي
        let mut mountain_equation = base_equation.clone();
        mountain_equation.add_sigmoid_component(150.0, ComplexNumber::new(1.5, 0.3), 0.0);
        mountain_equation.add_sigmoid_component(80.0, ComplexNumber::new(2.0, 0.5), -1.0);
        mountain_equation.add_sigmoid_component(60.0, ComplexNumber::new(2.5, 0.4), 1.0);
        self.shape_database.insert(BasicShape::Mountain, mountain_equation);

        // نهر - شكل متعرج انسيابي
        let mut river_equation = base_equation.clone();
        river_equation.add_sigmoid_component(30.0, ComplexNumber::new(0.8, 1.5), 0.0);
        river_equation.add_sigmoid_component(20.0, ComplexNumber::new(1.2, 2.0), 2.0);
        river_equation.add_linear_component(0.1, 0.2, 0.3);
        self.shape_database.insert(BasicShape::River, river_equation);

        // سحابة - شكل منتفخ ناعم
        let mut cloud_equation = base_equation.clone();
        cloud_equation.add_sigmoid_component(70.0, ComplexNumber::new(3.0, 1.8), 0.0);
        cloud_equation.add_sigmoid_component(50.0, ComplexNumber::new(4.0, 1.2), -0.8);
        cloud_equation.add_sigmoid_component(40.0, ComplexNumber::new(3.5, 1.5), 0.8);
        self.shape_database.insert(BasicShape::Cloud, cloud_equation);

        // شمس - شكل دائري مشع
        let mut sun_equation = base_equation.clone();
        sun_equation.add_sigmoid_component(120.0, ComplexNumber::new(8.0, 6.28), 0.0);
        sun_equation.add_sigmoid_component(80.0, ComplexNumber::new(12.0, 4.0), 0.0);
        sun_equation.add_linear_component(0.2, 0.0, 1.0);
        self.shape_database.insert(BasicShape::Sun, sun_equation);

        // قمر - شكل دائري مع فوهات
        let mut moon_equation = base_equation.clone();
        moon_equation.add_sigmoid_component(90.0, ComplexNumber::new(7.0, 3.14), 0.0);
        moon_equation.add_sigmoid_component(-15.0, ComplexNumber::new(15.0, 0.5), 0.3);
        moon_equation.add_sigmoid_component(-10.0, ComplexNumber::new(20.0, 0.3), -0.5);
        self.shape_database.insert(BasicShape::Moon, moon_equation);

        // نجمة - شكل مدبب متناظر
        let mut star_equation = base_equation.clone();
        star_equation.add_sigmoid_component(100.0, ComplexNumber::new(5.0, 2.51), 0.0);
        star_equation.add_sigmoid_component(-50.0, ComplexNumber::new(10.0, 1.26), 0.628);
        star_equation.add_linear_component(0.1, 0.0, 0.9);
        self.shape_database.insert(BasicShape::Star, star_equation);
    }

    fn initialize_property_transforms(&mut self) {
        // تحويلات الألوان
        self.property_transforms.insert(
            "color_black".to_string(),
            PropertyTransform {
                property_type: "color".to_string(),
                transform_function: TransformFunction::Linear(0.2, 0.0),
                intensity: 1.0,
            }
        );

        // تحويلات الحجم
        self.property_transforms.insert(
            "size_large".to_string(),
            PropertyTransform {
                property_type: "size".to_string(),
                transform_function: TransformFunction::Linear(2.0, 0.0),
                intensity: 1.0,
            }
        );

        // تحويلات الموضع
        self.property_transforms.insert(
            "position_standing".to_string(),
            PropertyTransform {
                property_type: "position".to_string(),
                transform_function: TransformFunction::Sigmoid(1.0, 2.0, 0.0),
                intensity: 0.8,
            }
        );
    }

    pub fn get_shape(&self, shape: BasicShape) -> Option<&GeneralShapeEquation> {
        self.shape_database.get(&shape)
    }

    pub fn apply_property(&self, equation: &GeneralShapeEquation, property: ShapeProperty) -> GeneralShapeEquation {
        let mut modified_equation = equation.clone();

        match property {
            ShapeProperty::Size(factor) => {
                // تطبيق عامل الحجم على جميع المكونات
                for component in &mut modified_equation.sigmoid_components {
                    component.alpha *= factor;
                }
                for component in &mut modified_equation.linear_components {
                    component.weight *= factor;
                }
            },
            ShapeProperty::Color(color) => {
                // تطبيق تحويل اللون (محاكاة)
                if color == "black" {
                    for component in &mut modified_equation.sigmoid_components {
                        component.cutting_factor *= 0.3;
                    }
                }
            },
            ShapeProperty::Position(pos) => {
                // تطبيق تحويل الموضع
                if pos == "sleeping" {
                    for component in &mut modified_equation.sigmoid_components {
                        component.x0 += 1.0;
                        component.alpha *= 0.7;
                    }
                }
            },
            _ => {
                // خصائص أخرى - تطبيق تحويلات عامة
            }
        }

        modified_equation
    }

    pub fn equation_to_image(&mut self, equation: &GeneralShapeEquation) -> RenderedImage {
        // فحص الذاكرة المؤقتة أولاً
        let cache_key = format!("{:?}", equation);
        if let Some(cached_image) = self.shape_cache.get(&cache_key) {
            return cached_image.clone();
        }

        // رسم الصورة
        let rendered_image = self.rendering_engine.render_equation(equation);

        // حفظ في الذاكرة المؤقتة
        self.shape_cache.insert(cache_key, rendered_image.clone());

        rendered_image
    }

    pub fn create_shape_with_properties(&mut self, shape: BasicShape, properties: Vec<ShapeProperty>) -> Option<RenderedImage> {
        if let Some(base_equation) = self.get_shape(shape).cloned() {
            let mut modified_equation = base_equation;

            // تطبيق جميع الخصائص
            for property in properties {
                modified_equation = self.apply_property(&modified_equation, property);
            }

            // تحويل إلى صورة
            Some(self.equation_to_image(&modified_equation))
        } else {
            None
        }
    }
}
