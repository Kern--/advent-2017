use regex::Regex;
use std::u64;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
/// Represents a particle in space with a position, velocity, and acceleration
struct Particle {
    id: usize,
    position: [i64;3],
    velocity: [i64;3],
    acceleration: [i64;3],
}

impl Particle {
    pub fn parse(input: &str, id: usize) -> Option<Particle> {
        lazy_static! {
            /// matches p=<#, #, #>, v=<#, #, #>, a=<#, #, #>
            ///  where # is any integere number (positive or negative)
            static ref REGEX: Regex = Regex::new(r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();
        }
        let captures = REGEX.captures(input);
        if let Some(captures) = captures {
            let mut position = [0i64;3];
            let mut velocity = [0i64;3];
            let mut acceleration = [0i64;3];

            for i in 0..3 {
                position[i] = captures[1 + i].parse::<i64>().unwrap();
                velocity[i] = captures[4 + i].parse::<i64>().unwrap();
                acceleration[i] = captures[7 + i].parse::<i64>().unwrap();
            }

            return Some(Particle{id, position, velocity, acceleration});
        }
        None
    }

    /// Simulates `n` ticks.
    /// Returns the index of the particle which stays closes to the origin
    pub fn simulate(&self, n: u64) -> u64  {
        let mut position = [0u64;3];
        for i in 0..3 {
            position[i] = self.position[i].abs() as u64 
                + self.velocity[i].abs() as u64 
                + n * self.acceleration[i].abs() as u64;
        }
        position[0] + position[1] + position[2]
    }

    /// Updates the velocity and position of the particle
    pub fn tick(&mut self) {
        for i in 0..3 {
            self.velocity[i] += self.acceleration[i];
            self.position[i] += self.velocity[i];
        }
    }
}

/// Parses a string as a series of particles and determines which stays closes to the origin
/// returns the index of the particle which stays closes to the origin in the long term
pub fn simulate(input: &str) -> usize {
    let particles = input.split("\n").enumerate()
        .map(|(id, unparsed)| Particle::parse(unparsed, id))
        .collect::<Option<Vec<Particle>>>();
    if let Some(particles) = particles {
        let (index, _) = particles.iter().map(|particle| particle.simulate(1_000_000)) // There's nothing special about 1,000,000. just a guess
            .enumerate().fold((0, u64::MAX), 
                // This could use ID, but it was implemented before particles had IDs
                //  and that change seems to trivial to be worth it.
                |(min_i, min_val), (i, value)| {
                    if value < min_val {
                        (i, value)
                    } else {
                        (min_i, min_val)
                    }
                });
        return index;
    }
    0
}

/// Parses a string as a series of particles and determines how many particles do not collide
pub fn simulate_with_collision(input: &str) -> usize {
    // This is probably too slow. Avoiding the clone would make it better
    let particles = input.split("\n").enumerate()
        .map(|(id, unparsed)| Particle::parse(unparsed, id))
        .collect::<Option<Vec<Particle>>>();
    if let Some(mut particles) = particles {
        // 10,000 seems pretty big. I should have a reason for this
        for _ in 0..10_000 {
            let mut positions = HashMap:: <[i64;3], Vec<Particle>>::new();
            for mut particle in particles {
                particle.tick();
                let ps = positions.entry(particle.position.clone()).or_insert(Vec::new());
                ps.push(particle);
            }
            particles = positions.values()
                .filter(|vec| vec.len() == 1)
                .flat_map(|x| x.iter())
                .map(|x| x.clone()) // I don't like that I have to clone here, but I'm not sure how to get values with ownership :/
                .collect::<Vec<Particle>>();
        }
        return particles.len();  
    }
    0
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>";
        assert_eq!(Particle::parse(input), Some(Particle {position: [3,0,0], velocity: [2,0, 0], acceleration: [-1,0,0]}));
    }

    #[test]
    fn test_simulation() {
        let input = "p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>\np=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>";
        assert_eq!(simulate(input), 0);
    }
}