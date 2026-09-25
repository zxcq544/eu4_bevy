from PIL import Image


def get_center_pos(bigger_size, smaller_size):
    return ((bigger_size - smaller_size) // 2, (bigger_size - smaller_size) // 2)


def build_flag_with_shield_on_cpu():
    # 1. Загрузка всех изображений (замените пути на свои)
    # Убеждаемся, что изображения с прозрачностью загружены в режиме RGBA
    flag = Image.open("../assets/gfx/flags/RUS.tga")  # 128x128
    shield = Image.open("../assets/gfx/interface/shield_frame.dds")  # 92x92
    mask = Image.open("../assets/gfx/interface/shield_mask.tga")  # 62x62
    glow = Image.open("../assets/gfx/interface/shield_thin_glow.dds")  # 79x96

    # Финальный размер холста оставляем 128x128
    canvas_size = (92, 92)
    final_img = Image.new("RGBA", canvas_size, (0, 0, 0, 0))

    # --- НОВОЕ: УМЕНЬШАЕМ ФЛАГ ---
    # Меняем 100 на любой нужный вам размер (например, 92, 110 и т.д.)
    new_flag_size = (60, 60)
    flag_resized = flag.resize(new_flag_size, Image.Resampling.LANCZOS)

    # --- ШАГ 1: Подготовка маскированного флага ---
    # Создаем прозрачную подложку под размер уменьшенного флага
    masked_flag = Image.new("RGBA", flag_resized.size, (0, 0, 0, 0))

    # Создаем маску под размер уменьшенного флага
    full_mask = Image.new("L", flag_resized.size, 0)
    mask_pos = get_center_pos(flag_resized.size[0], mask.size[0])
    mask_x, mask_y = mask_pos
    # Переносим маску щита в центр новой маски флага
    full_mask.paste(mask.split()[-1], (mask_x, mask_y - 2))

    # Вырезаем уменьшенный флаг по маске
    masked_flag = Image.composite(flag_resized.convert("RGBA"), masked_flag, full_mask)

    # Накладываем маскированный флаг строго по центру финального холста 128x128
    flag_pos = get_center_pos(canvas_size[0], flag_resized.size[0])
    flag_x, flag_y = flag_pos
    final_img.alpha_composite(masked_flag, (flag_x, flag_y + 2))

    # --- ШАГ 2: Наложение свечения (Glow) ---
    glow_pos = get_center_pos(canvas_size[0], glow.size[0])
    # Свечение может быть не квадратным (79x96), поэтому передаем кортежи размеров целиком:
    glow_pos_exact = (
        (canvas_size[0] - glow.size[0]) // 2 + 1,
        (canvas_size[1] - glow.size[1]) // 2 - 3,
    )
    final_img.alpha_composite(glow, glow_pos_exact)

    # --- ШАГ 3: Наложение рамки щита (Shield Frame) ---
    shield_pos = get_center_pos(canvas_size[0], shield.size[0])
    final_img.alpha_composite(shield, (shield_pos))

    # Сохраняем результат
    final_img.save("result_shield_small_flag.png")


if __name__ == "__main__":
    build_flag_with_shield_on_cpu()
