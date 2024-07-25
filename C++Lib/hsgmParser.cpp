#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <unordered_map>
#include <stdexcept>

#include <png.h>

class Binding {
public:
    std::unordered_map<std::string, std::string> textureBindings;
    std::unordered_map<std::string, std::string> colliderBindings;
    std::unordered_map<std::string, std::string> triggerBindings;

    std::string GetTexturePath(const std::string& textureName) const {
        auto it = textureBindings.find(textureName);
        return it != textureBindings.end() ? it->second : "";
    }

    std::string GetColliderColor(const std::string& colliderType) const {
        auto it = colliderBindings.find(colliderType);
        return it != colliderBindings.end() ? it->second : "";
    }

    std::string GetTriggerColor(const std::string& triggerType) const {
        auto it = triggerBindings.find(triggerType);
        return it != triggerBindings.end() ? it->second : "";
    }
};

void ParseMapFile(const std::string& filePath, Binding& binding,
                  std::unordered_map<std::string, std::pair<int, int>>& textures,
                  std::unordered_map<std::string, std::pair<int, int>>& colliders,
                  std::unordered_map<std::string, std::pair<int, int>>& triggers) {
    std::ifstream file(filePath);
    if (!file.is_open()) {
        throw std::runtime_error("Failed to open file.");
    }

    std::unordered_map<std::string, std::string> textureBindings;
    std::unordered_map<std::string, std::string> colliderBindings;
    std::unordered_map<std::string, std::string> triggerBindings;

    std::string line;
    while (std::getline(file, line)) {
        size_t equalPos = line.find('=');
        if (equalPos == std::string::npos) continue;

        std::string key = line.substr(0, equalPos);
        std::string value = line.substr(equalPos + 1);
        value.erase(0, value.find_first_not_of(" \t"));
        value.erase(value.find_last_not_of(" \t") + 1);

        if (key.find("Texture") != std::string::npos) {
            textures[key] = ParseTuple(value);
        } else if (key.find("Collider") != std::string::npos) {
            colliders[key] = ParseTuple(value);
        } else if (key.find("Trigger") != std::string::npos) {
            triggers[key] = ParseTuple(value);
        } else if (key.find("Binding") != std::string::npos) {
            auto bindingParts = Split(key, ':');
            if (bindingParts.size() == 2) {
                std::string type = bindingParts[0];
                std::string name = bindingParts[1];
                if (type == "Texture") {
                    textureBindings[name] = value;
                } else if (type == "Collider") {
                    colliderBindings[name] = value;
                } else if (type == "Trigger") {
                    triggerBindings[name] = value;
                }
            }
        }
    }

    binding.textureBindings = textureBindings;
    binding.colliderBindings = colliderBindings;
    binding.triggerBindings = triggerBindings;
}

std::pair<int, int> ParseTuple(const std::string& str) {
    size_t commaPos = str.find(',');
    if (commaPos == std::string::npos) throw std::runtime_error("Invalid tuple format.");

    int x = std::stoi(str.substr(0, commaPos));
    int y = std::stoi(str.substr(commaPos + 1));
    return { x, y };
}

void WritePng(const std::string& filePath, int width, int height, const std::vector<unsigned char>& pixelData) {
    FILE *fp = fopen(filePath.c_str(), "wb");
    if (!fp) throw std::runtime_error("Failed to open file.");

    png_structp png = png_create_write_struct(PNG_LIBPNG_VER_STRING, nullptr, nullptr, nullptr);
    if (!png) {
        fclose(fp);
        throw std::runtime_error("Failed to create PNG write structure.");
    }

    png_infop info = png_create_info_struct(png);
    if (!info) {
        png_destroy_write_struct(&png, nullptr);
        fclose(fp);
        throw std::runtime_error("Failed to create PNG info structure.");
    }

    if (setjmp(png_jmpbuf(png))) {
        png_destroy_write_struct(&png, &info);
        fclose(fp);
        throw std::runtime_error("Error during PNG creation.");
    }

    png_init_io(png, fp);
    png_set_IHDR(
        png,
        info,
        width, height,
        8, PNG_COLOR_TYPE_RGBA, PNG_INTERLACE_NONE,
        PNG_COMPRESSION_TYPE_DEFAULT,
        PNG_FILTER_TYPE_DEFAULT
    );

    png_write_info(png, info);

    std::vector<png_bytep> rows(height);
    for (int y = 0; y < height; ++y) {
        rows[y] = const_cast<png_bytep>(&pixelData[y * width * 4]);
    }
    png_write_image(png, rows.data());
    png_write_end(png, nullptr);

    png_destroy_write_struct(&png, &info);
    fclose(fp);
}

int main(int argc, char* argv[]) {
    if (argc != 3) {
        std::cerr << "Usage: MapParserAndPngEncoder <mapFile> <outputPng>" << std::endl;
        return 1;
    }

    std::string mapFile = argv[1];
    std::string outputPng = argv[2];

    try {
        Binding binding;
        std::unordered_map<std::string, std::pair<int, int>> textures;
        std::unordered_map<std::string, std::pair<int, int>> colliders;
        std::unordered_map<std::string, std::pair<int, int>> triggers;

        // Parse the map file
        ParseMapFile(mapFile, binding, textures, colliders, triggers);

        // Generate a basic image data array
        int width = 256;
        int height = 256;
        std::vector<unsigned char> pixelData(width * height * 4); // RGBA

        // Fill image data with a simple pattern for demonstration
        for (int y = 0; y < height; ++y) {
            for (int x = 0; x < width; ++x) {
                int index = (y * width + x) * 4;
                pixelData[index] = static_cast<unsigned char>(x % 256);       // Red
                pixelData[index + 1] = static_cast<unsigned char>(y % 256);   // Green
                pixelData[index + 2] = 0;                                      // Blue
                pixelData[index + 3] = 255;                                    // Alpha
            }
        }

        // Save as PNG
        WritePng(outputPng, width, height, pixelData);

        std::cout << "PNG file generated: " << outputPng << std::endl;
    } catch (const std::exception& ex) {
        std::cerr << "Error: " << ex.what() << std::endl;
        return 1;
    }

    return 0;
}
