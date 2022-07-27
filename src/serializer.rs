pub struct Serializer {}

impl Serializer {
    pub fn convert_to_bytes<T: Serializable>(target: &T) -> Vec<u8> {
        target.to_bytes()
    }
}

pub trait Serializable {
    fn to_bytes(&self) -> Vec<u8>;
}

impl Serializable for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serializable for u16 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serializable for u32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serializable for i8 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serializable for i16 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serializable for i32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serializable for f32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serializable for f64 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Serializable for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.clone().into_bytes()
    }
}

impl<T: Serializable, const LEN: usize> Serializable for [T; LEN] {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        for number in self {
            buffer.append(&mut number.to_bytes())
        }
        buffer
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        for number in self {
            buffer.append(&mut number.to_bytes())
        }
        buffer
    }
}

pub trait StringPrefix {
    fn to_header(&self) -> Vec<u8>;
}

impl StringPrefix for u8 {
    fn to_header(&self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl StringPrefix for u16 {
    fn to_header(&self) -> Vec<u8> {
        self.to_bytes()
    }
}

impl StringPrefix for u32 {
    fn to_header(&self) -> Vec<u8> {
        self.to_bytes()
    }
}
