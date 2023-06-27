use crate::{
    vm::{opcode::Operation, CompletionType},
    Context, JsError, JsNativeError, JsResult,
};

/// `Throw` implements the Opcode Operation for `Opcode::Throw`
///
/// Operation:
///  - Throw exception.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Throw;

impl Operation for Throw {
    const NAME: &'static str = "Throw";
    const INSTRUCTION: &'static str = "INST - Throw";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let error = JsError::from_opaque(context.vm.pop());
        context.vm.err = Some(error);

        if let Some(try_entry) = context.vm.frame_mut().try_stack.pop() {
            let catch_address = try_entry.catch();
            let env_fp = try_entry.env_fp() as usize;
            let fp = try_entry.fp() as usize;

            context.vm.frame_mut().pc = catch_address;
            context.vm.environments.truncate(env_fp);
            context.vm.stack.truncate(fp);
            return Ok(CompletionType::Normal);
        }

        Ok(CompletionType::Throw)
    }
}

/// `ReThrow` implements the Opcode Operation for `Opcode::ReThrow`
///
/// Operation:
///  - Rethrow thrown exception.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ReThrow;

impl Operation for ReThrow {
    const NAME: &'static str = "ReThrow";
    const INSTRUCTION: &'static str = "INST - ReThrow";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        assert!(context.vm.err.is_some(), "Need exception to rethrow");

        if let Some(try_entry) = context.vm.frame_mut().try_stack.pop() {
            let catch_address = try_entry.catch();
            let env_fp = try_entry.env_fp() as usize;
            let fp = try_entry.fp() as usize;

            context.vm.frame_mut().pc = catch_address;
            context.vm.environments.truncate(env_fp);
            context.vm.stack.truncate(fp);
            return Ok(CompletionType::Normal);
        }
        return Ok(CompletionType::Throw);
    }
}

/// `Exception` implements the Opcode Operation for `Opcode::Exception`
///
/// Operation:
///  - Get the thrown exception and push on the stack.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Exception;

impl Operation for Exception {
    const NAME: &'static str = "Exception";
    const INSTRUCTION: &'static str = "INST - Exception";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        if let Some(error) = context.vm.err.take() {
            let error = error.to_opaque(context);
            context.vm.push(error);
            return Ok(CompletionType::Normal);
        }
        unreachable!("")
    }
}

/// `ThrowNewTypeError` implements the Opcode Operation for `Opcode::ThrowNewTypeError`
///
/// Operation:
///  - Throws a `TypeError` exception.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ThrowNewTypeError;

impl Operation for ThrowNewTypeError {
    const NAME: &'static str = "ThrowNewTypeError";
    const INSTRUCTION: &'static str = "INST - ThrowNewTypeError";

    fn execute(context: &mut Context<'_>) -> JsResult<CompletionType> {
        let index = context.vm.read::<u32>();
        let msg = context.vm.frame().code_block.literals[index as usize]
            .as_string()
            .expect("throw message must be a string")
            .clone();
        let msg = msg
            .to_std_string()
            .expect("throw message must be an ASCII string");
        Err(JsNativeError::typ().with_message(msg).into())
    }
}
