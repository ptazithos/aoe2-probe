pub struct Source {
    len: usize,
    pub raw: Vec<u8>,
    pub pos: usize,
}

impl Source {
    pub fn new(buffer: Vec<u8>) -> Self {
        Source {
            len: buffer.len(),
            pos: 0,
            raw: buffer,
        }
    }

    pub fn get_vec(&mut self, expect: usize) -> Result<Vec<u8>, String> {
        if expect + self.pos > self.len + 1 {
            return Err(String::from("Request too many data!"));
        }

        let mut buffer = Vec::with_capacity(expect);

        let from = &self.raw[self.pos..(self.pos + expect)];

        buffer.extend_from_slice(from);
        self.pos += expect;

        Ok(buffer)
    }

    pub fn get_rest_vec(&mut self) -> Vec<u8> {
        let rest = self.raw[self.pos..].to_vec();
        self.pos = self.len + 1;
        rest
    }
}
