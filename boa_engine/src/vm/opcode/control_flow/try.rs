use crate::{
    vm::{call_frame::TryStackEntry, opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `TryStart` implements the Opcode Operation for `Opcode::TryStart`
///
/// Operation:
///  - Start of a try block.
#[derive(Debug, Clone, Copy)]
pub(crate) struct TryStart;

impl Operation for TryStart {
    const NAME: &'static str = "TryStart";
    const INSTRUCTION: &'static str = "INST - TryStart";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let catch = context.vm.read::<u32>();
        let finally = context.vm.read::<u32>();

        let fp = context.vm.stack.len() as u32;
        let env_fp = context.vm.environments.len() as u32;
        context
            .vm
            .frame_mut()
            .try_stack
            .push(TryStackEntry::new(catch, finally, fp, env_fp));

        Ok(CompletionType::Normal)
    }
}

/// `TryEnd` implements the Opcode Operation for `Opcode::TryEnd`
///
/// Operation:
///  - End of a try block
#[derive(Debug, Clone, Copy)]
pub(crate) struct TryEnd;

impl Operation for TryEnd {
    const NAME: &'static str = "TryEnd";
    const INSTRUCTION: &'static str = "INST - TryEnd";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        context
            .vm
            .frame_mut()
            .try_stack
            .pop()
            .expect("There should be a try entry");

        Ok(CompletionType::Normal)
    }
}
