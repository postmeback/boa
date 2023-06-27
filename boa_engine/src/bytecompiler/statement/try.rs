use crate::{
    bytecompiler::{jump_control::JumpControlInfoFlags, ByteCompiler, Label},
    vm::{BindingOpcode, Opcode},
};
use boa_ast::{
    declaration::Binding,
    operations::bound_names,
    statement::{Catch, Finally, Try},
};

impl ByteCompiler<'_, '_> {
    pub(crate) fn compile_try(&mut self, t: &Try, use_expr: bool) {
        // stack:

        let try_start = self.next_opcode_location();
        let (catch_start, finally_loc) = self.emit_opcode_with_two_operands(Opcode::TryStart);
        self.patch_jump_with_target(finally_loc, u32::MAX);

        // If there is a finally block, initialize the finally control block prior to pushing the try block jump_control
        let has_finally = t.finally().is_some();
        let has_catch = t.catch().is_some();
        self.push_try_control_info(t.finally().is_some(), try_start, use_expr);

        self.compile_block(t.block(), use_expr);

        self.emit_opcode(Opcode::TryEnd);

        if has_finally {
            self.emit_opcode(Opcode::PushZero);
            if has_catch {
                self.emit_opcode(Opcode::PushFalse);
            } else {
                self.emit_opcode(Opcode::PushFalse);
            }
        }

        let finally = self.jump();
        self.patch_jump(catch_start);

        if let Some(catch) = t.catch() {
            self.compile_catch_stmt(catch, has_finally, use_expr);
        } else {
            self.emit_opcode(Opcode::Exception);
            self.emit_opcode(Opcode::PushTrue);
        }

        self.patch_jump(finally);

        if let Some(finally) = t.finally() {
            // Pop and push control loops post FinallyStart, as FinallyStart resets flow control variables.
            // Handle finally header operations
            let finally_start = self.next_opcode_location();
            let finally_end = self.emit_opcode_with_operand(Opcode::FinallyStart);

            self.jump_info
                .last_mut()
                .expect("there should be a try block")
                .flags |= JumpControlInfoFlags::IN_FINALLY;
            self.patch_jump_with_target(finally_loc, finally_start);
            // Compile finally statement body
            self.compile_finally_stmt(finally, finally_end, has_catch);

            self.pop_try_control_info(finally_start);
        } else {
            let try_end = self.next_opcode_location();
            self.pop_try_control_info(try_end);
        }
    }

    pub(crate) fn compile_catch_stmt(&mut self, catch: &Catch, has_finally: bool, use_expr: bool) {
        // stack: exception

        self.push_compile_environment(false);
        let push_env = self.emit_opcode_with_operand(Opcode::PushDeclarativeEnvironment);

        self.emit_opcode(Opcode::Exception);
        if let Some(binding) = catch.parameter() {
            match binding {
                Binding::Identifier(ident) => {
                    self.create_mutable_binding(*ident, false);
                    self.emit_binding(BindingOpcode::InitLet, *ident);
                }
                Binding::Pattern(pattern) => {
                    for ident in bound_names(pattern) {
                        self.create_mutable_binding(ident, false);
                    }
                    self.compile_declaration_pattern(pattern, BindingOpcode::InitLet);
                }
            }
        } else {
            self.emit_opcode(Opcode::Pop);
        }

        self.compile_block(catch.block(), use_expr);

        let env_index = self.pop_compile_environment();
        self.patch_jump_with_target(push_env, env_index);
        self.emit_opcode(Opcode::PopEnvironment);

        if has_finally {
            self.emit_opcode(Opcode::PushZero);
            self.emit_opcode(Opcode::PushFalse);
        }
    }

    pub(crate) fn compile_finally_stmt(
        &mut self,
        finally: &Finally,
        finally_end_label: Label,
        has_catch: bool,
    ) {
        // TODO: We could probably remove the Get/SetReturnValue if we check that there is no break/continues statements.
        self.emit_opcode(Opcode::GetReturnValue);
        self.compile_block(finally.block(), true);
        self.emit_opcode(Opcode::SetReturnValue);

        let has_throw_exit = self.jump_if_false();
        if !has_catch {
            self.emit_opcode(Opcode::Throw);
        }

        // Rethrow error if error happend!
        self.emit_opcode(Opcode::ReThrow);
        self.patch_jump(has_throw_exit);

        self.patch_jump(finally_end_label);
    }
}
