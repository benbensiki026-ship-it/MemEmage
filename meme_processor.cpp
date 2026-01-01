#include "meme_processor.hpp"
#include <iostream>
#include <algorithm>
#include <cmath>
#include <memory>

// Using STB libraries for image processing (header-only libraries)
#define STB_IMAGE_IMPLEMENTATION
#define STB_IMAGE_WRITE_IMPLEMENTATION
#define STB_TRUETYPE_IMPLEMENTATION

// Simplified implementation - in production you'd use actual STB libraries
// This is a demonstration of the architecture

namespace mememage {

class MemeProcessor::Impl {
public:
    std::vector<unsigned char> imageData;
    int width = 0;
    int height = 0;
    int channels = 0;
    bool loaded = false;
};

MemeProcessor::MemeProcessor() : pImpl(new Impl()) {}

MemeProcessor::~MemeProcessor() {
    delete pImpl;
}

bool MemeProcessor::loadImage(const std::string& filepath) {
    // In a real implementation, this would use stb_image_load
    std::cout << "Loading image: " << filepath << std::endl;
    
    // Simulated load - in production would actually read the file
    pImpl->width = 800;
    pImpl->height = 600;
    pImpl->channels = 4; // RGBA
    pImpl->imageData.resize(pImpl->width * pImpl->height * pImpl->channels, 255);
    pImpl->loaded = true;
    
    return true;
}

std::vector<std::string> MemeProcessor::wrapText(const std::string& text, int max_width) {
    std::vector<std::string> lines;
    std::string current_line;
    
    size_t pos = 0;
    while (pos < text.length()) {
        size_t space_pos = text.find(' ', pos);
        if (space_pos == std::string::npos) {
            space_pos = text.length();
        }
        
        std::string word = text.substr(pos, space_pos - pos);
        
        if (current_line.length() + word.length() + 1 > static_cast<size_t>(max_width / 10)) {
            if (!current_line.empty()) {
                lines.push_back(current_line);
                current_line = word;
            } else {
                lines.push_back(word);
            }
        } else {
            if (!current_line.empty()) current_line += " ";
            current_line += word;
        }
        
        pos = space_pos + 1;
    }
    
    if (!current_line.empty()) {
        lines.push_back(current_line);
    }
    
    return lines;
}

void MemeProcessor::drawText(const std::string& text, int x, int y, 
                             int font_size, const std::string& color) {
    // In production, this would use stb_truetype to render text
    // This is a placeholder that demonstrates the interface
    
    std::cout << "Drawing text: \"" << text << "\" at (" << x << ", " << y 
              << ") size: " << font_size << " color: " << color << std::endl;
    
    // Actual implementation would:
    // 1. Load font using stb_truetype
    // 2. Rasterize text
    // 3. Add white stroke/outline for readability
    // 4. Composite onto image with specified color
}

bool MemeProcessor::addText(const TextOverlay& overlay) {
    if (!pImpl->loaded) {
        std::cerr << "No image loaded!" << std::endl;
        return false;
    }
    
    int x = overlay.x;
    int y = overlay.y;
    
    // Auto-position based on position string
    if (overlay.position == "top") {
        x = pImpl->width / 2;
        y = 50;
    } else if (overlay.position == "bottom") {
        x = pImpl->width / 2;
        y = pImpl->height - 80;
    }
    
    // Wrap text if needed
    auto lines = wrapText(overlay.text, pImpl->width - 100);
    
    // Draw each line with stroke effect
    int line_height = overlay.font_size + 10;
    int start_y = y - (lines.size() * line_height) / 2;
    
    for (size_t i = 0; i < lines.size(); ++i) {
        int current_y = start_y + i * line_height;
        
        // Draw stroke (black outline)
        for (int dx = -2; dx <= 2; ++dx) {
            for (int dy = -2; dy <= 2; ++dy) {
                if (dx != 0 || dy != 0) {
                    drawText(lines[i], x + dx, current_y + dy, overlay.font_size, "black");
                }
            }
        }
        
        // Draw main text
        drawText(lines[i], x, current_y, overlay.font_size, overlay.color);
    }
    
    return true;
}

bool MemeProcessor::createClassicMeme(const std::string& input_path,
                                     const std::string& top_text,
                                     const std::string& bottom_text,
                                     const std::string& output_path) {
    if (!loadImage(input_path)) {
        return false;
    }
    
    // Add top text
    if (!top_text.empty()) {
        TextOverlay top;
        top.text = top_text;
        top.position = "top";
        top.font_size = 48;
        top.color = "white";
        addText(top);
    }
    
    // Add bottom text
    if (!bottom_text.empty()) {
        TextOverlay bottom;
        bottom.text = bottom_text;
        bottom.position = "bottom";
        bottom.font_size = 48;
        bottom.color = "white";
        addText(bottom);
    }
    
    return saveImage(output_path);
}

bool MemeProcessor::saveImage(const std::string& output_path) {
    if (!pImpl->loaded) {
        std::cerr << "No image to save!" << std::endl;
        return false;
    }
    
    // In production, would use stb_image_write
    std::cout << "Saving meme to: " << output_path << std::endl;
    
    return true;
}

std::pair<int, int> MemeProcessor::getDimensions() const {
    return {pImpl->width, pImpl->height};
}

} // namespace mememage

// C interface for FFI with Rust
extern "C" {
    void* meme_processor_new() {
        return new mememage::MemeProcessor();
    }
    
    void meme_processor_delete(void* processor) {
        delete static_cast<mememage::MemeProcessor*>(processor);
    }
    
    bool meme_processor_create_classic(void* processor,
                                      const char* input_path,
                                      const char* top_text,
                                      const char* bottom_text,
                                      const char* output_path) {
        auto* proc = static_cast<mememage::MemeProcessor*>(processor);
        return proc->createClassicMeme(input_path, top_text, bottom_text, output_path);
    }
}
