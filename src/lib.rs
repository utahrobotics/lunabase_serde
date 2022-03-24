use gdnative::prelude::*;
use gdnative::core_types::{ByteArray, Vector3};

/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(Node)]
pub struct Serde;

// You may add any number of ordinary `impl` blocks as you want. However, ...
impl Serde {
    /// The "constructor" of the class.
    fn new(_owner: &Node) -> Self {
        Serde
    }
}


fn byte_array<const N: usize>(g_arr: ByteArray) -> [u8; N] {
    let mut arr = [0; N];
    for i in 0..N {
        arr[i] = g_arr.get(i as i32);
    }
    arr
}


// Only __one__ `impl` block can have the `#[methods]` attribute, which
// will generate code to automatically bind any exported methods to Godot.
#[methods]
impl Serde {
    #[export]
    fn serialize_i64(&self, _owner: &Node, num: i64) -> ByteArray {
        ByteArray::from_slice(num.to_le_bytes().as_slice())
    }
    #[export]
    fn serialize_f32(&self, _owner: &Node, num: f32) -> ByteArray {
        ByteArray::from_slice(num.to_le_bytes().as_slice())
    }
    #[export]
    fn serialize_f64(&self, _owner: &Node, num: f64) -> ByteArray {
        ByteArray::from_slice(num.to_le_bytes().as_slice())
    }
    #[export]
    fn serialize_vector2(&self, _owner: &Node, vec: Vector2) -> ByteArray {
        ByteArray::from_vec([
            vec.x.to_le_bytes().as_slice(),
            vec.y.to_le_bytes().as_slice()
        ].concat())
    }
    #[export]
    fn serialize_vector3(&self, _owner: &Node, vec: Vector3) -> ByteArray {
        ByteArray::from_vec([
            vec.x.to_le_bytes().as_slice(),
            vec.y.to_le_bytes().as_slice(),
            vec.z.to_le_bytes().as_slice()
        ].concat())
    }
    #[export]
    fn deserialize_i64(&self, _owner: &Node, bytes: ByteArray) -> i64 {
        i64::from_le_bytes(byte_array(bytes))
    }
    #[export]
    fn deserialize_f32(&self, _owner: &Node, bytes: ByteArray) -> f32 {
        f32::from_le_bytes(byte_array(bytes))
    }
    #[export]
    fn deserialize_f64(&self, _owner: &Node, bytes: ByteArray) -> f64 {
        f64::from_le_bytes(byte_array(bytes))
    }
    #[export]
    fn deserialize_vector2(&self, _owner: &Node, bytes: ByteArray) -> Vector2 {
        Vector2::new(
            f32::from_le_bytes([bytes.get(0), bytes.get(1), bytes.get(2), bytes.get(3)]),
            f32::from_le_bytes([bytes.get(4), bytes.get(5), bytes.get(6), bytes.get(7)])
        )
    }
    #[export]
    fn deserialize_vector3(&self, _owner: &Node, bytes: ByteArray) -> Vector3 {
        Vector3::new(
            f32::from_le_bytes([bytes.get(0), bytes.get(1), bytes.get(2), bytes.get(3)]),
            f32::from_le_bytes([bytes.get(4), bytes.get(5), bytes.get(6), bytes.get(7)]),
            f32::from_le_bytes([bytes.get(8), bytes.get(9), bytes.get(10), bytes.get(11)])
        )
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    // Register the new `HelloWorld` type we just declared.
    handle.add_class::<Serde>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);