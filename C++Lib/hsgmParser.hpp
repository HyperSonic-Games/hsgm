// HsgmParser.hpp

#pragma once

#include <string>
#include <vector>
#include <map>
#include <tuple>

// Color type
struct Color {
    unsigned char r, g, b, a;
};

// Binding class
class Binding {
public:
    Binding(const std::map<std::string, std::string>& texturePaths,
            const std::map<std::string, Color>& colliderColors,
            const std::map<std::string, Color>& triggerColors);

    std::string GetTexturePath(const std::string& textureName) const;
    Color GetColliderColor(const std::string& colliderName) const;
    Color GetTriggerColor(const std::string& triggerName) const;

private:
    std::map<std::string, std::string> texturePaths;
    std::map<std::string, Color> colliderColors;
    std::map<std::string, Color> triggerColors;
};

// Texture, Collider, and Trigger structures
struct TextureData {
    std::string name;
    std::tuple<int, int> position;
};

struct ColliderData {
    std::string name;
    std::tuple<int, int> position;
};

struct TriggerData {
    std::string name;
    std::tuple<int, int> position;
};

// Function declarations
extern "C" {
    void GenerateTextures(const Binding& binding, const std::vector<TextureData>& textureData, const std::string& outputPath);
    void GenerateColliders(const Binding& binding, const std::vector<ColliderData>& colliderData, const std::string& outputPath);
    void GenerateTriggers(const Binding& binding, const std::vector<TriggerData>& triggerData, const std::string& outputPath);
