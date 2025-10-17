# 📚 أمثلة عملية: مكتبات الرياضيات والذكاء الاصطناعي في البيان
# Practical Examples: Math & AI Libraries in AlBayan

## 1️⃣ مثال NDArray الأساسي

```albayan
use math::ndarray;

fn main() -> int {
    // إنشاء مصفوفة
    let arr = ndarray::zeros([3, 4]);
    
    // ملء البيانات
    arr.set([0, 0], 1.5);
    arr.set([1, 1], 2.5);
    
    // الحصول على البيانات
    let value = arr.get([0, 0]);
    print("القيمة: " + value.to_string());
    
    // العمليات الأساسية
    let arr2 = ndarray::ones([3, 4]);
    let result = arr.add(arr2);
    
    return 0;
}
```

---

## 2️⃣ مثال ضرب المصفوفات

```albayan
use math::matrix;

fn main() -> int {
    // إنشاء مصفوفتين
    let a = matrix::Matrix::new([
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0]
    ]);
    
    let b = matrix::Matrix::new([
        [7.0, 8.0],
        [9.0, 10.0],
        [11.0, 12.0]
    ]);
    
    // ضرب المصفوفات
    let result = a.multiply(b);
    
    // طباعة النتيجة
    result.print();
    
    return 0;
}
```

---

## 3️⃣ مثال الإحصائيات

```albayan
use math::statistics;

fn main() -> int {
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    
    // الحسابات الإحصائية
    let mean = statistics::mean(data);
    let median = statistics::median(data);
    let std = statistics::std(data);
    let variance = statistics::var(data);
    
    print("المتوسط: " + mean.to_string());
    print("الوسيط: " + median.to_string());
    print("الانحراف المعياري: " + std.to_string());
    print("التباين: " + variance.to_string());
    
    return 0;
}
```

---

## 4️⃣ مثال الشبكة العصبية البسيطة

```albayan
use ai::neural_network;

fn main() -> int {
    // إنشاء شبكة عصبية
    let mut nn = neural_network::NeuralNetwork::new([
        784,  // input layer (28x28 صورة)
        128,  // hidden layer 1
        64,   // hidden layer 2
        10    // output layer (10 أرقام)
    ]);
    
    // تعيين معدل التعلم
    nn.set_learning_rate(0.01);
    
    // تحميل البيانات
    let training_data = load_mnist_data();
    
    // التدريب
    for epoch in 0..100 {
        for (image, label) in training_data {
            let output = nn.forward(image);
            let loss = nn.compute_loss(output, label);
            nn.backward(loss);
            nn.update_weights();
        }
        
        if epoch % 10 == 0 {
            print("Epoch " + epoch.to_string() + " - Loss: " + loss.to_string());
        }
    }
    
    return 0;
}
```

---

## 5️⃣ مثال التعلم الآلي - Linear Regression

```albayan
use ai::regression;

fn main() -> int {
    // البيانات: (x, y)
    let x_data = [[1.0], [2.0], [3.0], [4.0], [5.0]];
    let y_data = [[2.0], [4.0], [6.0], [8.0], [10.0]];
    
    // إنشاء نموذج الانحدار الخطي
    let mut model = regression::LinearRegression::new();
    
    // التدريب
    model.fit(x_data, y_data);
    
    // التنبؤ
    let prediction = model.predict([[6.0]]);
    print("التنبؤ للقيمة 6: " + prediction.to_string());
    
    // الحصول على المعاملات
    let slope = model.get_slope();
    let intercept = model.get_intercept();
    
    print("الميل: " + slope.to_string());
    print("التقاطع: " + intercept.to_string());
    
    return 0;
}
```

---

## 6️⃣ مثال التصنيف - Logistic Regression

```albayan
use ai::classification;

fn main() -> int {
    // بيانات التدريب
    let x_train = load_training_features();
    let y_train = load_training_labels();
    
    // إنشاء نموذج الانحدار اللوجستي
    let mut model = classification::LogisticRegression::new();
    
    // التدريب
    model.fit(x_train, y_train, 0.01, 1000);
    
    // التنبؤ
    let x_test = load_test_features();
    let predictions = model.predict(x_test);
    
    // حساب الدقة
    let accuracy = model.score(x_test, load_test_labels());
    print("الدقة: " + accuracy.to_string());
    
    return 0;
}
```

---

## 7️⃣ مثال التجميع - K-Means

```albayan
use ai::clustering;

fn main() -> int {
    // البيانات
    let data = load_data();
    
    // إنشاء نموذج K-Means
    let mut kmeans = clustering::KMeans::new(3);  // 3 مجموعات
    
    // التدريب
    kmeans.fit(data, 100);  // 100 تكرار
    
    // الحصول على المراكز
    let centers = kmeans.get_centers();
    
    // تصنيف البيانات الجديدة
    let new_data = [[1.0, 2.0], [5.0, 6.0]];
    let labels = kmeans.predict(new_data);
    
    print("التصنيفات: " + labels.to_string());
    
    return 0;
}
```

---

## 8️⃣ مثال معالجة الصور

```albayan
use ai::image_processing;

fn main() -> int {
    // تحميل الصورة
    let img = image_processing::load_image("photo.jpg");
    
    // تحويل الصورة
    let rotated = img.rotate(45.0);
    let scaled = img.scale(0.5);
    let cropped = img.crop([10, 10, 100, 100]);
    
    // الكشف عن الحواف
    let edges = img.detect_edges();
    
    // التمويه
    let blurred = img.blur(5);
    
    // حفظ النتائج
    rotated.save("rotated.jpg");
    edges.save("edges.jpg");
    blurred.save("blurred.jpg");
    
    return 0;
}
```

---

## 9️⃣ مثال معالجة اللغة الطبيعية

```albayan
use ai::nlp;

fn main() -> int {
    let text = "مرحبا بك في لغة البيان الرائعة!";
    
    // التحليل الأساسي
    let tokens = nlp::tokenize(text);
    let lemmas = nlp::lemmatize(tokens);
    
    // إزالة الكلمات الشائعة
    let filtered = nlp::remove_stopwords(tokens);
    
    // تحليل المشاعر
    let sentiment = nlp::analyze_sentiment(text);
    print("المشاعر: " + sentiment.to_string());
    
    // التمثيل الدلالي
    let embedding = nlp::get_embedding(text);
    
    // البحث عن الكيانات المسماة
    let entities = nlp::extract_entities(text);
    
    return 0;
}
```

---

## 🔟 مثال متقدم: نظام التوصيات

```albayan
use ai::recommendation;
use math::matrix;

fn main() -> int {
    // مصفوفة التقييمات (المستخدمون × المنتجات)
    let ratings = matrix::Matrix::new([
        [5.0, 3.0, 0.0, 1.0],
        [4.0, 0.0, 0.0, 1.0],
        [1.0, 1.0, 0.0, 5.0],
        [1.0, 0.0, 0.0, 4.0],
        [0.0, 1.0, 5.0, 4.0]
    ]);
    
    // إنشاء نموذج التوصيات
    let mut recommender = recommendation::CollaborativeFiltering::new();
    
    // التدريب
    recommender.fit(ratings);
    
    // الحصول على التوصيات للمستخدم 0
    let recommendations = recommender.recommend(0, 2);
    
    print("التوصيات: " + recommendations.to_string());
    
    return 0;
}
```

---

## 📊 مقارنة الأداء

```albayan
use math::matrix;
use time;

fn main() -> int {
    let size = 1000;
    
    // إنشاء مصفوفات كبيرة
    let a = matrix::Matrix::random([size, size]);
    let b = matrix::Matrix::random([size, size]);
    
    // قياس الأداء
    let start = time::now();
    let result = a.multiply(b);
    let elapsed = time::now() - start;
    
    print("وقت الضرب: " + elapsed.to_string() + " ms");
    print("العمليات: " + (size * size * size).to_string());
    print("GFLOPS: " + ((size * size * size) / elapsed / 1e9).to_string());
    
    return 0;
}
```

---

## 🎯 الخلاصة

هذه الأمثلة توضح:
- ✅ سهولة استخدام المكتبات
- ✅ دعم كامل للعربية
- ✅ أداء عالي
- ✅ مرونة عالية
- ✅ تكامل سلس مع اللغة

**الهدف**: جعل AlBayan Language الخيار الأول للحسابات العلمية والذكاء الاصطناعي! 🚀

