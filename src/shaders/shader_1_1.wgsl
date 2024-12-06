@group(0) @binding(0) var<storage, read_write> array_a: array<u32>;
@group(0) @binding(1) var<storage, read_write> array_b: array<u32>;
@group(0) @binding(2) var<storage, read_write> result: atomic<u32>; // Single atomic to store the sum

fn abs_diff(a: u32, b: u32) -> u32 {
    return select(b - a, a - b, a >= b);
}

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;

    // Check if the index is within bounds
    if idx >= arrayLength(&array_a) {
        return;
    }

    // would be nice with a sort here

    // Compute the absolute difference for this index
    let diff = abs_diff(array_a[idx], array_b[idx]);

    // Atomically add the difference to the total sum
    atomicAdd(&result, diff);
}
