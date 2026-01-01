# 🎭 MemEmage - Modern Meme Generator

**MemEmage** is a full-stack meme generator application built with **C++** for high-performance image processing and **Rust** for a secure, blazingly fast backend API.

## 🚀 Features

### Core Features
- ✨ **User Authentication** - Secure signup and login with JWT
- 🎨 **Classic Meme Creation** - Top and bottom text overlays
- 📸 **Template Library** - Pre-built meme templates
- 💾 **Save & Share** - Store your creations
- 📊 **Analytics** - View counts and likes
- 👤 **User Profiles** - Manage your meme collection

### Technical Highlights
- **C++ Image Processor** - Fast text overlay rendering
- **Rust Backend** - Memory-safe, concurrent API server
- **PostgreSQL Database** - Reliable data storage
- **JWT Authentication** - Secure user sessions
- **RESTful API** - Clean, documented endpoints
- **Responsive UI** - Works on desktop and mobile

## 🏗️ Architecture

```
┌─────────────────┐
│   Frontend      │  HTML/CSS/JS
│   (Web UI)      │
└────────┬────────┘
         │ HTTP/REST
         ▼
┌─────────────────┐
│  Rust Backend   │  Actix-Web Server
│  (API Server)   │  • Auth (JWT)
│                 │  • Meme CRUD
└────────┬────────┘  • User Management
         │
         ├──► PostgreSQL (Data)
         │
         ▼ FFI
┌─────────────────┐
│ C++ Processor   │  Image Processing
│ (libmeme_proc)  │  • Text Overlay
│                 │  • Font Rendering
└─────────────────┘  • Image Export
```

## 📁 Project Structure

```
mememage/
├── backend/              # Rust API server
│   ├── src/
│   │   ├── main.rs      # Server entry point
│   │   ├── lib.rs       # Library exports
│   │   ├── models.rs    # Data models
│   │   ├── database.rs  # Database operations
│   │   ├── auth.rs      # Authentication
│   │   ├── handlers.rs  # HTTP handlers
│   │   └── image_ffi.rs # C++ FFI bindings
│   ├── Cargo.toml       # Rust dependencies
│   ├── schema.sql       # Database schema
│   └── .env.example     # Environment template
│
├── image-processor/     # C++ image library
│   ├── meme_processor.hpp
│   ├── meme_processor.cpp
│   ├── example.cpp
│   └── CMakeLists.txt
│
└── frontend/            # Web interface
    ├── index.html
    ├── styles.css
    └── app.js
```

## 🛠️ Installation & Setup

### Prerequisites

- **Rust** (1.70+): https://rustup.rs/
- **C++ Compiler** (GCC 11+ or Clang 14+)
- **CMake** (3.15+)
- **PostgreSQL** (13+)
- **Node.js** (optional, for frontend dev server)

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/mememage.git
cd mememage
```

### 2. Build C++ Image Processor

```bash
cd image-processor
mkdir build && cd build
cmake ..
make
sudo make install  # Optional: install system-wide
```

### 3. Setup Database

```bash
# Create database
createdb mememage

# Run schema
psql -d mememage -f backend/schema.sql
```

### 4. Configure Backend

```bash
cd backend
cp .env.example .env

# Edit .env with your settings
nano .env
```

### 5. Build & Run Backend

```bash
# Build
cargo build --release

# Run
cargo run --release
```

Server will start at `http://localhost:8080`

### 6. Serve Frontend

```bash
cd frontend

# Option 1: Simple Python server
python3 -m http.server 3000

# Option 2: Node.js server
npx serve -p 3000
```

Frontend will be at `http://localhost:3000`

## 📚 API Documentation

### Authentication

#### Signup
```http
POST /api/auth/signup
Content-Type: application/json

{
  "username": "mememaster",
  "email": "meme@example.com",
  "password": "securepass123"
}
```

#### Login
```http
POST /api/auth/login
Content-Type: application/json

{
  "username": "mememaster",
  "password": "securepass123"
}
```

### Memes

#### Create Meme
```http
POST /api/memes
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "My Awesome Meme",
  "top_text": "WHEN YOU CODE",
  "bottom_text": "IN C++ AND RUST",
  "template_name": "drake"
}
```

#### Get All Memes
```http
GET /api/memes?limit=20&offset=0
```

#### Get Meme by ID
```http
GET /api/memes/{id}
```

#### Get User's Memes
```http
GET /api/memes/user/my-memes
Authorization: Bearer <token>
```

#### Like Meme
```http
POST /api/memes/{id}/like
```

## 🔧 Development

### Running Tests

**Rust Backend:**
```bash
cd backend
cargo test
```

**C++ Processor:**
```bash
cd image-processor/build
ctest
```

### Code Formatting

**Rust:**
```bash
cargo fmt
cargo clippy
```

**C++:**
```bash
clang-format -i *.cpp *.hpp
```

## 🐳 Docker Deployment

```bash
# Build all services
docker-compose build

# Start services
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

## 🎯 Roadmap

- [x] Core meme creation
- [x] User authentication
- [x] Database storage
- [ ] Image upload support
- [ ] Advanced text positioning
- [ ] Multiple fonts
- [ ] Color customization
- [ ] Meme templates marketplace
- [ ] Social sharing
- [ ] Comment system
- [ ] Mobile apps (iOS/Android)

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

## 👥 Authors

- **Your Name** - *Initial work* - [YourGithub](https://github.com/yourusername)

## 🙏 Acknowledgments

- STB Libraries for image processing
- Actix-Web for the awesome Rust web framework
- The Rust and C++ communities

## 📧 Contact

- Website: https://mememage.example.com
- Email: support@mememage.example.com
- Twitter: [@MemEmage](https://twitter.com/mememage)

---

**Built with ❤️ using C++ and Rust**
