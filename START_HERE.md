# 🎉 ابدأ هنا - AlBayan Language - Start Here!

## 👋 مرحباً بك في لغة البيان!

لغة البيان (AlBayan) هي لغة برمجة دلالية مع ذكاء اصطناعي مدمج، مبنية بـ Rust مع LLVM backend.

---

## ⚡ التثبيت السريع (5 دقائق)

### على Linux/macOS:

```bash
# 1. استنساخ المستودع
git clone https://github.com/al-mubtakir/bayan.git
cd bayan

# 2. الانتقال إلى الفرع الصحيح
git checkout feature/agent-migration

# 3. بناء المشروع
cargo build --release

# 4. التحقق من التثبيت
./target/release/albayan --version
```

### على Windows (PowerShell):

```powershell
git clone https://github.com/al-mubtakir/bayan.git
cd bayan
git checkout feature/agent-migration
cargo build --release
.\target\release\albayan.exe --version
```

---

## 📋 المتطلبات المسبقة

قبل البدء، تأكد من تثبيت:

| المتطلب | الرابط |
|--------|--------|
| Rust 1.70+ | https://rustup.rs/ |
| Git | https://git-scm.com/ |
| C++ Compiler | GCC/Clang أو MSVC |

---

## 💻 أول برنامج

### إنشاء ملف `hello.ab`:

```albayan
fn main() -> int {
    return 1;
}
```

### تشغيل البرنامج:

```bash
./target/release/albayan run hello.ab
```

---

## 🎯 تشغيل الأمثلة المبهرة

```bash
# مثال 1: عرض شامل يجمع كل الميزات
./target/release/albayan run examples/AMAZING_DEMO.ab

# مثال 2: متسلسلة فيبوناتشي
./target/release/albayan run examples/FIBONACCI_DEMO.ab

# مثال 3: عمليات رياضية متقدمة
./target/release/albayan run examples/MATH_OPERATIONS_DEMO.ab

# مثال 4: عمليات مصفوفات
./target/release/albayan run examples/ARRAY_OPERATIONS_DEMO.ab
```

---

## 📝 الأوامر الأساسية

```bash
# تشغيل برنامج
./target/release/albayan run <file.ab>

# فحص الكود
./target/release/albayan check <file.ab>

# تنسيق الكود
./target/release/albayan format <file.ab>

# REPL التفاعلي
./target/release/albayan repl

# عرض المساعدة
./target/release/albayan --help
```

---

## 🌟 ميزات اللغة

### ✅ الدوال:
```albayan
fn square(x: int) -> int {
    return x * x;
}
```

### ✅ المتغيرات:
```albayan
let x = 10;
let mut y = 20;
y = 30;
```

### ✅ الحلقات:
```albayan
for i in [1, 2, 3] {
    // ...
}

while i < 10 {
    i = i + 1;
}
```

### ✅ الشروط:
```albayan
if x > 0 {
    // ...
} else {
    // ...
}
```

### ✅ المصفوفات:
```albayan
let arr = [1, 2, 3, 4, 5];
let first = arr[0];
```

---

## 🔧 استخدام VS Code

### 1. تثبيت الإضافات:
- **Rust Analyzer** - للتحليل الذكي
- **CodeLLDB** - لتصحيح الأخطاء
- **Better TOML** - لملفات Cargo.toml

### 2. فتح المشروع:
```bash
code .
```

### 3. بناء المشروع:
- اضغط `Ctrl+Shift+B`

### 4. تشغيل البرنامج:
- استخدم Terminal المدمج

---

## 📚 الملفات المهمة

| الملف | الوصف |
|------|--------|
| `DEVELOPER_GUIDE.md` | دليل شامل للمطورين |
| `COMPLETE_DEMO_REPORT.md` | ملخص الأمثلة والنتائج |
| `RESERVED_KEYWORDS_COMPLETE.md` | قائمة الكلمات المحجوزة |
| `examples/` | مجلد الأمثلة |

---

## 🆘 استكشاف الأخطاء

### المشكلة: `cargo build` فشل

```bash
rustup update
cargo clean
cargo build --release
```

### المشكلة: `albayan` لم يتم العثور عليه

```bash
# استخدم المسار الكامل
./target/release/albayan run <file.ab>
```

### المشكلة: خطأ في LLVM

**Linux:**
```bash
sudo apt-get install llvm-14-dev
```

**macOS:**
```bash
brew install llvm
```

---

## 🎓 الخطوات التالية

1. ✅ ثبّت لغة البيان
2. ✅ شغّل الأمثلة
3. ✅ اكتب برنامجك الأول
4. ✅ استكشف الميزات المتقدمة
5. ✅ شارك مشاريعك على GitHub

---

## 📞 الدعم والمساعدة

- **GitHub**: https://github.com/al-mubtakir/bayan
- **Issues**: https://github.com/al-mubtakir/bayan/issues
- **الفرع الرئيسي**: feature/agent-migration

---

## 🚀 ماذا بعد؟

- اقرأ `DEVELOPER_GUIDE.md` للتثبيت التفصيلي
- استعرض الأمثلة في مجلد `examples/`
- جرّب كتابة برامجك الخاصة
- شارك مشاريعك مع المجتمع

---

**آخر تحديث:** 2025-10-17  
**الإصدار:** v0.1.0  
**الحالة:** ✅ جاهز للاستخدام

