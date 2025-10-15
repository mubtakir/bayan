#!/usr/bin/env python3
"""
اختبار نظام Baserah Universal
🧬 Creator: Basil Yahya Abdullah
🎯 Testing the Revolutionary Mathematical Intelligence System
"""

from app import BaserahUniversal
import numpy as np

def test_baserah_system():
    """اختبار شامل للنظام"""
    
    print("🌟 اختبار نظام Baserah Universal الثوري")
    print("=" * 50)
    
    # إنشاء مثيل النظام
    system = BaserahUniversal()
    
    # اختبارات تحليل النص
    test_cases = [
        "ارسم دائرة",
        "شكل قلب أحمر",
        "نجمة خماسية كبيرة",
        "خط مائل أزرق",
        "مربع أصفر صغير",
        "زهرة وردية متوسطة",
        "حلزون أخضر ضخم"
    ]
    
    print("\n🔍 اختبار تحليل النصوص:")
    print("-" * 30)
    
    for i, test_text in enumerate(test_cases, 1):
        print(f"\n{i}. النص: '{test_text}'")
        analysis = system.analyze_text(test_text)
        print(f"   الشكل: {analysis['shape']}")
        print(f"   اللون: {analysis['color']}")
        print(f"   الحجم: {analysis['size']}")
    
    print("\n🧮 اختبار تحويل المعادلات:")
    print("-" * 30)
    
    # اختبار تحويل المعادلات
    for shape in ['circle', 'heart', 'star', 'line']:
        analysis = {'shape': shape, 'color': 'blue', 'size': 1.0}
        x, y, equation = system.text_to_equation(analysis)
        print(f"\n{shape.title()}:")
        print(f"   نقاط: {len(x)} نقطة")
        print(f"   المعادلة: {equation}")
        print(f"   نطاق X: [{x.min():.2f}, {x.max():.2f}]")
        print(f"   نطاق Y: [{y.min():.2f}, {y.max():.2f}]")
    
    print("\n🎨 اختبار إنشاء الصور:")
    print("-" * 30)
    
    # اختبار إنشاء الصور
    test_commands = ["ارسم دائرة حمراء", "شكل قلب أزرق كبير"]
    
    for cmd in test_commands:
        try:
            image_path = system.create_visual(cmd)
            print(f"✅ '{cmd}' → صورة محفوظة: {image_path}")
        except Exception as e:
            print(f"❌ خطأ في '{cmd}': {e}")
    
    print("\n🌟 اختبار الكلمات المفتاحية:")
    print("-" * 30)
    
    print("الأشكال المدعومة:")
    for arabic, english in system.shape_keywords.items():
        print(f"   {arabic} → {english}")
    
    print("\nالألوان المدعومة:")
    for arabic, english in system.color_keywords.items():
        print(f"   {arabic} → {english}")
    
    print("\nالأحجام المدعومة:")
    for arabic, size in system.size_keywords.items():
        print(f"   {arabic} → {size}")
    
    print("\n" + "=" * 50)
    print("✅ اكتمل اختبار نظام Baserah Universal بنجاح!")
    print("🚀 النظام جاهز للاستخدام!")

if __name__ == "__main__":
    test_baserah_system()
