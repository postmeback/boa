//! `JumpControlInfo` tracks relevant jump information used during compilation.
//!
//! Primarily, jump control tracks information related to the compilation of [iteration
//! statements][iteration spec], [switch statements][switch spec], [try statements][try spec],
//! and [labelled statements][labelled spec].
//!
//! [iteration spec]: https://tc39.es/ecma262/#sec-iteration-statements
//! [switch spec]: https://tc39.es/ecma262/#sec-switch-statement
//! [try spec]: https://tc39.es/ecma262/#sec-try-statement
//! [labelled spec]: https://tc39.es/ecma262/#sec-labelled-statements

use crate::{
    bytecompiler::{ByteCompiler, Label},
    vm::Opcode,
};
use bitflags::bitflags;
use boa_interner::Sym;

#[derive(Debug, Clone, Copy)]
pub(crate) enum JumpRecordAction {
    CreateJump,
    Transfter { index: u32 },
    PopEnvironments { count: u32 },
    HandleFinally { value: i32 },
    CloseIterator { r#async: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum JumpRecordKind {
    Break,
    Continue,
    Return,
}

#[derive(Debug, Clone)]
pub(crate) struct JumpRecord {
    kind: JumpRecordKind,
    label: Label,
    actions: Vec<JumpRecordAction>,
}

impl JumpRecord {
    pub(crate) const fn new(
        kind: JumpRecordKind,
        label: Label,
        actions: Vec<JumpRecordAction>,
    ) -> Self {
        Self {
            kind,
            label,
            actions,
        }
    }

    pub(crate) fn perform_actions(
        mut self,
        start_address: u32,
        compiler: &mut ByteCompiler<'_, '_>,
    ) {
        while let Some(action) = self.actions.pop() {
            match action {
                JumpRecordAction::Transfter { index } => {
                    compiler.jump_info[index as usize].jumps.push(self);
                    return;
                }
                JumpRecordAction::PopEnvironments { count } => {
                    for _ in 0..count {
                        compiler.emit_opcode(Opcode::PopEnvironment);
                    }
                }
                JumpRecordAction::HandleFinally { value } => {
                    compiler.emit_push_integer(value);
                    compiler.emit_opcode(Opcode::PushFalse);
                }
                JumpRecordAction::CreateJump => {
                    self.label = compiler.jump();
                }
                JumpRecordAction::CloseIterator { r#async } => {
                    compiler.iterator_close(r#async);
                }
            }
        }

        match self.kind {
            JumpRecordKind::Break => compiler.patch_jump(self.label),
            JumpRecordKind::Continue => compiler.patch_jump_with_target(self.label, start_address),
            JumpRecordKind::Return => compiler.returns_to_patch.push(self.label),
        }
    }
}

/// Boa's `ByteCompiler` jump information tracking struct.
#[derive(Debug, Clone)]
pub(crate) struct JumpControlInfo {
    label: Option<Sym>,
    start_address: u32,
    pub(crate) flags: JumpControlInfoFlags,
    pub(crate) jumps: Vec<JumpRecord>,
    current_open_environments_count: u32,
}

bitflags! {
    /// A bitflag that contains the type flags and relevant booleans for `JumpControlInfo`.
    #[derive(Debug, Clone, Copy)]
    pub(crate) struct JumpControlInfoFlags: u16 {
        const LOOP = 0b0000_0001;
        const SWITCH = 0b0000_0010;
        const TRY_BLOCK = 0b0000_0100;
        const LABELLED = 0b0000_1000;
        const HAS_FINALLY = 0b0010_0000;
        const ITERATOR_LOOP = 0b0100_0000;
        const FOR_AWAIT_OF_LOOP = 0b1000_0000;

        /// Is the statement compiled with use_expr set to true.
        ///
        /// This bitflag is inherited if the previous [`JumpControlInfo`].
        const USE_EXPR = 0b0001_0000_0000;

        const IN_FINALLY = 0b0010_0000_0000;
    }
}

impl Default for JumpControlInfoFlags {
    fn default() -> Self {
        Self::empty()
    }
}

/// ---- `JumpControlInfo` Creation Methods ----
impl JumpControlInfo {
    fn new(current_open_environments_count: u32) -> Self {
        Self {
            label: None,
            start_address: u32::MAX,
            flags: JumpControlInfoFlags::default(),
            jumps: Vec::new(),
            current_open_environments_count,
        }
    }

    pub(crate) const fn with_label(mut self, label: Option<Sym>) -> Self {
        self.label = label;
        self
    }

    pub(crate) const fn with_start_address(mut self, address: u32) -> Self {
        self.start_address = address;
        self
    }

    pub(crate) fn with_loop_flag(mut self, value: bool) -> Self {
        self.flags.set(JumpControlInfoFlags::LOOP, value);
        self
    }

    pub(crate) fn with_switch_flag(mut self, value: bool) -> Self {
        self.flags.set(JumpControlInfoFlags::SWITCH, value);
        self
    }

    pub(crate) fn with_try_block_flag(mut self, value: bool) -> Self {
        self.flags.set(JumpControlInfoFlags::TRY_BLOCK, value);
        self
    }

    pub(crate) fn with_labelled_block_flag(mut self, value: bool) -> Self {
        self.flags.set(JumpControlInfoFlags::LABELLED, value);
        self
    }

    pub(crate) fn with_has_finally(mut self, value: bool) -> Self {
        self.flags.set(JumpControlInfoFlags::HAS_FINALLY, value);
        self
    }

    pub(crate) fn with_iterator_loop(mut self, value: bool) -> Self {
        self.flags.set(JumpControlInfoFlags::ITERATOR_LOOP, value);
        self
    }

    pub(crate) fn with_for_await_of_loop(mut self, value: bool) -> Self {
        self.flags
            .set(JumpControlInfoFlags::FOR_AWAIT_OF_LOOP, value);
        self
    }
}

/// ---- `JumpControlInfo` const fn methods ----
impl JumpControlInfo {
    pub(crate) const fn label(&self) -> Option<Sym> {
        self.label
    }

    pub(crate) const fn start_address(&self) -> u32 {
        self.start_address
    }

    pub(crate) const fn is_loop(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::LOOP)
    }

    pub(crate) const fn is_switch(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::SWITCH)
    }

    pub(crate) const fn is_try_block(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::TRY_BLOCK)
    }

    pub(crate) const fn is_labelled(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::LABELLED)
    }

    pub(crate) const fn has_finally(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::HAS_FINALLY)
    }

    pub(crate) const fn in_finally(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::IN_FINALLY)
    }

    pub(crate) const fn use_expr(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::USE_EXPR)
    }

    #[allow(dead_code)]
    pub(crate) const fn iterator_loop(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::ITERATOR_LOOP)
    }

    #[allow(dead_code)]
    pub(crate) const fn for_await_of_loop(&self) -> bool {
        self.flags.contains(JumpControlInfoFlags::FOR_AWAIT_OF_LOOP)
    }
}

/// ---- `JumpControlInfo` interaction methods ----
impl JumpControlInfo {
    /// Sets the `label` field of `JumpControlInfo`.
    pub(crate) fn set_label(&mut self, label: Option<Sym>) {
        assert!(self.label.is_none());
        self.label = label;
    }

    /// Sets the `start_address` field of `JumpControlInfo`.
    pub(crate) fn set_start_address(&mut self, start_address: u32) {
        self.start_address = start_address;
    }
}

// `JumpControlInfo` related methods that are implemented on `ByteCompiler`.
impl ByteCompiler<'_, '_> {
    /// Pushes a generic `JumpControlInfo` onto `ByteCompiler`
    ///
    /// Default `JumpControlInfoKind` is `JumpControlInfoKind::Loop`
    pub(crate) fn push_empty_loop_jump_control(&mut self, use_expr: bool) {
        let new_info =
            JumpControlInfo::new(self.current_open_environments_count).with_loop_flag(true);
        self.push_contol_info(new_info, use_expr);
    }

    pub(crate) fn current_jump_control_mut(&mut self) -> Option<&mut JumpControlInfo> {
        self.jump_info.last_mut()
    }

    pub(crate) fn push_contol_info(&mut self, mut info: JumpControlInfo, use_expr: bool) {
        info.flags.set(JumpControlInfoFlags::USE_EXPR, use_expr);

        if let Some(last) = self.jump_info.last() {
            // Inherits the `JumpControlInfoFlags::USE_EXPR` flag.
            info.flags |= last.flags & JumpControlInfoFlags::USE_EXPR;
        }

        self.jump_info.push(info);
    }

    /// Does the jump control info have the `use_expr` flag set to true.
    ///
    /// See [`JumpControlInfoFlags`].
    pub(crate) fn jump_control_info_has_use_expr(&self) -> bool {
        if let Some(last) = self.jump_info.last() {
            return last.use_expr();
        }

        false
    }

    // ---- Labelled Statement JumpControlInfo methods ---- //

    /// Pushes a `LabelledStatement`'s `JumpControlInfo` onto the `jump_info` stack.
    pub(crate) fn push_labelled_control_info(
        &mut self,
        label: Sym,
        start_address: u32,
        use_expr: bool,
    ) {
        let new_info = JumpControlInfo::new(self.current_open_environments_count)
            .with_labelled_block_flag(true)
            .with_label(Some(label))
            .with_start_address(start_address);

        self.push_contol_info(new_info, use_expr);
    }

    /// Pops and handles the info for a label's `JumpControlInfo`
    ///
    /// # Panic
    ///  - Will panic if `jump_info` stack is empty.
    ///  - Will panic if popped `JumpControlInfo` is not for a `LabelledStatement`.
    pub(crate) fn pop_labelled_control_info(&mut self) {
        assert!(!self.jump_info.is_empty());
        let info = self.jump_info.pop().expect("no jump information found");

        assert!(info.is_labelled());
        assert!(info.label().is_some());

        let start_address = info.start_address();
        for jump_record in info.jumps {
            jump_record.perform_actions(start_address, self);
        }
    }
    // ---- `IterationStatement`'s `JumpControlInfo` methods ---- //

    /// Pushes an `WhileStatement`, `ForStatement` or `DoWhileStatement`'s `JumpControlInfo` on to the `jump_info` stack.
    pub(crate) fn push_loop_control_info(
        &mut self,
        label: Option<Sym>,
        start_address: u32,
        use_expr: bool,
    ) {
        let new_info = JumpControlInfo::new(self.current_open_environments_count)
            .with_loop_flag(true)
            .with_label(label)
            .with_start_address(start_address);

        self.push_contol_info(new_info, use_expr);
    }

    /// Pushes a `ForInOfStatement`'s `JumpControlInfo` on to the `jump_info` stack.
    pub(crate) fn push_loop_control_info_for_of_in_loop(
        &mut self,
        label: Option<Sym>,
        start_address: u32,
        use_expr: bool,
    ) {
        let new_info = JumpControlInfo::new(self.current_open_environments_count)
            .with_loop_flag(true)
            .with_label(label)
            .with_start_address(start_address)
            .with_iterator_loop(true);

        self.push_contol_info(new_info, use_expr);
    }

    pub(crate) fn push_loop_control_info_for_await_of_loop(
        &mut self,
        label: Option<Sym>,
        start_address: u32,
        use_expr: bool,
    ) {
        let new_info = JumpControlInfo::new(self.current_open_environments_count)
            .with_loop_flag(true)
            .with_label(label)
            .with_start_address(start_address)
            .with_iterator_loop(true)
            .with_for_await_of_loop(true);

        self.push_contol_info(new_info, use_expr);
    }

    /// Pops and handles the info for a loop control block's `JumpControlInfo`
    ///
    /// # Panic
    ///  - Will panic if `jump_info` stack is empty.
    ///  - Will panic if popped `JumpControlInfo` is not for a loop block.
    pub(crate) fn pop_loop_control_info(&mut self) {
        assert!(!self.jump_info.is_empty());
        let info = self.jump_info.pop().expect("no jump information found");

        assert!(info.is_loop());

        let start_address = info.start_address();
        for jump_record in info.jumps {
            jump_record.perform_actions(start_address, self);
        }
    }

    // ---- `SwitchStatement` `JumpControlInfo` methods ---- //

    /// Pushes a `SwitchStatement`'s `JumpControlInfo` on to the `jump_info` stack.
    pub(crate) fn push_switch_control_info(
        &mut self,
        label: Option<Sym>,
        start_address: u32,
        use_expr: bool,
    ) {
        let new_info = JumpControlInfo::new(self.current_open_environments_count)
            .with_switch_flag(true)
            .with_label(label)
            .with_start_address(start_address);

        self.push_contol_info(new_info, use_expr);
    }

    /// Pops and handles the info for a switch block's `JumpControlInfo`
    ///
    /// # Panic
    ///  - Will panic if `jump_info` stack is empty.
    ///  - Will panic if popped `JumpControlInfo` is not for a switch block.
    pub(crate) fn pop_switch_control_info(&mut self) {
        assert!(!self.jump_info.is_empty());
        let info = self.jump_info.pop().expect("no jump information found");

        assert!(info.is_switch());

        let start_address = info.start_address();
        for jump_record in info.jumps {
            jump_record.perform_actions(start_address, self);
        }
    }

    // ---- `TryStatement`'s `JumpControlInfo` methods ---- //

    /// Pushes a `TryStatement`'s `JumpControlInfo` onto the `jump_info` stack.
    pub(crate) fn push_try_control_info(
        &mut self,
        has_finally: bool,
        start_address: u32,
        use_expr: bool,
    ) {
        let new_info = JumpControlInfo::new(self.current_open_environments_count)
            .with_try_block_flag(true)
            .with_start_address(start_address)
            .with_has_finally(has_finally);

        self.push_contol_info(new_info, use_expr);
    }

    /// Pops and handles the info for a try block's `JumpControlInfo`
    ///
    /// # Panic
    ///  - Will panic if `jump_info` is empty.
    ///  - Will panic if popped `JumpControlInfo` is not for a try block.
    pub(crate) fn pop_try_control_info(&mut self, try_end: u32) {
        assert!(!self.jump_info.is_empty());
        let info = self.jump_info.pop().expect("no jump information found");

        assert!(info.is_try_block());

        // Handle breaks. If there is a finally, breaks should go to the finally
        if info.has_finally() {
            for JumpRecord { label, .. } in &info.jumps {
                self.patch_jump_with_target(*label, try_end);
            }

            let (jumps, default) = self.jump_table(info.jumps.len() as u32);

            // Handle breaks in a finally block
            for (i, label) in jumps.iter().enumerate() {
                self.patch_jump(*label);

                let jump_record = info.jumps[i].clone();
                jump_record.perform_actions(u32::MAX, self);
            }

            self.patch_jump(default);
        } else {
            for jump in info.jumps {
                // NOTE: There shouldn't be any breaks or continues attched on a try block without finally
                assert_eq!(jump.kind, JumpRecordKind::Return);
            }
        }
    }

    pub(crate) fn jump_info_open_environment_count(&self, index: usize) -> u32 {
        let current = &self.jump_info[index];
        if let Some(next) = self.jump_info.get(index + 1) {
            return next.current_open_environments_count - current.current_open_environments_count;
        }

        self.current_open_environments_count - current.current_open_environments_count
    }
}
