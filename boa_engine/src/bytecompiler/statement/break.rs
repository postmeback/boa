use crate::{bytecompiler::ByteCompiler, vm::Opcode};
use boa_ast::statement::Break;

impl ByteCompiler<'_, '_> {
    /// Compile a [`Break`] `boa_ast` node
    pub(crate) fn compile_break(&mut self, node: Break, _use_expr: bool) {
        let target_jump_info_index = self.break_jump_info_target_index(node);

        for i in (target_jump_info_index..self.jump_info.len()).rev() {
            let count = self.jump_info_open_environment_count(i);
            for _ in 0..count {
                self.emit_opcode(Opcode::PopEnvironment);
            }

            let info = &mut self.jump_info[i];
            if info.is_try_block() && info.has_finally() {
                let next_index = info.jumps.len();

                self.emit_push_integer(next_index as i32 + 1);
                self.emit_opcode(Opcode::PushFalse);
                let break_label = self.emit_opcode_with_operand(Opcode::Break);

                let info = &mut self.jump_info[i];
                info.push_break_label(node.label(), break_label, Some(target_jump_info_index));
                break;
            }

            if i == target_jump_info_index {
                let break_label = self.emit_opcode_with_operand(Opcode::Break);
                let info = &mut self.jump_info[i];
                info.push_break_label(node.label(), break_label, None);
                break;
            }
        }
    }

    fn break_jump_info_target_index(&self, node: Break) -> usize {
        if let Some(label) = node.label() {
            for (i, info) in self.jump_info.iter().enumerate().rev() {
                if info.label() == Some(label) {
                    return i;
                }
            }
        } else {
            for (i, info) in self.jump_info.iter().enumerate().rev() {
                if info.is_loop() || info.is_switch() {
                    return i;
                }
            }
        }

        unreachable!("There should be a valid break jump target")
    }
}
