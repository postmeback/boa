use crate::{bytecompiler::ByteCompiler, vm::Opcode};
use boa_ast::statement::Continue;

impl ByteCompiler<'_, '_> {
    #[allow(clippy::unnecessary_wraps)]
    pub(crate) fn compile_continue(&mut self, node: Continue, _use_expr: bool) {
        // if let Some(info) = self.jump_info.last().filter(|info| info.is_try_block()) {
        //     // let in_finally = info.in_finally();
        //     let in_finally_or_has_finally = in_finally || info.has_finally();

        //     // 1. Handle if node has a label.
        //     if let Some(node_label) = node.label() {
        //         let items = self.jump_info.iter().rev().filter(|info| info.is_loop());
        //         let mut iterator_closes = Vec::new();

        //         for info in items {
        //             if info.label() == Some(node_label) {
        //                 break;
        //             }

        //             if info.iterator_loop() {
        //                 iterator_closes.push(info.for_await_of_loop());
        //             }
        //         }

        //         for r#async in iterator_closes {
        //             self.iterator_close(r#async);
        //         }

        //         let cont_label = self.emit_opcode_with_operand(Opcode::Continue);

        //         let loops = self
        //             .jump_info
        //             .iter_mut()
        //             .rev()
        //             .filter(|info| info.is_loop());
        //         // let mut set_continue_as_break = false;
        //         for info in loops {
        //             let found_label = info.label() == Some(node_label);
        //             if found_label && in_finally_or_has_finally {
        //                 // set_continue_as_break = true;
        //                 break;
        //             } else if found_label && !in_finally_or_has_finally {
        //                 info.push_try_continue_label(Some(node_label), cont_label);
        //                 break;
        //             }
        //         }
        //         // if set_continue_as_break {
        //         //     self.jump_info
        //         //         .last_mut()
        //         //         .expect("no jump information found")
        //         //         .push_break_label(Some(node_label), cont_label);
        //         // }
        //     } else {
        //         // TODO: Add has finally or in finally here
        //         let cont_label= self.emit_opcode_with_operand(Opcode::Continue);
        //         // if in_finally_or_has_finally {
        //         //     self.jump_info
        //         //         .last_mut()
        //         //         .expect("Must exist and be a try block")
        //         //         .push_break_label(None, cont_label);
        //         // };
        //         let mut items = self
        //             .jump_info
        //             .iter_mut()
        //             .rev()
        //             .filter(|info| info.is_loop());
        //         let jump_info = items.next().expect("continue must be inside loop");
        //         if !in_finally_or_has_finally {
        //             jump_info.push_try_continue_label(None, cont_label);
        //         };
        //     };
        // } else if let Some(node_label) = node.label() {
        //     let items = self.jump_info.iter().rev().filter(|info| info.is_loop());
        //     let mut iterator_closes = Vec::new();
        //     for info in items {
        //         if info.label() == Some(node_label) {
        //             break;
        //         }

        //         if info.iterator_loop() {
        //             iterator_closes.push(info.for_await_of_loop());
        //         }
        //     }

        //     for r#async in iterator_closes {
        //         self.iterator_close(r#async);
        //     }

        //     let cont_label = self.emit_opcode_with_operand(Opcode::Continue);
        //     let loops = self
        //         .jump_info
        //         .iter_mut()
        //         .rev()
        //         .filter(|info| info.is_loop());

        //     for info in loops {
        //         if info.label() == Some(node_label) {
        //             info.push_try_continue_label(Some(node_label), cont_label);
        //         }
        //     }
        // } else {
        //     let cont_label = self.emit_opcode_with_operand(Opcode::Continue);
        //     let mut items = self
        //         .jump_info
        //         .iter_mut()
        //         .rev()
        //         .filter(|info| info.is_loop());
        //     let jump_info = items.next().expect("continue must be inside loop");
        //     jump_info.push_try_continue_label(None, cont_label);
        // }

        // for i in (target_jump_info_index..self.jump_info.len()).rev() {
        //     let count = self.jump_info_open_environment_count(i);
        //     for _ in 0..count {
        //         self.emit_opcode(Opcode::PopEnvironment);
        //     }

        //     let info = &mut self.jump_info[i];
        //     if info.is_try_block() && info.has_finally() {
        //         let next_index = info.breaks.len();

        //         self.emit_push_integer(next_index as i32 + 1);
        //         self.emit_opcode(Opcode::PushFalse);
        //         let continue_label = self.emit_opcode_with_operand(Opcode::Continue);

        //         let info = &mut self.jump_info[i];
        //         info.push_(node.label(), continue_label, Some(target_jump_info_index));
        //         break;
        //     }

        //     if i == target_jump_info_index {
        //         let break_label = self.emit_opcode_with_operand(Opcode::Break);
        //         let info = &mut self.jump_info[i];
        //         info.push_break_label(node.label(), break_label, None);
        //         break;
        //     }
        // }
        let target_jump_info_index = self.continue_jump_info_target_index(node);

        for i in (target_jump_info_index..self.jump_info.len()).rev() {
            let count = self.jump_info_open_environment_count(i);
            for _ in 0..count {
                self.emit_opcode(Opcode::PopEnvironment);
            }

            if i == target_jump_info_index {
                let continue_label = self.emit_opcode_with_operand(Opcode::Continue);
                let info = &mut self.jump_info[i];
                info.push_continue_label(node.label(), continue_label, None);
                break;
            }

            let info = &mut self.jump_info[i];
            if info.is_try_block() && info.has_finally() {
                let next_index = info.jumps.len();

                self.emit_push_integer(next_index as i32 + 1);
                self.emit_opcode(Opcode::PushFalse);
                let continue_label = self.emit_opcode_with_operand(Opcode::Continue);

                let info = &mut self.jump_info[i];
                info.push_continue_label(
                    node.label(),
                    continue_label,
                    Some(target_jump_info_index),
                );
                break;
            }

            let info = &self.jump_info[i];
            if info.iterator_loop() {
                let r#async = info.for_await_of_loop();
                self.iterator_close(r#async);
            }
        }
    }

    fn continue_jump_info_target_index(&self, node: Continue) -> usize {
        if let Some(label) = node.label() {
            for (i, info) in self.jump_info.iter().enumerate().rev() {
                if info.label() == Some(label) {
                    return i;
                }
            }
        } else {
            for (i, info) in self.jump_info.iter().enumerate().rev() {
                if info.is_loop() {
                    return i;
                }
            }
        }

        unreachable!("There should be a valid break jump target")
    }
}
