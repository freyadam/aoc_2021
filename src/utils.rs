use std::any::type_name;
use std::fs;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
