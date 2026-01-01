#ifndef MEME_PROCESSOR_HPP
#define MEME_PROCESSOR_HPP

#include <string>
#include <vector>

namespace mememage {

struct TextOverlay {
    std::string text;
    int x;
    int y;
    int font_size;
    std::string position; // "top", "bottom", "custom"
    std::string color;
};

class MemeProcessor {
public:
    MemeProcessor();
    ~MemeProcessor();

    // Load an image from file
    bool loadImage(const std::string& filepath);
    
    // Add text overlay to the image
    bool addText(const TextOverlay& overlay);
    
    // Save the processed image
    bool saveImage(const std::string& output_path);
    
    // Process meme with top and bottom text (classic meme format)
    bool createClassicMeme(const std::string& input_path,
                          const std::string& top_text,
                          const std::string& bottom_text,
                          const std::string& output_path);
    
    // Get image dimensions
    std::pair<int, int> getDimensions() const;

private:
    class Impl;
    Impl* pImpl;
    
    void drawText(const std::string& text, int x, int y, int font_size, 
                  const std::string& color);
    std::vector<std::string> wrapText(const std::string& text, int max_width);
};

} // namespace mememage

#endif // MEME_PROCESSOR_HPP
