@group(0)
@binding(0)
var<storage, read> left_values: array<u32>;

@group(0)
@binding(1)
var<storage, read> right_values: array<u32>;

@group(0)
@binding(2)
var<storage, read> mask: array<u32>;

@group(0)
@binding(3)
var<storage, write> new_values: array<u32>;

fn is_set(index: u32) -> bool {
    let index_by_32 = index / 32u;
    return (mask[index_by_32] & (1u << (index % 32u))) != 0u;
}

@compute
@workgroup_size(256)
fn merge(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if is_set(global_id.x) {
        new_values[global_id.x] = left_values[global_id.x];
    } else {
        new_values[global_id.x] = right_values[global_id.x];
    }
}