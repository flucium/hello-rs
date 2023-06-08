fn main() {}



// Rust 1.70.0
// https://www.rust-lang.org
mod rust1_70_0 {

    // OnceCell and OnceLock
    // https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html
    // https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html#oncecell-and-oncelock
    pub fn oncecell_and_oncelock_example() {
        use std::sync::OnceLock;
        static WINNER: OnceLock<&str> = OnceLock::new();

        fn main() {
            let winner = std::thread::scope(|s| {
                s.spawn(|| WINNER.set("thread"));

                std::thread::yield_now(); // give them a chance...

                WINNER.get_or_init(|| "main")
            });

            println!("{winner} wins!");
        }

        main();
    }

    // IsTerminal
    // https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html
    // https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html#isterminal
    fn isterminal() {
        use std::io::{stdout, IsTerminal};
        fn main() {
            let use_color = stdout().is_terminal();
        }
    }

    // Stabilized APIs
    // https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html
    // https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html#stabilized-apis
    //
    // NonZero*::MIN/MAX
    // BinaryHeap::retain
    // Default for std::collections::binary_heap::IntoIter
    // Default for std::collections::btree_map::{IntoIter, Iter, IterMut}
    // Default for std::collections::btree_map::{IntoKeys, Keys}
    // Default for std::collections::btree_map::{IntoValues, Values}
    // Default for std::collections::btree_map::Range
    // Default for std::collections::btree_set::{IntoIter, Iter}
    // Default for std::collections::btree_set::Range
    // Default for std::collections::linked_list::{IntoIter, Iter, IterMut}
    // Default for std::vec::IntoIter
    // Default for std::iter::Chain
    // Default for std::iter::Cloned
    // Default for std::iter::Copied
    // Default for std::iter::Enumerate
    // Default for std::iter::Flatten
    // Default for std::iter::Fuse
    // Default for std::iter::Rev
    // Default for std::slice::Iter
    // Default for std::slice::IterMut
    // Rc::into_inner
    // Arc::into_inner
    // std::cell::OnceCell
    // Option::is_some_and
    // NonNull::slice_from_raw_parts
    // Result::is_ok_and
    // Result::is_err_and
    // std::sync::atomic::Atomic*::as_ptr
    // std::io::IsTerminal
    // std::os::linux::net::SocketAddrExt
    // std::os::unix::net::UnixDatagram::bind_addr
    // std::os::unix::net::UnixDatagram::connect_addr
    // std::os::unix::net::UnixDatagram::send_to_addr
    // std::os::unix::net::UnixListener::bind_addr
    // std::path::Path::as_mut_os_str
    // std::sync::OnceLock
}