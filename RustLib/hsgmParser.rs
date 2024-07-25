use std::fs::File;
use std::io::{self, Write, BufRead};
use std::collections::HashMap;

const TILE_SIZE: u32 = 16;
const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

struct Binding {
    texture_paths: HashMap<String, String>,
    collider_colors: HashMap<String, [u8; 4]>,
    trigger_colors: HashMap<String, [u8; 4]>,
}

impl Binding {
    fn get_texture_path(&self, texture_name: &str) -> Option<&String> {
        self.texture_paths.get(texture_name)
    }

    fn get_collider_color(&self, collider_name: &str) -> Option<&[u8; 4]> {
        self.collider_colors.get(collider_name)
    }

    fn get_trigger_color(&self, trigger_name: &str) -> Option<&[u8; 4]> {
        self.trigger_colors.get(trigger_name)
    }
}

fn write_png(file_path: &str, width: u32, height: u32, pixel_data: &[u8]) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    // PNG file signature
    file.write_all(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A])?;

    // IHDR chunk
    write_chunk(&mut file, "IHDR", |writer| {
        writer.write_all(&width.to_be_bytes())?;
        writer.write_all(&height.to_be_bytes())?;
        writer.write_all(&[8, 6, 0, 0, 0])?; // 8-bit depth, RGBA, no compression, no filter, no interlace
        Ok(())
    })?;

    // IDAT chunk
    let idat_data = generate_idat_data(pixel_data);
    write_chunk(&mut file, "IDAT", |writer| writer.write_all(&idat_data))?;

    // IEND chunk
    write_chunk(&mut file, "IEND", |writer| Ok(()))?;

    Ok(())
}

fn write_chunk<F>(writer: &mut File, chunk_type: &str, data_fn: F) -> io::Result<()>
where
    F: FnOnce(&mut Vec<u8>) -> io::Result<()>,
{
    let mut data = Vec::new();
    data_fn(&mut data)?;
    let length = data.len() as u32;
    writer.write_all(&length.to_be_bytes())?;
    writer.write_all(chunk_type.as_bytes())?;
    writer.write_all(&data)?;

    // Compute CRC for the chunk type
    let crc = crc32(chunk_type.as_bytes());
    writer.write_all(&crc.to_be_bytes())?;
    Ok(())
}

fn crc32(data: &[u8]) -> u32 {
    const POLY: u32 = 0xedb88320;
    let mut crc = 0xffffffff;
    for &byte in data {
        let mut temp = byte as u32 ^ crc;
        for _ in 0..8 {
            if temp & 1 != 0 {
                temp = (temp >> 1) ^ POLY;
            } else {
                temp >>= 1;
            }
        }
        crc = temp ^ (crc >> 8);
    }
    !crc
}

fn generate_idat_data(pixel_data: &[u8]) -> Vec<u8> {
    let mut data = Vec::new();
    data.push(0x00); // No filter
    for y in 0..HEIGHT {
        let offset = (y * WIDTH * 4) as usize;
        let row = &pixel_data[offset..offset + (WIDTH * 4) as usize];
        data.extend_from_slice(row);
    }
    
    let mut compressed_data = Vec::new();
    compress_deflate(&data, &mut compressed_data);
    compressed_data
}

fn compress_deflate(input: &[u8], output: &mut Vec<u8>) {
    // Minimal compression placeholder; a real implementation would use DEFLATE.
    output.extend_from_slice(input);
}

fn parse_map_file(file_path: &str) -> io::Result<(Vec<u8>, Vec<u8>, Vec<u8>, Binding)> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut textures = Vec::new();
    let mut colliders = Vec::new();
    let mut triggers = Vec::new();
    let mut width = 0;
    let mut height = 0;

    let mut current_section = "";
    let mut binding = Binding {
        texture_paths: HashMap::new(),
        collider_colors: HashMap::new(),
        trigger_colors: HashMap::new(),
    };

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("[TEXTURES]") {
            current_section = "textures";
            continue;
        } else if line.starts_with("[COLLIDERS]") {
            current_section = "colliders";
            continue;
        } else if line.starts_with("[TRIGGERS]") {
            current_section = "triggers";
            continue;
        } else if line.starts_with("[METADATA]") {
            current_section = "metadata";
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().strip_prefix('[').and_then(|v| v.strip_suffix(']')).unwrap_or("").trim();
            
            match current_section {
                "textures" => {
                    if let Some((name, path)) = key.split_once('=') {
                        binding.texture_paths.insert(name.trim().to_string(), path.trim().to_string());
                    }
                }
                "colliders" => {
                    if let Some((name, color)) = key.split_once('=') {
                        let parts: Vec<&str> = color.split(',').collect();
                        if parts.len() == 4 {
                            let color = [
                                parts[0].parse::<u8>().unwrap_or(0),
                                parts[1].parse::<u8>().unwrap_or(0),
                                parts[2].parse::<u8>().unwrap_or(0),
                                parts[3].parse::<u8>().unwrap_or(255),
                            ];
                            binding.collider_colors.insert(name.trim().to_string(), color);
                        }
                    }
                }
                "triggers" => {
                    if let Some((name, color)) = key.split_once('=') {
                        let parts: Vec<&str> = color.split(',').collect();
                        if parts.len() == 4 {
                            let color = [
                                parts[0].parse::<u8>().unwrap_or(0),
                                parts[1].parse::<u8>().unwrap_or(0),
                                parts[2].parse::<u8>().unwrap_or(0),
                                parts[3].parse::<u8>().unwrap_or(255),
                            ];
                            binding.trigger_colors.insert(name.trim().to_string(), color);
                        }
                    }
                }
                "metadata" => {
                    if line.starts_with("MAP_SIZE") {
                        let parts: Vec<&str> = line.split('=').nth(1).unwrap_or("").trim().strip_prefix('[').and_then(|v| v.strip_suffix(']')).unwrap_or("").split(',').collect();
                        if parts.len() == 2 {
                            width = parts[0].parse::<u32>().unwrap_or(256);
                            height = parts[1].parse::<u32>().unwrap_or(256);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let texture_data = generate_pixel_data(&textures, width, height, [0xFF, 0x00, 0x00, 0xFF]); // Red for textures
    let collider_data = generate_pixel_data_from_bindings(&colliders, width, height, &binding, true); // Using Binding for colors
    let trigger_data = generate_pixel_data_from_bindings(&triggers, width, height, &binding, false); // Using Binding for colors

    Ok((texture_data, collider_data, trigger_data, binding))
}

fn generate_pixel_data(items: &[(String, u32, u32)], width: u32, height: u32, color: [u8; 4]) -> Vec<u8> {
    let mut pixel_data = vec![0; (width * height * 4) as usize];

    for (_, x, y) in items {
        let index = ((y * width + x) * 4) as usize;
        if index + 3 < pixel_data.len() {
            pixel_data[index] = color[0]; // Red
            pixel_data[index + 1] = color[1]; // Green
            pixel_data[index + 2] = color[2]; // Blue
            pixel_data[index + 3] = color[3]; // Alpha
        }
    }

    pixel_data
}

fn generate_pixel_data_from_bindings(items: &[(String, u32, u32)], width: u32, height: u32, binding: &Binding, is_collider: bool) -> Vec<u8> {
    let mut pixel_data = vec![0; (width * height * 4) as usize];

    for (name, x, y) in items {
        let color = if is_collider {
            binding.get_collider_color(name).unwrap_or(&[0, 0, 0, 255])
        } else {
            binding.get_trigger_color(name).unwrap_or(&[0, 0, 0, 255])
        };

        let index = ((y * width + x) * 4) as usize;
        if index + 3 < pixel_data.len() {
            pixel_data[index] = color[0]; // Red
            pixel_data[index + 1] = color[1]; // Green
            pixel_data[index + 2] = color[2]; // Blue
            pixel_data[index + 3] = color[3]; // Alpha
        }
    }

    pixel_data
}

fn main() -> io::Result<()> {
    let (texture_data, collider_data, trigger_data, binding) = parse_map_file("example.map")?;
    write_png("textures.png", WIDTH, HEIGHT, &texture_data)?;
    write_png("colliders.png", WIDTH, HEIGHT, &collider_data)?;
    write_png("triggers.png", WIDTH, HEIGHT, &trigger_data)?;

    Ok(())
}
