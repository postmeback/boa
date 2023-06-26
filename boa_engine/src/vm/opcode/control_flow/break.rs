use crate::{
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `Break` implements the Opcode Operation for `Opcode::Break`
///
/// Operation:
///   - Pop required environments and jump to address.
pub(crate) struct Break;

impl Operation for Break {
    const NAME: &'static str = "Break";
    const INSTRUCTION: &'static str = "INST - Break";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let jump_address = context.vm.read::<u32>();

        // 3. Set program counter and finally return fields.
        context.vm.frame_mut().pc = jump_address;
        Ok(CompletionType::Normal)
    }
}
