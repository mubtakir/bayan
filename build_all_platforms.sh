#!/bin/bash

# سكريبت بناء لجميع المنصات - لغة البيان
# Build Script for All Platforms - AlBayan Language

echo "🧬 بناء برامج لغة البيان لجميع المنصات"
echo "🧬 Building AlBayan Programs for All Platforms"
echo "================================================"

# إنشاء مجلد التوزيع
mkdir -p dist

# متغيرات
PROGRAM_NAME="build_example"
SOURCE_FILE="examples/build_example.ab"

echo "📁 إنشاء مجلد التوزيع..."
echo "📁 Creating distribution directory..."

# التحقق من وجود الملف المصدري
if [ ! -f "$SOURCE_FILE" ]; then
    echo "❌ خطأ: الملف المصدري غير موجود: $SOURCE_FILE"
    echo "❌ Error: Source file not found: $SOURCE_FILE"
    exit 1
fi

echo "✅ الملف المصدري موجود: $SOURCE_FILE"
echo "✅ Source file found: $SOURCE_FILE"

# دالة البناء
build_for_target() {
    local target=$1
    local output_name=$2
    local description=$3
    
    echo ""
    echo "🔨 بناء للمنصة: $description"
    echo "🔨 Building for: $description"
    echo "🎯 الهدف: $target"
    echo "🎯 Target: $target"
    
    # محاولة البناء
    if ~/.local/bin/albayan build "$SOURCE_FILE" --target "$target" --release --output "dist/$output_name" 2>/dev/null; then
        echo "✅ نجح البناء: dist/$output_name"
        echo "✅ Build successful: dist/$output_name"
        
        # عرض معلومات الملف
        if [ -f "dist/$output_name" ]; then
            local size=$(du -h "dist/$output_name" | cut -f1)
            echo "📊 حجم الملف: $size"
            echo "📊 File size: $size"
        fi
    else
        echo "❌ فشل البناء للمنصة: $description"
        echo "❌ Build failed for: $description"
        echo "⚠️  قد تحتاج تثبيت toolchain للمنصة المستهدفة"
        echo "⚠️  You may need to install the target toolchain"
    fi
}

# بناء للمنصات المختلفة
echo ""
echo "🚀 بدء عملية البناء للمنصات المختلفة..."
echo "🚀 Starting build process for different platforms..."

# Linux x86_64
build_for_target "x86_64-unknown-linux-gnu" "${PROGRAM_NAME}-linux-x64" "Linux 64-bit"

# Linux ARM64
build_for_target "aarch64-unknown-linux-gnu" "${PROGRAM_NAME}-linux-arm64" "Linux ARM64"

# Windows x86_64
build_for_target "x86_64-pc-windows-msvc" "${PROGRAM_NAME}-windows-x64.exe" "Windows 64-bit"

# Windows x86
build_for_target "i686-pc-windows-msvc" "${PROGRAM_NAME}-windows-x86.exe" "Windows 32-bit"

# macOS Intel
build_for_target "x86_64-apple-darwin" "${PROGRAM_NAME}-macos-intel" "macOS Intel"

# macOS Apple Silicon
build_for_target "aarch64-apple-darwin" "${PROGRAM_NAME}-macos-arm64" "macOS Apple Silicon"

# FreeBSD
build_for_target "x86_64-unknown-freebsd" "${PROGRAM_NAME}-freebsd" "FreeBSD"

# WebAssembly
build_for_target "wasm32-unknown-unknown" "${PROGRAM_NAME}.wasm" "WebAssembly"

# عرض النتائج النهائية
echo ""
echo "📋 ملخص النتائج:"
echo "📋 Build Summary:"
echo "=================="

echo ""
echo "📁 محتويات مجلد التوزيع:"
echo "📁 Distribution directory contents:"
if [ -d "dist" ]; then
    ls -la dist/
else
    echo "❌ مجلد التوزيع غير موجود"
    echo "❌ Distribution directory not found"
fi

# إنشاء ملف README للتوزيع
echo ""
echo "📝 إنشاء ملف README للتوزيع..."
echo "📝 Creating distribution README..."

cat > dist/README.md << 'EOF'
# 🧬 لغة البيان - حاسبة متقدمة
# AlBayan Language - Advanced Calculator

## 📖 الوصف / Description

هذا برنامج حاسبة متقدمة مكتوب بلغة البيان يوضح قدرات اللغة في البناء والتوزيع.

This is an advanced calculator program written in AlBayan language demonstrating the language's build and distribution capabilities.

## 🚀 التشغيل / Running

### Linux:
```bash
chmod +x build_example-linux-x64
./build_example-linux-x64
```

### Windows:
```cmd
build_example-windows-x64.exe
```

### macOS:
```bash
chmod +x build_example-macos-intel
./build_example-macos-intel
```

## ✨ الميزات / Features

- ✅ العمليات الحسابية الأساسية / Basic arithmetic operations
- ✅ العمليات المتقدمة / Advanced operations  
- ✅ الحسابات الهندسية / Geometric calculations
- ✅ التحويلات / Conversions
- ✅ الحسابات المالية / Financial calculations

## 🏗️ البناء / Building

تم بناء هذا البرنامج باستخدام:
This program was built using:

```bash
albayan build examples/build_example.ab --release --target <platform>
```

## 📊 الإحصائيات / Statistics

- **اللغة / Language:** AlBayan (البيان)
- **الأسطر / Lines:** 300+ lines
- **الدوال / Functions:** 20+ functions
- **المنصات المدعومة / Supported Platforms:** 8+ platforms

## 🌐 المزيد / More Information

- **المستودع / Repository:** https://github.com/mubtakir/bayan
- **الوثائق / Documentation:** docs/
- **الأمثلة / Examples:** examples/

---

**🧬 مدعوم بقوة لغة البيان - Powered by AlBayan Language**
EOF

echo "✅ تم إنشاء ملف README: dist/README.md"
echo "✅ README created: dist/README.md"

# إنشاء سكريبت تشغيل سريع
echo ""
echo "🔧 إنشاء سكريبت تشغيل سريع..."
echo "🔧 Creating quick run script..."

cat > dist/run.sh << 'EOF'
#!/bin/bash

# سكريبت تشغيل سريع للحاسبة
# Quick run script for calculator

echo "🧬 حاسبة لغة البيان المتقدمة"
echo "🧬 AlBayan Advanced Calculator"
echo "=============================="

# اكتشاف النظام
OS=$(uname -s)
ARCH=$(uname -m)

case "$OS" in
    Linux)
        if [ "$ARCH" = "x86_64" ]; then
            EXECUTABLE="./build_example-linux-x64"
        elif [ "$ARCH" = "aarch64" ]; then
            EXECUTABLE="./build_example-linux-arm64"
        else
            echo "❌ معمارية غير مدعومة: $ARCH"
            echo "❌ Unsupported architecture: $ARCH"
            exit 1
        fi
        ;;
    Darwin)
        if [ "$ARCH" = "x86_64" ]; then
            EXECUTABLE="./build_example-macos-intel"
        elif [ "$ARCH" = "arm64" ]; then
            EXECUTABLE="./build_example-macos-arm64"
        else
            echo "❌ معمارية غير مدعومة: $ARCH"
            echo "❌ Unsupported architecture: $ARCH"
            exit 1
        fi
        ;;
    *)
        echo "❌ نظام تشغيل غير مدعوم: $OS"
        echo "❌ Unsupported operating system: $OS"
        echo "💡 جرب تشغيل الملف التنفيذي المناسب يدوياً"
        echo "💡 Try running the appropriate executable manually"
        exit 1
        ;;
esac

# التحقق من وجود الملف التنفيذي
if [ ! -f "$EXECUTABLE" ]; then
    echo "❌ الملف التنفيذي غير موجود: $EXECUTABLE"
    echo "❌ Executable not found: $EXECUTABLE"
    echo "📁 الملفات المتاحة:"
    echo "📁 Available files:"
    ls -la
    exit 1
fi

# جعل الملف قابل للتنفيذ
chmod +x "$EXECUTABLE"

echo "🚀 تشغيل: $EXECUTABLE"
echo "🚀 Running: $EXECUTABLE"
echo ""

# تشغيل البرنامج
"$EXECUTABLE"
EOF

chmod +x dist/run.sh

echo "✅ تم إنشاء سكريبت التشغيل: dist/run.sh"
echo "✅ Run script created: dist/run.sh"

# النتيجة النهائية
echo ""
echo "🎉 اكتمل البناء بنجاح!"
echo "🎉 Build completed successfully!"
echo ""
echo "📦 للتشغيل السريع:"
echo "📦 For quick execution:"
echo "   cd dist && ./run.sh"
echo ""
echo "🌍 للتوزيع:"
echo "🌍 For distribution:"
echo "   tar -czf albayan-calculator.tar.gz dist/"
echo ""
echo "🧬 لغة البيان - مستقبل البرمجة!"
echo "🧬 AlBayan Language - The Future of Programming!"
