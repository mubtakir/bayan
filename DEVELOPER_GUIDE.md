# 🚀 دليل المطورين - AlBayan Language Developer Guide

## 📋 جدول المحتويات

1. [المتطلبات الأساسية](#المتطلبات-الأساسية)
2. [التثبيت على Linux](#التثبيت-على-linux)
3. [التثبيت على macOS](#التثبيت-على-macos)
4. [التثبيت على Windows](#التثبيت-على-windows)
5. [التثبيت من المصدر](#التثبيت-من-المصدر)
6. [التحقق من التثبيت](#التحقق-من-التثبيت)
7. [أول برنامج](#أول-برنامج)
8. [تشغيل الأمثلة](#تشغيل-الأمثلة)
9. [استخدام VS Code](#استخدام-vs-code)
10. [الأوامر الأساسية](#الأوامر-الأساسية)

---

## 📦 المتطلبات الأساسية

### المتطلبات المشتركة لجميع الأنظمة:

- **Rust 1.70+** - لغة البرمجة الأساسية
- **Cargo** - مدير الحزم (يأتي مع Rust)
- **Git** - للتحكم بالإصدارات
- **LLVM 14+** - لتجميع الكود (اختياري للتطوير)

---

## 🐧 التثبيت على Linux

### 1. تثبيت Rust و Cargo

```bash
# تحميل وتثبيت Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# تفعيل Rust في الجلسة الحالية
source $HOME/.cargo/env

# التحقق من التثبيت
rustc --version
cargo --version
```

### 2. تثبيت المتطلبات الإضافية

**على Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y build-essential git llvm-14 clang
```

**على Fedora/RHEL:**
```bash
sudo dnf install -y gcc g++ git llvm-devel clang
```

**على Arch:**
```bash
sudo pacman -S base-devel git llvm clang
```

### 3. استنساخ المستودع

```bash
git clone https://github.com/al-mubtakir/bayan.git
cd bayan
git checkout feature/agent-migration
```

### 4. بناء المشروع

```bash
cargo build --release
```

---

## 🍎 التثبيت على macOS

### 1. تثبيت Homebrew (إذا لم يكن مثبتاً)

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### 2. تثبيت Rust و Cargo

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 3. تثبيت المتطلبات الإضافية

```bash
brew install llvm clang git
```

### 4. استنساخ المستودع

```bash
git clone https://github.com/al-mubtakir/bayan.git
cd bayan
git checkout feature/agent-migration
```

### 5. بناء المشروع

```bash
cargo build --release
```

---

## 🪟 التثبيت على Windows

### 1. تثبيت Rust

- اذهب إلى https://rustup.rs/
- حمّل وشغّل `rustup-init.exe`
- اتبع التعليمات على الشاشة
- اختر الخيار الافتراضي (MSVC)

### 2. تثبيت Visual Studio Build Tools

```powershell
# تحميل Visual Studio Build Tools
# من: https://visualstudio.microsoft.com/downloads/
# اختر "Desktop development with C++"
```

### 3. تثبيت Git

- اذهب إلى https://git-scm.com/download/win
- حمّل وشغّل المثبت
- اتبع التعليمات الافتراضية

### 4. استنساخ المستودع

```powershell
git clone https://github.com/al-mubtakir/bayan.git
cd bayan
git checkout feature/agent-migration
```

### 5. بناء المشروع

```powershell
cargo build --release
```

---

## 🔨 التثبيت من المصدر

### الخطوات الموحدة لجميع الأنظمة:

```bash
# 1. استنساخ المستودع
git clone https://github.com/al-mubtakir/bayan.git
cd bayan

# 2. الانتقال إلى الفرع الصحيح
git checkout feature/agent-migration

# 3. بناء المشروع
cargo build --release

# 4. التحقق من البناء
./target/release/albayan --version
```

---

## ✅ التحقق من التثبيت

```bash
# التحقق من إصدار البيان
./target/release/albayan --version

# عرض المساعدة
./target/release/albayan --help

# تشغيل REPL التفاعلي
./target/release/albayan repl
```

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

## 🎯 تشغيل الأمثلة

```bash
# تشغيل مثال واحد
./target/release/albayan run examples/AMAZING_DEMO.ab

# تشغيل جميع الأمثلة المبهرة
./target/release/albayan run examples/AMAZING_DEMO.ab
./target/release/albayan run examples/FIBONACCI_DEMO.ab
./target/release/albayan run examples/MATH_OPERATIONS_DEMO.ab
./target/release/albayan run examples/ARRAY_OPERATIONS_DEMO.ab

# تشغيل جميع اختبارات الكلمات المحجوزة
for file in examples/test_keyword_*.ab; do
    ./target/release/albayan run "$file"
done
```

---

## 🔧 استخدام VS Code

### 1. تثبيت الإضافات

- **Rust Analyzer** - للتحليل الذكي
- **CodeLLDB** - لتصحيح الأخطاء
- **Better TOML** - لملفات Cargo.toml

### 2. إنشاء ملف `.vscode/settings.json`:

```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "[rust]": {
        "editor.formatOnSave": true,
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

### 3. تشغيل البرامج من VS Code

- اضغط `Ctrl+Shift+B` لبناء المشروع
- استخدم Terminal المدمج لتشغيل الأوامر

---

## 📝 الأوامر الأساسية

```bash
# بناء المشروع
cargo build --release

# تشغيل برنامج
./target/release/albayan run <file.ab>

# فحص الكود
./target/release/albayan check <file.ab>

# تنسيق الكود
./target/release/albayan format <file.ab>

# عرض معلومات الملف
./target/release/albayan info <file.ab>

# تشغيل REPL التفاعلي
./target/release/albayan repl

# تشغيل خادم LSP
./target/release/albayan lsp
```

---

## 🎓 الخطوات التالية

1. اقرأ [الأمثلة](./examples/) المتاحة
2. استعرض [التوثيق](./COMPLETE_DEMO_REPORT.md)
3. جرّب كتابة برامجك الخاصة
4. شارك مشاريعك على GitHub

---

## 🆘 استكشاف الأخطاء

### المشكلة: `cargo build` فشل

**الحل:**
```bash
# تحديث Rust
rustup update

# تنظيف المشروع
cargo clean

# إعادة البناء
cargo build --release
```

### المشكلة: `albayan` لم يتم العثور عليه

**الحل:**
```bash
# تأكد من أنك في المجلد الصحيح
cd /path/to/bayan

# استخدم المسار الكامل
./target/release/albayan run <file.ab>
```

### المشكلة: خطأ في LLVM

**الحل:**
```bash
# على Linux
sudo apt-get install llvm-14-dev

# على macOS
brew install llvm

# على Windows
# أعد تثبيت Visual Studio Build Tools
```

---

## 📞 الدعم والمساعدة

- **GitHub Issues**: https://github.com/al-mubtakir/bayan/issues
- **المستودع**: https://github.com/al-mubtakir/bayan
- **الفرع الرئيسي**: feature/agent-migration

---

**آخر تحديث:** 2025-10-17  
**الإصدار:** v0.1.0  
**الحالة:** ✅ جاهز للاستخدام

