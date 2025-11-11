use crate::tasks::runner;
use crate::tasks::task::Task;

/// A value factory that uses the result of an asynchronous task to compute an output value.
///
/// The fetcher takes a task and a factory function.
/// It produces an output value `O` by taking an [Option] of an input value `I`.
///
/// If the inner [Task] is not ready yet, the factory will be called with `None`.
/// Once the inner [Task] is ready, the factory will be called with `Some(I)`.
///
/// This is similar to the [FutureBuilder](https://api.flutter.dev/flutter/widgets/FutureBuilder-class.html) from Flutter.
pub struct Fetcher<I: Send + 'static, O> {
    task: Option<Box<dyn Task<I>>>,
    factory: Box<dyn Fn(Option<I>) -> O>,
    value: Option<O>,
}

impl<I: Send + 'static, O> Fetcher<I, O> {
    /// Create a new task fetcher with the given task and factory function.
    #[inline(always)]
    pub const fn new(
        task: Box<dyn Task<I>>,
        factory: Box<dyn Fn(Option<I>) -> O + 'static>,
    ) -> Self {
        Self {
            task: Some(task),
            factory,
            value: None,
        }
    }

    /// Spawn a future using the [TaskRunner](runner::TaskRunner)
    /// and create a new [Fetcher] with the resulting task and the given factory function.
    ///
    /// If you want to spawn a blocking task, use [Fetcher::spawn_blocking] instead.
    #[inline(always)]
    pub fn spawn(
        future: impl Future<Output = I> + Send + 'static,
        factory: impl Fn(Option<I>) -> O + 'static,
    ) -> Self {
        let runner = runner();
        let task = runner.spawn(future);

        Self::new(task, Box::new(factory))
    }

    /// Spawn a blocking function using the [TaskRunner](runner::TaskRunner)
    /// and create a new [Fetcher] with the resulting task and the given factory function.
    ///
    /// If your task is non-blocking (a simple future), use [Fetcher::spawn] instead.
    #[inline(always)]
    #[cfg(native)]
    pub fn spawn_blocking(
        func: impl Fn() -> I + Send + 'static,
        factory: impl Fn(Option<I>) -> O + 'static,
    ) -> Self {
        let runner = runner();
        let task = runner.spawn_blocking(func);

        Self::new(task, Box::new(factory))
    }

    /// Returns a mutable reference to the possible output value.
    ///
    /// This is mostly used internally and will return the cached output value.
    #[inline(always)]
    pub const fn value_mut(&mut self) -> Option<&mut O> {
        self.value.as_mut()
    }

    /// Returns a reference to the possible output value.
    ///
    /// This is mostly used internally and will return the cached output value.
    #[inline(always)]
    pub const fn value_ref(&self) -> Option<&O> {
        self.value.as_ref()
    }

    /// Returns whether the inner task is ready.
    ///
    /// **NOTE:** This will only return `true`, if the inner task has finished,
    /// but the value has not been consumed yet.
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        self.task.as_ref().map(|t| t.is_ready()).unwrap_or(false)
    }

    /// Returns whether the inner task has finished and the value has been computed.
    #[inline(always)]
    pub const fn is_fetched(&self) -> bool {
        self.task.is_none() && self.value.is_some()
    }

    /// Computes the output value from the inner task.
    ///
    /// If the inner task is not ready, the factory function will be called with `None`.
    /// Otherwise, it will be called with `Some(I)`.
    #[inline(always)]
    pub fn compute(&mut self) {
        if self.is_ready() {
            let mut task = self.task.take().unwrap();
            let value = task.take().unwrap();

            self.value = Some((self.factory)(Some(value)));
        } else {
            self.value = Some((self.factory)(None));
        }
    }

    /// Fetches the output value and returns a mutable reference.
    ///
    /// This will only actually compute the value,
    /// if the task is not fully finished yet,
    /// so it's safe to call, even if the task is already done.
    #[inline(always)]
    pub fn fetch(&mut self) -> &mut O {
        if !self.is_fetched() {
            self.compute();
        }

        self.value_mut().unwrap()
    }
}
