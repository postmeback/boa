use crate::{
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `LabelledStart` implements the Opcode Operation for `Opcode::LabelledStart`
///
/// Operation:
///  - Start of a labelled block.
#[derive(Debug, Clone, Copy)]
pub(crate) struct LabelledStart;

impl Operation for LabelledStart {
    const NAME: &'static str = "LabelledStart";
    const INSTRUCTION: &'static str = "INST - LabelledStart";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let _end = context.vm.read::<u32>();
        Ok(CompletionType::Normal)
    }
}

/// `LabelledEnd` implements the Opcode Operation for `Opcode::LabelledEnd`
///
/// Operation:
///  - Clean up environments at the end of labelled block.
#[derive(Debug, Clone, Copy)]
pub(crate) struct LabelledEnd;

impl Operation for LabelledEnd {
    const NAME: &'static str = "LabelledEnd";
    const INSTRUCTION: &'static str = "INST - LabelledEnd";

    fn execute(_context: &mut Context<'_>) -> JsResult<CompletionType> {
        Ok(CompletionType::Normal)
    }
}
