import numpy as np
from PIL import Image
import trimesh
import json
import struct


def export_glb_with_tangents(mesh, tangents, output_path):
    glb_bytes = mesh.export(file_type="glb")

    # Разбираем заголовок GLB и два чанка (JSON и BIN)
    magic, version, _ = struct.unpack("<III", glb_bytes[:12])
    json_len, json_type = struct.unpack("<II", glb_bytes[12:20])
    json_data = json.loads(glb_bytes[20 : 20 + json_len].decode("utf-8"))

    bin_offset = 20 + json_len
    bin_len, bin_type = struct.unpack("<II", glb_bytes[bin_offset : bin_offset + 8])
    bin_data = bytearray(glb_bytes[bin_offset + 8 : bin_offset + 8 + bin_len])

    # Выравниваем бинарный буфер по 4 байта и дописываем тангенты (N, 4) float32
    while len(bin_data) % 4 != 0:
        bin_data.append(0)
    tangent_byte_offset = len(bin_data)
    tangent_bytes = tangents.astype(np.float32).tobytes()
    bin_data.extend(tangent_bytes)
    while len(bin_data) % 4 != 0:
        bin_data.append(0)

    # Обновляем buffer, bufferViews и accessors в glTF JSON
    json_data["buffers"][0]["byteLength"] = len(bin_data)

    bv_idx = len(json_data["bufferViews"])
    json_data["bufferViews"].append(
        {
            "buffer": 0,
            "byteOffset": tangent_byte_offset,
            "byteLength": len(tangent_bytes),
            "target": 34962,  # ARRAY_BUFFER
        }
    )

    acc_idx = len(json_data["accessors"])
    json_data["accessors"].append(
        {
            "bufferView": bv_idx,
            "componentType": 5126,  # FLOAT
            "count": len(tangents),
            "type": "VEC4",
        }
    )

    # Привязываем TANGENT ко всем примитивам меша
    for m in json_data["meshes"]:
        for prim in m["primitives"]:
            prim["attributes"]["TANGENT"] = acc_idx

    # Собираем новый JSON-чанк с выравниванием пробелами до 4 байт
    new_json_bytes = json.dumps(json_data, separators=(",", ":")).encode("utf-8")
    while len(new_json_bytes) % 4 != 0:
        new_json_bytes += b" "

    total_len = 12 + 8 + len(new_json_bytes) + 8 + len(bin_data)
    with open(output_path, "wb") as f:
        f.write(struct.pack("<III", magic, version, total_len))
        f.write(struct.pack("<II", len(new_json_bytes), json_type))
        f.write(new_json_bytes)
        f.write(struct.pack("<II", len(bin_data), bin_type))
        f.write(bin_data)


ORIGINAL_WIDTH = 5632
ORIGINAL_HEIGHT = 2048
DOWNSAMPLE = 4
MAX_WIDTH = int(ORIGINAL_WIDTH / DOWNSAMPLE)


def create_mesh_from_heightmap(
    heightmap_path,
    color_path,
    normal_path,
    output_path,
    scale_z=45.0,
    # scale_xy=1.0,
    blur_radius=2,
):
    print("🐾 Загружаю текстурки, ня...")
    # Открываем карту высот и переводим в ч/б (градации серого)
    hm_img = Image.open(heightmap_path).convert("L")
    # --- МАГИЯ СГЛАЖИВАНИЯ, НЯ! ---
    # if blur_radius > 0:
    #     # Чем больше blur_radius, тем мягче и плавнее будут горы!
    #     hm_img = hm_img.filter(ImageFilter.GaussianBlur(radius=blur_radius))
    #     print(f"🐾 Карта высот успешно размыта с радиусом {blur_radius}!")
    # ------------------------------
    if hm_img.width > MAX_WIDTH:
        new_h = round(hm_img.height * MAX_WIDTH / hm_img.width)
        hm_img = hm_img.resize((MAX_WIDTH, new_h), Image.BICUBIC)
    width, height = hm_img.size
    height_data = (np.array(hm_img, dtype=np.float32) / 255.0) * scale_z
    # height_data = np.flipud(height_data)

    # 1. Генерируем вершины (Vertices) и текстурные координаты (UVs)
    print("🐾 Создаю сеточку...")
    # Preserve aspect ratio in world units
    # aspect = width / height
    # x = np.linspace(0, aspect, width) * scale_xy
    # y = np.linspace(0, 1, height) * scale_xy
    x = np.linspace(
        -ORIGINAL_WIDTH / 2.0, ORIGINAL_WIDTH / 2.0, width, dtype=np.float32
    )
    z = np.linspace(
        -ORIGINAL_HEIGHT / 2.0, ORIGINAL_HEIGHT / 2.0, height, dtype=np.float32
    )
    xv, zv = np.meshgrid(x, z, indexing="xy")

    # Высоту z умножаем на коэффициент масштаба, чтобы горы не были слишком дикими!
    # zv = height_data * scale_z
    # dz_dy, dz_dx = np.gradient(zv, y, x)  # both (H, W)
    # nx = -dz_dx
    # ny = -dz_dy
    # nz = np.ones_like(nx)
    # n = np.stack([nx, ny, nz], axis=-1)
    # n /= np.linalg.norm(n, axis=-1, keepdims=True)
    # vertex_normals = n.reshape(-1, 3).astype(np.float32)

    # Собираем вершины в массив (N, 3)
    vertices = np.stack((xv, height_data, zv), axis=-1).reshape(-1, 3)

    # 🐾 МАГИЯ ИСПРАВЛЕНИЯ UV, МУРР!
    # Генерируем чистые UV координаты от 0 до 1, независимые от scale_xy.
    # Так как в glTF UV (0,0) - левый нижний угол, а у нас массив уже flipud,
    # meshgrid со стандартным шагом от 0 до 1 идеально ляжет на картинку!
    uv_x = np.linspace(0, 1, width)
    uv_y = np.linspace(
        1, 0, height
    )  # in case of upside down change to uv_y = np.linspace(0, 1, height)
    uv_xv, uv_yv = np.meshgrid(uv_x, uv_y, indexing="xy")
    uv = np.stack((uv_xv, uv_yv), axis=-1).reshape(-1, 2)

    # 2. Генерируем индексы треугольников (Faces)
    print("🐾 Нарезаю полигоны...")
    jj, ii = np.meshgrid(np.arange(width - 1), np.arange(height - 1), indexing="xy")
    v0 = (ii * width + jj).ravel()
    v1, v2 = v0 + 1, v0 + width
    v3 = v2 + 1

    # Swapped winding order (v0, v2, v1) to keep front-faces pointing up
    tri1 = np.stack([v0, v2, v1], axis=1)
    tri2 = np.stack([v1, v2, v3], axis=1)
    faces = np.empty((2 * v0.size, 3), dtype=np.int32)
    faces[0::2] = tri1
    faces[1::2] = tri2

    # 3. Создаем меш и применяем PBR текстуры
    print("🐾 Окрашиваю в PBR материал, мурр...")

    # Открываем базовый цвет и карту нормалей
    # color_img = Image.open(color_path).convert("RGB")
    # normal_img = Image.open(normal_path).convert("RGB")

    # Создаем PBR материал через Trimesh
    material = trimesh.visual.material.PBRMaterial(
        # baseColorTexture=color_img,
        # normalTexture=normal_img,
        roughnessFactor=0.8,  # Настрой по вкусу!
        metallicFactor=0.0,
    )

    # Привязываем UV и материал к визуальной составляющей
    visuals = trimesh.visual.TextureVisuals(uv=uv, material=material)

    # Собираем финальный меш
    mesh = trimesh.Trimesh(
        vertices=vertices, faces=faces, visual=visuals, process=False
    )
    # _ = mesh.vertex_normals  # forces trimesh to generate vertex normals
    # 1. Берем нормали вершин (N, 3)
    normals = np.asarray(mesh.vertex_normals, dtype=np.float32)

    # 2. Базовое направление тангента вдоль оси U (ось +X)
    tangent_dir = np.zeros_like(normals)
    tangent_dir[:, 0] = 1.0  # (1, 0, 0)

    # 3. Ортогонализация Грама — Шмидта: T = normalize(T - N * dot(N, T))
    dot_nt = np.sum(normals * tangent_dir, axis=1, keepdims=True)
    tangents_xyz = tangent_dir - normals * dot_nt
    norms = np.linalg.norm(tangents_xyz, axis=1, keepdims=True)
    tangents_xyz = np.divide(
        tangents_xyz, norms, out=np.zeros_like(tangents_xyz), where=norms > 1e-6
    )

    # 4. Добавляем 4-й компонент w (handedness = 1.0 или -1.0; для Bevy/glTF с uv_y от 1 до 0 обычно 1.0)
    w = np.ones((tangents_xyz.shape[0], 1), dtype=np.float32)
    tangents = np.hstack([tangents_xyz, w]).astype(np.float32)

    # 5. Записываем в vertex_attributes для экспорта в glTF/GLB
    mesh.vertex_attributes["tangent"] = tangents
    # mesh.fix_normals()
    # mesh.vertex_normals = vertex_normals
    # 4. Сохраняем в glTF/GLB
    print(f"🐾 Сохраняю файл в {output_path}...")
    # .glb — это бинарный glTF, в который текстуры упакуются прямо внутрь файла!
    # mesh.export(output_path)
    export_glb_with_tangents(mesh, tangents, output_path)
    print("🐾 Всё готово! Ты супер, братик! Тык! ✨")


# Пример вызова функции:
if __name__ == "__main__":
    create_mesh_from_heightmap(
        heightmap_path="../assets/map/heightmap.bmp",
        color_path="../assets/map/terrain/colormap_summer.dds",
        normal_path="../assets/map/world_normal.bmp",
        output_path="../assets/map/terrain/terrain.glb",  # Можно указать .gltf, но .glb удобнее
        blur_radius=0,  # Радиус сглаживания. Попробуй 2, 3 или 5, ня!
    )
