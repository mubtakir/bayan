#!/usr/bin/env python3
"""
Baserah Universal - الذكاء الرياضي الثوري
🧬 Creator: Basil Yahya Abdullah
🌟 Revolutionary Mathematical Intelligence without Traditional AI
🎯 Features: Natural Language → Mathematical Equations → Visual Shapes
"""

import gradio as gr
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import tempfile
import re
import math

def baserah_sigmoid(x, alpha=1.0, k=1.0, x0=0.0):
    """دالة السيجمويد الثورية - نواة نظام بصيرة"""
    return alpha / (1 + np.exp(-k * (x - x0)))

def sigmoid_wave_approximation(t, amplitude=1.0, num_waves=2, k_steepness=1.5, phase_shift=0.0):
    """تقريب الموجات باستخدام sigmoid - النواة الثورية لبصيرة"""
    approx = np.zeros_like(t, dtype=float)
    period = 2 * np.pi

    for n in range(-int(num_waves), int(num_waves) + 1):
        x0_pos = n * period + 0.25 * period + phase_shift
        x0_neg = n * period - 0.25 * period + phase_shift

        approx += baserah_sigmoid(t, alpha=1.0, k=k_steepness, x0=x0_pos) - \
                 baserah_sigmoid(t, alpha=1.0, k=k_steepness, x0=x0_neg)

    # تطبيع النتيجة
    min_val = np.min(approx)
    max_val = np.max(approx)
    range_val = max_val - min_val
    if range_val > 1e-9:
        approx = 2 * (approx - min_val) / range_val - 1
    else:
        approx = np.zeros_like(t)

    return amplitude * approx

def linear_transform(x, m=1.0, c=0.0):
    """التحويل الخطي الأساسي"""
    return m * x + c

class BaserahUniversal:
    """النظام الثوري لفهم اللغة وتحويلها لمعادلات رياضية"""
    
    def __init__(self):
        # قاموس الكلمات والأشكال
        self.shape_keywords = {
            'دائرة': 'circle',
            'قلب': 'heart', 
            'نجمة': 'star',
            'خط': 'line',
            'مربع': 'square',
            'مثلث': 'triangle',
            'زهرة': 'flower',
            'حلزون': 'spiral',
            'موجة': 'wave'
        }
        
        # قاموس الألوان
        self.color_keywords = {
            'أحمر': 'red',
            'أزرق': 'blue', 
            'أخضر': 'green',
            'أصفر': 'yellow',
            'بنفسجي': 'purple',
            'برتقالي': 'orange',
            'وردي': 'pink',
            'أسود': 'black'
        }
        
        # قاموس الأحجام
        self.size_keywords = {
            'صغير': 0.5,
            'متوسط': 1.0,
            'كبير': 1.5,
            'ضخم': 2.0
        }
    
    def analyze_text(self, text):
        """تحليل النص وفهم المطلوب"""
        text = text.strip().lower()
        
        # البحث عن الشكل
        shape = 'circle'  # افتراضي
        for arabic_word, english_shape in self.shape_keywords.items():
            if arabic_word in text:
                shape = english_shape
                break
        
        # البحث عن اللون
        color = 'blue'  # افتراضي
        for arabic_color, english_color in self.color_keywords.items():
            if arabic_color in text:
                color = english_color
                break
        
        # البحث عن الحجم
        size = 1.0  # افتراضي
        for arabic_size, size_value in self.size_keywords.items():
            if arabic_size in text:
                size = size_value
                break
        
        return {
            'shape': shape,
            'color': color,
            'size': size,
            'original_text': text
        }
    
    def text_to_equation(self, analysis):
        """تحويل التحليل إلى معادلات رياضية"""
        shape = analysis['shape']
        size = analysis['size']
        
        t = np.linspace(0, 2*np.pi, 1000)
        
        if shape == 'circle':
            # دائرة ثورية باستخدام sigmoid فقط!
            x = sigmoid_wave_approximation(t, amplitude=size, num_waves=2, k_steepness=1.5, phase_shift=np.pi/2)
            y = sigmoid_wave_approximation(t, amplitude=size, num_waves=2, k_steepness=1.5, phase_shift=0)
            equation = f"x = SigmoidWave(t, φ=π/2), y = SigmoidWave(t, φ=0) - PURE SIGMOID!"
            
        elif shape == 'heart':
            # قلب ثوري باستخدام sigmoid فقط!
            sin_t = sigmoid_wave_approximation(t, amplitude=1, num_waves=2, k_steepness=2)
            cos_t = sigmoid_wave_approximation(t, amplitude=1, num_waves=2, k_steepness=2, phase_shift=np.pi/2)
            cos_2t = sigmoid_wave_approximation(2*t, amplitude=1, num_waves=4, k_steepness=2, phase_shift=np.pi/2)
            cos_3t = sigmoid_wave_approximation(3*t, amplitude=1, num_waves=6, k_steepness=2, phase_shift=np.pi/2)
            cos_4t = sigmoid_wave_approximation(4*t, amplitude=1, num_waves=8, k_steepness=2, phase_shift=np.pi/2)

            x = size * 16 * (sin_t ** 3) / 16
            y = size * (13 * cos_t - 5 * cos_2t - 2 * cos_3t - cos_4t) / 16
            equation = f"Heart using PURE SIGMOID approximations - NO trigonometry!"
            
        elif shape == 'star':
            # نجمة ثورية باستخدام sigmoid فقط!
            cos_t = sigmoid_wave_approximation(t, amplitude=1, num_waves=2, k_steepness=3, phase_shift=np.pi/2)
            sin_t = sigmoid_wave_approximation(t, amplitude=1, num_waves=2, k_steepness=3)
            cos_5t = sigmoid_wave_approximation(5*t, amplitude=1, num_waves=10, k_steepness=3, phase_shift=np.pi/2)

            # معادلة النجمة الثورية
            spike_factor = 0.3
            x = size * cos_t * (1 + spike_factor * cos_5t)
            y = size * sin_t * (1 + spike_factor * cos_5t)
            equation = f"Star using PURE SIGMOID spikes - Revolutionary!"
            
        elif shape == 'line':
            x = np.linspace(-size, size, 1000)
            y = x * 0.5  # خط مائل
            equation = f"y = 0.5x, length = {size*2}"
            
        elif shape == 'square':
            # مربع
            side = size
            x = np.array([-side, side, side, -side, -side])
            y = np.array([-side, -side, side, side, -side])
            equation = f"Square with side = {side*2}"
            
        elif shape == 'triangle':
            # مثلث ثوري باستخدام sigmoid فقط!
            # نقاط المثلث المتساوي الأضلاع بـ sigmoid
            angle1 = 0
            angle2 = 2*np.pi/3
            angle3 = 4*np.pi/3

            cos_0 = sigmoid_wave_approximation(np.array([angle1]), amplitude=1, num_waves=2, k_steepness=3, phase_shift=np.pi/2)[0]
            sin_0 = sigmoid_wave_approximation(np.array([angle1]), amplitude=1, num_waves=2, k_steepness=3)[0]
            cos_120 = sigmoid_wave_approximation(np.array([angle2]), amplitude=1, num_waves=2, k_steepness=3, phase_shift=np.pi/2)[0]
            sin_120 = sigmoid_wave_approximation(np.array([angle2]), amplitude=1, num_waves=2, k_steepness=3)[0]
            cos_240 = sigmoid_wave_approximation(np.array([angle3]), amplitude=1, num_waves=2, k_steepness=3, phase_shift=np.pi/2)[0]
            sin_240 = sigmoid_wave_approximation(np.array([angle3]), amplitude=1, num_waves=2, k_steepness=3)[0]

            x = size * np.array([cos_0, cos_120, cos_240, cos_0])
            y = size * np.array([sin_0, sin_120, sin_240, sin_0])
            equation = f"SIGMOID Triangle - NO trigonometry!"
            
        elif shape == 'flower':
            # زهرة ثورية باستخدام sigmoid فقط!
            cos_t = sigmoid_wave_approximation(t, amplitude=1, num_waves=2, k_steepness=2, phase_shift=np.pi/2)
            sin_t = sigmoid_wave_approximation(t, amplitude=1, num_waves=2, k_steepness=2)
            cos_3t = sigmoid_wave_approximation(3*t, amplitude=1, num_waves=6, k_steepness=2, phase_shift=np.pi/2)

            r = size * np.abs(cos_3t)
            x = r * cos_t
            y = r * sin_t
            equation = f"Flower using PURE SIGMOID petals - Revolutionary!"
            
        elif shape == 'spiral':
            # حلزون ثوري باستخدام sigmoid فقط!
            r = size * t / (2*np.pi)
            cos_t = sigmoid_wave_approximation(t, amplitude=1, num_waves=2, k_steepness=2, phase_shift=np.pi/2)
            sin_t = sigmoid_wave_approximation(t, amplitude=1, num_waves=2, k_steepness=2)
            x = r * cos_t
            y = r * sin_t
            equation = f"SIGMOID Spiral: r×SigmoidCos(t), r×SigmoidSin(t)"
            
        elif shape == 'wave':
            # موجة ثورية باستخدام sigmoid فقط!
            x = np.linspace(-2*np.pi*size, 2*np.pi*size, 1000)
            y = sigmoid_wave_approximation(x, amplitude=size, num_waves=4, k_steepness=2)
            equation = f"SIGMOID Wave: y = SigmoidWave(x) - PURE BASERAH!"
            
        else:
            # دائرة افتراضية ثورية - sigmoid فقط!
            x = sigmoid_wave_approximation(t, amplitude=size, num_waves=2, k_steepness=1.5, phase_shift=np.pi/2)
            y = sigmoid_wave_approximation(t, amplitude=size, num_waves=2, k_steepness=1.5, phase_shift=0)
            equation = f"DEFAULT SIGMOID CIRCLE: x = SigmoidWave(t, φ=π/2), y = SigmoidWave(t, φ=0) - PURE BASERAH!"
        
        return x, y, equation
    
    def create_visual(self, text_input):
        """إنشاء الشكل البصري من النص"""
        try:
            # تحليل النص
            analysis = self.analyze_text(text_input)
            
            # تحويل لمعادلات
            x, y, equation = self.text_to_equation(analysis)
            
            # إنشاء الرسم
            fig, ax = plt.subplots(figsize=(8, 8))
            
            # رسم الشكل
            ax.plot(x, y, linewidth=3, color=analysis['color'], alpha=0.8)
            ax.fill(x, y, alpha=0.3, color=analysis['color'])
            
            # تنسيق الرسم
            ax.set_aspect('equal')
            ax.grid(True, alpha=0.3)
            ax.set_title(f'Baserah Universal: {analysis["shape"].title()}\nCreated from: "{text_input}"', 
                        fontsize=14, fontweight='bold')
            
            # إضافة معلومات التحليل
            info_text = f"""Analysis Results:
Shape: {analysis['shape']}
Color: {analysis['color']}
Size: {analysis['size']}

Mathematical Equation:
{equation}

Creator: Basil Yahya Abdullah
Method: Revolutionary Mathematical Intelligence"""
            
            ax.text(0.02, 0.98, info_text, transform=ax.transAxes, 
                   fontsize=10, verticalalignment='top',
                   bbox=dict(boxstyle="round,pad=0.5", facecolor="lightyellow", alpha=0.8))
            
            plt.tight_layout()
            
            # حفظ الصورة
            temp_file = tempfile.NamedTemporaryFile(delete=False, suffix='.png')
            plt.savefig(temp_file.name, format='png', dpi=150, bbox_inches='tight')
            plt.close(fig)
            
            return temp_file.name
            
        except Exception as e:
            # معالجة الأخطاء
            fig, ax = plt.subplots(figsize=(8, 6))
            ax.text(0.5, 0.5, f'Error in Baserah Universal:\n{str(e)}\n\nPlease try a different command', 
                    ha='center', va='center', fontsize=14, color='red',
                    bbox=dict(boxstyle="round,pad=0.5", facecolor="lightyellow"))
            ax.set_xlim(0, 1)
            ax.set_ylim(0, 1)
            ax.axis('off')
            
            temp_file = tempfile.NamedTemporaryFile(delete=False, suffix='.png')
            plt.savefig(temp_file.name, format='png', dpi=150, bbox_inches='tight')
            plt.close(fig)
            
            return temp_file.name

# إنشاء مثيل النظام
baserah_system = BaserahUniversal()

def process_command(text_input):
    """معالجة الأمر النصي"""
    return baserah_system.create_visual(text_input)

# إنشاء واجهة Gradio
def create_interface():
    """إنشاء واجهة المستخدم"""
    
    with gr.Blocks(
        title="Baserah Universal - الذكاء الرياضي الثوري",
        theme=gr.themes.Soft()
    ) as interface:
        
        gr.Markdown("""
        # 🌟 Baserah Universal - الذكاء الرياضي الثوري
        ## 🧬 Creator: Basil Yahya Abdullah
        
        ### ⚡ Revolutionary Mathematical Intelligence
        **🚫 No Neural Networks • 🚫 No Big Data • ✅ Pure Mathematical Understanding**
        
        ---
        """)
        
        with gr.Row():
            with gr.Column(scale=1):
                gr.Markdown("### 🎯 أدخل أمرك باللغة العربية")
                
                text_input = gr.Textbox(
                    label="الأمر النصي",
                    placeholder="مثال: ارسم دائرة حمراء كبيرة",
                    lines=3,
                    value="ارسم دائرة"
                )
                
                generate_btn = gr.Button("🚀 تنفيذ الأمر", variant="primary", size="lg")
                
                gr.Markdown("""
                ### 📝 أمثلة للتجربة:
                - "ارسم دائرة"
                - "شكل قلب أحمر"
                - "نجمة خماسية كبيرة"
                - "خط مائل أزرق"
                - "مربع أصفر صغير"
                - "زهرة وردية"
                - "حلزون أخضر"
                """)
                
            with gr.Column(scale=2):
                output_image = gr.Image(label="🎨 النتيجة البصرية", type="filepath")
        
        # ربط الأحداث
        generate_btn.click(
            fn=process_command,
            inputs=text_input,
            outputs=output_image
        )
        
        text_input.submit(
            fn=process_command,
            inputs=text_input,
            outputs=output_image
        )
        
        gr.Markdown("""
        ---
        ## 🧮 كيف يعمل النظام الثوري:
        
        ### 1️⃣ فهم اللغة الطبيعية
        - تحليل الجملة العربية
        - استخراج الشكل المطلوب
        - تحديد اللون والحجم
        
        ### 2️⃣ التحويل للمعادلات
        - ترجمة المعنى لمعادلات رياضية
        - استخدام دوال رياضية نقية
        - بناء المعادلات الديناميكية
        
        ### 3️⃣ التنفيذ البصري
        - رسم الشكل من المعادلات
        - تطبيق الألوان والأحجام
        - عرض المعادلة المستخدمة
        
        ### 🌟 لماذا هذا ثوري؟
        ✅ **فهم طبيعي للغة** بدون شبكات عصبية  
        ✅ **شفافية كاملة** - كل معادلة مرئية  
        ✅ **لا بيانات ضخمة** - منطق رياضي خالص  
        ✅ **تفاعل فوري** - من النص للشكل مباشرة  
        
        **🎯 هذا هو مستقبل الذكاء الاصطناعي الشفاف!**
        """)
    
    return interface

if __name__ == "__main__":
    interface = create_interface()
    interface.launch(server_name="0.0.0.0", server_port=7862)
