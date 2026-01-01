# Technical Documentation: C++ & Rust Integration

## Overview

MemEmage demonstrates a powerful hybrid architecture combining C++'s performance for image processing with Rust's safety and concurrency for the backend API.

## Architecture Design

### Why C++ for Image Processing?

1. **Performance**: Native code execution with minimal overhead
2. **Library Ecosystem**: Access to mature image processing libraries (STB, OpenCV)
3. **Memory Control**: Direct pixel manipulation and buffer management
4. **SIMD Optimization**: Vectorized operations for text rendering

### Why Rust for Backend?

1. **Memory Safety**: Prevents common bugs (null pointers, buffer overflows)
2. **Concurrency**: Safe concurrent request handling without data races
3. **Type System**: Strong guarantees at compile time
4. **Performance**: Zero-cost abstractions, comparable to C++
5. **Ecosystem**: Excellent web frameworks (Actix-Web)

## C++ Image Processor

### Core Components

```cpp
class MemeProcessor {
    // Private implementation idiom (Pimpl)
    // Hides implementation details, reduces compile dependencies
    class Impl;
    Impl* pImpl;
};
```

### Text Rendering Pipeline

1. **Load Image**: Read source image into memory buffer
2. **Font Rasterization**: Convert text to pixels using TrueType fonts
3. **Stroke Generation**: Create black outline for readability
4. **Compositing**: Blend text onto image with alpha channel
5. **Export**: Save processed image as JPEG/PNG

### Memory Management

- RAII principles for resource management
- Automatic cleanup via destructors
- No manual memory leaks

## Rust Backend

### FFI (Foreign Function Interface)

Rust communicates with C++ through a C-compatible ABI:

```rust
#[link(name = "meme_processor")]
extern "C" {
    fn meme_processor_new() -> *mut c_void;
    fn meme_processor_create_classic(...) -> bool;
}
```

### Safety Guarantees

1. **Wrapper Type**: `MemeProcessor` struct wraps raw C pointer
2. **Drop Trait**: Automatic cleanup when Rust object goes out of scope
3. **Send/Sync**: Thread-safety markers for concurrent usage

```rust
impl Drop for MemeProcessor {
    fn drop(&mut self) {
        unsafe { meme_processor_delete(self.processor); }
    }
}
```

## API Request Flow

```
1. HTTP Request → Actix-Web Handler
2. JWT Validation → Extract User Claims
3. Request Validation → Check Input Data
4. Image Processing → Call C++ via FFI
5. Database Save → Store Meme Metadata
6. HTTP Response → Return Success/Error
```

## Database Schema

### Users Table
- `id`: UUID primary key
- `username`: Unique username
- `email`: Unique email
- `password_hash`: BCrypt hashed password
- `created_at`, `updated_at`: Timestamps

### Memes Table
- `id`: UUID primary key
- `user_id`: Foreign key to users
- `title`: Meme title
- `image_url`: Path to generated image
- `top_text`, `bottom_text`: Meme text
- `views`, `likes`: Analytics counters
- `created_at`: Timestamp

## Security Considerations

### Authentication
- **Password Hashing**: BCrypt with cost factor 12
- **JWT Tokens**: HS256 algorithm, 7-day expiration
- **Token Storage**: Client-side localStorage

### Input Validation
- **Rust Validator**: Compile-time validation rules
- **SQL Injection**: Prevented by SQLx prepared statements
- **XSS Protection**: Sanitized HTML output

### Authorization
- **JWT Claims**: User ID embedded in token
- **Ownership Check**: Users can only modify their own memes

## Performance Optimizations

### C++ Side
- Stack allocation for small buffers
- Memory pooling for frequent allocations
- SIMD instructions for pixel operations
- Compile-time optimizations (-O3, LTO)

### Rust Side
- Connection pooling (SQLx)
- Async I/O (Tokio runtime)
- Zero-copy deserialization (Serde)
- Compiled to native code

### Database
- Indexed queries (user_id, created_at)
- Prepared statements (query plan caching)
- Connection pooling (max 5 connections)

## Error Handling

### C++ Layer
```cpp
bool createMeme(...) {
    if (!imageData) return false;
    try {
        // Process image
        return true;
    } catch (...) {
        return false;
    }
}
```

### Rust Layer
```rust
async fn create_meme(...) -> HttpResponse {
    match database::create_meme(...).await {
        Ok(meme) => HttpResponse::Created().json(...),
        Err(e) => HttpResponse::InternalServerError().json(...)
    }
}
```

## Testing Strategy

### Unit Tests
- C++: Google Test framework
- Rust: Built-in test framework (`cargo test`)

### Integration Tests
- API endpoint testing
- Database transaction testing
- FFI boundary testing

### Load Testing
- Apache Bench (ab)
- wrk2
- Target: 1000 req/s

## Deployment Architecture

```
┌─────────────┐
│   Nginx     │ (Reverse Proxy, Static Files)
└──────┬──────┘
       │
┌──────▼──────┐
│   Rust API  │ (Actix-Web, Port 8080)
└──────┬──────┘
       │
       ├──► PostgreSQL (Database)
       │
       └──► C++ Library (Image Processing)
```

## Build Process

1. **C++ Compilation**: CMake → Make → Shared library (.so/.dll)
2. **Rust Compilation**: Cargo → Link C++ library → Executable
3. **Docker Build**: Multi-stage build for optimized image size

## Future Enhancements

### Technical
- [ ] GPU acceleration (CUDA/Vulkan)
- [ ] Image format detection
- [ ] WebP support
- [ ] CDN integration for image serving
- [ ] Horizontal scaling with Redis cache
- [ ] gRPC for internal services

### Features
- [ ] Video meme support
- [ ] GIF generation
- [ ] Face detection for auto-positioning
- [ ] AI-powered caption suggestions
- [ ] Meme template marketplace

## Resources

- **C++ Standards**: ISO C++17
- **Rust Edition**: 2021
- **FFI Guide**: https://doc.rust-lang.org/nomicon/ffi.html
- **Actix-Web**: https://actix.rs/
- **SQLx**: https://github.com/launchbadge/sqlx
