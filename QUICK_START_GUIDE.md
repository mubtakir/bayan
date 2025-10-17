# 🚀 دليل البدء السريع - AlBayan Language Quick Start Guide

## 📥 التثبيت والتشغيل (Installation & Running)

### الخطوة 1: بناء المشروع
```bash
cd /home/al-mubtakir/Documents/bayan1
cargo build --release
```

### الخطوة 2: تشغيل ملف AlBayan
```bash
./target/release/albayan run examples/IMPRESSIVE_SHOWCASE.ab
```

---

## 📚 أمثلة سريعة (Quick Examples)

### 1️⃣ مثال بسيط جداً (Hello World)
```albayan
fn main() -> int {
    let x = 10;
    let y = 20;
    let sum = x + y;
    return sum;
}
```

### 2️⃣ الدوال (Functions)
```albayan
fn add(a: int, b: int) -> int {
    return a + b;
}

fn main() -> int {
    let result = add(5, 10);
    return result;
}
```

### 3️⃣ الشروط (Conditionals)
```albayan
fn max(a: int, b: int) -> int {
    if a > b {
        return a;
    } else {
        return b;
    }
}

fn main() -> int {
    return max(10, 20);
}
```

### 4️⃣ الحلقات (Loops)
```albayan
fn factorial(n: int) -> int {
    let result = 1;
    let i = 1;
    while i <= n {
        result = result * i;
        i = i + 1;
    }
    return result;
}

fn main() -> int {
    return factorial(5);
}
```

### 5️⃣ المصفوفات (Arrays)
```albayan
fn main() -> int {
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    return first + second;
}
```

### 6️⃣ الهياكل (Structs)
```albayan
struct Point {
    x: int;
    y: int;
}

fn main() -> int {
    let p = Point { x: 10, y: 20 };
    return p.x + p.y;
}
```

### 7️⃣ التطابق (Match)
```albayan
fn classify(n: int) -> int {
    match n {
        0 => { return 0; }
        1 => { return 1; }
        _ => { return -1; }
    }
}

fn main() -> int {
    return classify(1);
}
```

---

## 🎯 الأمثلة المتقدمة (Advanced Examples)

### 📁 الملفات المتاحة:

| الملف | الوصف |
|------|-------|
| `examples/test_basics.ab` | اختبار الحسابات الأساسية |
| `examples/test_functions.ab` | اختبار الدوال |
| `examples/test_conditionals.ab` | اختبار الشروط |
| `examples/test_loops.ab` | اختبار الحلقات |
| `examples/test_arrays_simple.ab` | اختبار المصفوفات |
| `examples/test_structs_simple.ab` | اختبار الهياكل |
| `examples/test_match.ab` | اختبار التطابق |
| `examples/COMPREHENSIVE_TEST.ab` | اختبار شامل |
| `examples/IMPRESSIVE_SHOWCASE.ab` | عرض مبهر متقدم |

### 🚀 تشغيل الأمثلة:
```bash
./target/release/albayan run examples/IMPRESSIVE_SHOWCASE.ab
./target/release/albayan run examples/COMPREHENSIVE_TEST.ab
./target/release/albayan run examples/test_basics.ab
```

---

## 🔧 قواعد اللغة (Language Rules)

### ✅ ما يعمل:
- ✅ العمليات الحسابية: `+`, `-`, `*`, `/`, `%`, `^`
- ✅ المقارنات: `>`, `<`, `==`, `!=`, `>=`, `<=`
- ✅ العمليات المنطقية: `&&`, `||`
- ✅ الدوال مع معاملات وقيم إرجاع
- ✅ الشروط: `if-else`
- ✅ الحلقات: `while`
- ✅ المصفوفات: `[1, 2, 3]`
- ✅ الهياكل: `struct Point { x: int; y: int; }`
- ✅ التطابق: `match n { 0 => {...} _ => {...} }`

### ⚠️ قيود معروفة:
- ⚠️ لا يمكن تمرير المصفوفات كمعاملات دالة
- ⚠️ استخدم `;` بين حقول الهياكل (ليس `,`)
- ⚠️ لا تدعم استدعاءات الدوال متعددة الأسطر

---

## 📊 الأوامر المتاحة (Available Commands)

```bash
# تشغيل ملف
./target/release/albayan run <file.ab>

# بناء ملف
./target/release/albayan build <file.ab>

# فحص الصيغة
./target/release/albayan check <file.ab>

# تنسيق الكود
./target/release/albayan format <file.ab>

# الحصول على المساعدة
./target/release/albayan --help
```

---

## 💡 نصائح مهمة (Important Tips)

1. **ابدأ بسيط:** ابدأ بأمثلة بسيطة ثم انتقل للمعقدة
2. **استخدم الهياكل:** استخدم الهياكل لتنظيم البيانات
3. **اختبر الحلقات:** استخدم الحلقات للعمليات المتكررة
4. **استخدم الدوال:** قسّم الكود إلى دوال صغيرة
5. **اقرأ الأخطاء:** رسائل الخطأ واضحة ومفيدة جداً

---

## 🎓 موارد إضافية (Additional Resources)

- 📖 `LANGUAGE_FEATURES_STATUS.md` - حالة جميع الميزات
- 📋 `LANGUAGE_VERIFICATION_REPORT.md` - تقرير التحقق الشامل
- 📚 `bay.md` - التوثيق الكامل للغة
- 🔍 `examples/` - مجلد الأمثلة

---

## ❓ الأسئلة الشائعة (FAQ)

**س: كيف أنشئ دالة؟**  
ج: استخدم `fn name(param: type) -> return_type { ... }`

**س: كيف أنشئ حلقة؟**  
ج: استخدم `while condition { ... }`

**س: كيف أنشئ هيكل؟**  
ج: استخدم `struct Name { field: type; ... }`

**س: كيف أصل إلى عنصر مصفوفة؟**  
ج: استخدم `arr[index]`

**س: كيف أصل إلى حقل هيكل؟**  
ج: استخدم `struct_var.field`

---

## 🎉 ابدأ الآن! (Get Started Now!)

```bash
cd /home/al-mubtakir/Documents/bayan1
./target/release/albayan run examples/IMPRESSIVE_SHOWCASE.ab
```

**استمتع بلغة البيان الثورية!** 🌟


