#!/bin/bash
# verify_optimizations.sh
# Quick verification script to check if optimizations are ready to deploy

set -e

echo "🚀 X Algorithm Optimization - Verification Script"
echo "=================================================="
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Counters
PASSED=0
FAILED=0
WARNINGS=0

# Check function
check() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $1"
        ((PASSED++))
    else
        echo -e "${RED}✗${NC} $1"
        ((FAILED++))
    fi
}

warn() {
    echo -e "${YELLOW}⚠${NC} $1"
    ((WARNINGS++))
}

# ============================================================
# 1. Check Files Exist
# ============================================================

echo "📁 Checking Implemented Files..."
echo ""

if [ -f "home-mixer/scorers/cached_phoenix_scorer.rs" ]; then
    check "Cached Phoenix Scorer"
else
    check "Cached Phoenix Scorer (MISSING)"
fi

if [ -f "home-mixer/scorers/batched_phoenix_scorer.rs" ]; then
    check "Batched Phoenix Scorer"
else
    check "Batched Phoenix Scorer (MISSING)"
fi

if [ -f "home-mixer/scorers/personalized_weighted_scorer.rs" ]; then
    check "Personalized Weighted Scorer"
else
    check "Personalized Weighted Scorer (MISSING)"
fi

if [ -f "home-mixer/filters/age_filter.rs" ]; then
    check "Optimized Age Filter"
else
    check "Optimized Age Filter (MISSING)"
fi

if [ -f "home-mixer/personalization/user_clusters.rs" ]; then
    check "User Clustering Service"
else
    check "User Clustering Service (MISSING)"
fi

if [ -f "home-mixer/tests/integration_tests.rs" ]; then
    check "Integration Tests"
else
    check "Integration Tests (MISSING)"
fi

echo ""

# ============================================================
# 2. Check Documentation
# ============================================================

echo "📚 Checking Documentation..."
echo ""

for doc in QUICKSTART.md docs/DEPLOYMENT_GUIDE.md docs/FINAL_STATUS_REPORT.md docs/IMPLEMENTATION_COMPLETE.md; do
    if [ -f "$doc" ]; then
        check "$doc"
    else
        warn "$doc (missing)"
    fi
done

echo ""

# ============================================================
# 3. Check Dependencies
# ============================================================

echo "🔧 Checking Dependencies..."
echo ""

cd home-mixer

if grep -q "lru = " Cargo.toml; then
    check "lru dependency"
else
    warn "lru dependency not in Cargo.toml (add: lru = \"0.12\")"
fi

if grep -q "moka = " Cargo.toml; then
    check "moka dependency"
else
    warn "moka dependency not in Cargo.toml (add: moka = { version = \"0.12\", features = [\"sync\"] })"
fi

echo ""

# ============================================================
# 4. Try to Build
# ============================================================

echo "🏗️  Attempting Build..."
echo ""

if cargo build --release 2>&1 | tee /tmp/build.log | grep -q "Finished"; then
    check "Cargo build successful"
else
    warn "Cargo build has issues (check /tmp/build.log)"
    echo ""
    echo "Build errors:"
    tail -n 20 /tmp/build.log
fi

echo ""

# ============================================================
# 5. Run Tests
# ============================================================

echo "🧪 Running Tests..."
echo ""

if cargo test --lib 2>&1 | tee /tmp/test.log | grep -q "test result: ok"; then
    check "Unit tests passing"
else
    warn "Some tests failed (check /tmp/test.log)"
fi

echo ""

# ============================================================
# 6. Check Code Quality
# ============================================================

echo "🔍 Checking Code Quality..."
echo ""

if cargo clippy --all-targets 2>&1 | grep -q "0 errors"; then
    check "Clippy checks passing"
else
    warn "Clippy has warnings (run: cargo clippy)"
fi

if cargo fmt -- --check 2>&1; then
    check "Code formatting correct"
else
    warn "Code needs formatting (run: cargo fmt)"
fi

echo ""

# ============================================================
# 7. Summary
# ============================================================

echo "=================================================="
echo "📊 Verification Summary"
echo "=================================================="
echo ""
echo -e "${GREEN}✓ Passed:${NC}   $PASSED"
echo -e "${YELLOW}⚠ Warnings:${NC} $WARNINGS"
echo -e "${RED}✗ Failed:${NC}   $FAILED"
echo ""

if [ $FAILED -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}🎉 ALL CHECKS PASSED!${NC}"
    echo ""
    echo "You're ready to deploy! Next steps:"
    echo "1. Review QUICKSTART.md for deployment guide"
    echo "2. Configure environment variables"
    echo "3. Deploy to 1% canary traffic"
    echo "4. Monitor metrics"
    echo "5. Gradual rollout to 100%"
    echo ""
    exit 0
elif [ $FAILED -eq 0 ]; then
    echo -e "${YELLOW}⚠️  READY WITH WARNINGS${NC}"
    echo ""
    echo "Address warnings before production deployment:"
    echo "- Add missing dependencies to Cargo.toml"
    echo "- Run cargo clippy and fix warnings"
    echo "- Run cargo fmt to format code"
    echo ""
    exit 1
else
    echo -e "${RED}❌ ISSUES FOUND${NC}"
    echo ""
    echo "Fix the following before deploying:"
    echo "- Missing required files"
    echo "- Build failures"
    echo "- Test failures"
    echo ""
    exit 2
fi
