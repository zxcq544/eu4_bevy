@group(1) @binding(0) var flag_texture: texture_2d<f32>;
@group(1) @binding(1) var flag_sampler: sampler;
@group(1) @binding(2) var shield_texture: texture_2d<f32>;
@group(1) @binding(3) var shield_sampler: sampler;
@group(1) @binding(4) var mask_texture: texture_2d<f32>;
@group(1) @binding(5) var mask_sampler: sampler;

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
    let flag_raw = centered_smaller_texture(flag_texture, flag_sampler, in.uv, 1.53);
    let shield_raw = textureSample(shield_texture, shield_sampler, in.uv);
    let mask_raw = centered_smaller_texture(mask_texture, mask_sampler, in.uv, 1.53);

    // 2. THE FIX: Force sRGB Math
    // We convert the sampled colors back to sRGB so the math matches your GLSL code exactly.
    // NOTE: If your colors STILL look blown out/white after this, it means Bevy loaded 
    // your textures as Linear. In that case, REMOVE the linear_to_srgb calls and just use flag_raw.rgb.
    let flag_srgb = linear_to_srgb(flag_raw.rgb);
    let shield_srgb = linear_to_srgb(shield_raw.rgb);

    // 3. Fix: Use .r for grayscale masks! (Alpha is usually 1.0 in standard images)
    let mask_value = mask_raw.a; 

    // 4. Set up foreground and background using sRGB values
    let fg = vec4<f32>(shield_srgb, shield_raw.a);
    var bg = vec4<f32>(flag_srgb, flag_raw.a);
    bg.a = bg.a * mask_value; // Apply mask to alpha only
    //let fg_rbg = fg.rgb * fg.a;
    let bg_rgba_premul = bg.rgb * bg.a;
    let fg_rgba_premul = fg.rgb * fg.a;

    // 5. Standard Porter-Duff "Over" Math (Done in sRGB space now)
    let final_alpha: f32 = fg.a + bg.a * (1.0 - fg.a);
    
    if (final_alpha == 0.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    
    let final_rgb_srgb: vec3<f32> = fg_rgba_premul + bg_rgba_premul * (1.0 - fg.a);

    // 6. Convert back to Linear for Bevy's output
    // This prevents the "Double Gamma" blowout by giving the GPU the correct Linear values 
    // so that when the GPU applies its final sRGB conversion, it looks exactly like your GLSL.
    let final_linear = srgb_to_linear(final_rgb_srgb);

    // Return Straight Alpha (Keep specialize removed so Bevy uses its default ALPHA_BLENDING)
    return vec4<f32>(final_linear, final_alpha);
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