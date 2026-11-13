import numpy as np
from PIL import Image
from pydelatin import Delatin
import trimesh

ERROR = 1.0
HEIGHT = 37.5
print(f"Starting mesh generation with error = {ERROR}")
# 1. Load your 8-bit grayscale heightmap (values 0 to 255)
img = Image.open("../assets/map/heightmap.bmp").convert("L")
width, height = img.size
heightmap = np.asarray(img, dtype=np.float32)
print("Heightmap image loaded")
# Optional: Smooth out 8-bit "stair-stepping" bands if terrain looks blocky
# from scipy.ndimage import gaussian_filter
# heightmap = gaussian_filter(heightmap, sigma=1.0)

# 2. Generate the mesh with a polygon budget for smooth game performance
# z_scale turns 0..255 into world height (e.g., 0.2 -> max height of 51 units)
print("Mesh generation started")
tin = Delatin(
    heightmap,
    max_error=ERROR,
    # max_triangles=20000,  # Keeps polycount game-friendly!
    z_scale=(1.0 / 255.0) * HEIGHT,
)

vertices = tin.vertices.copy()
triangles = tin.triangles.copy()

# 3. Generate UV coordinates (0.0 to 1.0) BEFORE changing vertex positions
# U = X / width, V = 1.0 - (Y / height) so textures align right-side up
uvs = np.zeros((len(vertices), 2), dtype=np.float32)
uvs[:, 0] = vertices[:, 0] / (width - 1)
uvs[:, 1] = 1.0 - (vertices[:, 1] / (height - 1))

# 4. Convert from Z-up (image space) to Y-up (standard game engine space)
# Swap Y and Z, then flip Z so the map isn't mirrored
vertices = vertices[:, [0, 2, 1]]
vertices[:, 2] = -vertices[:, 2]

# Flip triangle winding order so normals point UP toward the sky
triangles = triangles[:, [0, 2, 1]]

# 5. Build the mesh with UVs and export to .glb
mesh = trimesh.Trimesh(
    vertices=vertices,
    faces=triangles,
    visual=trimesh.visual.TextureVisuals(uv=uvs),
)
print("Generating normals")
_ = mesh.vertex_normals
print("Saving to file")
file_name = f"game_terrain_error_{ERROR}.glb"
mesh.export(file_name)
print(f"Purr~ Saved {file_name} with {len(triangles):_} triangles!")
