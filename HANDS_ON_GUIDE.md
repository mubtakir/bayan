# دليل التجربة العملية - لغة البيان
# Hands-On Guide - AlBayan Language

## 🎯 مرحباً بك في رحلة البرمجة بالعربية!

### ✅ ما ستتعلمه في هذا الدليل:
1. **كيفية كتابة أول برنامج** بلغة البيان
2. **الميزات الفريدة** التي تميز لغة البيان
3. **أمثلة عملية** من الحياة الواقعية
4. **مقارنات مع اللغات الأخرى**
5. **مشاريع تطبيقية** يمكنك بناؤها

## 🚀 الخطوة 1: التحقق من التثبيت

```bash
# تأكد من أن المترجم يعمل
./target/debug/albayan --version

# جرب فحص مثال بسيط
./target/debug/albayan check examples/simple.ab
```

**النتيجة المتوقعة:**
```
AI Engine initialized
Syntax check passed!
```

## 📝 الخطوة 2: أول برنامج لك

### إنشاء ملف جديد:
```bash
touch my_calculator.ab
```

### كتابة الكود:
```albayan
// حاسبتي الأولى بلغة البيان
fn main() -> int {
    // حساب راتب موظف
    let basic_salary = 8000;
    let overtime_hours = 20;
    let hourly_rate = 50;
    
    let gross_salary = basic_salary + (overtime_hours * hourly_rate);
    let tax = gross_salary * 5 / 100;
    let net_salary = gross_salary - tax;
    
    return net_salary;
}
```

### اختبار البرنامج:
```bash
./target/debug/albayan check my_calculator.ab
```

## 🎨 الخطوة 3: استكشاف الميزات

### أ) دعم اللغة العربية الطبيعي:
```albayan
fn حساب_المعدل(الرياضيات: int, العلوم: int, العربية: int) -> int {
    let المجموع = الرياضيات + العلوم + العربية;
    return المجموع / 3;
}
```

### ب) الأمان المدمج:
```albayan
fn safe_divide(a: int, b: int) -> int {
    if b != 0 {
        return a / b;
    }
    return 0; // حماية من القسمة على صفر
}
```

### ج) العمليات المالية:
```albayan
fn calculate_loan_payment(amount: int, rate: int, years: int) -> int {
    let total_interest = amount * rate * years / 100;
    let total_amount = amount + total_interest;
    return total_amount / (years * 12);
}
```

## 🏗️ الخطوة 4: مشاريع تطبيقية

### مشروع 1: نظام إدارة الرواتب
```albayan
fn employee_salary_system() -> int {
    let basic = 10000;
    let allowances = 2000;
    let overtime = 1500;
    let gross = basic + allowances + overtime;
    
    let insurance = gross * 9 / 100;
    let tax = gross * 5 / 100;
    let deductions = insurance + tax;
    
    let net = gross - deductions;
    return net;
}
```

### مشروع 2: حاسبة الاستثمار
```albayan
fn investment_calculator() -> int {
    let principal = 100000;
    let annual_rate = 8;
    let years = 5;
    
    let simple_interest = principal * annual_rate * years / 100;
    let final_amount = principal + simple_interest;
    
    return final_amount;
}
```

### مشروع 3: نظام تقييم الطلاب
```albayan
fn student_grading_system() -> string {
    let math = 85;
    let science = 90;
    let arabic = 88;
    let english = 82;
    
    let total = math + science + arabic + english;
    let average = total / 4;
    
    if average >= 90 {
        return "متفوق";
    }
    if average >= 80 {
        return "ممتاز";
    }
    if average >= 70 {
        return "جيد";
    }
    return "يحتاج تحسين";
}
```

## 📊 الخطوة 5: مقارنة مع اللغات الأخرى

### نفس المهمة في Python:
```python
def calculate_salary(basic, overtime_hours, rate):
    gross = basic + (overtime_hours * rate)
    tax = gross * 0.05
    return gross - tax

result = calculate_salary(8000, 20, 50)
```

### نفس المهمة في لغة البيان:
```albayan
fn calculate_salary(basic: int, overtime_hours: int, rate: int) -> int {
    let gross = basic + (overtime_hours * rate);
    let tax = gross * 5 / 100;
    return gross - tax;
}
```

### المقارنة:
| الميزة | Python | لغة البيان |
|--------|--------|------------|
| **الوضوح** | ✅ | ✅ |
| **الأمان** | ❌ | ✅ |
| **الأداء** | ❌ | ✅ |
| **دعم العربية** | ❌ | ✅ |
| **فحص الأنواع** | ❌ | ✅ |

## 🎯 الخطوة 6: تحديات عملية

### تحدي 1: حاسبة الزكاة
```albayan
fn calculate_zakat(wealth: int) -> int {
    let nisab = 85000; // النصاب بالريال السعودي
    
    if wealth >= nisab {
        return wealth * 25 / 1000; // 2.5%
    }
    return 0;
}
```

### تحدي 2: حاسبة المسافة والوقود
```albayan
fn trip_cost_calculator(distance: int, fuel_price: int, efficiency: int) -> int {
    let fuel_needed = distance / efficiency;
    let fuel_cost = fuel_needed * fuel_price;
    let other_costs = distance * 2; // تكاليف إضافية
    return fuel_cost + other_costs;
}
```

### تحدي 3: نظام الخصومات
```albayan
fn discount_system(price: int, customer_type: string, quantity: int) -> int {
    let base_discount = 0;
    
    if customer_type == "VIP" {
        let base_discount = 20;
    }
    if customer_type == "Premium" {
        let base_discount = 15;
    }
    if customer_type == "Regular" {
        let base_discount = 10;
    }
    
    let quantity_discount = 0;
    if quantity >= 10 {
        let quantity_discount = 5;
    }
    
    let total_discount = base_discount + quantity_discount;
    let discount_amount = price * total_discount / 100;
    
    return price - discount_amount;
}
```

## 🏆 الخطوة 7: مشروع متكامل

### نظام إدارة مالية شخصية:
```albayan
fn personal_finance_manager() -> int {
    // الدخل الشهري
    let salary = 12000;
    let side_income = 2000;
    let total_income = salary + side_income;
    
    // المصروفات
    let rent = 4000;
    let food = 1500;
    let transportation = 800;
    let utilities = 600;
    let entertainment = 500;
    let total_expenses = rent + food + transportation + utilities + entertainment;
    
    // التوفير والاستثمار
    let monthly_savings = total_income - total_expenses;
    let emergency_fund = monthly_savings * 30 / 100;
    let investment = monthly_savings * 50 / 100;
    let discretionary = monthly_savings * 20 / 100;
    
    // أهداف مالية
    let house_goal = 500000;
    let months_to_house = house_goal / investment;
    
    return months_to_house;
}
```

## 🎊 تهانينا!

لقد أكملت رحلة التجربة العملية للغة البيان! 

### ما تعلمته:
✅ **كتابة برامج آمنة وواضحة**
✅ **استخدام اللغة العربية في البرمجة**
✅ **حل مشاكل حقيقية من الحياة اليومية**
✅ **فهم مميزات لغة البيان الفريدة**

### الخطوات التالية:
1. **جرب الأمثلة المتقدمة** في مجلد `examples/`
2. **اقرأ الوثائق التفصيلية** في `docs/`
3. **انضم للمجتمع** وشارك تجربتك
4. **ابدأ مشروعك الخاص** بلغة البيان

## 🚀 مرحباً بك في مستقبل البرمجة العربية!

لغة البيان ليست مجرد لغة برمجة - إنها **ثورة في طريقة تفكيرنا في التكنولوجيا**!

**هل أنت مستعد لتكون جزءاً من هذه الثورة؟** 🌟
