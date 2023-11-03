macro_rules! interpose_impl {
    ($real_fn:path => fn $local_fn:ident ( $($v:ident : $t:ty),* ) -> $r:ty $body:block) => {
        // #[no_mangle]
        fn $local_fn( $($v : $t),* ) -> $r $body

        mod $local_fn {
            #[allow(dead_code)]
            pub struct Interpose {
                new: *const (),
                old: *const (),
            }

            #[link_section = "__DATA, __interpose"]
            #[used]
            pub static mut INTERPOSE: Interpose = Interpose {
                new: super::$local_fn as *const (),
                old: $real_fn as *const (),
            };
        }
    };
}

macro_rules! interpose {
    ($real_fn:ident => fn $local_fn:ident ( $($v:ident : $t:ty),* ) -> $r:ty $body:block) => {
        $crate::os::darwin::interpose_macro::interpose_impl!(super::$real_fn => fn $local_fn ( $($v : $t),* ) -> $r $body);
    };

    ($real_fn:path => fn $local_fn:ident ( $($v:ident : $t:ty),* ) -> $r:ty $body:block) => {
        $crate::os::darwin::interpose_macro::interpose_impl!($real_fn => fn $local_fn ( $($v : $t),* ) -> $r $body);
    };

    ($real_fn:path => fn $local_fn:ident ( $($v:ident : $t:ty),* ) $body:block) => {
        $crate::os::darwin::interpose_macro::interpose_impl!($real_fn => fn $local_fn ( $($v : $t),* ) -> () $body);
    };
}

pub(super) use interpose;
pub(super) use interpose_impl;
