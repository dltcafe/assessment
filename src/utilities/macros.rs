/// Counts number of elements.
///
/// It's used in other macros.
///
/// # Examples
///
/// ```
/// # pub use assessment::count;
/// assert_eq!(count![3 "a" 5.7], 3);
/// ```
///
#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + $crate::count!($($xs)*));
}
