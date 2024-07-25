-- Constants
local tileSize = 16
local width = 256
local height = 256

-- Function to parse the map file
local function parseMapFile(path)
    local file = io.open(path, "r")
    if not file then
        error("Could not open file: " .. path)
    end

    local textures = {}
    local colliders = {}
    local triggers = {}
    local binding = {}

    local section = nil

    for line in file:lines() do
        line = line:match("^%s*(.-)%s*$") -- Trim whitespace

        if line:match("^%[TEXTURES%]$") then
            section = "textures"
        elseif line:match("^%[COLLIDERS%]$") then
            section = "colliders"
        elseif line:match("^%[TRIGGERS%]$") then
            section = "triggers"
        elseif section == "textures" then
            local key, value = line:match("^(.-)%s*=%s*(.-)$")
            if key and value then
                local x, y = value:match("%[(%d+),%s*(%d+)%]")
                textures[key] = {tonumber(x), tonumber(y)}
            end
        elseif section == "colliders" then
            local key, value = line:match("^(.-)%s*=%s*(.-)$")
            if key and value then
                local x, y = value:match("%[(%d+),%s*(%d+)%]")
                colliders[key] = {tonumber(x), tonumber(y)}
            end
        elseif section == "triggers" then
            local key, value = line:match("^(.-)%s*=%s*(.-)$")
            if key and value then
                local x, y = value:match("%[(%d+),%s*(%d+)%]")
                triggers[key] = {tonumber(x), tonumber(y)}
            end
        end
    end

    file:close()

    return textures, colliders, triggers, binding
end

-- Basic Simulation of zlib compression
local function basicCompress(data)
    local compressed = {}
    for i = 1, #data do
        compressed[i] = data:byte(i) -- Very simple "compression" (not real compression)
    end
    return string.char(table.unpack(compressed))
end

-- PNG Helper Functions
local function crc32(str)
    local crc = 0xFFFFFFFF
    for i = 1, #str do
        local byte = str:byte(i)
        crc = crc ~ byte
        for _ = 0, 7 do
            if (crc & 1) ~= 0 then
                crc = (crc >> 1) ~ 0xEDB88320
            else
                crc = crc >> 1
            end
        end
    end
    return crc ~ 0xFFFFFFFF
end

local function createChunk(type, data)
    local length = #data
    local crc = crc32(type .. data)
    return string.pack(">I4", length) .. type .. data .. string.pack(">I4", crc)
end

local function createIHDRChunk()
    local widthBytes = string.pack(">I4", width)
    local heightBytes = string.pack(">I4", height)
    local ihdrData = widthBytes .. heightBytes .. string.char(8, 6, 0, 0, 0) -- 8-bit depth, RGB
    return createChunk("IHDR", ihdrData)
end

local function createIDATChunk(pixelData)
    -- Compress pixel data
    local compressedData = basicCompress(pixelData)
    return createChunk("IDAT", compressedData)
end

local function createIENDChunk()
    return createChunk("IEND", "")
end

local function writePng(path, pixelData)
    local file = io.open(path, "wb")
    if not file then
        error("Could not open file: " .. path)
    end

    file:write("\137PNG\r\n\26\n")
    file:write(createIHDRChunk())
    file:write(createIDATChunk(pixelData))
    file:write(createIENDChunk())
    file:close()
end

-- Image Generation
local function generatePixelData(items, binding, isCollider)
    local pixelData = {}

    for y = 0, height - 1 do
        for x = 0, width - 1 do
            local index = (y * width + x) * 4
            local color = {0, 0, 0, 255} -- Default color

            if isCollider then
                -- Get color from binding (placeholder logic)
                color = {0, 0, 255, 255}
            else
                -- Get color from binding (placeholder logic)
                color = {255, 0, 0, 255}
            end

            pixelData[index + 1] = color[1]
            pixelData[index + 2] = color[2]
            pixelData[index + 3] = color[3]
            pixelData[index + 4] = color[4]
        end
    end

    return pixelData
end
