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
    //let scale = 2.0;
    //let centered_uv = (in.uv - vec2<f32>(0.5)) * scale + vec2<f32>(0.5);
    //if (centered_uv.x < 0.0 || centered_uv.x > 1.0 || centered_uv.y < 0.0 || centered_uv.y > 1.0) {
    //    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    //}
    let scale = 1.5;
     let flag_uv = (in.uv - vec2<f32>(1/scale)) * scale + vec2<f32>(1/scale);     
     let mask_uv = flag_uv;

    // Sample our textures using the provided UV coordinates
    let flag_color = textureSample(flag_texture, flag_sampler, flag_uv);
    let shield_color = textureSample(shield_texture, shield_sampler, in.uv);
    let mask_color = textureSample(mask_texture, mask_sampler, mask_uv);
    let brighter_shield_color = shield_color * 1.2;

    let flag_masked_color = flag_color * mask_color.a;
    let result_color = flag_masked_color + brighter_shield_color;
    return result_color;
    
    //return mix(flag_masked_color, shield_color, shield_color.a);
    // Multiply by the UI node's underlying tint color if necessary
    //return final_color* in.color;    
}
