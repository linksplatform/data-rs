use std::ops::ControlFlow;

/// Represents the control flow of an operation, similar to `ControlFlow`.
///
/// This is a simplified enum that can be used with iterators and callbacks
/// to indicate whether to continue or break early.
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flow {
    Continue,
    Break,
}

impl Flow {
    /// Returns `true` if this is `Flow::Continue`.
    #[must_use]
    pub const fn is_continue(&self) -> bool {
        matches!(self, Self::Continue)
    }

    /// Returns `true` if this is `Flow::Break`.
    #[must_use]
    pub const fn is_break(&self) -> bool {
        matches!(self, Self::Break)
    }

    /// Converts this `Flow` into a `ControlFlow` that can be used with `try_for_each`.
    ///
    /// ```
    /// use platform_data::Flow;
    /// use std::ops::ControlFlow;
    ///
    /// let mut count = 0;
    /// let result = (0..10).try_for_each(|i| {
    ///     count += 1;
    ///     if i == 5 { Flow::Break.into_control_flow() } else { Flow::Continue.into_control_flow() }
    /// });
    /// assert_eq!(count, 6);
    /// ```
    pub const fn into_control_flow(self) -> ControlFlow<()> {
        match self {
            Self::Continue => ControlFlow::Continue(()),
            Self::Break => ControlFlow::Break(()),
        }
    }
}

impl<C, B> From<ControlFlow<C, B>> for Flow {
    fn from(flow: ControlFlow<C, B>) -> Self {
        match flow {
            ControlFlow::Continue(_) => Self::Continue,
            ControlFlow::Break(_) => Self::Break,
        }
    }
}

impl From<Flow> for ControlFlow<()> {
    fn from(flow: Flow) -> Self {
        match flow {
            Flow::Continue => Self::Continue(()),
            Flow::Break => Self::Break(()),
        }
    }
}
