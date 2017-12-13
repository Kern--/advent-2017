use std::num::ParseIntError;

/// A representation of one layer of a firewall
pub struct Layer {
    position: u64,
    range: u64
}

impl Layer {
    pub fn from_str(input: &str) -> Result<Layer, ParseIntError> {
        let layer = input.split(": ").map(|s| s.parse::<u64>()).collect::<Result<Vec<u64>, ParseIntError>>()?;
        Ok(Layer {position: layer[0], range: layer[1]})
    }
}

/// A representation of a firewall
pub struct Firewall {
    layers: Vec<Layer>
}

impl Firewall {
    
    pub fn from_str(input: &str) -> Result<Firewall, ParseIntError> {
        let layers = input.split("\n").map(Layer::from_str).collect::<Result<Vec<Layer>, ParseIntError>>()?;
        Ok(Firewall {layers})
    }

    /// Computes the severity of a packet traversing the firewall with no delays.
    /// 
    /// The key is that for a scanner to go from 0 to the end of it's `range` takes `range - 1` steps = `range - 1` ps.
    ///  to get from `range` back to 0 also takes `range - 1` ps. Therefore, the period of the scanner is `2(range - 1)` ps.
    /// Since the packet makes no delays, it always lands on position `p` at exactly `p`ps. This means that the packet is caught
    ///  if it lands on a firewall layer at position `p` if the layer is at the beginning of it's cycle, i.e. `p % 2(range - 1) == 0`
    pub fn compute_severity(&self) -> u64 {
        (&self.layers).into_iter()
            .filter(|layer| layer.position % (2 * (layer.range - 1)) == 0)
            .map(|layer| layer.position * layer.range)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_firewall() {
        let input = "0: 3\n1: 2\n4: 4\n6: 4";
        let firewall = Firewall::from_str(input).unwrap();
        assert_eq!(firewall.compute_severity(), 24);
    }
}