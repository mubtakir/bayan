#!/bin/bash

# ═══════════════════════════════════════════════════════════════════════════
# سكريبت تشغيل اختبارات وكيل البيان المساعد
# Script to run Agent Tests
# ═══════════════════════════════════════════════════════════════════════════

set -e

# الألوان
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# المتغيرات
TESTS_DIR="tests"
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
START_TIME=$(date +%s)

# ═══════════════════════════════════════════════════════════════════════════
# دوال مساعدة
# ═══════════════════════════════════════════════════════════════════════════

print_header() {
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}  $1"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

print_section() {
    echo -e "${YELLOW}📋 $1${NC}"
    echo "───────────────────────────────────────────────────────────────"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

# ═══════════════════════════════════════════════════════════════════════════
# التحقق من المتطلبات
# ═══════════════════════════════════════════════════════════════════════════

check_requirements() {
    print_section "التحقق من المتطلبات"
    
    # التحقق من وجود مجلد الاختبارات
    if [ ! -d "$TESTS_DIR" ]; then
        print_error "مجلد الاختبارات غير موجود: $TESTS_DIR"
        exit 1
    fi
    print_success "مجلد الاختبارات موجود"
    
    # التحقق من وجود ملفات الاختبارات
    local test_files=(
        "$TESTS_DIR/agent_tests.ab"
        "$TESTS_DIR/integration_tests.ab"
        "$TESTS_DIR/performance_tests.ab"
        "$TESTS_DIR/cli_tests.ab"
    )
    
    for file in "${test_files[@]}"; do
        if [ ! -f "$file" ]; then
            print_error "ملف الاختبار غير موجود: $file"
            exit 1
        fi
        print_success "ملف الاختبار موجود: $file"
    done
    
    # التحقق من وجود مترجم البيان
    if ! command -v bayan &> /dev/null; then
        print_error "مترجم البيان غير مثبت"
        exit 1
    fi
    print_success "مترجم البيان مثبت"
    
    echo ""
}

# ═══════════════════════════════════════════════════════════════════════════
# تشغيل الاختبارات
# ═══════════════════════════════════════════════════════════════════════════

run_test() {
    local test_file=$1
    local test_name=$2
    
    print_info "تشغيل: $test_name"
    
    if bayan "$test_file" 2>&1; then
        print_success "$test_name نجح"
        ((PASSED_TESTS++))
    else
        print_error "$test_name فشل"
        ((FAILED_TESTS++))
    fi
    
    ((TOTAL_TESTS++))
    echo ""
}

# ═══════════════════════════════════════════════════════════════════════════
# تشغيل جميع الاختبارات
# ═══════════════════════════════════════════════════════════════════════════

run_all_tests() {
    print_header "🧪 تشغيل اختبارات وكيل البيان المساعد 🧪"
    
    # اختبارات الوحدات
    print_section "اختبارات الوحدات (Unit Tests)"
    run_test "$TESTS_DIR/agent_tests.ab" "اختبارات الوحدات"
    
    # اختبارات التكامل
    print_section "اختبارات التكامل (Integration Tests)"
    run_test "$TESTS_DIR/integration_tests.ab" "اختبارات التكامل"
    
    # اختبارات الأداء
    print_section "اختبارات الأداء (Performance Tests)"
    run_test "$TESTS_DIR/performance_tests.ab" "اختبارات الأداء"
    
    # اختبارات CLI
    print_section "اختبارات CLI (CLI Tests)"
    run_test "$TESTS_DIR/cli_tests.ab" "اختبارات CLI"
}

# ═══════════════════════════════════════════════════════════════════════════
# طباعة النتائج
# ═══════════════════════════════════════════════════════════════════════════

print_results() {
    local end_time=$(date +%s)
    local duration=$((end_time - START_TIME))
    
    echo ""
    print_header "📊 نتائج الاختبارات 📊"
    
    echo -e "${BLUE}┌────────────────────────────────────────────────────────────────┐${NC}"
    echo -e "${BLUE}│${NC} إجمالي الاختبارات:  $TOTAL_TESTS"
    echo -e "${BLUE}│${NC} الاختبارات الناجحة:  ${GREEN}$PASSED_TESTS${NC}"
    echo -e "${BLUE}│${NC} الاختبارات الفاشلة:  ${RED}$FAILED_TESTS${NC}"
    echo -e "${BLUE}│${NC} المدة الزمنية:      ${duration}s"
    echo -e "${BLUE}└────────────────────────────────────────────────────────────────┘${NC}"
    echo ""
    
    if [ $FAILED_TESTS -eq 0 ]; then
        print_success "جميع الاختبارات نجحت! ✨"
        echo ""
        print_header "🎉 تم إكمال المرحلة 2 بنجاح! 🎉"
        return 0
    else
        print_error "بعض الاختبارات فشلت"
        echo ""
        return 1
    fi
}

# ═══════════════════════════════════════════════════════════════════════════
# البرنامج الرئيسي
# ═══════════════════════════════════════════════════════════════════════════

main() {
    check_requirements
    run_all_tests
    print_results
}

# تشغيل البرنامج الرئيسي
main

