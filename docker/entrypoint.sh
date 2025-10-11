#!/bin/bash
# 🚀 نقطة دخول Docker للغة البيان
# AlBayan Docker Entrypoint - URGENT

set -e

echo "🔥 === مرحباً بك في لغة البيان === 🔥"
echo "🐳 تشغيل داخل Docker Container"
echo "⚡ استجابة عاجلة لطلب الجماهير!"
echo ""

# عرض معلومات النظام
echo "📊 معلومات النظام:"
echo "  🖥️  النظام: $(uname -s)"
echo "  🏗️  المعمارية: $(uname -m)"
echo "  🐳 Docker: $(cat /etc/os-release | grep PRETTY_NAME | cut -d'"' -f2)"
echo "  👤 المستخدم: $(whoami)"
echo "  📁 مجلد العمل: $(pwd)"
echo ""

# عرض معلومات لغة البيان
echo "🧬 معلومات لغة البيان:"
if command -v albayan >/dev/null 2>&1; then
    albayan --version 2>/dev/null || echo "  📦 الإصدار: 1.0.0-urgent"
    echo "  📍 المسار: $(which albayan)"
else
    echo "  ❌ مترجم لغة البيان غير متوفر!"
    exit 1
fi
echo ""

# عرض الأمثلة المتوفرة
echo "📚 الأمثلة المتوفرة:"
if [ -d "/examples" ]; then
    ls -la /examples/*.ab 2>/dev/null | head -10 | while read line; do
        filename=$(echo "$line" | awk '{print $9}' | xargs basename)
        echo "  📄 $filename"
    done
else
    echo "  📁 /examples/ غير موجود"
fi
echo ""

# عرض ملفات مجلد العمل
echo "💻 ملفات مجلد العمل:"
if [ "$(ls -A /workspace 2>/dev/null)" ]; then
    ls -la /workspace/*.ab 2>/dev/null | head -5 | while read line; do
        filename=$(echo "$line" | awk '{print $9}' | xargs basename)
        echo "  📄 $filename"
    done
else
    echo "  📁 مجلد العمل فارغ"
fi
echo ""

# إذا لم يتم تمرير أوامر، عرض المساعدة
if [ $# -eq 0 ]; then
    echo "🚀 أوامر مفيدة:"
    echo "  📖 عرض المساعدة:"
    echo "     albayan --help"
    echo ""
    echo "  🏃 تشغيل مثال:"
    echo "     albayan run welcome.ab"
    echo "     albayan run /examples/hello.ab"
    echo ""
    echo "  🔍 فحص الكود:"
    echo "     albayan check myfile.ab"
    echo ""
    echo "  🏗️  بناء مشروع:"
    echo "     albayan build myfile.ab"
    echo ""
    echo "  🧪 تشغيل الاختبارات:"
    echo "     albayan test"
    echo ""
    echo "💡 نصائح:"
    echo "  📁 ضع ملفاتك في /workspace/"
    echo "  📚 استكشف الأمثلة في /examples/"
    echo "  🔄 استخدم docker run -v \$(pwd):/workspace لربط مجلد محلي"
    echo ""
    
    # تشغيل shell تفاعلي
    echo "🐚 بدء shell تفاعلي..."
    echo "   اكتب 'exit' للخروج"
    echo ""
    exec /bin/bash
fi

# تشغيل الأوامر المُمررة
echo "🚀 تشغيل: $@"
echo ""

# التحقق من وجود الملف إذا كان الأمر run
if [ "$1" = "albayan" ] && [ "$2" = "run" ] && [ -n "$3" ]; then
    if [ ! -f "$3" ] && [ ! -f "/workspace/$3" ] && [ ! -f "/examples/$3" ]; then
        echo "❌ الملف غير موجود: $3"
        echo ""
        echo "💡 الملفات المتوفرة:"
        echo "📁 في /workspace/:"
        ls -1 /workspace/*.ab 2>/dev/null || echo "   (لا توجد ملفات .ab)"
        echo "📁 في /examples/:"
        ls -1 /examples/*.ab 2>/dev/null || echo "   (لا توجد ملفات .ab)"
        echo ""
        exit 1
    fi
fi

# تنفيذ الأمر
exec "$@"
