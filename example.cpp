#include "meme_processor.hpp"
#include <iostream>

int main() {
    std::cout << "=== MemEmage C++ Image Processor Example ===" << std::endl;
    
    mememage::MemeProcessor processor;
    
    // Example 1: Classic meme with top and bottom text
    std::cout << "\n1. Creating classic meme..." << std::endl;
    processor.createClassicMeme(
        "input/drake.jpg",
        "USING BASIC PHOTO EDITORS",
        "USING MEMEMAGE",
        "output/drake_meme.jpg"
    );
    
    // Example 2: Custom positioned text
    std::cout << "\n2. Creating custom positioned meme..." << std::endl;
    processor.loadImage("input/distracted_boyfriend.jpg");
    
    mememage::TextOverlay overlay1;
    overlay1.text = "NEW MEME GENERATOR";
    overlay1.position = "custom";
    overlay1.x = 200;
    overlay1.y = 150;
    overlay1.font_size = 36;
    overlay1.color = "white";
    processor.addText(overlay1);
    
    mememage::TextOverlay overlay2;
    overlay2.text = "OLD MEME TOOLS";
    overlay2.position = "custom";
    overlay2.x = 600;
    overlay2.y = 150;
    overlay2.font_size = 36;
    overlay2.color = "white";
    processor.addText(overlay2);
    
    processor.saveImage("output/custom_meme.jpg");
    
    // Get dimensions
    auto [width, height] = processor.getDimensions();
    std::cout << "\nProcessed image dimensions: " << width << "x" << height << std::endl;
    
    std::cout << "\n✅ Meme processing complete!" << std::endl;
    
    return 0;
}
