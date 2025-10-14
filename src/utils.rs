use std::{fmt, io::Write, ops::DerefMut};
use tokio::io::{AsyncWrite, AsyncWriteExt};

#[macro_export]
macro_rules! deque {
    [] => (
        std::collections::VecDeque::new()
    );
    [$elem:expr; $n:expr] => (
        std::collections::VecDeque::from([$elem; $n])
    );
    [$($x:expr),+ $(,)?] => (
        std::collections::VecDeque::from([$($x),+])
    );
}

pub use deque;

#[macro_export]
macro_rules! tree_map {
    {} => {{
        std::collections::BTreeMap::new()
    }};

    { $( $key:expr => $value:expr ),* $(,)? } => {{
        let mut map = std::collections::BTreeMap::new();
        $( map.insert($key, $value); )*
        map
    }}
}

pub use tree_map;

#[macro_export]
macro_rules! ix_map {
    {} => {{
        indexmap::IndexMap::new()
    }};

    { $( $key:expr => $value:expr ),* $(,)? } => {{
        let mut map = indexmap::IndexMap::new();
        $( map.insert($key, $value); )*
        map
    }}
}

pub use ix_map;

pub async fn print_to_async<T>(args: fmt::Arguments<'_>, global_s: fn() -> T, label: &str)
where
    T: AsyncWrite + AsyncWriteExt + Unpin,
{
    let mut res = vec![];
    res.write_fmt(args);
    let mut s = global_s();
    if let Err(e) = s.write_all(&res).await {
        panic!("failed printing to {label}: {e}");
    }
    s.flush().await;
}

#[macro_export]
macro_rules! print_async {
    () => {
        async {}
    };
    ($($arg:tt)*) => {async {
        $crate::utils::print_to_async(std::format_args!($($arg)*), tokio::io::stdout, "stdout").await;
    }};
}

pub use print_async;

#[macro_export]
macro_rules! println_async {
    () => {async {
        $crate::utils::print_async!("\n").await;
    }};
    ($($arg:tt)*) => {async {
        $crate::utils::print_to_async(std::format_args_nl!($($arg)*), tokio::io::stdout, "stdout").await;
    }};
}

pub use println_async;

#[macro_export]
macro_rules! eprint_async {
    () => {
        async {}
    };
    ($($arg:tt)*) => {async {
        $crate::utils::print_to_async(std::format_args!($($arg)*), tokio::io::stderr, "stderr").await;
    }};
}

pub use eprint_async;

#[macro_export]
macro_rules! eprintln_async {
    () => {async {
        $crate::utils::eprint_async!("\n").await;
    }};
    ($($arg:tt)*) => {async {
        $crate::utils::print_to_async(std::format_args_nl!($($arg)*), tokio::io::stderr, "stderr").await;
    }};
}

pub use eprintln_async;

pub async fn write_to_async<T, U>(args: fmt::Arguments<'_>, mut dst: T) -> tokio::io::Result<()>
where
    T: DerefMut<Target = U>,
    U: AsyncWrite + AsyncWriteExt + Unpin,
{
    let mut res = vec![];
    res.write_fmt(args);
    dst.deref_mut().write_all(&res).await
}

#[macro_export]
macro_rules! write_async {
    ($dst:expr, $($arg:tt)*) => {{
        $crate::utils::write_to_async(std::format_args!($($arg)*), &mut $dst)
    }};
}

#[macro_export]
macro_rules! writeln_async {
    ($dst:expr, $($arg:tt)*) => {{
        $crate::utils::write_to_async(std::format_args_nl!($($arg)*), &mut $dst)
    }};
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_printers() {
    print_async!("Hello\n").await;
    println_async!("Hello {}", 2).await;
}
