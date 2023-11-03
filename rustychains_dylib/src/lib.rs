use rustychains_shared::{config, message::Debug};
use static_manager::StaticManager;

mod net;
mod os;
mod proxy_protocol;
mod static_manager;
mod stream;

pub(crate) static STATIC_MANAGER: StaticManager = StaticManager::new();

#[ctor::ctor]
fn dylib_init() {
    let socket_name =
        std::env::var(config::SOCKET_ADDRESS_ENV_VAR).expect("Unable to get the socket addr");

    let rustychains_process_stream =
        interprocess::local_socket::LocalSocketStream::connect(socket_name)
            .expect("Unable to open interprocess conn to parent");

    STATIC_MANAGER.initialize(rustychains_process_stream);

    STATIC_MANAGER
        .write_message(Debug {
            message: "INIT DONE".into(),
        })
        .unwrap();
}
