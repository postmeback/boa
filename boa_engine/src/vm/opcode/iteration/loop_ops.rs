use crate::JsNativeError;
use crate::{
    vm::{opcode::Operation, CompletionType},
    Context, JsResult,
};

/// `IteratorLoopStart` implements the Opcode Operation for `Opcode::IteratorLoopStart`
///
/// Operation:
///  - Push iterator loop start marker.
#[derive(Debug, Clone, Copy)]
pub(crate) struct IteratorLoopStart;

impl Operation for IteratorLoopStart {
    const NAME: &'static str = "IteratorLoopStart";
    const INSTRUCTION: &'static str = "INST - IteratorLoopStart";

    fn execute(_context: &mut Context<'_>) -> JsResult<CompletionType> {
        Ok(CompletionType::Normal)
    }
}

/// `LoopStart` implements the Opcode Operation for `Opcode::LoopStart`
///
/// Operation:
///  - Push loop start marker.
#[derive(Debug, Clone, Copy)]
pub(crate) struct LoopStart;

impl Operation for LoopStart {
    const NAME: &'static str = "LoopStart";
    const INSTRUCTION: &'static str = "INST - LoopStart";

    fn execute(_context: &mut Context<'_>) -> JsResult<CompletionType> {
        Ok(CompletionType::Normal)
    }
}

/// `LoopContinue` implements the Opcode Operation for `Opcode::LoopContinue`.
///
/// Operation:
///  - Pushes a clean environment onto the frame's `EnvEntryStack`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct LoopContinue;

impl Operation for LoopContinue {
    const NAME: &'static str = "LoopContinue";
    const INSTRUCTION: &'static str = "INST - LoopContinue";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let previous_iteration_count = context.vm.frame_mut().loop_iteration_count;

        let max = context.vm.runtime_limits.loop_iteration_limit();
        if previous_iteration_count > max {
            return Err(JsNativeError::runtime_limit()
                .with_message(format!("Maximum loop iteration limit {max} exceeded"))
                .into());
        }

        context.vm.frame_mut().loop_iteration_count = previous_iteration_count.wrapping_add(1);

        Ok(CompletionType::Normal)
    }
}

/// `LoopEnd` implements the Opcode Operation for `Opcode::LoopEnd`
///
/// Operation:
///  - Clean up environments at the end of a loop.
#[derive(Debug, Clone, Copy)]
pub(crate) struct LoopEnd;

impl Operation for LoopEnd {
    const NAME: &'static str = "LoopEnd";
    const INSTRUCTION: &'static str = "INST - LoopEnd";

    fn execute(_context: &mut Context<'_>) -> JsResult<CompletionType> {
        Ok(CompletionType::Normal)
    }
}
