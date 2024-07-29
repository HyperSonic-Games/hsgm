Code Documentation
==================

Binding Class
-------------

The `Binding` class encapsulates the associations between texture, collider, and trigger names with their respective attributes.

### Attributes

*   **`texture_bindings`** (`dict`): A dictionary mapping texture names to file paths.
*   **`collider_bindings`** (`dict`): A dictionary mapping collider types to their associated color strings.
*   **`trigger_bindings`** (`dict`): A dictionary mapping trigger types to their associated color strings.

### Methods

#### `__init__(self, texture_bindings, collider_bindings, trigger_bindings)`

Initializes the `Binding` object with texture, collider, and trigger bindings.

**Parameters:**

*   `texture_bindings` (`dict`): A dictionary for texture name-to-path mappings.
*   `collider_bindings` (`dict`): A dictionary for collider type-to-color mappings.
*   `trigger_bindings` (`dict`): A dictionary for trigger type-to-color mappings.

#### `get_texture_path(self, texture_name)`

Returns the file path for a given texture name.

**Parameters:**

*   `texture_name` (`str`): The name of the texture.

**Returns:** `str`: The file path associated with the texture name. Returns an empty string if not found.

#### `get_collider_color(self, collider_type)`

Returns the color associated with a given collider type.

**Parameters:**

*   `collider_type` (`str`): The type of the collider.

**Returns:** `str`: The color associated with the collider type. Returns an empty string if not found.

#### `get_trigger_color(self, trigger_type)`

Returns the color associated with a given trigger type.

**Parameters:**

*   `trigger_type` (`str`): The type of the trigger.

**Returns:** `str`: The color associated with the trigger type. Returns an empty string if not found.

File Format
-----------

The file parsed by the `parse_map_file` function must follow a specific format, where each line defines a texture, collider, trigger, or binding. The format uses key-value pairs separated by an equals sign (`=`). Values can be tuples of integers (for colors or coordinates) or strings (for file paths).

### Syntax

*   **Texture Definition:** `Texture[Name] = [R, G, B, A]`
*   **Collider Definition:** `Collider[Type] = [R, G, B, A]`
*   **Trigger Definition:** `Trigger[Type] = [R, G, B, A]`
*   **Binding Definition:** `Binding: Type = Path`

### Example

    Texture[Grass] = [34, 139, 34, 255]
    Collider[Wall] = [0, 0, 0, 255]
    Trigger[SpawnPoint] = [255, 255, 0, 255]
    Binding: Texture:Grass = path/to/grass_texture.png
    Binding: Collider:Wall = path/to/wall_texture.png
    Binding: Trigger:SpawnPoint = path/to/spawn_point_texture.png
    

In this example, `Grass` is defined as a texture with an RGBA color of (34, 139, 34, 255), `Wall` is defined as a collider with a black color (0, 0, 0, 255), and `SpawnPoint` is defined as a trigger with a yellow color (255, 255, 0, 255). The bindings associate these elements with specific file paths.