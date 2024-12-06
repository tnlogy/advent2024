@group(0)
@binding(0)
var<storage, read_write> v_indices: array<u32>; // this is used as both input and output for convenience

@group(0)
@binding(1)
var<storage, read> v_indices_2: array<u32>; // col1

// this just does absolute difference between two numbers
fn day1(col0: u32, col1: u32) -> u32{
    return select(col1 - col0, col0 - col1, col0 >= col1);
}

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_indices[global_id.x] = day1(v_indices[global_id.x], v_indices_2[global_id.x]);
}