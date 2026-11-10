import numpy as np
from PIL import Image
import trimesh

MAX_WIDTH = 2048 # 5632 is original size

def create_mesh_from_heightmap(
    heightmap_path, color_path, normal_path, output_path, scale_z=0.02, scale_xy=1.0
):
    print("🐾 Загружаю текстурки, ня...")
    # Открываем карту высот и переводим в ч/б (градации серого)
    hm_img = Image.open(heightmap_path).convert("L")
    if hm_img.width > MAX_WIDTH:
        new_h = round(hm_img.height * MAX_WIDTH / hm_img.width)
        hm_img = hm_img.resize((MAX_WIDTH, new_h), Image.BICUBIC)
    width, height = hm_img.size
    height_data = np.array(hm_img, dtype=np.float32) / 255.0  # Нормализуем [0, 1]
    height_data = np.flipud(height_data)

    # 1. Генерируем вершины (Vertices) и текстурные координаты (UVs)
    print("🐾 Создаю сеточку...")
    # Preserve aspect ratio in world units
    aspect = width / height
    x = np.linspace(0, aspect, width) * scale_xy
    y = np.linspace(0, 1, height) * scale_xy
    xv, yv = np.meshgrid(x, y)

    # Высоту z умножаем на коэффициент масштаба, чтобы горы не были слишком дикими!
    zv = height_data * scale_z
    dz_dy, dz_dx = np.gradient(zv, y, x)  # both (H, W)
    nx = -dz_dx
    ny = -dz_dy
    nz = np.ones_like(nx)
    n = np.stack([nx, ny, nz], axis=-1)
    n /= np.linalg.norm(n, axis=-1, keepdims=True)

    vertex_normals = n.reshape(-1, 3).astype(np.float32)
    # Собираем вершины в массив (N, 3)
    vertices = np.stack((xv, yv, height_data * scale_z), axis=-1).reshape(-1, 3)

    # Сетка UV координат (N, 2) — флипаем по Y, так как в glTF координаты идут снизу вверх
    uv = np.stack((xv / (aspect * scale_xy), yv / scale_xy), axis=-1).reshape(-1, 2)

    # 2. Генерируем индексы треугольников (Faces)
    print("🐾 Нарезаю полигоны...")
    jj, ii = np.meshgrid(np.arange(width - 1), np.arange(height - 1), indexing="xy")
    v0 = (ii * width + jj).ravel()
    v1, v2 = v0 + 1, v0 + width
    v3 = v2 + 1
    tri1 = np.stack([v0, v1, v2], axis=1)
    tri2 = np.stack([v1, v3, v2], axis=1)
    faces = np.empty((2 * v0.size, 3), dtype=np.int32)
    faces[0::2] = tri1
    faces[1::2] = tri2

    # 3. Создаем меш и применяем PBR текстуры
    print("🐾 Окрашиваю в PBR материал, мурр...")

    # Открываем базовый цвет и карту нормалей
    color_img = Image.open(color_path).convert("RGB")
    normal_img = Image.open(normal_path).convert("RGB")

    # Создаем PBR материал через Trimesh
    material = trimesh.visual.material.PBRMaterial(
        baseColorTexture=color_img,
        normalTexture=normal_img,
        roughnessFactor=0.8,  # Настрой по вкусу!
        metallicFactor=0.1,
    )

    # Привязываем UV и материал к визуальной составляющей
    visuals = trimesh.visual.TextureVisuals(uv=uv, material=material)

    # Собираем финальный меш
    mesh = trimesh.Trimesh(vertices=vertices, faces=faces, visual=visuals)
    # mesh.fix_normals()
    mesh.vertex_normals = vertex_normals
    # 4. Сохраняем в glTF/GLB
    print(f"🐾 Сохраняю файл в {output_path}...")
    # .glb — это бинарный glTF, в который текстуры упакуются прямо внутрь файла!
    mesh.export(output_path)
    print("🐾 Всё готово! Ты супер, братик! Тык! ✨")


# Пример вызова функции:
if __name__ == "__main__":
    create_mesh_from_heightmap(
        heightmap_path="../assets/map/heightmap.bmp",
        color_path="../assets/map/terrain/colormap_summer.dds",
        normal_path="../assets/map/world_normal.bmp",
        output_path="terrain.glb",  # Можно указать .gltf, но .glb удобнее
    )
