# البيان (AlBayan) Programming Language

<div align="center">

**لغة برمجة حديثة تدمج البرمجة التقليدية مع البرمجة المنطقية وميزات الذكاء الاصطناعي**

[![Build Status](https://github.com/albayan-team/albayan/workflows/CI/badge.svg)](https://github.com/albayan-team/albayan/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/albayan-team/albayan/releases)

[الوثائق](docs/) | [أمثلة](examples/) | [دليل التثبيت](#التثبيت) | [المساهمة](CONTRIBUTING.md)

</div>

## 🌟 الميزات الرئيسية

### 🔧 البرمجة التقليدية
- **نظام أنواع قوي**: دعم للأنواع الأساسية والمعقدة
- **إدارة الذاكرة الآمنة**: نظام ملكية مشابه لـ Rust
- **البرمجة غير المتزامنة**: دعم async/await والقنوات
- **هياكل البيانات المتقدمة**: مصفوفات، خرائط، أشجار، رسوم بيانية

### 🧠 البرمجة المنطقية
- **العلاقات والقواعد**: تعريف المعرفة بطريقة منطقية
- **محرك الاستدلال**: حل الاستعلامات بالتراجع والتوحيد
- **قاعدة المعرفة**: تخزين وإدارة الحقائق والقواعد
- **الاستعلامات المعقدة**: دعم الاستعلامات المتداخلة والشرطية

### 🤖 الذكاء الاصطناعي
- **التنسورات**: عمليات متقدمة على المصفوفات متعددة الأبعاد
- **الشبكات العصبية**: طبقات مختلفة (Dense, Conv2D, LSTM, Attention)
- **معالجة اللغة الطبيعية**: تحليل النصوص والترميز
- **نماذج مدربة مسبقاً**: دعم ONNX وPyTorch

### ⚡ الأداء والتحسين
- **مولد LLVM**: إنتاج كود آلة محسن
- **مستويات التحسين**: O0, O1, O2, O3
- **التجميع المتوازي**: استغلال المعالجات متعددة النوى
- **إدارة الذاكرة الذكية**: تجميع القمامة التدريجي

### 🛠️ أدوات التطوير
- **خادم اللغة (LSP)**: دعم المحررات الحديثة
- **مصحح الأخطاء**: تتبع التنفيذ ونقاط التوقف
- **محلل الأداء**: قياس الأداء واستخدام الذاكرة
- **منسق الكود**: تنسيق تلقائي للكود
- **فاحص الكود**: اكتشاف المشاكل والتحسينات

## 🚀 التثبيت السريع

### المتطلبات الأساسية
- Rust 1.70+ ([تثبيت Rust](https://rustup.rs/))
- LLVM 17+ (اختياري للتحسينات المتقدمة)
- Git

### التثبيت من المصدر

```bash
# استنساخ المستودع
git clone https://github.com/albayan-team/albayan.git
cd albayan

# تجميع المشروع
cargo build --release

# إضافة إلى PATH (اختياري)
export PATH=$PATH:$(pwd)/target/release

# اختبار التثبيت
./target/release/albayan --version
```

### التثبيت باستخدام Cargo

```bash
cargo install albayan
```

## 📖 دليل البداية السريعة

### 1. إنشاء أول برنامج

```bash
# إنشاء ملف جديد
echo 'fn main() -> int {
    println("مرحباً بالعالم!");
    return 0;
}' > hello.ab

# تجميع وتشغيل
albayan run hello.ab
```

### 2. استخدام البرمجة المنطقية

```albayan
// تعريف العلاقات
relation parent(string, string);
relation sibling(string, string);

// إضافة حقائق
fact parent("أحمد", "فاطمة").
fact parent("أحمد", "محمد").
fact parent("سارة", "علي").

// تعريف قاعدة
rule sibling(X, Y) :- 
    parent(P, X), 
    parent(P, Y), 
    X != Y.

fn main() -> int {
    // استعلام عن الأشقاء
    query sibling("فاطمة", "محمد");
    return 0;
}
```

### 3. استخدام الذكاء الاصطناعي

```albayan
use ai::*;

fn main() -> int {
    // إنشاء تنسور
    let input = tensor([[1.0, 2.0], [3.0, 4.0]]);
    
    // إنشاء شبكة عصبية
    let mut network = NeuralNetwork::new();
    network.add_layer(Dense::new(2, 4));
    network.add_layer(Dense::new(4, 1));
    
    // تنبؤ
    let output = network.forward(input);
    println("النتيجة: {}", output);
    
    return 0;
}
```

## 🏗️ هيكل المشروع

```
albayan/
├── src/                    # الكود المصدري
│   ├── lexer/             # المحلل المعجمي
│   ├── parser/            # المحلل النحوي
│   ├── semantic/          # التحليل الدلالي
│   ├── codegen/           # توليد الكود
│   ├── runtime/           # نظام التشغيل
│   ├── ai/                # مكتبات الذكاء الاصطناعي
│   ├── modules/           # نظام الوحدات
│   ├── tools/             # أدوات التطوير
│   ├── lsp/               # خادم اللغة
│   └── cli/               # واجهة سطر الأوامر
├── examples/              # أمثلة البرمجة
├── tests/                 # الاختبارات
├── docs/                  # الوثائق
├── vscode-extension/      # إضافة VS Code
└── Cargo.toml            # إعدادات المشروع
```

## 📚 الوثائق الشاملة

- **[دليل المطور](docs/developer_guide.md)** - دليل شامل للبرمجة بلغة البيان
- **[مرجع API](docs/api_reference.md)** - وثائق مفصلة لجميع الوظائف
- **[أمثلة متقدمة](docs/examples.md)** - أمثلة عملية ومشاريع كاملة
- **[دليل التثبيت](docs/installation.md)** - تعليمات التثبيت المفصلة
- **[دليل المساهمة](CONTRIBUTING.md)** - كيفية المساهمة في المشروع

## 🎯 أمثلة متقدمة

### نظام إدارة المكتبة
```albayan
struct Book {
    id: int,
    title: string,
    author: string,
    available: bool,
}

struct Library {
    books: Vec<Book>,
    members: Set<string>,
}

impl Library {
    fn borrow_book(&mut self, member: string, book_id: int) -> Result<(), string> {
        if !self.members.contains(member) {
            return Err("عضو غير مسجل");
        }
        
        for book in &mut self.books {
            if book.id == book_id {
                if book.available {
                    book.available = false;
                    return Ok(());
                } else {
                    return Err("الكتاب غير متاح");
                }
            }
        }
        
        Err("الكتاب غير موجود")
    }
}
```

### تحليل المشاعر
```albayan
use ai::natural_language::*;

fn analyze_sentiment(text: string) -> string {
    let mut analyzer = SentimentAnalyzer::new(1000, 100, 50, 128);
    
    // تدريب النموذج (مبسط)
    let training_texts = [
        "أحب هذا المنتج كثيراً",
        "منتج سيء جداً",
        "منتج عادي"
    ];
    let labels = [2, 0, 1]; // إيجابي، سلبي، محايد
    
    analyzer.train(&training_texts, &labels, 10);
    
    // تحليل النص
    let (sentiment, confidence) = analyzer.analyze(text);
    return format!("المشاعر: {} (الثقة: {:.2})", sentiment, confidence);
}
```

## 🔧 أوامر CLI

```bash
# تجميع ملف
albayan build program.ab

# تشغيل ملف
albayan run program.ab

# فحص الصيغة
albayan check program.ab

# تنسيق الكود
albayan format program.ab

# بدء REPL
albayan repl

# بدء خادم اللغة
albayan lsp

# عرض معلومات اللغة
albayan info
```

## 🧪 تشغيل الاختبارات

```bash
# تشغيل جميع الاختبارات
cargo test

# تشغيل اختبارات محددة
cargo test test_lexer

# تشغيل اختبارات التكامل
cargo test --test integration_test

# تشغيل الاختبارات مع التفاصيل
cargo test -- --nocapture
```

## 🤝 المساهمة

نرحب بجميع أنواع المساهمات! يمكنك المساهمة من خلال:

- **الإبلاغ عن الأخطاء**: [فتح issue جديد](https://github.com/albayan-team/albayan/issues)
- **اقتراح ميزات**: مشاركة أفكارك الجديدة
- **تحسين الكود**: إرسال pull requests
- **تحسين الوثائق**: إضافة أو تحديث الوثائق
- **إضافة أمثلة**: مشاركة أمثلة عملية

اقرأ [دليل المساهمة](CONTRIBUTING.md) للمزيد من التفاصيل.

## 📄 الترخيص

هذا المشروع مرخص تحت [رخصة MIT](LICENSE) - انظر الملف للتفاصيل.

## 🙏 شكر وتقدير

- **فريق Rust** - للغة البرمجة الرائعة
- **مجتمع LLVM** - لأدوات التجميع المتقدمة
- **مطوري VS Code** - لمنصة التطوير الممتازة
- **جميع المساهمين** - لجهودهم في تطوير المشروع

## 📞 التواصل

- **الموقع الرسمي**: [albayan-lang.org](https://albayan-lang.org)
- **GitHub**: [github.com/albayan-team/albayan](https://github.com/albayan-team/albayan)
- **Discord**: [discord.gg/albayan](https://discord.gg/albayan)
- **Twitter**: [@AlBayanLang](https://twitter.com/AlBayanLang)

---

<div align="center">
صُنع بـ ❤️ من قبل فريق البيان
</div>
