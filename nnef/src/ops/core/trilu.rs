use crate::internal::*;
use crate::ser::*;
use tract_core::ops;

pub fn register(registry: &mut Registry) {
    registry.register_dumper(TypeId::of::<ops::array::Trilu>(), ser_trilu);
    registry.register_primitive(
        "tract_core_trilu",
        &[
            TypeName::Scalar.tensor().named("input"),
            TypeName::Scalar.tensor().named("k"),
            TypeName::Logical.named("upper"),
        ],
        &[("output", TypeName::Scalar.tensor())],
        de_trilu,
    );
}

fn ser_trilu(ast: &mut IntoAst, node: &TypedNode) -> TractResult<Option<Arc<RValue>>> {
    let op = node.op().downcast_ref::<ops::array::Trilu>().unwrap();
    let input = ast.mapping[&node.inputs[0]].clone();
    let k = ast.mapping[&node.inputs[1]].clone();
    Ok(Some(invocation(
        "tract_core_trilu",
        &[input, k],
        &[
            ("upper", logical(op.upper)),
        ],
    )))
}

fn de_trilu(
    builder: &mut ModelBuilder,
    invocation: &ResolvedInvocation,
) -> TractResult<Value> {
    let input = invocation.named_arg_as(builder, "input")?;
    let k = invocation.named_arg_as(builder, "k")?;
    let upper = invocation.named_arg_as(builder, "upper")?;
    builder.wire(ops::array::Trilu { upper }, &[input, k])
}

