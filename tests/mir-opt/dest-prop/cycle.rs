// ignore-wasm32 compiled with panic=abort by default
//! Tests that cyclic assignments don't hang DestinationPropagation, and result in reasonable code.
// unit-test: DestinationPropagation
fn val() -> i32 {
    1
}

// EMIT_MIR cycle.main.DestinationPropagation.diff
fn main() {
    let mut x = val();
    let y = x;
    let z = y;
    x = z;

    drop(x);
}
