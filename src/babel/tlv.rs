pub use super::{hello::Hello, ihu::Ihu, update::Update};

/// A single `Tlv` in a babel packet body.
#[derive(Debug, Clone, PartialEq)]
pub enum Tlv {
    /// Hello Tlv type.
    Hello(Hello),
    /// Hello Tlv type.
    Ihu(Ihu),
    /// Hello Tlv type.
    Update(Update),
}

impl Tlv {
    /// Calculate the size on the wire for this `Tlv`. This DOES NOT included the TLV header size
    /// (2 bytes).
    pub fn wire_size(&self) -> u8 {
        match self {
            Self::Hello(hello) => hello.wire_size(),
            Self::Ihu(ihu) => ihu.wire_size(),
            Self::Update(update) => update.wire_size(),
        }
    }

    /// Encode this `Tlv` as part of a packet.
    pub fn write_bytes(&self, dst: &mut bytes::BytesMut) {
        match self {
            Self::Hello(hello) => hello.write_bytes(dst),
            Self::Ihu(ihu) => ihu.write_bytes(dst),
            Self::Update(update) => update.write_bytes(dst),
        }
    }
}

impl From<Update> for Tlv {
    fn from(v: Update) -> Self {
        Self::Update(v)
    }
}

impl From<Ihu> for Tlv {
    fn from(v: Ihu) -> Self {
        Self::Ihu(v)
    }
}

impl From<Hello> for Tlv {
    fn from(v: Hello) -> Self {
        Self::Hello(v)
    }
}
