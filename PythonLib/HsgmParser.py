from PIL import Image

TileSize = 16

class Binding:
    def __init__(self, texture_bindings, collider_bindings, trigger_bindings):
        self.texture_bindings = texture_bindings
        self.collider_bindings = collider_bindings
        self.trigger_bindings = trigger_bindings

    def get_texture_path(self, texture_name):
        return self.texture_bindings.get(texture_name, "")

    def get_collider_color(self, collider_type):
        return self.collider_bindings.get(collider_type, "")

    def get_trigger_color(self, trigger_type):
        return self.trigger_bindings.get(trigger_type, "")

def parse_map_file(file_path):
    textures = {}
    colliders = {}
    triggers = {}

    texture_bindings = {}
    collider_bindings = {}
    trigger_bindings = {}

    with open(file_path) as file:
        for line in file:
            if '=' not in line:
                continue

            key, value = map(str.strip, line.split('=', 1))
            value = value.strip('[]').split(',')

            if key.startswith('Texture'):
                textures[key] = tuple(map(int, value))
            elif key.startswith('Collider'):
                colliders[key] = tuple(map(int, value))
            elif key.startswith('Trigger'):
                triggers[key] = tuple(map(int, value))
            elif key.startswith('Binding'):
                parts = key.split(':')
                if len(parts) == 2:
                    binding_type = parts[0].strip()
                    name = parts[1].strip()
                    if binding_type == 'Texture':
                        texture_bindings[name] = value[0].strip()
                    elif binding_type == 'Collider':
                        collider_bindings[name] = value[0].strip()
                    elif binding_type == 'Trigger':
                        trigger_bindings[name] = value[0].strip()

    binding = Binding(texture_bindings, collider_bindings, trigger_bindings)
    return binding, textures, colliders, triggers

def WritePng(file_path, width, height, pixel_data):
    image = Image.new('RGBA', (width, height))
    image.putdata(pixel_data)
    image.save(file_path)
