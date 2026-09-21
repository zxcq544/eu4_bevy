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

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {        

    // Sample our textures using the provided UV coordinates
    let flag_color = centered_smaller_texture(flag_texture, flag_sampler, in.uv, 1.53);
    let shield_color = textureSample(shield_texture, shield_sampler, in.uv);
    let mask_color = centered_smaller_texture(mask_texture, mask_sampler, in.uv, 1.53);

    //Porter-Duff alpha compositing formula from LLM
    // 1. Prepare background and foreground textures
    let bg = flag_color * mask_color.a;
    let fg = shield_color;
    // 2. Calculate the final output alpha channel
    // Formula: out_a = fg_a + bg_a * (1.0 - fg_a)
    let final_alpha: f32 = fg.a + bg.a * (1.0 - fg.a);
    // 3. Prevent division by zero if both pixels are completely transparent
    if (final_alpha == 0.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    // 4. Combine the RGB colors using the Porter-Duff "Over" equation
    // Formula: out_rgb = (fg_rgb * fg_a + bg_rgb * bg_a * (1.0 - fg_a)) / out_a
    let final_rgb: vec3<f32> = (fg.rgb * fg.a + bg.rgb * bg.a * (1.0 - fg.a)) / final_alpha;

    return vec4<f32>(final_rgb, final_alpha);
}

fn centered_smaller_texture(texture: texture_2d<f32>, sampler: sampler, uv: vec2<f32>, scale: f32) -> vec4<f32> {    
    var centered_uv = (uv - vec2<f32>(1/scale)) * scale + vec2<f32>(1/scale);
    centered_uv.x = centered_uv.x + 0.05;
    centered_uv.y = centered_uv.y + 0.05;
    if (centered_uv.x < 0.0 || centered_uv.x > 1.0 || centered_uv.y < 0.0 || centered_uv.y > 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    return textureSample(texture, sampler, centered_uv);
}