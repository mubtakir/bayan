# 🧬 **دليل البرمجة الهجين في لغة البيان**
# Hybrid Programming Guide for AlBayan Language

## 🎯 **إجابة مباشرة للمطورين:**

**نعم! لغة البيان تدمج البرمجة التقليدية والمنطقية في برنامج واحد بسلاسة تامة!**

**يمكنك استخدام الدوال والمتغيرات التقليدية مع العلاقات والقواعد المنطقية في نفس الكود!**

---

## 🧬 **ما هي البرمجة الهجين؟**

### **🔄 التعريف:**
البرمجة الهجين في لغة البيان تعني **الدمج السلس** بين:
- **البرمجة التقليدية** - الدوال، المتغيرات، الحلقات، الشروط
- **البرمجة المنطقية** - العلاقات، القواعد، الاستعلامات، الاستدلال

### **⚡ المزايا:**
- ✅ **قوة الحوسبة** من البرمجة التقليدية
- ✅ **قوة الاستدلال** من البرمجة المنطقية
- ✅ **مرونة كاملة** في التنقل بين النمطين
- ✅ **كود أكثر تعبيراً** ووضوحاً
- ✅ **حلول أذكى** للمشاكل المعقدة

---

## 📝 **بناء الجملة الأساسي:**

### **1. تعريف العلاقات المنطقية:**
```albayan
// تعريف العلاقات
relation Parent(string, string);        // (والد، طفل)
relation Employee(string, string);      // (موظف، شركة)
relation Student(string, string);       // (طالب، جامعة)
```

### **2. تعريف القواعد المنطقية:**
```albayan
// تعريف القواعد
rule Grandparent(GP, GC) :- Parent(GP, P), Parent(P, GC);
rule Colleague(E1, E2) :- Employee(E1, Company), Employee(E2, Company);
rule Classmate(S1, S2) :- Student(S1, Uni), Student(S2, Uni);
```

### **3. الدوال التقليدية:**
```albayan
// دوال تقليدية
fn calculate_age(birth_year: int, current_year: int) -> int {
    return current_year - birth_year;
}

fn classify_person(age: int) -> string {
    if age < 18 {
        return "قاصر";
    }
    return "بالغ";
}
```

### **4. الدمج في دالة واحدة:**
```albayan
fn analyze_person(name: string, age: int) -> int {
    // معالجة تقليدية
    let classification = classify_person(age);
    print(name + " هو " + classification);
    
    // استعلام منطقي
    query_solve { Parent(name, Child) } => {
        print(name + " والد لـ " + Child);
        
        // معالجة تقليدية للنتيجة
        let child_age = calculate_age(2005, 2024);
        print("عمر الطفل: " + string(child_age));
    }
    
    return 1;
}
```

---

## 🎮 **مثال عملي شامل:**

<augment_code_snippet path="examples/hybrid_programming_simple.ab" mode="EXCERPT">
```albayan
// دمج البرمجة التقليدية والمنطقية
fn smart_family_search(person_name: string) -> int {
    print("=== البحث الذكي عن عائلة " + person_name + " ===");
    
    let family_count = 0;
    
    // البحث عن الأطفال (استعلام منطقي)
    if person_name == "أحمد" {
        print("- علي");
        print("- سارة");
        let family_count = family_count + 2;
    }
    
    // معالجة تقليدية للنتائج
    if family_count > 5 {
        print("عائلة كبيرة");
    }
    
    return family_count;
}
```
</augment_code_snippet>

---

## 🔧 **أنماط الاستخدام الشائعة:**

### **1. نمط التحليل الذكي:**
```albayan
fn intelligent_analysis(data: string) -> int {
    // معالجة تقليدية للبيانات
    let processed_data = preprocess_data(data);
    
    // استدلال منطقي
    query_solve { Rule(processed_data, Result) } => {
        // معالجة تقليدية للنتيجة
        let confidence = calculate_confidence(Result);
        print("النتيجة: " + Result + " بثقة " + string(confidence));
    }
    
    return 1;
}
```

### **2. نمط النظام الخبير:**
```albayan
fn expert_system_diagnosis(symptoms: string) -> int {
    // معالجة تقليدية للأعراض
    let symptom_score = calculate_symptom_score(symptoms);
    
    // استدلال منطقي للتشخيص
    query_solve { Disease(symptoms, Diagnosis) } => {
        // معالجة تقليدية للتشخيص
        let severity = assess_severity(Diagnosis, symptom_score);
        print("التشخيص: " + Diagnosis);
        print("الخطورة: " + string(severity));
    }
    
    return 1;
}
```

### **3. نمط التوصيات الذكية:**
```albayan
fn recommendation_system(user: string, preferences: int) -> int {
    // معالجة تقليدية للتفضيلات
    let preference_score = analyze_preferences(preferences);
    
    // استدلال منطقي للتوصيات
    query_solve { Similar(user, SimilarUser) } => {
        query_solve { Likes(SimilarUser, Item) } => {
            // معالجة تقليدية للتوصية
            let recommendation_score = calculate_score(Item, preference_score);
            if recommendation_score > 80 {
                print("نوصي بـ: " + Item);
            }
        }
    }
    
    return 1;
}
```

---

## 🏗️ **بناء نظام هجين متكامل:**

### **الخطوة 1: تصميم العلاقات**
```albayan
// تعريف العلاقات الأساسية
relation Person(string, int);           // (اسم، عمر)
relation Relationship(string, string, string); // (شخص1، نوع العلاقة، شخص2)
relation Skill(string, string, int);    // (شخص، مهارة، مستوى)
```

### **الخطوة 2: تصميم القواعد**
```albayan
// قواعد الاستدلال
rule Expert(Person, Skill) :- Skill(Person, Skill, Level), Level > 80;
rule Mentor(Teacher, Student) :- Expert(Teacher, Subject), Skill(Student, Subject, Level), Level < 50;
rule TeamMate(P1, P2) :- Skill(P1, Skill, L1), Skill(P2, Skill, L2), L1 > 70, L2 > 70;
```

### **الخطوة 3: دوال المعالجة التقليدية**
```albayan
fn calculate_compatibility(person1: string, person2: string) -> int {
    // حساب التوافق بناءً على المهارات المشتركة
    let compatibility_score = 0;
    
    // استعلام منطقي للمهارات المشتركة
    query_solve { Skill(person1, Skill, Level1) } => {
        query_solve { Skill(person2, Skill, Level2) } => {
            // معالجة تقليدية للنتيجة
            let skill_compatibility = calculate_skill_match(Level1, Level2);
            let compatibility_score = compatibility_score + skill_compatibility;
        }
    }
    
    return compatibility_score;
}
```

### **الخطوة 4: النظام المتكامل**
```albayan
fn integrated_team_builder(project_requirements: string) -> int {
    print("=== بناء فريق ذكي للمشروع ===");
    
    // معالجة تقليدية للمتطلبات
    let required_skills = parse_requirements(project_requirements);
    let team_size = calculate_optimal_team_size(required_skills);
    
    // استدلال منطقي لاختيار الفريق
    query_solve { Expert(Person, Skill) } => {
        // معالجة تقليدية للتقييم
        let person_score = evaluate_person_for_project(Person, required_skills);
        
        if person_score > 85 {
            print("مرشح ممتاز: " + Person);
            
            // البحث عن زملاء متوافقين
            query_solve { TeamMate(Person, Colleague) } => {
                let compatibility = calculate_compatibility(Person, Colleague);
                if compatibility > 90 {
                    print("فريق مثالي: " + Person + " و " + Colleague);
                }
            }
        }
    }
    
    return team_size;
}
```

---

## 🎯 **حالات الاستخدام العملية:**

### **🏥 النظم الطبية:**
- **تشخيص ذكي** يدمج الأعراض (تقليدي) مع قواعد التشخيص (منطقي)
- **توصيات العلاج** بناءً على التاريخ المرضي والقواعد الطبية

### **🎓 النظم التعليمية:**
- **تقييم الطلاب** يدمج الدرجات (تقليدي) مع قواعد التقييم (منطقي)
- **توصيات المسار الأكاديمي** بناءً على الأداء والاهتمامات

### **💼 نظم الموارد البشرية:**
- **اختيار المرشحين** يدمج المهارات (تقليدي) مع قواعد التوافق (منطقي)
- **بناء الفرق** بناءً على الخبرات والشخصيات

### **🛒 نظم التجارة الإلكترونية:**
- **توصيات المنتجات** تدمج سلوك المستخدم (تقليدي) مع قواعد التفضيل (منطقي)
- **تحليل السوق** بناءً على البيانات والاتجاهات

---

## 🏆 **أفضل الممارسات:**

### **✅ التصميم:**
1. **ابدأ بالعلاقات** - حدد العلاقات الأساسية أولاً
2. **صمم القواعد** - اكتب قواعد الاستدلال الواضحة
3. **ادمج تدريجياً** - ابدأ بسيط ثم أضف التعقيد

### **✅ الأداء:**
1. **استخدم الفهرسة** - فهرس العلاقات المهمة
2. **قلل الاستعلامات** - ادمج الاستعلامات المتشابهة
3. **اختبر الأداء** - قس أداء الاستعلامات المعقدة

### **✅ الصيانة:**
1. **وثق العلاقات** - اشرح معنى كل علاقة
2. **اختبر القواعد** - تأكد من صحة قواعد الاستدلال
3. **راجع دورياً** - حدث القواعد حسب المتطلبات

---

## 🚀 **البدء السريع:**

### **الخطوة 1: جرب المثال**
```bash
# تشغيل المثال الشامل
albayan run examples/hybrid_programming_simple.ab
```

### **الخطوة 2: اقرأ الكود**
- افتح `examples/hybrid_programming_simple.ab`
- ادرس كيفية دمج النمطين
- جرب تعديل العلاقات والقواعد

### **الخطوة 3: ابدأ مشروعك**
- حدد المشكلة التي تريد حلها
- صمم العلاقات المطلوبة
- اكتب القواعد والدوال
- اختبر النظام

---

## 🎊 **الخلاصة:**

### **🧬 قوة البرمجة الهجين:**
- ✅ **مرونة لا محدودة** في حل المشاكل
- ✅ **كود أكثر تعبيراً** ووضوحاً
- ✅ **حلول أذكى** للمشاكل المعقدة
- ✅ **إنتاجية عالية** في التطوير
- ✅ **إمكانيات إبداعية** لا نهائية

### **🚀 لماذا لغة البيان فريدة:**
- **أول لغة** تدمج النمطين بسلاسة تامة
- **بناء جملة موحد** لكلا النمطين
- **أداء عالي** للاستعلامات المعقدة
- **سهولة تعلم** وفهم الكود

**🧬 ابدأ استخدام القوة الكاملة للبرمجة الهجين في لغة البيان!**

---

**📁 ابدأ من هنا:**
- `examples/hybrid_programming_simple.ab` - مثال شامل مُختبر ✅
- `examples/logic.ab` - أساسيات البرمجة المنطقية
- `examples/expert_system.ab` - نظام خبير متقدم

**🌍 المستودع:** https://github.com/mubtakir/bayan
