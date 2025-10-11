#!/bin/bash
# 🚀 سكريبت التثبيت السريع للغة البيان
# AlBayan Quick Install Script - URGENT RESPONSE

set -e

echo "🔥 === تثبيت لغة البيان السريع === 🔥"
echo "⚡ استجابة عاجلة لطلب الجماهير!"
echo ""

# متغيرات النظام
OS=$(uname -s)
ARCH=$(uname -m)
INSTALL_DIR="$HOME/.local/bin"
ALBAYAN_VERSION="latest"

echo "🖥️  النظام: $OS"
echo "🏗️  المعمارية: $ARCH"
echo "📁 مجلد التثبيت: $INSTALL_DIR"
echo ""

# إنشاء مجلد التثبيت
mkdir -p "$INSTALL_DIR"

# تحديد رابط التحميل حسب النظام
case "$OS" in
    "Linux")
        case "$ARCH" in
            "x86_64")
                DOWNLOAD_URL="https://github.com/mubtakir/bayan/releases/download/v1.0.0/albayan-linux-x64"
                ;;
            "aarch64"|"arm64")
                DOWNLOAD_URL="https://github.com/mubtakir/bayan/releases/download/v1.0.0/albayan-linux-arm64"
                ;;
            *)
                echo "❌ معمارية غير مدعومة: $ARCH"
                exit 1
                ;;
        esac
        ;;
    "Darwin")
        case "$ARCH" in
            "x86_64")
                DOWNLOAD_URL="https://github.com/mubtakir/bayan/releases/download/v1.0.0/albayan-macos-intel"
                ;;
            "arm64")
                DOWNLOAD_URL="https://github.com/mubtakir/bayan/releases/download/v1.0.0/albayan-macos-apple"
                ;;
            *)
                echo "❌ معمارية غير مدعومة: $ARCH"
                exit 1
                ;;
        esac
        ;;
    *)
        echo "❌ نظام تشغيل غير مدعوم: $OS"
        echo "💡 استخدم Docker: docker run -it albayan/compiler:latest"
        exit 1
        ;;
esac

echo "📥 تحميل من: $DOWNLOAD_URL"
echo ""

# تحميل المترجم
echo "🔄 تحميل مترجم لغة البيان..."
if command -v curl >/dev/null 2>&1; then
    curl -L -o "$INSTALL_DIR/albayan" "$DOWNLOAD_URL"
elif command -v wget >/dev/null 2>&1; then
    wget -O "$INSTALL_DIR/albayan" "$DOWNLOAD_URL"
else
    echo "❌ curl أو wget غير متوفر!"
    echo "💡 قم بتثبيت curl أو wget أولاً"
    exit 1
fi

# جعل الملف قابل للتنفيذ
chmod +x "$INSTALL_DIR/albayan"

echo "✅ تم تحميل المترجم بنجاح!"
echo ""

# إضافة إلى PATH
echo "🔧 إعداد متغير PATH..."

# تحديد ملف الشل
if [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_RC="$HOME/.bashrc"
else
    SHELL_RC="$HOME/.profile"
fi

# إضافة PATH إذا لم يكن موجود
if ! grep -q "$INSTALL_DIR" "$SHELL_RC" 2>/dev/null; then
    echo "" >> "$SHELL_RC"
    echo "# لغة البيان - AlBayan Programming Language" >> "$SHELL_RC"
    echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_RC"
    echo "✅ تم إضافة PATH إلى $SHELL_RC"
else
    echo "✅ PATH موجود مسبقاً في $SHELL_RC"
fi

echo ""

# تحديث PATH للجلسة الحالية
export PATH="$INSTALL_DIR:$PATH"

# اختبار التثبيت
echo "🧪 اختبار التثبيت..."
if "$INSTALL_DIR/albayan" --version >/dev/null 2>&1; then
    echo "✅ تم التثبيت بنجاح!"
    echo ""
    echo "🎉 === لغة البيان جاهزة للاستخدام! ==="
    echo ""
    echo "📚 للبدء:"
    echo "   albayan --help"
    echo "   albayan run examples/hello.ab"
    echo ""
    echo "🌐 الوثائق:"
    echo "   https://github.com/mubtakir/bayan"
    echo ""
    echo "💬 المجتمع:"
    echo "   Discord: https://discord.gg/albayan"
    echo "   GitHub: https://github.com/mubtakir/bayan/discussions"
    echo ""
else
    echo "❌ فشل في اختبار التثبيت!"
    echo "💡 جرب إعادة تشغيل الطرفية أو تشغيل:"
    echo "   source $SHELL_RC"
    exit 1
fi

# إنشاء مثال سريع
echo "📝 إنشاء مثال سريع..."
mkdir -p "$HOME/albayan-examples"
cat > "$HOME/albayan-examples/hello.ab" << 'EOF'
// مثال سريع للغة البيان
// Quick AlBayan Example

fn main() -> int {
    print("🎉 مرحباً بك في لغة البيان!");
    print("🚀 أول لغة برمجة عربية بذكاء اصطناعي مدمج!");
    print("");
    print("🧬 ميزات لغة البيان:");
    print("  ✅ برمجة تقليدية + منطقية");
    print("  ✅ ذكاء اصطناعي مدمج");
    print("  ✅ أداء عالي مع Rust + LLVM");
    print("  ✅ دعم TensorFlow وPyTorch وONNX");
    print("");
    print("🔥 ابدأ رحلتك في البرمجة المتقدمة!");
    
    return 0;
}
EOF

echo "✅ تم إنشاء مثال في: $HOME/albayan-examples/hello.ab"
echo ""

# تشغيل المثال
echo "🚀 تشغيل المثال السريع..."
echo ""
"$INSTALL_DIR/albayan" run "$HOME/albayan-examples/hello.ab"
echo ""

echo "🎊 === التثبيت مكتمل بنجاح! ==="
echo ""
echo "🔥 الجماهير طلبت السرعة - وحصلت عليها!"
echo "⚡ لغة البيان جاهزة للاستخدام الفوري!"
echo ""
echo "💪 معاً نبني مستقبل البرمجة العربية!"
echo ""
echo "🧬 لغة البيان - سرعة + قوة + إبداع!"
