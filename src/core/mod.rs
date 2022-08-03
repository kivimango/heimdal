/// An application event.
/// If within that tick rate no input event happens, the application will emit a Tick.
/// Otherwise, the input will be emitted.
pub enum Event<I> {
    /// User input from keyboard
    Input(I),
    ///
    Tick,
}
