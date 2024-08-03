pub mod grab;
pub mod yawpitch;
pub mod mouse;

// Система
// 1 - состояние
// 2 - код инициализации состояния
// 3 - код обновления состояния - loop
pub trait System<T> {

    fn init() -> Self;

    fn update(&self, state: T);
}