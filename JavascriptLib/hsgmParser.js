const fs = require('fs');
const zlib = require('zlib'); // Node.js built-in zlib module for compression

// Constants
const tileSize = 16;
const width = 256;
const height = 256;

// Function to parse the map file
function parseMapFile(path) {
    const data = fs.readFileSync(path, 'utf-8');
    const textures = {};
    const colliders = {};
    const triggers = {};
    let section = null;

    data.split('\n').forEach(line => {
        line = line.trim();

        if (line === '[TEXTURES]') {
            section = 'textures';
        } else if (line === '[COLLIDERS]') {
            section = 'colliders';
        } else if (line === '[TRIGGERS]') {
            section = 'triggers';
        } else if (section === 'textures') {
            const [key, value] = line.split('=').map(part => part.trim());
            if (key && value) {
                const [x, y] = value.slice(1, -1).split(',').map(Number);
                textures[key] = [x, y];
            }
        } else if (section === 'colliders') {
            const [key, value] = line.split('=').map(part => part.trim());
            if (key && value) {
                const [x, y] = value.slice(1, -1).split(',').map(Number);
                colliders[key] = [x, y];
            }
        } else if (section === 'triggers') {
            const [key, value] = line.split('=').map(part => part.trim());
            if (key && value) {
                const [x, y] = value.slice(1, -1).split(',').map(Number);
                triggers[key] = [x, y];
            }
        }
    });

    return { textures, colliders, triggers };
}

// Function to calculate CRC32
function crc32(str) {
    const table = Array.from({ length: 256 }, (_, i) => {
        let crc = i;
        for (let j = 8; j > 0; j--) {
            if (crc & 1) {
                crc = (crc >>> 1) ^ 0xEDB88320;
            } else {
                crc >>>= 1;
            }
        }
        return crc;
    });

    let crc = 0xFFFFFFFF;
    for (let i = 0; i < str.length; i++) {
        const byte = str.charCodeAt(i) & 0xFF;
        crc = (crc >>> 8) ^ table[(crc ^ byte) & 0xFF];
    }

    return (crc ^ 0xFFFFFFFF) >>> 0;
}

// Function to create PNG chunks
function createChunk(type, data) {
    const length = data.length;
    const crc = crc32(type + data);
    const lengthBuf = Buffer.alloc(4);
    lengthBuf.writeUInt32BE(length, 0);
    const typeBuf = Buffer.from(type);
    const crcBuf = Buffer.alloc(4);
    crcBuf.writeUInt32BE(crc, 0);
    return Buffer.concat([lengthBuf, typeBuf, Buffer.from(data), crcBuf]);
}

// Function to create IHDR chunk
function createIHDRChunk() {
    const widthBuf = Buffer.alloc(4);
    widthBuf.writeUInt32BE(width, 0);
    const heightBuf = Buffer.alloc(4);
    heightBuf.writeUInt32BE(height, 0);
    const ihdrData = Buffer.concat([widthBuf, heightBuf, Buffer.from([8, 6, 0, 0, 0])]);
    return createChunk('IHDR', ihdrData.toString('binary'));
}

// Function to create IDAT chunk
function createIDATChunk(pixelData) {
    const compressedData = zlib.deflateSync(pixelData); // Using Node.js zlib for actual compression
    return createChunk('IDAT', compressedData.toString('binary'));
}

// Function to create IEND chunk
function createIENDChunk() {
    return createChunk('IEND', '');
}

// Function to write PNG file
function writePng(path, pixelData) {
    const file = fs.createWriteStream(path);
    file.write(Buffer.from([137, 80, 78, 71, 13, 10, 26, 10])); // PNG signature
    file.write(createIHDRChunk());
    file.write(createIDATChunk(pixelData));
    file.write(createIENDChunk());
    file.end();
}

// Function to generate pixel data
function generatePixelData(items, binding, isCollider) {
    const pixelData = Buffer.alloc(width * height * 4);

    for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
            const index = (y * width + x) * 4;
            const color = isCollider ? [0, 0, 255, 255] : [255, 0, 0, 255];
            pixelData[index] = color[0]; // R
            pixelData[index + 1] = color[1]; // G
            pixelData[index + 2] = color[2]; // B
            pixelData[index + 3] = color[3]; // A
        }
    }

    return pixelData;
}

