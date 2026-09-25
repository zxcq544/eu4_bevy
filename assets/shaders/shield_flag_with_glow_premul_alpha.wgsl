@group(1) @binding(0) var flag_texture: texture_2d_array<f32>;
@group(1) @binding(1) var flag_sampler: sampler;
@group(1) @binding(2) var shield_texture: texture_2d<f32>;
@group(1) @binding(3) var shield_sampler: sampler;
@group(1) @binding(4) var mask_texture: texture_2d<f32>;
@group(1) @binding(5) var mask_sampler: sampler;
@group(1) @binding(6) var glow_texture: texture_2d<f32>;
@group(1) @binding(7) var glow_sampler: sampler;
@group(1) @binding(8) var<uniform> hover_color: vec4<f32>;
@group(1) @binding(9) var<uniform> flag_index: u32;

struct UiVertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};

// Helper: Convert Linear to sRGB (To match GLSL playground math)
fn linear_to_srgb(c: vec3<f32>) -> vec3<f32> {
    let cutoff = 0.0031308;
    let slope = 12.92;
    let gamma = 1.0 / 2.4;
    let low = c * slope;
    let high = 1.055 * pow(c, vec3<f32>(gamma)) - 0.055;
    return select(low, high, c > vec3<f32>(cutoff));
}

// Helper: Convert sRGB to Linear (For Bevy's final output)
fn srgb_to_linear(c: vec3<f32>) -> vec3<f32> {
    let cutoff = 0.04045;
    let slope = 12.92;
    let gamma = 2.4;
    let low = c / slope;
    let high = pow((c + vec3<f32>(0.055)) / 1.055, vec3<f32>(gamma));
    return select(low, high, c > vec3<f32>(cutoff));
}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {  
    // 1. Sample textures    
    let flag_raw = centered_smaller_array_texture(flag_texture, flag_sampler, in.uv, 1.53, flag_index);
    let shield_raw = textureSample(shield_texture, shield_sampler, in.uv);
    let mask_raw = centered_smaller_texture(mask_texture, mask_sampler, in.uv, 1.53);
    // --- CONTROLLING THE GLOW TEXTURE ---
    // Adjust these configurations directly or pass them via a uniform block if you need runtime control
    let glow_scale  = vec2<f32>(1.15, 0.94);  // X > 1.0 makes it THINNER | Y < 1.0 makes it TALLER
    let glow_offset = vec2<f32>(0.0, -0.02); // X positive moves RIGHT  | Y negative moves UP
    
    let glow_raw = transformed_glow_texture(glow_texture, glow_sampler, in.uv, glow_scale, glow_offset);
    let glow_srgb = linear_to_srgb(glow_raw.rgb);
    let glow_alpha = glow_raw.a;    
    let glow_result = glow_srgb * glow_alpha;
    // 2. THE FIX: Force sRGB Math
    // We convert the sampled colors back to sRGB so the math matches your GLSL code exactly.
    // NOTE: If your colors STILL look blown out/white after this, it means Bevy loaded 
    // your textures as Linear. In that case, REMOVE the linear_to_srgb calls and just use flag_raw.rgb.
    let flag_srgb = linear_to_srgb(flag_raw.rgb * hover_color.rgb);
    let shield_srgb = linear_to_srgb(shield_raw.rgb);
    let mask_value = mask_raw.a; 
    // 4. Set up foreground and background using sRGB values
    let fg = vec4<f32>(shield_srgb, shield_raw.a);
    var bg = vec4<f32>(flag_srgb, flag_raw.a);
    bg.a = bg.a * mask_value; // Apply mask to alpha only
    let bg_rgb_premul = vec3<f32>(bg.rgb * bg.a);
    let fg_rgb_premul = vec3<f32>(fg.rgb * fg.a);
    // // 5. Standard Porter-Duff "Over" Math (Done in sRGB space now)
    // let final_alpha: f32 = fg.a + bg.a * (1.0 - fg.a);    
    // if (final_alpha == 0.0) {
    //     return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    // }    
    let final_rgb_srgb: vec3<f32> = fg_rgb_premul + bg_rgb_premul * (1.0 - fg.a);
    // Add glow layer here
    let fg_shield = vec4<f32>(final_rgb_srgb, fg.a);
    let bg_glow = vec4<f32>(glow_result, glow_alpha);
    let final_rgb_srgb_with_glow: vec3<f32> = fg_shield.rgb + bg_glow.rgb * (1.0 - fg_shield.a);
    let final_alpha: f32 = fg_shield.a + bg_glow.a * (1.0 - fg_shield.a);
    // if (final_alpha == 0.0) {
    //     return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    // }    
    let final_linear_with_glow = srgb_to_linear(final_rgb_srgb_with_glow);
    return vec4<f32>(final_linear_with_glow, final_alpha);
    // let final_linear = srgb_to_linear(final_rgb_srgb);
    // return vec4<f32>(final_linear, final_alpha);
    // return vec4<f32>(srgb_to_linear(glow_result), glow_alpha);
}
fn centered_smaller_array_texture(tex: texture_2d_array<f32>, smp: sampler, uv: vec2<f32>, scale: f32, layer: u32) -> vec4<f32> {    
    var centered_uv = (uv - vec2<f32>(1.0 / scale)) * scale + vec2<f32>(1.0 / scale);
    centered_uv.x += 0.05;
    centered_uv.y += 0.05;
    
    if (centered_uv.x < 0.0 || centered_uv.x > 1.0 || centered_uv.y < 0.0 || centered_uv.y > 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    } 
    return textureSample(tex, smp, centered_uv, layer); 
}
fn centered_smaller_texture(texture: texture_2d<f32>, sampler: sampler, uv: vec2<f32>, scale: f32) -> vec4<f32> {    
    var centered_uv = (uv - vec2<f32>(1.0 / scale)) * scale + vec2<f32>(1.0 / scale);
    centered_uv.x = centered_uv.x + 0.05;
    centered_uv.y = centered_uv.y + 0.05;
    
    if (centered_uv.x < 0.0 || centered_uv.x > 1.0 || centered_uv.y < 0.0 || centered_uv.y > 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    return textureSample(texture, sampler, centered_uv);
}

// Custom helper function to transform the glow texture from its center boundary
fn transformed_glow_texture(tex: texture_2d<f32>, smp: sampler, uv: vec2<f32>, scale: vec2<f32>, offset: vec2<f32>) -> vec4<f32> {    
    // 1. Shift UV origin to the center (0.5, 0.5)
    var centered_uv = uv - vec2<f32>(0.5);
    
    // 2. Apply independent X and Y scaling from the center
    centered_uv = centered_uv * scale;
    
    // 3. Shift origin back to top-left corner
    centered_uv = centered_uv + vec2<f32>(0.5);
    
    // 4. Move texture by adding/subtracting the offsets
    // Subtracting from UV moves the texture asset in the positive space direction
    centered_uv.x = centered_uv.x - offset.x;
    centered_uv.y = centered_uv.y - offset.y;
    
    // 5. Alpha clamp out-of-bounds rendering (prevents artifact smearing or trailing repetitions)
    if (centered_uv.x < 0.0 || centered_uv.x > 1.0 || centered_uv.y < 0.0 || centered_uv.y > 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    
    return textureSample(tex, smp, centered_uv);
}