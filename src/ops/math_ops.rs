use crate::AnyTensor;
use crate::Operation;
use crate::Result;
use crate::Scope;
use crate::Tensor;
use crate::TensorType;
use tensorflow_macros::define_op;

define_op!(add, Add, "Add", args { a, b });

/// Creates a constant.
///
/// The value can be anything convertible to a tensor, so possibilities include:
///
/// ```
/// # use std::error::Error;
/// # use tensorflow::Scope;
/// # use tensorflow::Tensor;
/// # use tensorflow::ops::constant;
/// # let mut scope = Scope::new_root_scope();
/// let a = constant(1.0f32, &mut scope)?;
/// let b = constant(&[1.0f32, 2.0][..], &mut scope)?;
/// let c = constant(Tensor::new(&[2, 2]).with_values(&[0f32, 1.0, 2.0, 3.0])?, &mut scope)?;
/// # Ok::<(), Box<Error>>(())
/// ```
///
/// Note that e.g. `&[1, 2][..]` is used instead of `&[1, 2]`.  This forces the
/// compiler to treat the value as a slice rather than a reference to a
/// fixed-size array.  This is necessary because `Into<Tensor>` is implemented
/// for slices but not arrays, and the compiler doesn't automatically fall back
/// to treating the array reference as a slice.
pub fn constant<T: TensorType, TT: Into<Tensor<T>>>(
    value: TT,
    scope: &mut Scope,
) -> Result<Operation> {
    let name = scope.get_unique_name_for_op("Const");
    let mut graph = scope.graph_mut();
    let mut c = graph.new_operation("Const", &name)?;
    c.set_attr_tensor("value", value.into())?;
    c.set_attr_type("dtype", T::data_type())?;
    c.finish()
}

pub(crate) fn any_constant(value: &AnyTensor, scope: &mut Scope) -> Result<Operation> {
    let name = scope.get_unique_name_for_op("Const");
    let mut graph = scope.graph_mut();
    let mut c = graph.new_operation("Const", &name)?;
    c.set_attr_any_tensor("value", value)?;
    c.set_attr_type("dtype", value.data_type())?;
    c.finish()
}

define_op!(mat_mul, MatMul, "MatMul", args {a, b}, attrs {
    transpose_a: bool => "transpose_a",
    transpose_b: bool => "transpose_b",
});

define_op!(multiply, Multiply, "Mul", args { a, b });

define_op!(subtract, Subtract, "Sub", args { a, b });

define_op!(tanh, Tanh, "Tanh", args { x });
