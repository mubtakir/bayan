# دليل المستخدم الشامل للبيان (AlBayan)

هذا الدليل يعرّفك على لغة البيان بوصفها إطار حوسبة دلالي يدمج البرمجة الرمزية/المنطقية/العددية/البصرية ومعالجة اللغة الطبيعية والذكاء الاصطناعي التقليدي والرياضي المبتكر.

- لمن هذا الدليل: للمبتدئ والخبرة على حد سواء.
- الهدف: أن تكتب برامج عملية مباشرة، وتستفيد من المحركات والمكتبات المدمجة، وتفهم الأسس الفلسفية والتقنية للبيان باختصار عملي.

## فهرس المحتويات
1. التثبيت على لينكس/ماك/ويندوز
2. نظرة عليا وبنية المشروع
3. أساسيات اللغة: الأنواع والمتغيرات والتعابير
4. المجموعات: القوائم والمصفوفات والفهارس
5. الدوال والوحدات والموديولات
6. التحكم في التدفق: الشرط والحلقات والمطابقة
7. البنى المعطيات العليا: Struct/Enum/Class/Interface/Trait/Impl/dyn
8. البرمجة المنطقية: Relation/Fact/Rule ولبنات الاستنتاج
9. محرك اللغة الطبيعية: كتلة semantic
10. الذكاء اللغوي (سيماء الحروف) من std/linguistic_intelligence
11. الذكاء الاصطناعي التقليدي (PyTorch) عبر std/torch
12. الذكاء الرياضي المبتكر والمعادلات اللغوية
13. القيود الحالية ونصائح الاستخدام
14. مراجع وأمثلة إضافية

---
## التثبيت على لينكس/ماك/ويندوز

المتطلبات الأساسية:
- Rust 1.70+ (يوصى باستخدام rustup)
- LLVM 17+ (أو 18+) مع أدوات التطوير

لينكس (Debian/Ubuntu كمثال):
```bash
sudo apt update
sudo apt install -y build-essential cmake llvm clang
# اختياري: llvm-18/clang-18 إن كانت المتوفرة تحمل رقم إصدار محدد
# sudo apt install -y llvm-18 clang-18

git clone https://github.com/your-username/albayan.git
cd albayan
cargo build --release
cargo install --path .
```

macOS (Homebrew):
```bash
# تثبيت LLVM
brew install llvm
# إضافة llvm إلى المسار (جلسة مؤقتة)
export PATH="$(brew --prefix llvm)/bin:$PATH"

# بناء وتثبيت
git clone https://github.com/your-username/albayan.git
cd albayan
cargo build --release
cargo install --path .
```

ويندوز (MSVC):
1) ثبّت Visual Studio Build Tools مع مكونات C++.
2) ثبّت Rust (MSVC toolchain) عبر https://rustup.rs
3) حمّل LLVM لنظام ويندوز وأضف مجلد bin إلى PATH.
4) افتح Developer Command Prompt أو PowerShell مع بيئة MSVC ثم:
```powershell
git clone https://github.com/your-username/albayan.git
cd albayan
cargo build --release
cargo install --path .
```

التحقق والتشغيل:
```bash
albayan --help
albayan run hello.ab
```

---


## 1) نظرة عليا وبنية المشروع
- التنفيذ بلغة Rust مع LLVM Backend.
- خط الأنابيب: Lexer → Parser → Semantic Analysis (أنواع/ملكية) → IR → LLVM.
- الملفات المهمة:
  - src/lexer, src/parser, src/semantic, src/runtime, albayan_runtime/ (محركات)
  - std/… (مكتبات قياسية: الذكاء اللغوي، PyTorch، …)
  - docs/AGENT_INTELLIGENT_MODEL_BRIEFING.md (موجز معمّق للوكيل الذكي)
  - tests/programs/*.ab (أمثلة سريعة)

مثال هيكل برنامج بسيط:
```albayan
fn main() -> int {
    print("مرحبا بالبيان!");
    return 0;
}
```
ملاحظة: print يدعم النصوص string مباشرة؛ لطباعة أنواع أخرى قم بتحويلها إلى string.

---

## 2) أساسيات اللغة: الأنواع والمتغيرات والتعابير
- الأنواع الأولية: `int`, `float`, `bool`, `string`, `char`, `null`.
- أنواع مركّبة: `List<T>`, `Tuple(T1, T2, ...)`, `&T`, `&mut T`, كائنات صفات `dyn Trait`.
- التعابير الحسابية والمنطقية: +, -, *, /, %, **, ==, !=, <, >, <=, >=, &&, ||, !

التصريح والمتغيرات:
```albayan
fn main() -> int {
    let x: int = 10;
    let mut y: int = 5;     // mut مسموح في let
    y = y + x;
    return y;
}
```

الدوال والأنواع المرجعية:
```albayan
fn inc(v: &int) -> int { return *v + 1; }
fn inc_mut(v: &mut int) -> int { *v = *v + 1; return *v; }
```

Tuple مبسطة:
```albayan
fn pair() -> Tuple(int, string) { return (42, "answer"); }
```

---

## 3) المجموعات: القوائم والمصفوفات والفهارس
- قائمة حرفية: `[1, 2, 3]` تنتج `List<int>`.
- الفهرسة: `arr[i]` وتكون `i: int`.
- القوائم المتداخلة مدعومة: `[[1,2],[3,4]][1][0]` ← 3.
- ملاحظة: للقوائم الفارغة يفضّل إسناد نوع صريح: `let xs: List<int> = [];` (إن كانت الصياغة مدعومة في إصدارك) أو ابدأ بعنصر ثم وسِّع.

أمثلة:
```albayan
fn sum5() -> int {
    let mut s: int = 0;
    for i in [0, 1, 2, 3, 4] { s = s + i; }
    return s; // 10
}

fn nested_get() -> int { return [[1,2],[3,4]][1][0]; }

fn index_literal() -> int { return [10,20,30,40][2]; }
```

---

## 4) الدوال والوحدات والموديولات
تعريف دالة وإرجاع قيمة:
```albayan
fn add(a: int, b: int) -> int { return a + b; }
```
الموديولات والاستخدام:
```albayan
module math_utils {
    fn sqr(x: float) -> float { return x * x; }
}

using math_utils;
fn demo() -> float { return sqr(3.0); }
```

---

## 5) التحكم في التدفق: الشرط والحلقات والمطابقة
- if/else (لا يوجد else if مباشرة؛ استخدم else { if .. }):
```albayan
fn sign(x: int) -> string {
    if x > 0 { return "+"; }
    else { if x < 0 { return "-"; } else { return "0"; } }
}
```
- while, loop, break, continue:
```albayan
fn count_to(n: int) -> int {
    let mut i: int = 0;
    let mut s: int = 0;
    while i < n { s = s + i; i = i + 1; }
    return s;
}
```
- for-over-List:
```albayan
fn sum(xs: List<int>) -> int {
    let mut s: int = 0;
    for v in xs { s = s + v; }
    return s;
}
```
- match (أساسيات):
```albayan
fn match_basic(x: int) -> int {
    match x {
        0 => { return 10; }
        _ => { return 20; }
    }
}
```

---

## 6) Struct / Enum / Class / Interface / Trait / Impl / dyn
Struct وتعريف حقول:
```albayan
struct Point { x: float; y: float; }

fn origin() -> Point { return Point { x: 0.0, y: 0.0 }; }
```
Enum أساسي:
```albayan
enum Color { Red, Green, Blue }
```
Class مع حقول وأساليب:
```albayan
class Counter {
    value: int;
    fn new() -> Counter { return Counter { value: 0 }; }
    fn inc(&mut self) { self.value = self.value + 1; }
}
```
Interface وتواقيع دوال:
```albayan
interface Drawable { fn draw(surface: string) -> void; }
```
Trait و Impl (نمط Rust):
```albayan
trait Show { fn show(&self) -> string; }
impl Show for Point { fn show(&self) -> string { return "(" + self.x.to_string() + "," + self.y.to_string() + ")"; } }

fn print_dyn(p: dyn Show) { print(p.show()); }
```

ملاحظة: تفاصيل التوليد/السمانتيك لبعض هذه البنى قيد الاستكمال حسب الإصدار.

---

## 7) البرمجة المنطقية: Relation / Fact / Rule
تعريف علاقة وحقائق وقاعدة:
```albayan
relation Parent(string, string);

fact Parent("Ali", "Huda");
fact Parent("Huda", "Sara");

rule Grandparent(GP, GC) :- Parent(GP, P), Parent(P, GC);
```
ملاحظة: يدعم المترجم بناء العلاقات والحقائق والقواعد. آليات الاستعلام Solve/Prove ستُتاح عبر واجهات تنفيذية متكاملة مع Runtime.

---

## 8) محرك اللغة الطبيعية: كتلة semantic
يمكن إدراج جُمَل لغة طبيعية ليتم تحليلها لاحقاً:
```albayan
semantic {
    "ارسم شجرة خضراء في منتصف الشاشة";
    "حرّك العنصر إلى اليمين بسرعة متوسطة";
}
```
ينتج عنها كتلة SemanticBlock داخلياً تُخزِّن الجمل وقيم ثقة/استدعاءات مولدة (إطار عمل التحليل متصل بالمحركات).

---

## 9) الذكاء اللغوي (سيماء الحروف)
واجهات عالية المستوى من std/linguistic_intelligence.ab:
```albayan
using std::linguistic_intelligence;

fn main() -> int {
    if !initialize_linguistic_intelligence() { return -1; }

    let word = "شجرة".to_string();
    let ok = analyze_arabic_word(word.clone());
    if ok {
        let meaning = get_word_meaning(word.clone(), "عربي".to_string());
        print("المعنى: " + meaning);
    }

    let count = get_analysis_statistics();
    print("عدد الكلمات المحللة: " + count.to_string());
    return 0;
}
```
يمكنك أيضاً إضافة حروف إنجليزية بدلالاتها لتعلّم استقرائي:
```albayan
add_english_letter('T', "الهيكل".to_string(), "شكل الجذع".to_string(), 0.8);
```

---

## 10) الذكاء الاصطناعي التقليدي (PyTorch) عبر std/torch
يوفر std/torch واجهات تدريب/استدلال عالية المستوى (يربطها المترجم بالـ Runtime):
```albayan
using std::torch;

fn train_demo() {
    let model = torch_create_model("example", 4, 8, 2);
    let opt = torch_create_optimizer(model, "adam", 0.01);

    let input_data = [1.0, 2.0, 3.0, 4.0];
    let input_shape = [1, 4];
    let x = torch_create_tensor("x", input_data, input_shape);

    let target_data = [0.0, 1.0];
    let target_shape = [1, 2];
    let y = torch_create_tensor("y", target_data, target_shape);

    for epoch in 0..10 {
        let r = torch_train_step(model, opt, x, y);
        if r.success { println("epoch " + epoch + ": loss=" + r.loss); }
    }
}
```
استعلم عن الجهاز:
```albayan
if torch_cuda_available() { println("GPU جاهز"); }
println("الجهاز: " + torch_get_device());
```

---

## 11) الذكاء الرياضي المبتكر والمعادلات اللغوية
- الفكرة: صياغة متجهات معنى للكلمات من سمات حروفها (صوت/شكل/موقع)، ثم تطبيق معادلات وزن/تطبيع/اندماج ("معادلة الشكل العام") لاشتقاق دلالة قابلة للقياس.
- التعلم التكيفي: تحديث الأوزان/المعاملات من التغذية الراجعة.

مثال من std/linguistic_intelligence:
```albayan
let words = vec!["شجرة".to_string(), "جبل".to_string()];
let patterns = discover_word_patterns(words, "عربي".to_string());
let stats = adaptive_learning_from_patterns(patterns);
print("الدقة: " + stats.prediction_accuracy.to_string());
```

---

## 12) القيود الحالية ونصائح الاستخدام
- else if غير مدعوم: استخدم else { if .. }.
- الصيغة العلمية للأعداد غير مدعومة حالياً.
- عامل التحويل `as` غير مدعوم.
- الطباعة تدعم string فقط (حوّل الأنواع الأخرى إلى string).
- الفهارس يجب أن تكون من `int`.
- القوائم الفارغة قد تحتاج تلميح نوع.
- `pub` غير فعّال حالياً في الترجمة.

نصائح:
- ابدأ بأمثلة صغيرة واجعل رسائل الأخطاء ترشدك.
- استخدم tests/programs/*.ab كنماذج، مثل: 05_for_loop.ab, 08_list_index_literal.ab, 09_nested_list_indexing.ab

---

## 13) مراجع وأمثلة إضافية
- موجز شامل للوكيل الذكي: docs/AGENT_INTELLIGENT_MODEL_BRIEFING.md
- الذكاء اللغوي: std/linguistic_intelligence.ab
- الذكاء التقليدي (PyTorch): std/torch/mod.ab
- أمثلة إضافية: tests/programs/*.ab

إن احتجت قسماً أوسع (مثلاً دليل مرئي لمحركات الرسم والتحريك)، سنضيف ملاحق لاحقة.
