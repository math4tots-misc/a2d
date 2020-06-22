#[cfg(target_os = "windows")]
macro_rules! sep {
    () => {
        "\\"
    };
}

#[cfg(not(target_os = "windows"))]
macro_rules! sep {
    () => {
        "/"
    };
}

macro_rules! get_bytes {
    ($filename:tt) => {
        include_bytes!(concat!(env!("OUT_DIR"), sep!(), $filename))
    };
}

pub const VERT: &[u8] = get_bytes!("shader.vert.spirv");
pub const FRAG: &[u8] = get_bytes!("shader.frag.spirv");
