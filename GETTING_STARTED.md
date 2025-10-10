# دليل البدء السريع - لغة البيان
# Quick Start Guide - AlBayan Language

## 🚀 التثبيت والإعداد

### 1. متطلبات النظام
- **نظام التشغيل:** Linux, macOS, Windows
- **Rust:** الإصدار 1.70 أو أحدث
- **الذاكرة:** 4GB RAM كحد أدنى
- **المساحة:** 2GB مساحة فارغة

### 2. تثبيت Rust (إذا لم يكن مثبت)
```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Windows
# تحميل من: https://rustup.rs/
```

### 3. تحميل وبناء لغة البيان
```bash
# استنساخ المشروع
git clone <repository-url>
cd albayan-lang

# بناء المترجم
cargo build --release

# اختبار التثبيت
./target/release/albayan --version
```

### 4. إعداد VS Code (اختياري)
```bash
# تثبيت إضافة VS Code
cd vscode-extension
npm install
npm run compile
code --install-extension .
```

## 🎯 أول برنامج لك

### إنشاء ملف جديد
```bash
mkdir my-first-albayan-project
cd my-first-albayan-project
touch hello.ab
```

### كتابة أول برنامج
```albayan
// hello.ab - أول برنامج بلغة البيان
fn main() -> int {
    let message = "مرحباً بك في عالم البرمجة بالعربية!";
    let number = 42;
    let result = number * 2;
    
    return result;
}
```

### تشغيل البرنامج
```bash
# فحص الصيغة
albayan check hello.ab

# تشغيل البرنامج
albayan run hello.ab

# تجميع البرنامج
albayan build hello.ab
```

## 📚 الخطوات التالية

### 1. جرب الأمثلة الموجودة
```bash
# الأمثلة الأساسية
albayan run examples/simple.ab
albayan run examples/variables_and_operations.ab
albayan run examples/working_example.ab

# الأمثلة المتقدمة
albayan check examples/oop_basics.ab
albayan check examples/library_system.ab
```

### 2. تعلم الأساسيات
- **المتغيرات والأنواع**
- **الدوال والتحكم في التدفق**
- **العمليات الحسابية والمنطقية**
- **التعامل مع النصوص**

### 3. استكشف الميزات المتقدمة
- **البرمجة المنطقية**
- **الذكاء الاصطناعي**
- **البرمجة غير المتزامنة**
- **نظام الوحدات**

## 🆘 المساعدة والدعم

### الأوامر المفيدة
```bash
albayan --help          # عرض المساعدة
albayan check <file>    # فحص الصيغة
albayan run <file>      # تشغيل البرنامج
albayan build <file>    # تجميع البرنامج
albayan format <file>   # تنسيق الكود
```

### الموارد
- **الوثائق:** `docs/developer_guide.md`
- **الأمثلة:** `examples/`
- **المرجع:** `docs/api_reference.md`

## 🎊 مبروك!
أنت الآن جاهز لبدء رحلتك مع لغة البيان! 🌟
