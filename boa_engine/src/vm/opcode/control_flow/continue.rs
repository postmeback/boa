use crate::{
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `Continue` implements the Opcode Operation for `Opcode::Continue`
///
/// Operands:
///   - Target address
///   - Initial environments to reconcile on continue (will be tracked along with changes to environment stack)
///
/// Operation:
///   - Initializes the `AbruptCompletionRecord` for a delayed continued in a `Opcode::FinallyEnd`
pub(crate) struct Continue;

impl Operation for Continue {
    const NAME: &'static str = "Continue";
    const INSTRUCTION: &'static str = "INST - Continue";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let jump_address = context.vm.read::<u32>();

        context.vm.frame_mut().pc = jump_address;
        Ok(CompletionType::Normal)
    }
}
