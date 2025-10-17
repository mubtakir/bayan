# 🚀 دليل المرجع السريع - Quick Reference Guide

**لغة البيان - AlBayan Language**  
**الإصدار**: v0.2.0-dev + Phase 6 Week 5  
**آخر تحديث**: 2025-10-17

---

## 📚 الملفات الأساسية

### للمبتدئين:
```
docs/BEGINNER_PROGRAMMING_GUIDE.md      - دليل البرمجة للمبتدئين
docs/QUICK_START_GUIDE.md               - دليل البدء السريع
examples/hello.ab                       - أول برنامج
examples/basic_math.ab                  - عمليات حسابية أساسية
```

### للمطورين المتقدمين:
```
docs/AGENT_INTELLIGENT_MODEL_BRIEFING.md - موجز تشغيل الوكيل
bay.md                                   - الرؤية الفلسفية
ALBAYAN_COMPREHENSIVE_OVERVIEW.md        - نظرة شاملة
docs/ADAPTIVE_AI_WHITEPAPER.md          - ورقة بيضاء للذكاء المتكيف
```

### للمكتبات الرياضية:
```
std/math/README.md                      - دليل مكتبات الرياضيات
std/math/ndarray.ab                     - مصفوفات متعددة الأبعاد
std/math/advanced_linalg.ab             - جبر خطي متقدم
std/math/optimization.ab                - خوارزميات التحسين
std/ffi/numpy_ffi.ab                    - تكاملات NumPy
examples/phase2_examples.ab             - أمثلة Phase 2
```

---

## 🎯 الميزات الرئيسية

### 1. سيماء الحروف (Linguistic Intelligence)
```
الحروف تحمل معاني جوهرية:
- ب: الامتلاء والنقل
- ج: التجميع والعزاء
- د: البداية والنهاية
- س: الزحف والاختفاء
- ق: الدقة والبعد
```

### 2. المعادلات الرياضية المتكيفة
```
O = (id, Φ, Ψ(t), Γ)
I = Ω(ΣO, ε, ρ)
E: S × C → S'
```

### 3. العمليات الأساسية
```
Go(o, loc)              - انتقال
Return(o, loc)          - عودة
Affect(o₁, o₂, params)  - تأثير
Bond(oᵢ, oⱼ, θ, κ)     - التحام
Transform(o, f)         - تحويل
```

---

## 🔧 الأوامر الأساسية

### البناء والتشغيل:
```bash
# بناء المشروع
cargo build --release

# تشغيل ملف
./target/release/bayan examples/hello.ab

# اختبار سريع
./quick_test.sh

# تشغيل جميع الاختبارات
./tests/run_tests.sh
```

### التثبيت:
```bash
# تثبيت سريع
./scripts/quick_install.sh

# بناء لجميع المنصات
./build_all_platforms.sh
```

---

## 📊 نظام الذاكرة

### قواعد البيانات (11 قاعدة):
```
databases/
├── revolutionary_knowledge_system.db    (44 KB)
├── semantic_knowledge.db                (104 KB)
├── linguistic_knowledge.db              (180 KB) ⭐
├── mathematical_knowledge.db            (104 KB)
├── logical_knowledge.db                 (104 KB)
├── interpretive_knowledge.db            (96 KB)
├── visual_knowledge.db                  (104 KB)
├── physical_knowledge.db                (96 KB)
├── external_knowledge.db                (28 KB)
├── harvested_knowledge.db               (40 KB)
└── symbolic_knowledge.db                (104 KB)
```

**الحجم الإجمالي**: ~1.0 MB

---

## 📚 مكتبات الرياضيات

### الدوال الأساسية:
```
NDArray:
  - zeros(shape)
  - ones(shape)
  - full(shape, value)
  - arange(start, stop, step)
  - linspace(start, stop, num)

Matrix:
  - identity(n)
  - random(m, n)
  - transpose()
  - determinant()
  - inverse()

Statistics:
  - mean()
  - std()
  - var()
  - min()
  - max()
```

### الجبر الخطي المتقدم:
```
AdvancedLinAlg:
  - qr_decomposition()      - تحليل QR
  - cholesky()              - تحليل Cholesky
  - power_iteration()       - القيم الذاتية
  - least_squares()         - المربعات الصغرى
  - matrix_norm()           - معايير المصفوفة
```

### التحسين:
```
Optimizer:
  - gradient_descent()      - الانحدار التدريجي
  - sgd()                   - SGD
  - adam()                  - Adam Optimizer
```

---

## 🎓 أمثلة سريعة

### مثال 1: مصفوفات أساسية
```
use std::math::ndarray::NDArray;

let arr = NDArray::zeros([3, 3]);
let sum = arr.sum();
print(sum);
```

### مثال 2: جبر خطي
```
use std::math::advanced_linalg::AdvancedLinAlg;

let matrix = [[1.0, 2.0], [3.0, 4.0]];
let (q, r) = AdvancedLinAlg::qr_decomposition(matrix);
```

### مثال 3: تحسين
```
use std::math::optimization::Optimizer;

let result = Optimizer::gradient_descent(
    initial_point,
    learning_rate,
    iterations
);
```

---

## 🔗 الموارد الإضافية

| المورد | الموقع |
|--------|--------|
| GitHub | https://github.com/mubtakir/bayan |
| الفرع الحالي | feature/agent-migration |
| آخر Commit | b2c7f92 |
| الإصدار | v0.2.0-dev |

---

## 📞 الدعم والمساعدة

- 📖 اقرأ `bay.md` للرؤية الفلسفية
- 🎓 اقرأ `docs/BEGINNER_PROGRAMMING_GUIDE.md` للبدء
- 🔬 اقرأ `docs/AGENT_INTELLIGENT_MODEL_BRIEFING.md` للتفاصيل التقنية
- 💾 اقرأ `DATABASE_PERSISTENCE_GUIDE.md` لنظام الذاكرة

---

**حالة النظام**: 🟢 جاهز للإنتاج  
**آخر تحديث**: 2025-10-17

