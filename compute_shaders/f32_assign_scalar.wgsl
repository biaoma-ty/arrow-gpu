@group(0)
@binding(0)
var<storage, read_write> values: array<f32>;

@group(0)
@binding(1)
var<storage, read> increment: f32;

@compute
@workgroup_size(1)
fn f32_add(@builtin(global_invocation_id) global_id: vec3<u32>) {
    values[global_id.x] = values[global_id.x] + increment;
}