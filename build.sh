#!/bin/bash

# MemEmage Build Script
# This script builds both the C++ image processor and Rust backend

set -e  # Exit on error

echo "🎭 Building MemEmage..."

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Build C++ Image Processor
echo -e "${BLUE}📦 Building C++ Image Processor...${NC}"
cd image-processor
if [ ! -d "build" ]; then
    mkdir build
fi
cd build
cmake .. || { echo -e "${RED}❌ CMake failed${NC}"; exit 1; }
make -j$(nproc) || { echo -e "${RED}❌ Make failed${NC}"; exit 1; }
echo -e "${GREEN}✅ C++ Image Processor built successfully${NC}"

# Return to root
cd ../..

# Build Rust Backend
echo -e "${BLUE}📦 Building Rust Backend...${NC}"
cd backend
cargo build --release || { echo -e "${RED}❌ Cargo build failed${NC}"; exit 1; }
echo -e "${GREEN}✅ Rust Backend built successfully${NC}"

# Return to root
cd ..

echo -e "${GREEN}🎉 Build complete!${NC}"
echo ""
echo "To run the application:"
echo "  1. Setup database: psql -d mememage -f backend/schema.sql"
echo "  2. Configure .env: cp backend/.env.example backend/.env"
echo "  3. Start backend: cd backend && cargo run --release"
echo "  4. Start frontend: cd frontend && python3 -m http.server 3000"
