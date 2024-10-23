

pub mod proto {
    pub mod cf3d_v1 {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/spec/proto/cf3d.v1.rs"));
    }
}