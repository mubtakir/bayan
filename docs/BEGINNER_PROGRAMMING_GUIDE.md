# 📚 **دليل تعلم لغة البيان للمبتدئين**
# Complete Beginner's Guide to AlBayan Programming Language

## 🚀 **مقدمة للمبتدئين**

**مرحباً بك في عالم البرمجة مع لغة البيان!**

هذا الدليل مصمم خصيصاً للمبرمجين المبتدئين الذين يريدون تعلم البرمجة من الصفر باللغة العربية.

---

## 📖 **الفهرس - Table of Contents**

1. **المتغيرات (Variables)**
2. **أنواع البيانات (Data Types)**
3. **العمليات الحسابية (Arithmetic Operations)**
4. **الشروط (Conditions)**
5. **الحلقات (Loops)**
6. **الدوال (Functions)**
7. **المصفوفات والقوائم (Arrays & Lists)**
8. **القواميس (Dictionaries)**
9. **الكلاسات والكائنات (Classes & Objects)**
10. **أمثلة عملية شاملة**

---

## 1️⃣ **المتغيرات (Variables)**

### **ما هو المتغير؟**
المتغير هو مكان في الذاكرة نحفظ فيه قيمة ونعطيها اسماً.

### **كيفية إنشاء متغير:**
```albayan
// إنشاء متغير رقمي
let age = 25;

// إنشاء متغير نصي
let name = "أحمد";

// إنشاء متغير منطقي
let is_student = true;
```

### **قواعد تسمية المتغيرات:**
- ✅ يمكن استخدام الحروف العربية والإنجليزية
- ✅ يمكن استخدام الأرقام (ليس في البداية)
- ✅ يمكن استخدام الشرطة السفلية `_`
- ❌ لا يمكن البدء برقم
- ❌ لا يمكن استخدام مسافات

### **أمثلة صحيحة:**
```albayan
let العمر = 30;
let student_name = "فاطمة";
let number1 = 100;
let اسم_الطالب = "محمد";
```

---

## 2️⃣ **أنواع البيانات (Data Types)**

### **الأنواع الأساسية:**

#### **🔢 الأرقام الصحيحة (Integers):**
```albayan
let count = 10;
let temperature = -5;
let score = 0;
```

#### **📝 النصوص (Strings):**
```albayan
let message = "مرحباً بالعالم";
let city = "الرياض";
let empty_text = "";
```

#### **✅ القيم المنطقية (Booleans):**
```albayan
let is_ready = true;
let is_finished = false;
```

#### **🔤 الأحرف (Characters):**
```albayan
let grade = 'A';
let symbol = '+';
```

### **مثال شامل:**
```albayan
fn data_types_example() -> int {
    let student_age = 20;           // رقم صحيح
    let student_name = "سارة";      // نص
    let is_passing = true;          // منطقي
    let grade_letter = 'B';         // حرف
    
    return student_age;
}
```

---

## 3️⃣ **العمليات الحسابية (Arithmetic Operations)**

### **العمليات الأساسية:**
```albayan
fn math_operations() -> int {
    let a = 10;
    let b = 3;
    
    let addition = a + b;        // الجمع = 13
    let subtraction = a - b;     // الطرح = 7
    let multiplication = a * b;  // الضرب = 30
    let division = a / b;        // القسمة = 3
    
    return addition + subtraction + multiplication + division;
}
```

### **عمليات المقارنة:**
```albayan
fn comparison_operations() -> int {
    let x = 5;
    let y = 10;
    
    let is_equal = x == y;       // هل متساوي؟ false
    let is_not_equal = x != y;   // هل غير متساوي؟ true
    let is_less = x < y;         // هل أصغر؟ true
    let is_greater = x > y;      // هل أكبر؟ false
    let is_less_equal = x <= y;  // هل أصغر أو يساوي؟ true
    let is_greater_equal = x >= y; // هل أكبر أو يساوي؟ false
    
    return x + y;
}
```

---

## 4️⃣ **الشروط (Conditions)**

### **الشرط البسيط (if):**
```albayan
fn simple_condition(age: int) -> int {
    if age >= 18 {
        // إذا كان العمر 18 أو أكثر
        return 1; // بالغ
    } else {
        // وإلا
        return 0; // قاصر
    }
}
```

### **الشروط المتعددة (if-else):**
```albayan
fn grade_system(score: int) -> int {
    if score >= 90 {
        return 1; // ممتاز
    } else {
        if score >= 80 {
            return 2; // جيد جداً
        } else {
            if score >= 70 {
                return 3; // جيد
            } else {
                if score >= 60 {
                    return 4; // مقبول
                } else {
                    return 5; // راسب
                }
            }
        }
    }
}
```

### **مثال عملي:**
```albayan
fn check_weather(temperature: int) -> int {
    if temperature > 35 {
        return 1; // حار جداً
    } else {
        if temperature > 25 {
            return 2; // دافئ
        } else {
            if temperature > 15 {
                return 3; // معتدل
            } else {
                return 4; // بارد
            }
        }
    }
}
```

---

## 5️⃣ **الحلقات (Loops)**

### **⚠️ ملاحظة مهمة:**
لغة البيان حالياً لا تدعم الحلقات التقليدية (for, while) بشكل مباشر، لكن يمكن محاكاتها باستخدام الدوال والاستدعاء المتكرر.

### **محاكاة حلقة العد:**
```albayan
fn count_loop(current: int, target: int, sum: int) -> int {
    if current > target {
        return sum;
    } else {
        let new_sum = sum + current;
        let next = current + 1;
        return count_loop(next, target, new_sum);
    }
}

fn sum_numbers_1_to_10() -> int {
    return count_loop(1, 10, 0); // مجموع الأرقام من 1 إلى 10
}
```

### **محاكاة حلقة الشرط:**
```albayan
fn factorial(n: int, result: int) -> int {
    if n <= 1 {
        return result;
    } else {
        return factorial(n - 1, result * n);
    }
}

fn calculate_factorial() -> int {
    return factorial(5, 1); // 5! = 120
}
```

---

## 6️⃣ **الدوال (Functions)**

### **إنشاء دالة بسيطة:**
```albayan
fn say_hello() -> int {
    return 1; // تم تنفيذ التحية
}
```

### **دالة مع معاملات:**
```albayan
fn add_numbers(a: int, b: int) -> int {
    let result = a + b;
    return result;
}
```

### **دالة مع عدة معاملات:**
```albayan
fn calculate_area(length: int, width: int, height: int) -> int {
    let area = length * width;
    let volume = area * height;
    return volume;
}
```

### **دالة تستدعي دالة أخرى:**
```albayan
fn multiply(x: int, y: int) -> int {
    return x * y;
}

fn calculate_square(number: int) -> int {
    return multiply(number, number);
}

fn power_of_three(base: int) -> int {
    let square = calculate_square(base);
    return multiply(square, base);
}
```

### **مثال شامل للدوال:**
```albayan
fn student_grade_calculator(math: int, science: int, arabic: int) -> int {
    let total = math + science + arabic;
    let average = total / 3;
    
    if average >= 90 {
        return 1; // ممتاز
    } else {
        if average >= 80 {
            return 2; // جيد جداً
        } else {
            if average >= 70 {
                return 3; // جيد
            } else {
                return 4; // مقبول
            }
        }
    }
}
```

---

## 7️⃣ **المصفوفات والقوائم (Arrays & Lists)**

### **⚠️ ملاحظة:**
لغة البيان حالياً لا تدعم المصفوفات بالصيغة التقليدية، لكن يمكن محاكاتها باستخدام متغيرات متعددة ودوال.

### **محاكاة مصفوفة بسيطة:**
```albayan
fn array_simulation() -> int {
    // محاكاة مصفوفة من 5 عناصر
    let element1 = 10;
    let element2 = 20;
    let element3 = 30;
    let element4 = 40;
    let element5 = 50;
    
    // حساب المجموع
    let sum = element1 + element2 + element3 + element4 + element5;
    
    return sum; // 150
}
```

### **دالة للوصول لعنصر في المصفوفة:**
```albayan
fn get_array_element(index: int) -> int {
    if index == 1 {
        return 100; // العنصر الأول
    } else {
        if index == 2 {
            return 200; // العنصر الثاني
        } else {
            if index == 3 {
                return 300; // العنصر الثالث
            } else {
                if index == 4 {
                    return 400; // العنصر الرابع
                } else {
                    return 500; // العنصر الخامس
                }
            }
        }
    }
}

fn array_operations() -> int {
    let first = get_array_element(1);
    let second = get_array_element(2);
    let third = get_array_element(3);
    
    return first + second + third; // 600
}
```

---

## 8️⃣ **القواميس (Dictionaries)**

### **محاكاة قاموس بسيط:**
```albayan
fn student_info(info_type: int) -> int {
    // قاموس معلومات الطالب
    // 1 = العمر، 2 = الدرجة، 3 = السنة الدراسية
    
    if info_type == 1 {
        return 20; // العمر
    } else {
        if info_type == 2 {
            return 85; // الدرجة
        } else {
            if info_type == 3 {
                return 3; // السنة الدراسية
            } else {
                return 0; // غير موجود
            }
        }
    }
}

fn use_student_dictionary() -> int {
    let age = student_info(1);
    let grade = student_info(2);
    let year = student_info(3);
    
    return age + grade + year; // 108
}
```

---

## 9️⃣ **الكلاسات والكائنات (Classes & Objects)**

### **⚠️ ملاحظة:**
لغة البيان حالياً لا تدعم الكلاسات بالصيغة التقليدية، لكن يمكن محاكاة السلوك باستخدام الدوال.

### **محاكاة كلاس بسيط:**
```albayan
// محاكاة كلاس "طالب"
fn create_student(name_id: int, age: int, grade: int) -> int {
    // إنشاء "كائن" طالب
    // نعيد معرف فريد للطالب
    return name_id + (age * 1000) + (grade * 100000);
}

fn get_student_age(student_object: int) -> int {
    // استخراج العمر من "الكائن"
    let temp = student_object / 1000;
    let age = temp - ((temp / 100) * 100);
    return age;
}

fn get_student_grade(student_object: int) -> int {
    // استخراج الدرجة من "الكائن"
    return student_object / 100000;
}

fn student_class_example() -> int {
    // إنشاء طالب: معرف=1، عمر=20، درجة=85
    let student1 = create_student(1, 20, 85);
    
    // الحصول على معلومات الطالب
    let age = get_student_age(student1);
    let grade = get_student_grade(student1);
    
    return age + grade; // 105
}
```

### **محاكاة كلاس أكثر تعقيداً:**
```albayan
// محاكاة كلاس "حساب بنكي"
fn create_bank_account(account_id: int, initial_balance: int) -> int {
    return account_id + (initial_balance * 1000);
}

fn deposit_money(account: int, amount: int) -> int {
    let current_balance = account / 1000;
    let account_id = account - (current_balance * 1000);
    let new_balance = current_balance + amount;
    return account_id + (new_balance * 1000);
}

fn withdraw_money(account: int, amount: int) -> int {
    let current_balance = account / 1000;
    let account_id = account - (current_balance * 1000);
    
    if current_balance >= amount {
        let new_balance = current_balance - amount;
        return account_id + (new_balance * 1000);
    } else {
        return account; // لا يمكن السحب - رصيد غير كافي
    }
}

fn get_balance(account: int) -> int {
    return account / 1000;
}

fn bank_account_example() -> int {
    // إنشاء حساب برقم 123 ورصيد 1000
    let my_account = create_bank_account(123, 1000);
    
    // إيداع 500
    my_account = deposit_money(my_account, 500);
    
    // سحب 200
    my_account = withdraw_money(my_account, 200);
    
    // الحصول على الرصيد الحالي
    let final_balance = get_balance(my_account);
    
    return final_balance; // 1300
}
```

---

## 🎯 **أمثلة عملية شاملة**

### **مثال 1: حاسبة بسيطة**
```albayan
fn calculator(operation: int, a: int, b: int) -> int {
    // 1=جمع، 2=طرح، 3=ضرب، 4=قسمة
    
    if operation == 1 {
        return a + b;
    } else {
        if operation == 2 {
            return a - b;
        } else {
            if operation == 3 {
                return a * b;
            } else {
                if operation == 4 {
                    if b != 0 {
                        return a / b;
                    } else {
                        return 0; // خطأ: قسمة على صفر
                    }
                } else {
                    return 0; // عملية غير صحيحة
                }
            }
        }
    }
}

fn calculator_test() -> int {
    let addition = calculator(1, 10, 5);      // 15
    let subtraction = calculator(2, 10, 3);   // 7
    let multiplication = calculator(3, 4, 6); // 24
    let division = calculator(4, 20, 4);      // 5
    
    return addition + subtraction + multiplication + division; // 51
}
```

### **مثال 2: نظام إدارة المكتبة**
```albayan
fn create_book(book_id: int, pages: int, year: int) -> int {
    return book_id + (pages * 1000) + (year * 1000000);
}

fn get_book_pages(book: int) -> int {
    let temp = book / 1000;
    let pages = temp - ((temp / 1000) * 1000);
    return pages;
}

fn get_book_year(book: int) -> int {
    return book / 1000000;
}

fn is_book_old(book: int, current_year: int) -> int {
    let book_year = get_book_year(book);
    let age = current_year - book_year;
    
    if age > 10 {
        return 1; // كتاب قديم
    } else {
        return 0; // كتاب حديث
    }
}

fn library_system() -> int {
    // إنشاء كتاب: معرف=1، صفحات=300، سنة=2020
    let book1 = create_book(1, 300, 2020);
    
    // إنشاء كتاب آخر: معرف=2، صفحات=150، سنة=2010
    let book2 = create_book(2, 150, 2010);
    
    // فحص إذا كان الكتاب قديماً (السنة الحالية 2024)
    let book1_old = is_book_old(book1, 2024); // 0 (حديث)
    let book2_old = is_book_old(book2, 2024); // 1 (قديم)
    
    // حساب مجموع الصفحات
    let total_pages = get_book_pages(book1) + get_book_pages(book2);
    
    return total_pages + book1_old + book2_old; // 451
}
```

---

## 📝 **نصائح للمبتدئين**

### **1. ابدأ بالبساطة:**
- اكتب برامج بسيطة أولاً
- لا تحاول فهم كل شيء مرة واحدة
- مارس كتابة الكود يومياً

### **2. فهم الأخطاء:**
- اقرأ رسائل الخطأ بعناية
- تأكد من الأقواس والفواصل المنقوطة
- تحقق من أسماء المتغيرات

### **3. التدرب المستمر:**
- اكتب أمثلة صغيرة
- جرب تعديل الأمثلة الموجودة
- أنشئ مشاريع بسيطة

### **4. استخدم التعليقات:**
```albayan
// هذا تعليق يشرح ما يفعله الكود
fn example() -> int {
    let x = 5; // متغير يحمل الرقم 5
    return x;
}
```

---

## 🎊 **الخلاصة**

**تهانينا! لقد تعلمت أساسيات لغة البيان:**

✅ **المتغيرات** - كيفية حفظ البيانات  
✅ **أنواع البيانات** - أرقام، نصوص، منطقية  
✅ **العمليات** - حسابية ومقارنة  
✅ **الشروط** - اتخاذ القرارات  
✅ **الدوال** - تنظيم الكود  
✅ **محاكاة المصفوفات** - التعامل مع البيانات المتعددة  
✅ **محاكاة الكلاسات** - البرمجة الكائنية  

**🚀 الآن أنت جاهز لكتابة برامجك الأولى بلغة البيان!**

---

**📅 تاريخ الإنشاء:** 2025-01-09  
**🎯 المستوى:** مبتدئ  
**✅ الحالة:** دليل شامل ومُختبر  
**📚 النوع:** تعليمي تفاعلي
