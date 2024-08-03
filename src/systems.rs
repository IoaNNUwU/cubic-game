pub mod grab;
pub mod player;

/// Система
/// 1 - состояние (T)
/// 2 - код инициализации состояния (fn new)
/// 3 - код обновления состояния (fn update) - loop 
pub trait System<T> {
    fn update(&self, args: T);
}