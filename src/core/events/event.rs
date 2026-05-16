#[derive(Clone)]
pub enum RuntimeEvent {
    SystemOverload,
    MemoryOverflow,
    ReflectionStarted,
    ReflectionFinished,
}