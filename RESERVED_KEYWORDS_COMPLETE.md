# 📚 قائمة الكلمات المحجوزة الكاملة - Complete Reserved Keywords

## 🔑 جميع الكلمات المحجوزة في لغة البيان

### 1️⃣ **كلمات التعريف** (Definition Keywords)
- `fn` - تعريف دالة
- `struct` - تعريف هيكل
- `enum` - تعريف تعداد
- `class` - تعريف فئة
- `interface` - تعريف واجهة
- `trait` - تعريف صفة (Trait)
- `impl` - تنفيذ صفة (Implementation)
- `let` - إعلان متغير
- `mut` - متغير قابل للتعديل
- `const` - ثابت
- `type` - تعريف نوع

### 2️⃣ **كلمات التحكم في التدفق** (Control Flow Keywords)
- `if` - شرط
- `else` - خلاف ذلك
- `while` - حلقة while
- `for` - حلقة for
- `loop` - حلقة لا نهائية
- `match` - تطابق
- `return` - إرجاع قيمة
- `break` - كسر الحلقة
- `continue` - متابعة الحلقة
- `in` - في (للحلقات)

### 3️⃣ **كلمات البرمجة المنطقية** (Logic Programming Keywords)
- `relation` - علاقة
- `fact` - حقيقة
- `rule` - قاعدة
- `query_solve` - حل استعلام
- `query_prove` - إثبات استعلام
- `assert` - إضافة حقيقة
- `retract` - إزالة حقيقة

### 4️⃣ **كلمات الذكاء الاصطناعي** (AI Keywords)
- `Model` - نموذج
- `Tensor` - موتر
- `tensor` - حرفي موتر
- `TorchModel` - نموذج PyTorch
- `TorchOptimizer` - محسّن PyTorch
- `TorchTensor` - موتر PyTorch
- `TrainingResult` - نتيجة التدريب

### 5️⃣ **كلمات التزامن** (Concurrency Keywords)
- `async` - غير متزامن
- `await` - انتظار
- `gpu` - معالج الرسومات

### 6️⃣ **كلمات الوحدات** (Module Keywords)
- `module` - وحدة
- `using` - استخدام
- `pub` - عام
- `priv` - خاص

### 7️⃣ **كلمات خاصة** (Special Keywords)
- `self` - الكائن الحالي
- `super` - الفئة الأب
- `init` - البناء
- `dynamic` - ديناميكي
- `semantic` - دلالي
- `true` - صحيح
- `false` - خاطئ
- `null` - فارغ

### 8️⃣ **كلمات إضافية** (Additional Keywords)
- `dyn` - ديناميكي (للصفات)

---

## ✅ اختبار كل كلمة محجوزة

### 1. `fn` - تعريف دالة
```albayan
fn add(a: int, b: int) -> int {
    return a + b;
}

fn main() -> int {
    return add(5, 3);
}
```

### 2. `let` - إعلان متغير
```albayan
fn main() -> int {
    let x = 10;
    let y = 20;
    return x + y;
}
```

### 3. `if` و `else` - الشروط
```albayan
fn max(a: int, b: int) -> int {
    if a > b {
        return a;
    } else {
        return b;
    }
}

fn main() -> int {
    return max(5, 10);
}
```

### 4. `while` - حلقة while
```albayan
fn sum_to_n(n: int) -> int {
    let sum = 0;
    let i = 1;
    while i <= n {
        sum = sum + i;
        i = i + 1;
    }
    return sum;
}

fn main() -> int {
    return sum_to_n(5);
}
```

### 5. `for` - حلقة for
```albayan
fn main() -> int {
    let arr = [1, 2, 3, 4, 5];
    let sum = 0;
    for i in arr {
        sum = sum + i;
    }
    return sum;
}
```

### 6. `match` - التطابق
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

### 7. `return` - الإرجاع
```albayan
fn get_value() -> int {
    return 42;
}

fn main() -> int {
    return get_value();
}
```

### 8. `struct` - الهيكل
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

### 9. `enum` - التعداد
```albayan
enum Color {
    Red,
    Green,
    Blue,
}

fn main() -> int {
    return 0;
}
```

### 10. `break` و `continue` - التحكم في الحلقات
```albayan
fn main() -> int {
    let i = 0;
    while i < 10 {
        if i == 5 {
            break;
        }
        i = i + 1;
    }
    return i;
}
```

### 11. `mut` - متغير قابل للتعديل
```albayan
fn main() -> int {
    let mut x = 10;
    x = 20;
    return x;
}
```

### 12. `const` - ثابت
```albayan
const PI = 314;

fn main() -> int {
    return PI;
}
```

### 13. `true` و `false` - القيم المنطقية
```albayan
fn main() -> int {
    let is_true = true;
    let is_false = false;
    if is_true {
        return 1;
    } else {
        return 0;
    }
}
```

### 14. `null` - القيمة الفارغة
```albayan
fn main() -> int {
    let x = null;
    return 0;
}
```

---

## 📊 ملخص الكلمات المحجوزة

| النوع | الكلمات |
|------|--------|
| التعريف | fn, struct, enum, class, interface, trait, impl, let, mut, const, type |
| التحكم | if, else, while, for, loop, match, return, break, continue, in |
| المنطق | relation, fact, rule, query_solve, query_prove, assert, retract |
| الذكاء الاصطناعي | Model, Tensor, tensor, TorchModel, TorchOptimizer, TorchTensor, TrainingResult |
| التزامن | async, await, gpu |
| الوحدات | module, using, pub, priv |
| خاص | self, super, init, dynamic, semantic, true, false, null, dyn |

**المجموع: 60+ كلمة محجوزة**


