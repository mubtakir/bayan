# مرجع API - لغة البيان (AlBayan)

## جدول المحتويات

1. [الأنواع الأساسية](#الأنواع-الأساسية)
2. [مكتبة الذكاء الاصطناعي](#مكتبة-الذكاء-الاصطناعي)
3. [البرمجة المنطقية](#البرمجة-المنطقية)
4. [البرمجة غير المتزامنة](#البرمجة-غير-المتزامنة)
5. [نظام الوحدات](#نظام-الوحدات)
6. [أدوات التطوير](#أدوات-التطوير)

## الأنواع الأساسية

### int
النوع العددي الصحيح 64-bit.

```albayan
let number: int = 42;
let negative: int = -10;
let hex: int = 0xFF;
let binary: int = 0b1010;
```

**الطرق:**
- `abs() -> int` - القيمة المطلقة
- `pow(exponent: int) -> int` - الأس
- `sqrt() -> float` - الجذر التربيعي
- `to_string() -> string` - تحويل إلى نص

### float
النوع العددي العشري 64-bit.

```albayan
let pi: float = 3.14159;
let scientific: float = 1.23e-4;
```

**الطرق:**
- `abs() -> float` - القيمة المطلقة
- `ceil() -> int` - التقريب لأعلى
- `floor() -> int` - التقريب لأسفل
- `round() -> int` - التقريب العادي
- `sin() -> float` - جيب الزاوية
- `cos() -> float` - جيب التمام
- `tan() -> float` - ظل الزاوية

### string
نوع النصوص مع دعم Unicode.

```albayan
let text: string = "مرحباً بالعالم";
let empty: string = "";
```

**الطرق:**
- `len() -> int` - طول النص
- `is_empty() -> bool` - فحص إذا كان فارغاً
- `contains(substring: string) -> bool` - فحص وجود نص فرعي
- `starts_with(prefix: string) -> bool` - فحص البداية
- `ends_with(suffix: string) -> bool` - فحص النهاية
- `split(delimiter: string) -> [string]` - تقسيم النص
- `replace(old: string, new: string) -> string` - استبدال النص
- `to_upper() -> string` - تحويل لأحرف كبيرة
- `to_lower() -> string` - تحويل لأحرف صغيرة
- `trim() -> string` - إزالة المسافات من الأطراف

### Vec<T>
مصفوفة ديناميكية.

```albayan
let numbers: Vec<int> = Vec::new();
let initialized: Vec<int> = [1, 2, 3, 4, 5];
```

**الطرق:**
- `new() -> Vec<T>` - إنشاء مصفوفة فارغة
- `with_capacity(capacity: int) -> Vec<T>` - إنشاء مع سعة محددة
- `push(item: T)` - إضافة عنصر
- `pop() -> Option<T>` - إزالة آخر عنصر
- `len() -> int` - عدد العناصر
- `is_empty() -> bool` - فحص إذا كانت فارغة
- `get(index: int) -> Option<T>` - الحصول على عنصر
- `insert(index: int, item: T)` - إدراج عنصر
- `remove(index: int) -> T` - إزالة عنصر
- `clear()` - مسح جميع العناصر

### Map<K, V>
خريطة (قاموس) للمفاتيح والقيم.

```albayan
let scores: Map<string, int> = Map::new();
scores.insert("أحمد", 95);
```

**الطرق:**
- `new() -> Map<K, V>` - إنشاء خريطة فارغة
- `insert(key: K, value: V) -> Option<V>` - إدراج مفتاح وقيمة
- `get(key: K) -> Option<V>` - الحصول على قيمة
- `remove(key: K) -> Option<V>` - إزالة مفتاح
- `contains_key(key: K) -> bool` - فحص وجود مفتاح
- `keys() -> Iterator<K>` - جميع المفاتيح
- `values() -> Iterator<V>` - جميع القيم
- `len() -> int` - عدد العناصر
- `is_empty() -> bool` - فحص إذا كانت فارغة

## مكتبة الذكاء الاصطناعي

### Tensor
فئة التنسورات للعمليات الرياضية المتقدمة.

```albayan
use ai::Tensor;

let tensor = Tensor::new([1.0, 2.0, 3.0, 4.0], [2, 2]);
```

**الطرق:**
- `new(data: [float], shape: [int]) -> Result<Tensor>` - إنشاء تنسور جديد
- `zeros(shape: [int]) -> Tensor` - تنسور مملوء بالأصفار
- `ones(shape: [int]) -> Tensor` - تنسور مملوء بالواحدات
- `random(shape: [int]) -> Tensor` - تنسور بقيم عشوائية
- `shape() -> [int]` - شكل التنسور
- `data() -> [float]` - البيانات الخام
- `add(other: Tensor) -> Result<Tensor>` - الجمع
- `mul(other: Tensor) -> Result<Tensor>` - الضرب العنصري
- `matmul(other: Tensor) -> Result<Tensor>` - ضرب المصفوفات
- `relu() -> Tensor` - دالة تفعيل ReLU
- `sigmoid() -> Tensor` - دالة تفعيل Sigmoid
- `softmax(dim: int) -> Result<Tensor>` - دالة Softmax
- `reshape(new_shape: [int]) -> Result<Tensor>` - إعادة تشكيل

### NeuralNetwork
فئة الشبكات العصبية.

```albayan
use ai::NeuralNetwork;

let mut network = NeuralNetwork::new();
```

**الطرق:**
- `new() -> NeuralNetwork` - إنشاء شبكة جديدة
- `add_layer(layer: Box<Layer>)` - إضافة طبقة
- `set_loss_function(loss: LossFunction)` - تعيين دالة الخسارة
- `set_optimizer(optimizer: Optimizer)` - تعيين محسن
- `forward(input: Tensor) -> Result<Tensor>` - التمرير الأمامي
- `train(inputs: [Tensor], targets: [Tensor], epochs: int) -> Result<()>` - التدريب
- `save(path: Path) -> Result<()>` - حفظ النموذج
- `load(path: Path) -> Result<NeuralNetwork>` - تحميل النموذج

### Dense
طبقة مكتملة الاتصال.

```albayan
use ai::neural_networks::Dense;

let layer = Dense::new(input_size: 784, output_size: 128);
```

**الطرق:**
- `new(input_size: int, output_size: int) -> Dense` - إنشاء طبقة جديدة

### Conv2D
طبقة التطبيق التطبيقي ثنائي الأبعاد.

```albayan
use ai::neural_networks::Conv2D;

let layer = Conv2D::new(
    in_channels: 3,
    out_channels: 32,
    kernel_size: (3, 3),
    stride: (1, 1),
    padding: (1, 1)
);
```

### LSTM
طبقة الذاكرة طويلة المدى.

```albayan
use ai::neural_networks::LSTM;

let layer = LSTM::new(input_size: 100, hidden_size: 128);
```

### Tokenizer
محلل النصوص لمعالجة اللغة الطبيعية.

```albayan
use ai::natural_language::Tokenizer;

let mut tokenizer = Tokenizer::new(max_length: 512);
```

**الطرق:**
- `new(max_length: int) -> Tokenizer` - إنشاء محلل جديد
- `build_vocab(texts: [string], min_freq: int)` - بناء المفردات
- `encode(text: string) -> [int]` - ترميز النص
- `decode(token_ids: [int]) -> string` - فك ترميز الرموز
- `vocab_size() -> int` - حجم المفردات

### SentimentAnalyzer
محلل المشاعر.

```albayan
use ai::natural_language::SentimentAnalyzer;

let mut analyzer = SentimentAnalyzer::new(
    vocab_size: 5000,
    embedding_dim: 100,
    hidden_dim: 64,
    max_length: 128
);
```

**الطرق:**
- `new(vocab_size: int, embedding_dim: int, hidden_dim: int, max_length: int) -> SentimentAnalyzer`
- `train(texts: [string], sentiments: [int], epochs: int) -> Result<()>` - التدريب
- `analyze(text: string) -> Result<(string, float)>` - تحليل المشاعر

## البرمجة المنطقية

### LogicEngine
محرك البرمجة المنطقية.

```albayan
use logic::LogicEngine;

let mut engine = LogicEngine::new();
```

**الطرق:**
- `new() -> LogicEngine` - إنشاء محرك جديد
- `assert_fact(fact: string) -> Result<()>` - إضافة حقيقة
- `assert_rule(rule: string) -> Result<()>` - إضافة قاعدة
- `solve_query(query: string) -> Result<[Solution]>` - حل استعلام
- `retract_fact(fact: string) -> Result<()>` - إزالة حقيقة
- `clear_knowledge_base()` - مسح قاعدة المعرفة

### Relation
تعريف العلاقات المنطقية.

```albayan
relation parent(string, string);
relation age(string, int);
```

### Rule
تعريف القواعد المنطقية.

```albayan
rule grandparent(GP, GC) :- parent(GP, P), parent(P, GC);
rule adult(Person) :- age(Person, Age), Age >= 18;
```

### Query
الاستعلامات المنطقية.

```albayan
query parent("أحمد", X);           // البحث عن أطفال أحمد
query grandparent(GP, "علي");      // البحث عن أجداد علي
query adult(Person);               // البحث عن البالغين
```

## البرمجة غير المتزامنة

### Future<T>
نوع للعمليات غير المتزامنة.

```albayan
use async::Future;

async fn async_function() -> int {
    return 42;
}
```

### Channel<T>
قناة للتواصل بين المهام.

```albayan
use async::Channel;

let (sender, receiver) = Channel::new(buffer_size: 10);
```

**الطرق:**
- `new(buffer_size: int) -> (Sender<T>, Receiver<T>)` - إنشاء قناة
- `send(item: T) -> Future<Result<()>>` - إرسال عنصر
- `recv() -> Future<Option<T>>` - استقبال عنصر

### Mutex<T>
قفل للوصول الآمن للبيانات المشتركة.

```albayan
use async::Mutex;

let mutex = Mutex::new(data: 0);
```

**الطرق:**
- `new(data: T) -> Mutex<T>` - إنشاء قفل جديد
- `lock() -> Future<MutexGuard<T>>` - قفل البيانات

### spawn
تشغيل مهمة غير متزامنة.

```albayan
use async::spawn;

let handle = spawn(async {
    // كود المهمة
    return result;
});
```

## نظام الوحدات

### Module
تعريف الوحدات.

```albayan
module my_module;

export fn public_function() -> int {
    return private_function();
}

fn private_function() -> int {
    return 42;
}
```

### import
استيراد الوحدات.

```albayan
import my_module;
import my_module::{public_function};
import std::collections::{Vec, Map};
```

### export
تصدير الرموز من الوحدة.

```albayan
export fn function_name() -> int;
export struct StructName;
export enum EnumName;
```

## أدوات التطوير

### Compiler
مترجم اللغة.

```albayan
use compiler::Compiler;

let compiler = Compiler::new(options);
let result = compiler.compile(source_code);
```

**الطرق:**
- `new(options: CompilerOptions) -> Compiler` - إنشاء مترجم
- `compile(source: string) -> Result<CompiledProgram>` - تجميع الكود
- `check_syntax(source: string) -> Result<()>` - فحص الصيغة

### Debugger
مصحح الأخطاء.

```albayan
use tools::Debugger;

let mut debugger = Debugger::new();
debugger.set_breakpoint(file: "main.ab", line: 10);
```

**الطرق:**
- `new() -> Debugger` - إنشاء مصحح جديد
- `set_breakpoint(file: string, line: int)` - تعيين نقطة توقف
- `remove_breakpoint(file: string, line: int)` - إزالة نقطة توقف
- `step_over()` - خطوة واحدة
- `step_into()` - دخول الدالة
- `continue_execution()` - متابعة التنفيذ

### Profiler
محلل الأداء.

```albayan
use tools::Profiler;

let mut profiler = Profiler::new();
profiler.start_session("session_name");
```

**الطرق:**
- `new() -> Profiler` - إنشاء محلل جديد
- `start_session(name: string) -> Result<()>` - بدء جلسة
- `end_session() -> Result<ProfileData>` - إنهاء جلسة
- `start_function_timing(name: string)` - بدء قياس دالة
- `end_function_timing(name: string)` - إنهاء قياس دالة

### Formatter
منسق الكود.

```albayan
use tools::Formatter;

let mut formatter = Formatter::new();
let formatted = formatter.format(source_code);
```

**الطرق:**
- `new() -> Formatter` - إنشاء منسق جديد
- `format(source: string) -> Result<string>` - تنسيق الكود

### Linter
فاحص الكود.

```albayan
use tools::Linter;

let mut linter = Linter::new();
let issues = linter.lint(source_code);
```

**الطرق:**
- `new() -> Linter` - إنشاء فاحص جديد
- `lint(source: string) -> Result<[LintIssue]>` - فحص الكود

هذا المرجع يغطي الواجهات البرمجية الرئيسية للغة البيان. للمزيد من التفاصيل والأمثلة، راجع [دليل المطور](developer_guide.md).
