//! `CallFrame`
//!
//! This module will provides everything needed to implement the `CallFrame`

mod try_stack;

use crate::{
    builtins::{iterable::IteratorRecord, promise::PromiseCapability},
    environments::BindingLocator,
    object::JsObject,
    vm::CodeBlock,
    JsValue,
};
use boa_gc::{Finalize, Gc, Trace};
use thin_vec::ThinVec;

pub(crate) use try_stack::TryStackEntry;

/// A `CallFrame` holds the state of a function call.
#[derive(Clone, Debug, Finalize, Trace)]
pub struct CallFrame {
    pub(crate) code_block: Gc<CodeBlock>,
    pub(crate) pc: u32,
    pub(crate) fp: u32,
    #[unsafe_ignore_trace]
    pub(crate) r#yield: bool,
    // Tracks the number of environments in environment entry.
    // On abrupt returns this is used to decide how many environments need to be pop'ed.
    #[unsafe_ignore_trace]
    pub(crate) try_stack: Vec<TryStackEntry>,
    pub(crate) argument_count: u32,
    #[unsafe_ignore_trace]
    pub(crate) generator_resume_kind: GeneratorResumeKind,
    pub(crate) promise_capability: Option<PromiseCapability>,

    // When an async generator is resumed, the generator object is needed
    // to fulfill the steps 4.e-j in [AsyncGeneratorStart](https://tc39.es/ecma262/#sec-asyncgeneratorstart).
    pub(crate) async_generator: Option<JsObject>,

    // Iterators and their `[[Done]]` flags that must be closed when an abrupt completion is thrown.
    pub(crate) iterators: ThinVec<IteratorRecord>,

    // The stack of bindings being updated.
    pub(crate) binding_stack: Vec<BindingLocator>,

    /// How many iterations a loop has done.
    pub(crate) loop_iteration_count: u64,

    /// The value that is returned from the function.
    //
    // TODO(HalidOdat): Remove this and put into the stack, maybe before frame pointer.
    pub(crate) return_value: JsValue,
}

/// ---- `CallFrame` public API ----
impl CallFrame {
    /// Retrieves the [`CodeBlock`] of this call frame.
    #[inline]
    pub const fn code_block(&self) -> &Gc<CodeBlock> {
        &self.code_block
    }
}

/// ---- `CallFrame` creation methods ----
impl CallFrame {
    /// Creates a new `CallFrame` with the provided `CodeBlock`.
    pub(crate) fn new(code_block: Gc<CodeBlock>) -> Self {
        Self {
            code_block,
            pc: 0,
            fp: 0,
            try_stack: Vec::default(),
            r#yield: false,
            argument_count: 0,
            generator_resume_kind: GeneratorResumeKind::Normal,
            promise_capability: None,
            async_generator: None,
            iterators: ThinVec::new(),
            binding_stack: Vec::new(),
            loop_iteration_count: 0,
            return_value: JsValue::undefined(),
        }
    }

    /// Updates a `CallFrame`'s `argument_count` field with the value provided.
    pub(crate) fn with_argument_count(mut self, count: u32) -> Self {
        self.argument_count = count;
        self
    }
}

/// ---- `CallFrame` stack methods ----
impl CallFrame {
    pub(crate) fn set_frame_pointer(&mut self, pointer: u32) {
        self.fp = pointer;
    }
}

/// Indicates how a generator function that has been called/resumed should return.
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub(crate) enum GeneratorResumeKind {
    #[default]
    Normal,
    Throw,
    Return,
}
