# 🚀 MemEmage Quick Start Guide

## What is MemEmage?

MemEmage is a full-stack meme generator that lets users:
- 📝 Sign up and create accounts
- 🎨 Create custom memes with text overlays
- 💾 Save and manage their meme collection
- 👀 View and like memes from other users

## Technology Stack

### Frontend
- **HTML/CSS/JavaScript**: Modern, responsive web interface
- **No frameworks**: Pure vanilla JS for simplicity

### Backend (Rust)
- **Actix-Web**: Fast, async web framework
- **SQLx**: Type-safe database queries
- **JWT**: Secure authentication
- **BCrypt**: Password hashing

### Image Processing (C++)
- **Custom library**: High-performance text rendering
- **STB libraries**: Image loading and writing
- **FFI integration**: Called from Rust via C ABI

### Database
- **PostgreSQL**: Reliable relational database
- **User management**: Accounts, authentication
- **Meme storage**: Metadata and relationships

## Quick Start (5 minutes)

### 1. Install Dependencies

**macOS:**
```bash
brew install rust postgresql cmake
```

**Ubuntu/Debian:**
```bash
sudo apt install rustc cargo postgresql cmake build-essential
```

**Windows:**
- Install Rust from https://rustup.rs/
- Install PostgreSQL from https://www.postgresql.org/
- Install CMake from https://cmake.org/

### 2. Setup Database

```bash
# Start PostgreSQL
sudo systemctl start postgresql  # Linux
brew services start postgresql   # macOS

# Create database
createdb mememage

# Load schema
psql -d mememage -f backend/schema.sql
```

### 3. Build & Run

```bash
# Make build script executable
chmod +x build.sh

# Run build script
./build.sh

# Configure environment
cp backend/.env.example backend/.env

# Start backend (in terminal 1)
cd backend
cargo run --release

# Start frontend (in terminal 2)
cd frontend
python3 -m http.server 3000
```

### 4. Access the Application

- **Frontend**: http://localhost:3000
- **API**: http://localhost:8080/api

## First Steps

1. **Sign Up**: Create an account at http://localhost:3000
2. **Login**: Sign in with your credentials
3. **Create Meme**: Click "Create Meme" and add text
4. **Browse**: View memes created by all users
5. **My Memes**: See your personal collection

## Project Structure

```
mememage/
├── backend/              # Rust API server
│   ├── src/             # Source code
│   ├── Cargo.toml       # Dependencies
│   └── schema.sql       # Database schema
│
├── image-processor/     # C++ image library
│   ├── *.cpp, *.hpp    # C++ source
│   └── CMakeLists.txt  # Build config
│
├── frontend/            # Web interface
│   ├── index.html      # Main page
│   ├── styles.css      # Styling
│   └── app.js          # JavaScript logic
│
├── README.md           # Main documentation
├── TECHNICAL.md        # Technical details
└── build.sh           # Build script
```

## API Endpoints

### Authentication
- `POST /api/auth/signup` - Create account
- `POST /api/auth/login` - Login

### Memes
- `POST /api/memes` - Create meme (requires auth)
- `GET /api/memes` - List all memes
- `GET /api/memes/{id}` - Get specific meme
- `GET /api/memes/user/my-memes` - Get user's memes (requires auth)
- `POST /api/memes/{id}/like` - Like a meme

## Common Issues & Solutions

### Port Already in Use
```bash
# Kill process on port 8080
sudo lsof -ti:8080 | xargs kill -9
```

### Database Connection Error
```bash
# Check PostgreSQL is running
sudo systemctl status postgresql

# Verify connection
psql -d mememage -c "SELECT 1"
```

### Build Errors
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

## Development Tips

### Hot Reload Backend
```bash
cargo install cargo-watch
cargo watch -x run
```

### View Logs
```bash
# Enable debug logging
export RUST_LOG=debug
cargo run
```

### Database Queries
```bash
# Connect to database
psql -d mememage

# List tables
\dt

# Query users
SELECT * FROM users;
```

## Testing

### Backend Tests
```bash
cd backend
cargo test
```

### API Testing (curl)
```bash
# Health check
curl http://localhost:8080/api/health

# Signup
curl -X POST http://localhost:8080/api/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"username":"test","email":"test@test.com","password":"password123"}'

# Login
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"password123"}'
```

## Next Steps

1. **Read TECHNICAL.md**: Understand the architecture
2. **Explore Code**: Look at the Rust and C++ implementations
3. **Add Features**: Extend the application
4. **Deploy**: Use Docker for production deployment

## Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **Actix-Web Docs**: https://actix.rs/docs/
- **C++ Reference**: https://en.cppreference.com/
- **PostgreSQL Docs**: https://www.postgresql.org/docs/

## Support

For issues or questions:
1. Check the README.md
2. Review TECHNICAL.md
3. Search existing issues
4. Create a new issue with details

Happy meme creating! 🎉
