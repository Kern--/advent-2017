#[derive(Debug)]
pub struct Knot {
    data: Vec<u8>,
    pos: usize,
    skip: usize,
}

/// A data structure for computing the knot tieing hash
impl Knot {
    pub fn new(len: u8) -> Knot {
        let mut data = Vec::new();
        for i in 0u8..len {
            data.push(i);
        }
        data.push(len);
        Knot{data, pos: 0, skip: 0}
    }

    /// Computes a knot hash round based on the input of lengths
    pub fn compute_round(&mut self, lengths: &[u8]) {
        for length in lengths {
            self.tie(*length as usize);
            //println!("{:?}", self.data);
        }

    }

    /// Computes a complete knot hash based on the input of lengths
    pub fn compute_hash(&mut self, lengths: &[u8]) -> [u8;16] {
        let mut dense_hash = [0u8;16];
        let secret = vec![17, 31, 73, 47, 23];
        for _ in 0..64 {
            self.compute_round(lengths);
            self.compute_round(&secret);
        }
        for i in 0..16 {
            for j in 0..16 {
                dense_hash[i] ^= self.data[i*16+j];
            }
        }
        dense_hash
    }

    /// Computes the fingerprint of the knot
    ///  i.e. the first two elements multiplied together.
    pub fn compute_fingerprint(&self) -> u32 {
        self.data[0] as u32 * self.data[1] as u32
    }

    /// Performs 1 tie
    fn tie(&mut self, length: usize) {
        let data_len = self.data.len();
        let mut start = self.pos;
        let mut end = (self.pos + length - 1) % data_len;
        let steps = length / 2;

        for _ in 0..steps {
            let swap = self.data[start];
            self.data[start] = self.data[end];
            self.data[end] = swap;
            start = (start + 1) % data_len;
            end = (data_len + end - 1) % data_len;
        }
        self.pos = (self.pos + length + self.skip) % data_len;
        self.skip += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use util;

    #[test]
    fn test_round() {
        let mut knot = Knot::new(4);
        let lengths = vec![3, 4, 1, 5];
        knot.compute_round(&lengths);
        assert_eq!(knot.compute_fingerprint(), 12);
    }

    #[test]
    fn test_hash() {
        let mut knot = Knot::new(255);
        let mut data = Vec::new();
        assert_eq!(util::to_hex_string(&knot.compute_hash(&data)), "A2582A3A0E66E6E86E3812DCB672A272");

        let mut input = "AoC 2017";
        knot = Knot::new(255);
        data = input.bytes().collect();
        assert_eq!(util::to_hex_string(&knot.compute_hash(&data)), "33EFEB34EA91902BB2F59C9920CAA6CD"); 

        input = "1,2,3";
        knot = Knot::new(255);
        data = input.bytes().collect();
        assert_eq!(util::to_hex_string(&knot.compute_hash(&data)), "3EFBE78A8D82F29979031A4AA0B16A9D"); 

        input = "1,2,4";
        knot = Knot::new(255);
        data = input.bytes().collect();
        assert_eq!(util::to_hex_string(&knot.compute_hash(&data)), "63960835BCDC130F0B66D7FF4F6A5A8E"); 
    }
}
