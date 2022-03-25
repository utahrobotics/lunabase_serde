use std::marker::PhantomData;
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
    fn u8_to_bools(mut num: u8) -> [bool; 8] {
        let mut out = [false; 8];

        for mut i in 0..8 {
            i = 7 - i;
            let factor = 2u8.pow(i);
            if num >= factor {
                num -= factor;
                out[i as usize] = true;
            }
        }
        out
    }
    fn bools_to_u8(bools: [bool; 8]) -> u8 {
        let mut out = 0;

        for (i, v) in bools.iter().enumerate() {
            if *v {
                out += 2u8.pow(i as u32);
            }
        }
        out
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
    fn serialize_quat(&self, _owner: &Node, quat: Quat) -> ByteArray {
        ByteArray::from_vec([
            quat.i.to_le_bytes().as_slice(),
            quat.j.to_le_bytes().as_slice(),
            quat.k.to_le_bytes().as_slice(),
            quat.r.to_le_bytes().as_slice()
        ].concat())
    }
    #[export]
    fn serialize_bool_array(&self, _owner: &Node, arr: VariantArray) -> ByteArray {
        let size = arr.len();
        let iterations = size / 8;
        let extra = size % 8 != 0;

        let mut out = ByteArray::new();
        out.resize(iterations + extra as i32);
        for i in 0..iterations {
            let idx = i * 8;
            out.set(i, Self::bools_to_u8([
                arr.get(idx).to_bool(),
                arr.get(idx + 1).to_bool(),
                arr.get(idx + 2).to_bool(),
                arr.get(idx + 3).to_bool(),
                arr.get(idx + 4).to_bool(),
                arr.get(idx + 5).to_bool(),
                arr.get(idx + 6).to_bool(),
                arr.get(idx + 7).to_bool()
            ]))
        }

        if extra {
            let offset = iterations * 8;
            let mut bools = [false; 8];
            for i in 0..(size - offset) {
                bools[i as usize] = arr.get(i + offset).to_bool();
            }
            out.set(iterations, Self::bools_to_u8(bools))
        }
        out
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
    #[export]
    fn deserialize_quat(&self, _owner: &Node, bytes: ByteArray) -> Quat {
        Quat{
            i: f32::from_le_bytes([bytes.get(0), bytes.get(1), bytes.get(2), bytes.get(3)]),
            j: f32::from_le_bytes([bytes.get(4), bytes.get(5), bytes.get(6), bytes.get(7)]),
            k: f32::from_le_bytes([bytes.get(8), bytes.get(9), bytes.get(10), bytes.get(11)]),
            r: f32::from_le_bytes([bytes.get(12), bytes.get(13), bytes.get(14), bytes.get(15)]),
            _unit: PhantomData
        }
    }
    #[export]
    fn deserialize_bool_array(&self, _owner: &Node, bytes: ByteArray, mut expected_size: usize) -> VariantArray<Unique> {
        let arr = VariantArray::new();
        arr.resize(expected_size as i32);
        for i in 0..bytes.len() {
            let bools = Self::u8_to_bools(bytes.get(i as i32));
            for j in 0..expected_size {
                arr.set(i * 8 + j as i32, bools[j]);
            }
            if expected_size <= 8 {
                break
            }
            expected_size -= 8;
        }
        arr
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    // Register the new `HelloWorld` type we just declared.
    handle.add_class::<Serde>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);