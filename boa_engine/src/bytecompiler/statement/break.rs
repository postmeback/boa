use crate::bytecompiler::{
    jump_control::{JumpRecord, JumpRecordAction, JumpRecordKind},
    ByteCompiler,
};
use boa_ast::statement::Break;

impl ByteCompiler<'_, '_> {
    /// Compile a [`Break`] `boa_ast` node
    pub(crate) fn compile_break(&mut self, node: Break, _use_expr: bool) {
        let actions = self.break_jump_record_actions(node);

        JumpRecord::new(JumpRecordKind::Break, Self::DUMMY_LABEL, actions)
            .perform_actions(u32::MAX, self);
    }

    fn break_jump_record_actions(&self, node: Break) -> Vec<JumpRecordAction> {
        let mut actions = Vec::default();

        if let Some(label) = node.label() {
            for (i, info) in self.jump_info.iter().enumerate().rev() {
                let count = self.jump_info_open_environment_count(i);
                actions.push(JumpRecordAction::PopEnvironments { count });

                if info.is_try_block() && info.has_finally() && !info.in_finally() {
                    let next_index = info.jumps.len();

                    actions.push(JumpRecordAction::HandleFinally {
                        value: next_index as i32 + 1,
                    });
                    actions.push(JumpRecordAction::CreateJump);
                    actions.push(JumpRecordAction::Transfter { index: i as u32 });
                }

                if info.label() == Some(label) {
                    actions.push(JumpRecordAction::CreateJump);
                    actions.push(JumpRecordAction::Transfter { index: i as u32 });
                    break;
                }

                if info.iterator_loop() {
                    actions.push(JumpRecordAction::CloseIterator {
                        r#async: info.for_await_of_loop(),
                    });
                }
            }
        } else {
            for (i, info) in self.jump_info.iter().enumerate().rev() {
                let count = self.jump_info_open_environment_count(i);
                actions.push(JumpRecordAction::PopEnvironments { count });

                if info.is_try_block() && info.has_finally() && !info.in_finally() {
                    let next_index = info.jumps.len();

                    actions.push(JumpRecordAction::HandleFinally {
                        value: next_index as i32 + 1,
                    });
                    actions.push(JumpRecordAction::CreateJump);
                    actions.push(JumpRecordAction::Transfter { index: i as u32 });
                }

                if info.is_loop() || info.is_switch() {
                    actions.push(JumpRecordAction::CreateJump);
                    actions.push(JumpRecordAction::Transfter { index: i as u32 });
                    break;
                }
            }
        }

        actions.reverse();

        actions
    }
}
