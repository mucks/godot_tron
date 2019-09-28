use gdnative as gd;

pub struct Client {
    network: gd::NetworkedMultiplayerENet,
}

unsafe impl Send for Client {}

impl gd::NativeClass for Client {
    type Base = gd::Node;
    type UserData = gd::user_data::MutexData<Client>;
    fn class_name() -> &'static str {
        "Client"
    }

    fn init(owner: Self::Base) -> Self {
        Self::_init(owner)
    }
}

#[gdnative::methods]
impl Client {
    fn _init(_owner: gd::Node) -> Self {
        Client {
            network: gd::NetworkedMultiplayerENet::new(),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, owner: gd::Node) {
        self.network
            .connect(
                "connection_failed".into(),
                owner.cast::<gd::Object>(),
                "_on_connection_failed".into(),
                gd::VariantArray::new(),
                0_i64,
            )
            .unwrap();

        self.connect(owner);
    }

    #[export]
    unsafe fn _on_connection_failed(&self, owner: gd::Node, error: gd::GodotString) {
        godot_print!("hello");
    }

    unsafe fn connect(&mut self, owner: gd::Node) {
        self.network
            .create_client("127.0.0.1".into(), 4242, 100, 100, 4252)
            .unwrap();
        owner
            .get_tree()
            .unwrap()
            .get_multiplayer()
            .unwrap()
            .set_network_peer(self.network.cast::<gd::NetworkedMultiplayerPeer>());

        owner
            .get_tree()
            .unwrap()
            .get_multiplayer()
            .unwrap()
            .connect(
                "network_peer_packet".into(),
                owner.cast::<gd::Object>(),
                "_on_packet_received".into(),
                gd::VariantArray::new(),
                0,
            )
            .unwrap();
    }

    #[export]
    unsafe fn _on_connect_button_pressed(&mut self, owner: gd::Node) {
        godot_print!("connect pressed");
    }

    #[export]
    unsafe fn _on_disconnect_button_pressed(&mut self, owner: gd::Node) {
        self.network.close_connection(0);
        owner.get_tree().unwrap().set_network_peer(None);
    }

    #[export]
    unsafe fn _on_packet_received(&mut self, owner: gd::Node, id: i64, packet: gd::ByteArray) {
        godot_print!("packet received");
    }
}
