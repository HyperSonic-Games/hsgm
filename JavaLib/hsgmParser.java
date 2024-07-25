package hsgmParser
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.OutputStream;
import java.util.Map;
import java.util.zip.Deflater;

public class hsgmParser {

    private static final int TILE_SIZE = 16;
    private static final int WIDTH = 256;
    private static final int HEIGHT = 256;

    public static byte[] generatePixelData(Map<String, int[]> items) {
        byte[] pixelData = new byte[WIDTH * HEIGHT * 4]; // 4 bytes per pixel (RGBA)

        for (Map.Entry<String, int[]> entry : items.entrySet()) {
            String name = entry.getKey();
            int[] position = entry.getValue();
            int x = position[0];
            int y = position[1];

            if (x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT) {
                int index = (y * WIDTH + x) * 4;
                byte[] color = name.equals("ColliderType") ? new byte[]{0, 0, (byte)255, (byte)255} : new byte[]{(byte)255, 0, 0, (byte)255};
                System.arraycopy(color, 0, pixelData, index, 4);
            }
        }

        return pixelData;
    }

    public static void writePng(String path, byte[] pixelData) throws IOException {
        try (OutputStream os = new FileOutputStream(path)) {
            os.write(new byte[]{(byte)137, 80, 78, 71, 13, 10, 26, 10}); // PNG signature
            os.write(createIHDRChunk());
            os.write(createIDATChunk(pixelData));
            os.write(createIENDChunk());
        }
    }

    private static byte[] createIHDRChunk() {
        byte[] widthBytes = intToBytes(WIDTH);
        byte[] heightBytes = intToBytes(HEIGHT);
        byte[] ihdrData = new byte[13];
        System.arraycopy(widthBytes, 0, ihdrData, 0, 4);
        System.arraycopy(heightBytes, 0, ihdrData, 4, 4);
        ihdrData[8] = 8; // bit depth
        ihdrData[9] = 6; // color type (RGBA)
        ihdrData[10] = 0; // compression method
        ihdrData[11] = 0; // filter method
        ihdrData[12] = 0; // interlace method
        return createChunk("IHDR", ihdrData);
    }

    private static byte[] createIDATChunk(byte[] pixelData) {
        byte[] compressedData = compressData(pixelData);
        return createChunk("IDAT", compressedData);
    }

    private static byte[] createIENDChunk() {
        return createChunk("IEND", new byte[0]);
    }

    private static byte[] createChunk(String type, byte[] data) {
        byte[] typeBytes = type.getBytes();
        byte[] lengthBytes = intToBytes(data.length);
        byte[] crcBytes = intToBytes(crc32(type.getBytes(), data));

        byte[] chunk = new byte[lengthBytes.length + typeBytes.length + data.length + crcBytes.length];
        int offset = 0;

        System.arraycopy(lengthBytes, 0, chunk, offset, lengthBytes.length);
        offset += lengthBytes.length;
        System.arraycopy(typeBytes, 0, chunk, offset, typeBytes.length);
        offset += typeBytes.length;
        System.arraycopy(data, 0, chunk, offset, data.length);
        offset += data.length;
        System.arraycopy(crcBytes, 0, chunk, offset, crcBytes.length);

        return chunk;
    }

    private static byte[] compressData(byte[] data) {
        Deflater deflater = new Deflater();
        deflater.setInput(data);
        deflater.finish();

        byte[] buffer = new byte[1024];
        int compressedDataLength = deflater.deflate(buffer);
        deflater.end();

        byte[] compressedData = new byte[compressedDataLength];
        System.arraycopy(buffer, 0, compressedData, 0, compressedDataLength);

        return compressedData;
    }

    private static byte[] intToBytes(int value) {
        return new byte[]{
            (byte) ((value >> 24) & 0xFF),
            (byte) ((value >> 16) & 0xFF),
            (byte) ((value >> 8) & 0xFF),
            (byte) (value & 0xFF)
        };
    }

    private static int crc32(byte[]... dataArrays) {
        int crc = 0xFFFFFFFF;
        for (byte[] data : dataArrays) {
            for (byte b : data) {
                crc ^= (b & 0xFF);
                for (int j = 0; j < 8; j++) {
                    if ((crc & 1) != 0) {
                        crc = (crc >>> 1) ^ 0xEDB88320;
                    } else {
                        crc >>>= 1;
                    }
                }
            }
        }
        return crc ^ 0xFFFFFFFF;
    }
}
