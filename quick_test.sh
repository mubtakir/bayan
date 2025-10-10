#!/bin/bash

# اختبار سريع للغة البيان
# Quick Test for AlBayan Language

echo "🚀 اختبار لغة البيان - AlBayan Language Test"
echo "================================================"

# تحقق من وجود المترجم
echo "🔍 فحص المترجم..."
if [ -f ~/.local/bin/albayan ]; then
    echo "✅ المترجم موجود في: ~/.local/bin/albayan"
else
    echo "❌ المترجم غير موجود!"
    exit 1
fi

# عرض الإصدار
echo ""
echo "📋 إصدار المترجم:"
~/.local/bin/albayan --version

# اختبار ملف التثبيت
echo ""
echo "🧪 اختبار ملف التثبيت..."
if ~/.local/bin/albayan check test_installation.ab; then
    echo "✅ فحص الصيغة نجح!"
else
    echo "❌ فحص الصيغة فشل!"
    exit 1
fi

# اختبار الأمثلة
echo ""
echo "📚 اختبار الأمثلة..."

# اختبار المثال الأساسي
if [ -f examples/working_operators_test.ab ]; then
    echo "🔍 فحص working_operators_test.ab..."
    if ~/.local/bin/albayan check examples/working_operators_test.ab; then
        echo "✅ working_operators_test.ab - نجح!"
    else
        echo "❌ working_operators_test.ab - فشل!"
    fi
fi

# اختبار نظام بصيرة
if [ -f examples/basera_working_demo.ab ]; then
    echo "🔍 فحص basera_working_demo.ab..."
    if ~/.local/bin/albayan check examples/basera_working_demo.ab; then
        echo "✅ basera_working_demo.ab - نجح!"
    else
        echo "❌ basera_working_demo.ab - فشل!"
    fi
fi

# اختبار التطبيقات المتقدمة
if [ -f examples/basera_advanced_applications.ab ]; then
    echo "🔍 فحص basera_advanced_applications.ab..."
    if ~/.local/bin/albayan check examples/basera_advanced_applications.ab; then
        echo "✅ basera_advanced_applications.ab - نجح!"
    else
        echo "❌ basera_advanced_applications.ab - فشل!"
    fi
fi

# فحص إعدادات VS Code
echo ""
echo "🔧 فحص إعدادات VS Code..."

if [ -f .vscode/settings.json ]; then
    echo "✅ إعدادات VS Code موجودة"
else
    echo "❌ إعدادات VS Code مفقودة"
fi

if [ -f .vscode/tasks.json ]; then
    echo "✅ مهام VS Code موجودة"
else
    echo "❌ مهام VS Code مفقودة"
fi

if [ -f .vscode/launch.json ]; then
    echo "✅ إعدادات التشغيل موجودة"
else
    echo "❌ إعدادات التشغيل مفقودة"
fi

# فحص دعم اللغة
echo ""
echo "🎨 فحص دعم اللغة..."

if [ -f albayan-language-support/package.json ]; then
    echo "✅ حزمة دعم اللغة موجودة"
else
    echo "❌ حزمة دعم اللغة مفقودة"
fi

if [ -f albayan-language-support/syntaxes/albayan.tmLanguage.json ]; then
    echo "✅ ملف تمييز الصيغة موجود"
else
    echo "❌ ملف تمييز الصيغة مفقود"
fi

if [ -f albayan-language-support/snippets/albayan.json ]; then
    echo "✅ المقاطع الجاهزة موجودة"
else
    echo "❌ المقاطع الجاهزة مفقودة"
fi

# النتيجة النهائية
echo ""
echo "🎉 انتهى الاختبار!"
echo "================================================"
echo "✅ لغة البيان مثبتة ومجهزة للاستخدام!"
echo ""
echo "📝 للبدء:"
echo "   1. افتح VS Code في هذا المجلد"
echo "   2. افتح ملف .ab"
echo "   3. استخدم Ctrl+F5 للتشغيل"
echo ""
echo "📚 الأمثلة متاحة في مجلد examples/"
echo "📖 الوثائق متاحة في مجلد docs/"
echo ""
echo "🚀 مرحباً بك في عالم البرمجة بلغة البيان!"
