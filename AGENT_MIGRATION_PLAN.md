# 🚀 خطة هجرة الوكيل من Python إلى البيان
# Agent Migration Implementation Plan

**الحالة:** جاهز للتنفيذ  
**المدة المتوقعة:** 5-7 أيام عمل  
**الأولوية:** عالية جداً

---

## 📋 المرحلة 1: البنية الأساسية (اليوم 1-2)

### المهمة 1.1: إنشاء ملفات الوحدات
```bash
std/agent/
├── mod.ab              # وحدة الوكيل الرئيسية
├── types.ab            # تعريف الأنواع
├── nlu.ab              # فهم النية
├── nlg.ab              # توليد الرسائل
├── optimizer.ab        # التحسينات
└── bridge.ab           # جسر التكامل
```

### المهمة 1.2: تعريف الأنواع الأساسية
```albayan
// types.ab
struct Intent {
    intent: string,
    target: string,
    code: string,
    hint: string,
}

struct Response {
    intent: string,
    text: string,
    data: list<string>,
    ok: bool,
}

struct Config {
    language: string,
    dry_run: bool,
    bayan_binary: string,
}

struct Event {
    intent: string,
    ok: bool,
    timestamp: int,
}
```

### المهمة 1.3: وحدة قراءة الإعدادات
```albayan
// mod.ab
fn load_config(path: string) -> Config {
    // قراءة config.yaml
    // تحليل بسيط
    // إرجاع Config
}

fn create_agent(config_path: string) -> Agent {
    let config = load_config(config_path);
    return Agent {
        config: config,
        nlu: create_nlu(),
        nlg: create_nlg(config.language),
        optimizer: create_optimizer(),
    };
}
```

---

## 📋 المرحلة 2: وحدة فهم النية (اليوم 2-3)

### المهمة 2.1: تحويل SimpleNLU
```albayan
// nlu.ab
fn parse_intent(text: string) -> Intent {
    // تحليل الأوامر العربية
    if contains(text, "شغّل") || contains(text, "run") {
        return Intent {
            intent: "run",
            target: extract_path(text),
            code: "",
            hint: "",
        };
    }
    
    if contains(text, "ترجم") || contains(text, "compile") {
        return Intent {
            intent: "compile",
            target: extract_path(text),
            code: "",
            hint: "",
        };
    }
    
    if contains(text, "حلّل") || contains(text, "analyze") {
        return Intent {
            intent: "analyze",
            target: "",
            code: extract_code(text),
            hint: "",
        };
    }
    
    return Intent {
        intent: "chat",
        target: "",
        code: "",
        hint: text,
    };
}

fn extract_path(text: string) -> string {
    // استخراج المسار من النص
    // مثال: "شغّل examples/hello.ab"
    // النتيجة: "examples/hello.ab"
}

fn extract_code(text: string) -> string {
    // استخراج الكود من النص
    // بحث عن ``` ... ```
}
```

### المهمة 2.2: اختبارات NLU
```albayan
fn test_nlu() {
    let intent1 = parse_intent("شغّل examples/hello.ab");
    assert(intent1.intent == "run");
    
    let intent2 = parse_intent("ترجم test.ab");
    assert(intent2.intent == "compile");
    
    let intent3 = parse_intent("حلّل الكود");
    assert(intent3.intent == "analyze");
}
```

---

## 📋 المرحلة 3: وحدة توليد الرسائل (اليوم 3-4)

### المهمة 3.1: تحويل SimpleNLG
```albayan
// nlg.ab
fn generate_reply(language: string, intent: string, data: list<string>) -> string {
    if language == "ar" {
        return generate_reply_ar(intent, data);
    }
    return generate_reply_en(intent, data);
}

fn generate_reply_ar(intent: string, data: list<string>) -> string {
    if intent == "run" {
        return "تم التشغيل بنجاح!";
    }
    if intent == "compile" {
        return "تمت الترجمة بنجاح!";
    }
    if intent == "analyze" {
        return "تم التحليل: " + data[0];
    }
    return "مرحباً! كيف أساعدك؟";
}

fn generate_reply_en(intent: string, data: list<string>) -> string {
    if intent == "run" {
        return "Execution successful!";
    }
    if intent == "compile" {
        return "Compilation successful!";
    }
    if intent == "analyze" {
        return "Analysis: " + data[0];
    }
    return "Hello! How can I help?";
}
```

---

## 📋 المرحلة 4: وحدة التحسينات (اليوم 4)

### المهمة 4.1: تحويل SimpleOptimizer
```albayan
// optimizer.ab
struct Optimizer {
    events: list<Event>,
}

fn create_optimizer() -> Optimizer {
    return Optimizer { events: [] };
}

fn observe_event(optimizer: mut Optimizer, event: Event) {
    optimizer.events.push(event);
}

fn propose_optimizations(optimizer: Optimizer) -> list<string> {
    let mut recommendations = [];
    let run_count = count_by_intent(optimizer.events, "run");
    
    if run_count >= 3 {
        recommendations.push("جرّب التخزين المؤقت");
    }
    
    return recommendations;
}
```

---

## 📋 المرحلة 5: التكامل والاختبارات (اليوم 5-7)

### المهمة 5.1: واجهة CLI
```albayan
fn main() -> int {
    let agent = create_agent("agent/config.yaml");
    
    loop {
        print("> ");
        let input = read_line();
        
        if input == "exit" || input == "quit" {
            break;
        }
        
        let response = process_input(agent, input);
        print(response.text);
    }
    
    return 0;
}

fn process_input(agent: Agent, input: string) -> Response {
    let intent = parse_intent(input);
    let reply = generate_reply(agent.config.language, intent.intent, []);
    
    return Response {
        intent: intent.intent,
        text: reply,
        data: [],
        ok: true,
    };
}
```

### المهمة 5.2: الاختبارات الشاملة
```albayan
fn run_all_tests() {
    test_nlu();
    test_nlg();
    test_optimizer();
    test_integration();
    print("✅ جميع الاختبارات نجحت!");
}
```

---

## 🎯 معايير النجاح

- ✅ جميع الوحدات تعمل بشكل مستقل
- ✅ التكامل بين الوحدات يعمل
- ✅ الاختبارات تمر بنجاح
- ✅ الأداء أفضل من Python
- ✅ الكود موثق جيداً
- ✅ دعم عربي كامل

---

## 📝 الخطوات التالية

1. ✅ الموافقة على الخطة
2. ⏳ بدء المرحلة 1
3. ⏳ بدء المرحلة 2
4. ⏳ بدء المرحلة 3
5. ⏳ بدء المرحلة 4
6. ⏳ بدء المرحلة 5
7. ⏳ الدفع إلى GitHub

**هل تريد أن نبدأ الآن؟ 🚀**

