# ⚡ **دليل البداية السريعة للمبتدئين**
# Quick Start Guide for Beginners

## 🚀 **ابدأ البرمجة في 5 دقائق!**

**مرحباً بك! هذا الدليل سيعلمك البرمجة بلغة البيان في أقل من 5 دقائق.**

---

## 1️⃣ **أول برنامج لك**

### **اكتب هذا الكود:**
```albayan
fn my_first_program() -> int {
    let my_age = 20;
    let my_score = 85;
    let result = my_age + my_score;
    return result;
}
```

### **ماذا يفعل هذا الكود؟**
- ✅ ينشئ متغير `my_age` ويضع فيه الرقم 20
- ✅ ينشئ متغير `my_score` ويضع فيه الرقم 85  
- ✅ يجمع الرقمين ويضع النتيجة في `result`
- ✅ يرجع النتيجة (105)

---

## 2️⃣ **تعلم الشروط**

### **اكتب هذا الكود:**
```albayan
fn check_grade(score: int) -> int {
    if score >= 90 {
        return 1; // ممتاز
    } else {
        if score >= 70 {
            return 2; // جيد
        } else {
            return 3; // مقبول
        }
    }
}
```

### **كيف تستخدمه؟**
```albayan
fn test_grades() -> int {
    let student1 = check_grade(95); // النتيجة: 1 (ممتاز)
    let student2 = check_grade(75); // النتيجة: 2 (جيد)
    let student3 = check_grade(65); // النتيجة: 3 (مقبول)
    
    return student1 + student2 + student3; // 6
}
```

---

## 3️⃣ **اصنع حاسبة بسيطة**

### **اكتب هذا الكود:**
```albayan
fn calculator(operation: int, a: int, b: int) -> int {
    if operation == 1 {
        return a + b; // جمع
    } else {
        if operation == 2 {
            return a - b; // طرح
        } else {
            if operation == 3 {
                return a * b; // ضرب
            } else {
                return a / b; // قسمة
            }
        }
    }
}
```

### **جرب الحاسبة:**
```albayan
fn test_calculator() -> int {
    let addition = calculator(1, 10, 5);      // 15
    let subtraction = calculator(2, 20, 8);   // 12
    let multiplication = calculator(3, 6, 7); // 42
    let division = calculator(4, 100, 4);     // 25
    
    return addition + subtraction + multiplication + division; // 94
}
```

---

## 4️⃣ **اعمل مع "المصفوفات"**

### **اكتب هذا الكود:**
```albayan
fn work_with_numbers() -> int {
    // "مصفوفة" من 5 أرقام
    let num1 = 10;
    let num2 = 25;
    let num3 = 30;
    let num4 = 15;
    let num5 = 20;
    
    // احسب المجموع
    let sum = num1 + num2 + num3 + num4 + num5;
    
    // احسب المتوسط
    let average = sum / 5;
    
    return average; // 20
}
```

### **ابحث عن أكبر رقم:**
```albayan
fn find_maximum() -> int {
    let a = 45;
    let b = 67;
    let c = 23;
    let d = 89;
    let e = 34;
    
    let max = a;
    
    if b > max {
        max = b;
    }
    if c > max {
        max = c;
    }
    if d > max {
        max = d;
    }
    if e > max {
        max = e;
    }
    
    return max; // 89
}
```

---

## 5️⃣ **اصنع "كلاس" بسيط**

### **اكتب هذا الكود:**
```albayan
// إنشاء "طالب"
fn create_student(id: int, age: int, grade: int) -> int {
    return id + (age * 1000) + (grade * 100000);
}

// الحصول على عمر الطالب
fn get_student_age(student: int) -> int {
    let temp = student / 1000;
    return temp - ((temp / 100) * 100);
}

// الحصول على درجة الطالب
fn get_student_grade(student: int) -> int {
    return student / 100000;
}
```

### **استخدم "الكلاس":**
```albayan
fn student_example() -> int {
    // إنشاء طالب: معرف=1، عمر=20، درجة=85
    let ahmed = create_student(1, 20, 85);
    
    // الحصول على معلومات الطالب
    let age = get_student_age(ahmed);     // 20
    let grade = get_student_grade(ahmed); // 85
    
    return age + grade; // 105
}
```

---

## 6️⃣ **مشروع كامل: نظام بنكي بسيط**

### **اكتب هذا الكود:**
```albayan
// إنشاء حساب بنكي
fn create_account(id: int, balance: int) -> int {
    return id + (balance * 1000);
}

// الحصول على الرصيد
fn get_balance(account: int) -> int {
    return account / 1000;
}

// إيداع مال
fn deposit(account: int, amount: int) -> int {
    let current_balance = get_balance(account);
    let account_id = account - (current_balance * 1000);
    let new_balance = current_balance + amount;
    
    return create_account(account_id, new_balance);
}

// سحب مال
fn withdraw(account: int, amount: int) -> int {
    let current_balance = get_balance(account);
    let account_id = account - (current_balance * 1000);
    
    if current_balance >= amount {
        let new_balance = current_balance - amount;
        return create_account(account_id, new_balance);
    } else {
        return account; // رصيد غير كافي
    }
}
```

### **استخدم النظام البنكي:**
```albayan
fn banking_example() -> int {
    // إنشاء حساب برقم 123 ورصيد 1000 ريال
    let my_account = create_account(123, 1000);
    
    // إيداع 500 ريال
    my_account = deposit(my_account, 500);
    
    // سحب 200 ريال
    my_account = withdraw(my_account, 200);
    
    // الحصول على الرصيد النهائي
    let final_balance = get_balance(my_account);
    
    return final_balance; // 1300
}
```

---

## 🎯 **تحديات للتدرب**

### **تحدي 1: حاسبة متقدمة**
```albayan
fn advanced_calculator(a: int, b: int, c: int) -> int {
    // احسب: (a + b) * c - 10
    let sum = a + b;
    let product = sum * c;
    let result = product - 10;
    
    return result;
}
```

### **تحدي 2: فحص الأرقام**
```albayan
fn number_checker(num: int) -> int {
    // فحص إذا كان الرقم زوجي وأكبر من 50
    let is_even = 0;
    let remainder = num - ((num / 2) * 2);
    
    if remainder == 0 {
        is_even = 1;
    }
    
    if is_even == 1 {
        if num > 50 {
            return 1; // زوجي وأكبر من 50
        } else {
            return 2; // زوجي لكن أصغر من أو يساوي 50
        }
    } else {
        return 3; // فردي
    }
}
```

### **تحدي 3: نظام درجات متقدم**
```albayan
fn advanced_grading(math: int, science: int, arabic: int) -> int {
    let total = math + science + arabic;
    let average = total / 3;
    
    if average >= 95 {
        return 1; // امتياز مع مرتبة الشرف
    } else {
        if average >= 90 {
            return 2; // امتياز
        } else {
            if average >= 85 {
                return 3; // جيد جداً مرتفع
            } else {
                if average >= 80 {
                    return 4; // جيد جداً
                } else {
                    if average >= 75 {
                        return 5; // جيد مرتفع
                    } else {
                        if average >= 70 {
                            return 6; // جيد
                        } else {
                            if average >= 65 {
                                return 7; // مقبول مرتفع
                            } else {
                                if average >= 60 {
                                    return 8; // مقبول
                                } else {
                                    return 9; // راسب
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
```

---

## 📝 **نصائح سريعة**

### **✅ افعل:**
- ابدأ بأمثلة بسيطة
- اكتب تعليقات لتذكر ما يفعله الكود
- جرب تغيير الأرقام وشاهد النتائج
- تدرب يومياً ولو 10 دقائق

### **❌ لا تفعل:**
- لا تحاول فهم كل شيء مرة واحدة
- لا تنس الفواصل المنقوطة `;`
- لا تنس الأقواس `{}`
- لا تيأس من الأخطاء - هي جزء من التعلم

---

## 🎊 **تهانينا!**

**لقد تعلمت أساسيات البرمجة بلغة البيان في 5 دقائق!**

### **ما تعلمته:**
✅ **المتغيرات** - حفظ البيانات  
✅ **الشروط** - اتخاذ القرارات  
✅ **الدوال** - تنظيم الكود  
✅ **العمليات** - الحساب والمقارنة  
✅ **محاكاة المصفوفات** - التعامل مع عدة قيم  
✅ **محاكاة الكلاسات** - إنشاء كائنات  

### **الخطوات التالية:**
1. **جرب الأمثلة** في `examples/beginner_examples.ab`
2. **اقرأ الدليل الشامل** في `docs/BEGINNER_PROGRAMMING_GUIDE.md`
3. **أنشئ مشاريعك الخاصة**
4. **شارك إنجازاتك** مع الآخرين

**🚀 أنت الآن مبرمج بلغة البيان!**

---

## 🔗 **ملفات مفيدة**

### **📚 للتعلم:**
- **`docs/BEGINNER_PROGRAMMING_GUIDE.md`** - دليل شامل مفصل
- **`examples/beginner_examples.ab`** - أمثلة عملية كثيرة
- **`docs/QUICK_START_GUIDE.md`** - هذا الدليل

### **🎨 للإبداع:**
- **`docs/DRAWING_GUIDE.md`** - تعلم الرسم والجرافيك
- **`docs/ANIMATION_DEVELOPER_GUIDE.md`** - تعلم الرسوم المتحركة
- **`examples/enhanced_artistic_demo.ab`** - أمثلة فنية

### **🤖 للذكاء الاصطناعي:**
- **`examples/ai_medical_diagnosis.ab`** - نظام طبي ذكي
- **`examples/smart_financial_trading.ab`** - نظام مالي ذكي
- **`examples/intelligent_education_system.ab`** - نظام تعليمي ذكي

**🌟 مرحباً بك في عالم البرمجة الذكية مع لغة البيان!**

---

**📅 تاريخ الإنشاء:** 2025-01-09  
**🎯 المستوى:** مبتدئ تماماً  
**⏱️ وقت التعلم:** 5 دقائق  
**✅ الحالة:** جاهز للاستخدام الفوري
