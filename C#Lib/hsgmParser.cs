using System;
using System.Collections.Generic;
using System.IO;
using System.Text;

namespace MapUtilities
{
    public static class MapParserAndPngEncoder
    {
        private const int TileSize = 16;
        private static readonly byte[] PngSignature = { 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A };

        public class Binding
        {
            private readonly Dictionary<string, string> textureBindings;
            private readonly Dictionary<string, string> colliderBindings;
            private readonly Dictionary<string, string> triggerBindings;

            public Binding(Dictionary<string, string> textureBindings,
                           Dictionary<string, string> colliderBindings,
                           Dictionary<string, string> triggerBindings)
            {
                this.textureBindings = textureBindings;
                this.colliderBindings = colliderBindings;
                this.triggerBindings = triggerBindings;
            }

            public string GetTexturePath(string textureName)
            {
                return textureBindings.TryGetValue(textureName, out var path) ? path : null;
            }

            public string GetColliderColor(string colliderType)
            {
                return colliderBindings.TryGetValue(colliderType, out var color) ? color : null;
            }

            public string GetTriggerColor(string triggerType)
            {
                return triggerBindings.TryGetValue(triggerType, out var color) ? color : null;
            }
        }

        public static void ParseMapFile(string filePath, out Binding binding,
                                        out Dictionary<string, Tuple<int, int>> textures,
                                        out Dictionary<string, Tuple<int, int>> colliders,
                                        out Dictionary<string, Tuple<int, int>> triggers)
        {
            textures = new Dictionary<string, Tuple<int, int>>();
            colliders = new Dictionary<string, Tuple<int, int>>();
            triggers = new Dictionary<string, Tuple<int, int>>();

            var textureBindings = new Dictionary<string, string>();
            var colliderBindings = new Dictionary<string, string>();
            var triggerBindings = new Dictionary<string, string>();

            foreach (var line in File.ReadLines(filePath))
            {
                var parts = line.Split('=');
                if (parts.Length != 2) continue;

                var key = parts[0].Trim();
                var value = parts[1].Trim().Trim('[', ']').Split(',');

                if (key.StartsWith("Texture"))
                    textures[key] = ParseTuple(value);
                else if (key.StartsWith("Collider"))
                    colliders[key] = ParseTuple(value);
                else if (key.StartsWith("Trigger"))
                    triggers[key] = ParseTuple(value);
                else if (key.StartsWith("Binding"))
                {
                    var bindingParts = key.Split(':');
                    if (bindingParts.Length == 2)
                    {
                        var type = bindingParts[0].Trim();
                        var name = bindingParts[1].Trim();
                        if (type == "Texture")
                            textureBindings[name] = value[0].Trim();
                        else if (type == "Collider")
                            colliderBindings[name] = value[0].Trim();
                        else if (type == "Trigger")
                            triggerBindings[name] = value[0].Trim();
                    }
                }
            }

            binding = new Binding(textureBindings, colliderBindings, triggerBindings);
        }

        private static Tuple<int, int> ParseTuple(string[] values)
        {
            if (values.Length != 2) throw new FormatException("Invalid tuple format.");
            return new Tuple<int, int>(int.Parse(values[0]), int.Parse(values[1]));
        }

        public static void SaveAsPng(int width, int height, byte[] pixelData, string filePath)
        {
            using (var stream = new FileStream(filePath, FileMode.Create))
            using (var writer = new BinaryWriter(stream))
            {
                WritePngSignature(writer);
                WriteIHDRChunk(writer, width, height);
                WriteIDATChunk(writer, pixelData);
                WriteIENDChunk(writer);
            }
        }

        private static void WritePngSignature(BinaryWriter writer)
        {
            writer.Write(PngSignature);
        }

        private static void WriteIHDRChunk(BinaryWriter writer, int width, int height)
        {
            writer.Write(13); // Length of IHDR chunk data
            writer.Write(Encoding.ASCII.GetBytes("IHDR"));
            writer.Write(width);
            writer.Write(height);
            writer.Write((byte)8); // Bit depth
            writer.Write((byte)6); // Color type: RGBA
            writer.Write((byte)0); // Compression method
            writer.Write((byte)0); // Filter method
            writer.Write((byte)0); // Interlace method
            WriteChunkCRC(writer, "IHDR");
        }

        private static void WriteIDATChunk(BinaryWriter writer, byte[] pixelData)
        {
            using (var idatStream = new MemoryStream())
            using (var idatWriter = new BinaryWriter(idatStream))
            {
                WriteImageData(idatWriter, pixelData);
                idatStream.Position = 0;

                // Compress the IDAT chunk data using DEFLATE
                using (var compressedStream = new MemoryStream())
                using (var deflateStream = new System.IO.Compression.DeflateStream(compressedStream, System.IO.Compression.CompressionMode.Compress, true))
                {
                    idatStream.CopyTo(deflateStream);
                }

                var idatData = compressedStream.ToArray();
                writer.Write(idatData.Length); // Length of IDAT chunk data
                writer.Write(Encoding.ASCII.GetBytes("IDAT"));
                writer.Write(idatData);
                WriteChunkCRC(writer, "IDAT");
            }
        }

        private static void WriteImageData(BinaryWriter writer, byte[] pixelData)
        {
            writer.Write((byte)0); // No filter
            writer.Write(pixelData);
        }

        private static void WriteIENDChunk(BinaryWriter writer)
        {
            writer.Write(0); // Length of IEND chunk data
            writer.Write(Encoding.ASCII.GetBytes("IEND"));
            WriteChunkCRC(writer, "IEND");
        }

        private static void WriteChunkCRC(BinaryWriter writer, string chunkType)
        {
            uint crc = Crc32(Encoding.ASCII.GetBytes(chunkType));
            writer.Write(crc);
        }

        private static uint Crc32(byte[] data)
        {
            const uint Polynomial = 0xedb88320;
            uint[] table = new uint[256];
            for (uint i = 0; i < 256; ++i)
            {
                uint crc = i;
                for (uint j = 8; j > 0; --j)
                {
                    if ((crc & 1) == 1)
                    {
                        crc = (crc >> 1) ^ Polynomial;
                    }
                    else
                    {
                        crc >>= 1;
                    }
                }
                table[i] = crc;
            }

            uint crcValue = 0xffffffff;
            foreach (byte b in data)
            {
                byte tableIndex = (byte)((crcValue & 0xff) ^ b);
                crcValue = (crcValue >> 8) ^ table[tableIndex];
            }

            return crcValue ^ 0xffffffff;
        }

        public static void Main(string[] args)
        {
            if (args.Length != 2)
            {
                Console.WriteLine("Usage: MapParserAndPngEncoder <mapFile> <outputPng>");
                return;
            }

            string mapFile = args[0];
            string outputPng = args[1];

            try
            {
                // Parse the map file
                ParseMapFile(mapFile, out var binding, out var textures, out var colliders, out var triggers);

                // Generate a basic image data array
                int width = 256;
                int height = 256;
                byte[] pixelData = new byte[width * height * 4]; // RGBA

                // Fill image data with a simple pattern for demonstration
                for (int y = 0; y < height; y++)
                {
                    for (int x = 0; x < width; x++)
                    {
                        int index = (y * width + x) * 4;
                        pixelData[index] = (byte)(x % 256);       // Red
                        pixelData[index + 1] = (byte)(y % 256);   // Green
                        pixelData[index + 2] = 0;                  // Blue
                        pixelData[index + 3] = 255;                // Alpha
                    }
                }

                // Save as PNG
                SaveAsPng(width, height, pixelData, outputPng);

                Console.WriteLine($"PNG file generated: {outputPng}");
            }
            catch (Exception ex)
            {
                Console.WriteLine($"Error: {ex.Message}");
            }
        }
    }
}
