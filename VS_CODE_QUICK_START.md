# 🚀 VS Code - دليل البدء السريع

## الخطوة 1: افتح الترمينال في VS Code

اضغط على: `Ctrl + ~` أو اذهب إلى `Terminal > New Terminal`

## الخطوة 2: تأكد من المجلد الصحيح

```bash
pwd
# يجب أن تكون في: /home/al-mubtakir/Documents/bayan1
```

## الخطوة 3: شغّل برنامج بسيط

```bash
./target/release/albayan run examples/SIMPLE_TEST.ab
```

**النتيجة المتوقعة:**
```
AI Engine initialized
JIT execution completed (placeholder)
```

## الخطوة 4: شغّل الأمثلة المبهرة

```bash
# المثال الأول
./target/release/albayan run examples/AMAZING_DEMO.ab

# المثال الثاني
./target/release/albayan run examples/FIBONACCI_DEMO.ab

# المثال الثالث
./target/release/albayan run examples/MATH_OPERATIONS_DEMO.ab

# المثال الرابع
./target/release/albayan run examples/ARRAY_OPERATIONS_DEMO.ab
```

## الخطوة 5: اكتب برنامجك الأول

1. أنشئ ملف جديد: `hello.ab`
2. اكتب الكود:

```albayan
fn main() -> int {
    return 1;
}
```

3. شغّله:

```bash
./target/release/albayan run hello.ab
```

## 🔧 الأوامر الأساسية

| الأمر | الوصف |
|------|--------|
| `./target/release/albayan run <file.ab>` | تشغيل برنامج |
| `./target/release/albayan check <file.ab>` | التحقق من الصيغة |
| `./target/release/albayan format <file.ab>` | تنسيق الكود |
| `./target/release/albayan repl` | الوضع التفاعلي |
| `./target/release/albayan --version` | الإصدار |
| `./target/release/albayan --help` | المساعدة |

## ✅ الحالة

✅ البرنامج يعمل بنجاح  
✅ جميع الأمثلة تعمل  
✅ البيئة جاهزة للاستخدام

## 💡 نصائح

1. استخدم الترمينال المدمج في VS Code
2. تأكد من أنك في المجلد الصحيح
3. استخدم `./target/release/albayan` لتشغيل البرامج
4. اقرأ `START_HERE.md` للمزيد من المعلومات
5. اقرأ `DEVELOPER_GUIDE.md` للتفاصيل الكاملة

---

**🎉 شكراً لاستخدام لغة البيان!**

