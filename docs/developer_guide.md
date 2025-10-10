# دليل المطور - لغة البيان (AlBayan)

## جدول المحتويات

1. [مقدمة](#مقدمة)
2. [أساسيات اللغة](#أساسيات-اللغة)
3. [البرمجة التقليدية](#البرمجة-التقليدية)
4. [البرمجة المنطقية](#البرمجة-المنطقية)
5. [الذكاء الاصطناعي](#الذكاء-الاصطناعي)
6. [نظام الوحدات](#نظام-الوحدات)
7. [البرمجة غير المتزامنة](#البرمجة-غير-المتزامنة)
8. [إدارة الذاكرة](#إدارة-الذاكرة)
9. [أدوات التطوير](#أدوات-التطوير)
10. [أمثلة متقدمة](#أمثلة-متقدمة)

## مقدمة

لغة البيان (AlBayan) هي لغة برمجة حديثة تهدف إلى دمج ثلاثة نماذج برمجية رئيسية:

- **البرمجة التقليدية**: للتطبيقات العامة والخوارزميات
- **البرمجة المنطقية**: للذكاء الاصطناعي الرمزي والاستدلال
- **التعلم الآلي**: للذكاء الاصطناعي الحديث والشبكات العصبية

## أساسيات اللغة

### الصيغة العامة

```albayan
// تعليق سطر واحد
/* تعليق متعدد
   الأسطر */

// تعريف دالة
fn function_name(param1: type1, param2: type2) -> return_type {
    // جسم الدالة
    return value;
}

// نقطة الدخول الرئيسية
fn main() -> int {
    return 0;
}
```

### الأنواع الأساسية

```albayan
// الأنواع العددية
let integer: int = 42;
let floating: float = 3.14;
let boolean: bool = true;

// النصوص
let text: string = "مرحباً بالعالم";
let character: char = 'أ';

// المصفوفات
let numbers: [int] = [1, 2, 3, 4, 5];
let matrix: [[int]] = [[1, 2], [3, 4]];

// الخرائط
let scores: Map<string, int> = {
    "أحمد": 95,
    "فاطمة": 87,
    "محمد": 92
};
```

### المتغيرات والثوابت

```albayan
// متغير قابل للتغيير
let mut counter: int = 0;
counter = counter + 1;

// ثابت غير قابل للتغيير
let PI: float = 3.14159;

// استنتاج النوع التلقائي
let auto_int = 42;        // int
let auto_float = 3.14;    // float
let auto_string = "نص";   // string
```

## البرمجة التقليدية

### الدوال

```albayan
// دالة بسيطة
fn add(a: int, b: int) -> int {
    return a + b;
}

// دالة مع معاملات اختيارية
fn greet(name: string, title: string = "السيد") -> string {
    return title + " " + name;
}

// دالة مع عدد متغير من المعاملات
fn sum(numbers: ...int) -> int {
    let total = 0;
    for num in numbers {
        total = total + num;
    }
    return total;
}

// دالة عليا (Higher-order function)
fn apply_operation(x: int, y: int, op: fn(int, int) -> int) -> int {
    return op(x, y);
}

fn main() -> int {
    let result1 = add(5, 3);
    let result2 = greet("أحمد");
    let result3 = sum(1, 2, 3, 4, 5);
    let result4 = apply_operation(10, 5, add);
    
    return 0;
}
```

### الهياكل والتعدادات

```albayan
// تعريف هيكل
struct Person {
    name: string,
    age: int,
    email: string,
}

// تنفيذ الطرق
impl Person {
    fn new(name: string, age: int, email: string) -> Person {
        return Person {
            name: name,
            age: age,
            email: email,
        };
    }
    
    fn is_adult(&self) -> bool {
        return self.age >= 18;
    }
    
    fn celebrate_birthday(&mut self) {
        self.age = self.age + 1;
    }
}

// تعداد
enum Status {
    Active,
    Inactive,
    Pending,
    Suspended(string), // مع قيمة مرتبطة
}

// استخدام الهياكل
fn main() -> int {
    let mut person = Person::new("سارة", 25, "sara@example.com");
    
    if person.is_adult() {
        println("شخص بالغ");
    }
    
    person.celebrate_birthday();
    
    let status = Status::Suspended("انتهاك القوانين");
    
    match status {
        Status::Active => println("نشط"),
        Status::Inactive => println("غير نشط"),
        Status::Pending => println("في الانتظار"),
        Status::Suspended(reason) => println("معلق: " + reason),
    }
    
    return 0;
}
```

### التحكم في التدفق

```albayan
fn control_flow_examples() -> int {
    // الشروط
    let x = 10;
    if x > 5 {
        println("x أكبر من 5");
    } else if x == 5 {
        println("x يساوي 5");
    } else {
        println("x أصغر من 5");
    }
    
    // الحلقات
    for i in 0..10 {
        println("العدد: " + i);
    }
    
    let numbers = [1, 2, 3, 4, 5];
    for num in numbers {
        if num % 2 == 0 {
            continue; // تخطي الأرقام الزوجية
        }
        println("رقم فردي: " + num);
    }
    
    let mut counter = 0;
    while counter < 5 {
        println("العداد: " + counter);
        counter = counter + 1;
    }
    
    // حلقة لا نهائية مع كسر
    loop {
        counter = counter + 1;
        if counter > 10 {
            break;
        }
    }
    
    return 0;
}
```

## البرمجة المنطقية

### العلاقات والحقائق

```albayan
// تعريف العلاقات
relation parent(string, string);
relation sibling(string, string);
relation ancestor(string, string);

// إضافة حقائق
fact parent("أحمد", "فاطمة").
fact parent("أحمد", "محمد").
fact parent("فاطمة", "علي").
fact parent("محمد", "سارة").

// تعريف القواعد
rule sibling(X, Y) :- 
    parent(P, X), 
    parent(P, Y), 
    X != Y.

rule ancestor(X, Y) :- parent(X, Y).
rule ancestor(X, Z) :- parent(X, Y), ancestor(Y, Z).

fn logic_programming_example() -> int {
    // استعلامات بسيطة
    query parent("أحمد", "فاطمة");  // true
    query sibling("فاطمة", "محمد"); // true
    query ancestor("أحمد", "علي");  // true
    
    // استعلامات مع متغيرات
    query parent("أحمد", X);        // X = "فاطمة", "محمد"
    query ancestor(X, "سارة");      // X = "محمد", "أحمد"
    
    return 0;
}
```

### الاستعلامات المعقدة

```albayan
relation student(string, string, int); // اسم، تخصص، درجة
relation course(string, string, int);  // اسم المقرر، المدرس، ساعات

fact student("أحمد", "هندسة", 85).
fact student("فاطمة", "طب", 92).
fact student("محمد", "هندسة", 78).

fact course("رياضيات", "د. علي", 3).
fact course("فيزياء", "د. سارة", 4).

// قاعدة معقدة
rule excellent_student(Name) :- 
    student(Name, _, Grade), 
    Grade >= 90.

rule engineering_student(Name) :- 
    student(Name, "هندسة", _).

rule high_achieving_engineer(Name) :- 
    engineering_student(Name),
    student(Name, _, Grade),
    Grade >= 80.

fn complex_queries() -> int {
    // العثور على الطلاب المتفوقين
    query excellent_student(X);
    
    // العثور على المهندسين المتفوقين
    query high_achieving_engineer(X);
    
    return 0;
}
```

## الذكاء الاصطناعي

### التنسورات والعمليات الأساسية

```albayan
use ai::*;

fn tensor_operations() -> int {
    // إنشاء التنسورات
    let tensor1 = Tensor::new([1.0, 2.0, 3.0, 4.0], [2, 2]);
    let tensor2 = Tensor::zeros([2, 2]);
    let tensor3 = Tensor::ones([2, 2]);
    let tensor4 = Tensor::random([2, 2]);
    
    // العمليات الأساسية
    let sum = tensor1.add(tensor3);
    let product = tensor1.mul(tensor2);
    let matrix_mult = tensor1.matmul(tensor3);
    
    // دوال التفعيل
    let relu_result = tensor1.relu();
    let sigmoid_result = tensor1.sigmoid();
    let softmax_result = tensor1.softmax(1);
    
    // إعادة التشكيل
    let reshaped = tensor1.reshape([4, 1]);
    
    println("مجموع التنسورات: " + sum);
    
    return 0;
}
```

### الشبكات العصبية

```albayan
use ai::neural_networks::*;

fn neural_network_example() -> int {
    // إنشاء شبكة عصبية
    let mut network = NeuralNetwork::new();
    
    // إضافة الطبقات
    network.add_layer(Dense::new(784, 128));  // طبقة الدخل
    network.add_layer(BatchNorm::new(128));   // تطبيع الدفعة
    network.add_layer(Dropout::new(0.3));     // إسقاط عشوائي
    network.add_layer(Dense::new(128, 64));   // طبقة مخفية
    network.add_layer(Dense::new(64, 10));    // طبقة الإخراج
    
    // إعداد النموذج
    network.set_loss_function(LossFunction::CrossEntropy);
    network.set_optimizer(Optimizer::Adam {
        learning_rate: 0.001,
        beta1: 0.9,
        beta2: 0.999,
    });
    
    // بيانات التدريب (مثال)
    let training_inputs = [
        Tensor::random([1, 784]),
        Tensor::random([1, 784]),
        // ... المزيد من البيانات
    ];
    
    let training_targets = [
        Tensor::new([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], [1, 10]),
        Tensor::new([0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], [1, 10]),
        // ... المزيد من الأهداف
    ];
    
    // تدريب النموذج
    network.train(training_inputs, training_targets, 100);
    
    // التنبؤ
    let test_input = Tensor::random([1, 784]);
    let prediction = network.forward(test_input);
    
    println("التنبؤ: " + prediction);
    
    return 0;
}
```

### معالجة اللغة الطبيعية

```albayan
use ai::natural_language::*;

fn nlp_example() -> int {
    // إنشاء محلل المشاعر
    let mut sentiment_analyzer = SentimentAnalyzer::new(5000, 100, 64, 128);
    
    // بيانات التدريب
    let training_texts = [
        "أحب هذا المنتج كثيراً، إنه رائع!",
        "منتج سيء جداً، لا أنصح به",
        "منتج عادي، لا بأس به",
        "ممتاز! أفضل منتج اشتريته",
        "مخيب للآمال تماماً"
    ];
    
    let sentiments = [2, 0, 1, 2, 0]; // 0=سلبي، 1=محايد، 2=إيجابي
    
    // تدريب النموذج
    sentiment_analyzer.train(training_texts, sentiments, 50);
    
    // تحليل نص جديد
    let test_text = "هذا المنتج جيد جداً";
    let (sentiment, confidence) = sentiment_analyzer.analyze(test_text);
    
    println("المشاعر: " + sentiment + " (الثقة: " + confidence + ")");
    
    // استخراج الكيانات المسماة
    let ner_labels = ["PERSON", "LOCATION", "ORGANIZATION", "O"];
    let mut ner_model = NERModel::new(5000, 100, 64, ner_labels, 128);
    
    let entities = ner_model.extract_entities("أحمد يعمل في شركة مايكروسوفت في الرياض");
    
    for (entity_type, text, start, end) in entities {
        println("كيان: " + text + " (" + entity_type + ")");
    }
    
    return 0;
}
```

## نظام الوحدات

### إنشاء وحدة

```albayan
// ملف: math_utils.ab
module math_utils;

export fn factorial(n: int) -> int {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

export fn gcd(a: int, b: int) -> int {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

fn private_helper() -> int {
    // دالة خاصة بالوحدة
    return 42;
}
```

### استيراد واستخدام الوحدات

```albayan
// ملف: main.ab
import math_utils;
import math_utils::{factorial, gcd}; // استيراد انتقائي

fn main() -> int {
    // استخدام الوحدة المستوردة
    let fact5 = math_utils::factorial(5);
    let result_gcd = gcd(48, 18);
    
    println("5! = " + fact5);
    println("gcd(48, 18) = " + result_gcd);
    
    return 0;
}
```

### إدارة التبعيات

```toml
# ملف: albayan.toml
[package]
name = "my_project"
version = "0.1.0"
authors = ["المطور <developer@example.com>"]

[dependencies]
math_utils = "1.0"
ai_toolkit = { version = "2.1", features = ["neural-networks", "nlp"] }
web_framework = { git = "https://github.com/example/web-framework.git" }
```

## البرمجة غير المتزامنة

### الدوال غير المتزامنة

```albayan
use async::*;

async fn fetch_data(url: string) -> Result<string, string> {
    // محاكاة طلب HTTP
    await sleep(1000); // انتظار ثانية واحدة
    
    if url.starts_with("https://") {
        return Ok("بيانات من " + url);
    } else {
        return Err("رابط غير صحيح");
    }
}

async fn process_multiple_requests() -> int {
    let urls = [
        "https://api.example.com/data1",
        "https://api.example.com/data2",
        "https://api.example.com/data3"
    ];
    
    // معالجة متوازية
    let futures = [];
    for url in urls {
        futures.push(fetch_data(url));
    }
    
    let results = await join_all(futures);
    
    for result in results {
        match result {
            Ok(data) => println("تم الحصول على: " + data),
            Err(error) => println("خطأ: " + error),
        }
    }
    
    return 0;
}
```

### القنوات والتزامن

```albayan
use async::{Channel, Mutex, Arc};

async fn producer_consumer_example() -> int {
    let (sender, receiver) = Channel::new(10);
    
    // المنتج
    spawn(async move {
        for i in 0..5 {
            await sender.send("رسالة " + i);
            await sleep(500);
        }
    });
    
    // المستهلك
    spawn(async move {
        while let Some(message) = await receiver.recv() {
            println("تم استلام: " + message);
        }
    });
    
    await sleep(3000); // انتظار انتهاء المعالجة
    
    return 0;
}

async fn shared_state_example() -> int {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = [];
    
    for i in 0..10 {
        let counter_clone = Arc::clone(counter);
        let handle = spawn(async move {
            let mut num = await counter_clone.lock();
            *num += 1;
            println("المهمة " + i + " زادت العداد إلى " + *num);
        });
        handles.push(handle);
    }
    
    // انتظار انتهاء جميع المهام
    for handle in handles {
        await handle;
    }
    
    let final_count = await counter.lock();
    println("العدد النهائي: " + *final_count);
    
    return 0;
}
```

## إدارة الذاكرة

### نظام الملكية

```albayan
fn ownership_example() -> int {
    // الملكية الأساسية
    let s1 = "مرحباً";
    let s2 = s1; // نقل الملكية
    // s1 لم تعد صالحة هنا
    
    println(s2); // صحيح
    // println(s1); // خطأ! s1 لم تعد مملوكة
    
    // الاستعارة
    let s3 = "عالم";
    print_string(&s3); // استعارة للقراءة
    println(s3); // s3 ما زالت صالحة
    
    let mut s4 = "قابل للتغيير";
    modify_string(&mut s4); // استعارة للتعديل
    println(s4); // s4 تم تعديلها
    
    return 0;
}

fn print_string(s: &string) {
    println("النص: " + s);
}

fn modify_string(s: &mut string) {
    *s = *s + " - تم التعديل";
}
```

### إدارة الذاكرة المتقدمة

```albayan
use memory::{Rc, RefCell, Weak};

struct Node {
    value: int,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Weak<RefCell<Node>>,
}

fn tree_example() -> int {
    let root = Rc::new(RefCell::new(Node {
        value: 1,
        children: Vec::new(),
        parent: Weak::new(),
    }));
    
    let child1 = Rc::new(RefCell::new(Node {
        value: 2,
        children: Vec::new(),
        parent: Rc::downgrade(&root),
    }));
    
    let child2 = Rc::new(RefCell::new(Node {
        value: 3,
        children: Vec::new(),
        parent: Rc::downgrade(&root),
    }));
    
    root.borrow_mut().children.push(Rc::clone(&child1));
    root.borrow_mut().children.push(Rc::clone(&child2));
    
    // طباعة الشجرة
    print_tree(&root, 0);
    
    return 0;
}

fn print_tree(node: &Rc<RefCell<Node>>, depth: int) {
    let indent = "  ".repeat(depth);
    println(indent + "العقدة: " + node.borrow().value);
    
    for child in &node.borrow().children {
        print_tree(child, depth + 1);
    }
}
```

## أمثلة تطبيقية متقدمة

### 1. البرمجة الكائنية التوجه

#### مثال: نظام الحيوانات الأليفة

```albayan
// فئة أساسية للحيوانات
class Animal {
    name: string,
    age: int,
    species: string,
}

impl Animal {
    fn new(name: string, age: int, species: string) -> Animal {
        return Animal { name, age, species };
    }

    // طريقة افتراضية للصوت
    virtual fn make_sound(&self) -> string {
        return "صوت حيوان";
    }
}

// فئة الكلب ترث من الحيوان
class Dog extends Animal {
    breed: string,
    is_trained: bool,
}

impl Dog {
    override fn make_sound(&self) -> string {
        return "هاو هاو!";
    }

    fn train(&mut self) {
        self.is_trained = true;
    }
}

// واجهة للحيوانات الأليفة
interface Pet {
    fn play(&self) -> string;
    fn feed(&mut self);
}

impl Pet for Dog {
    fn play(&self) -> string {
        return self.name + " يلعب!";
    }

    fn feed(&mut self) {
        println("إطعام " + self.name);
    }
}
```

### 2. نظام إدارة مكتبة

```albayan
// فئة أساسية للعناصر
abstract class LibraryItem {
    id: string,
    title: string,
    author: string,
    is_available: bool,
}

impl LibraryItem {
    abstract fn get_type(&self) -> string;
    abstract fn get_loan_period(&self) -> int;

    fn borrow(&mut self) -> bool {
        if self.is_available {
            self.is_available = false;
            return true;
        }
        return false;
    }
}

class Book extends LibraryItem {
    pages: int,
    isbn: string,
}

impl Book {
    override fn get_type(&self) -> string {
        return "كتاب";
    }

    override fn get_loan_period(&self) -> int {
        return 14; // أسبوعان
    }
}
```

### 3. محرك لعبة بسيط

```albayan
// فئة أساسية للكائنات في اللعبة
abstract class GameObject {
    position: Point2D,
    velocity: Point2D,
    active: bool,
}

impl GameObject {
    abstract fn get_type(&self) -> string;

    fn update(&mut self, delta_time: float) {
        if self.active {
            self.position.x += self.velocity.x * delta_time;
            self.position.y += self.velocity.y * delta_time;
        }
    }
}

class Player extends GameObject {
    health: int,
    score: int,
}

impl Player {
    override fn get_type(&self) -> string {
        return "لاعب";
    }

    fn take_damage(&mut self, damage: int) {
        self.health -= damage;
        if self.health <= 0 {
            self.active = false;
        }
    }
}
```

### 4. خادم ويب بسيط

```albayan
// هيكل طلب HTTP
struct HttpRequest {
    method: HttpMethod,
    path: string,
    headers: HashMap<string, string>,
    body: string,
}

// هيكل استجابة HTTP
struct HttpResponse {
    status_code: int,
    headers: HashMap<string, string>,
    body: string,
}

// فئة خادم الويب
class WebServer {
    address: string,
    port: int,
    routes: HashMap<string, RouteHandler>,
}

impl WebServer {
    async fn start(&self) -> Result<(), string> {
        println("بدء تشغيل الخادم على " + self.address + ":" + self.port);
        // منطق بدء الخادم
        Ok(())
    }

    async fn handle_request(&self, request: HttpRequest) -> HttpResponse {
        // معالجة الطلبات
        return HttpResponse::ok();
    }
}
```

### 5. نظام خبير للتشخيص

```albayan
// تعريف العلاقات المنطقية
relation Symptom(string, string);
relation Disease(string, string);
relation Treatment(string, string);

// قواعد التشخيص
rule Disease(Patient, "نزلة برد") :-
    Symptom(Patient, "سعال"),
    Symptom(Patient, "حمى خفيفة");

rule Treatment("نزلة برد", "راحة وسوائل");

class MedicalExpertSystem {
    neural_network: NeuralNetwork,
    knowledge_base: LogicEngine,
}

impl MedicalExpertSystem {
    fn diagnose(&self, patient: &Patient) -> Vec<Diagnosis> {
        // دمج البرمجة المنطقية مع الذكاء الاصطناعي
        let logic_results = self.logic_diagnosis(patient);
        let ai_results = self.ai_diagnosis(patient);

        return self.combine_results(logic_results, ai_results);
    }
}
```

### 6. البرمجة الوظيفية

```albayan
// دوال وظيفية عليا
fn map<T, U>(list: Vec<T>, func: fn(T) -> U) -> Vec<U> {
    let mut result = Vec::new();
    for item in list {
        result.push(func(item));
    }
    return result;
}

fn filter<T>(list: Vec<T>, predicate: fn(&T) -> bool) -> Vec<T> {
    let mut result = Vec::new();
    for item in list {
        if predicate(&item) {
            result.push(item);
        }
    }
    return result;
}

// استخدام الدوال الوظيفية
fn process_students(students: Vec<Student>) -> Vec<Student> {
    return students
        |> filter(|s| s.average_grade() >= 3.0)
        |> map(|s| s.with_honor_status(true));
}
```

## الملفات المرجعية

للحصول على أمثلة كاملة وقابلة للتشغيل، راجع:

- `examples/oop_basics.ab` - البرمجة الكائنية الأساسية
- `examples/library_system.ab` - نظام إدارة مكتبة متكامل
- `examples/game_engine.ab` - محرك لعبة بسيط
- `examples/web_server.ab` - خادم ويب غير متزامن
- `examples/expert_system.ab` - نظام خبير للتشخيص الطبي
- `examples/functional_programming.ab` - البرمجة الوظيفية ومعالجة البيانات

هذا الدليل يغطي الأساسيات والميزات المتقدمة للغة البيان. للمزيد من التفاصيل، راجع [مرجع API](api_reference.md) والأمثلة التطبيقية في مجلد `examples/`.
