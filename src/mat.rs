pub use mats::*;
use xege_ffi::*;

pub trait IntoEGEMatrix {
    fn into_ege_matrix(&self) -> ege_ege_transform_matrix;
}

impl IntoEGEMatrix for Mat3<f32> {
    fn into_ege_matrix(&self) -> ege_ege_transform_matrix {
        ege_ege_transform_matrix {
            m11: self[0][0],
            m12: self[0][1],
            m21: self[1][0],
            m22: self[1][1],
            m31: self[2][1],
            m32: self[2][1],
        }
    }
}
