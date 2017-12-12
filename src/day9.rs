use std::str::Chars;

pub struct Group {
    score: u32,
    sub_groups: Vec<Option<Group>>,
}

impl Group {
    fn from_consumer<'a, T>(consumer: &'a mut T, parent_score: u32) -> Option<Group>
        where T: GroupConsumer
     {
        let score = parent_score + 1;
        let mut sub_groups = Vec::new();
        while let Some(value) = consumer.consume_next() {
            match value {
                // If we find the start of a new group, recurse and store the result as a child
                '{' => sub_groups.push(Group::from_consumer(consumer, score)),
                // If we find garbage, consume it
                '<' => consumer.consume_garbage(),
                // If we find an ignored character, consume the ignored data
                '!' => consumer.consume_ignored(),
                // If we find the end of a group, then we're done processing this group, return it
                '}' => return Some(Group { score, sub_groups }),
                // If we find a comma, ignore it
                ',' => {},
                _ => return None
            }
        }
        None
    }

    /// Builds a group from a stream (string). Returns `None` if the stream cannot be parsed
    pub fn from_str(input: &str) -> Option<Group> {
        let mut consumer = input.chars();
        // Consume the first '{'
        if let Some(value) = consumer.consume_next() {
            if value == '{' {
                return Group::from_consumer(&mut consumer, 0);
            }
        } 
        None
    }

    /// Computes the total score of this group + all its sub-groups
    pub fn compute_total_score(&self) -> u32 {
        fn compute_total(group: &Option<Group>) -> u32 {
            if let &Some(ref group) = group { 
                group.compute_total_score() 
            } else { 
                0 
            }
        }
        self.score + self.sub_groups.iter().map(compute_total).sum::<u32>()
    }
}

/// A type which can consume portions of a stream while processing groups
trait GroupConsumer {
    fn consume_next(&mut self) -> Option<char>;
    fn consume_garbage(&mut self);
    fn consume_ignored(&mut self);
}

impl <'a> GroupConsumer for Chars<'a> {
    fn consume_next(&mut self) -> Option<char> {
        self.next()
    }

    fn consume_garbage(&mut self) {
        loop {
            let value = match self.consume_next() {
                None => return,
                Some(value) => value
            };
            match value {
                // If we find an ignore character, consume the ignored data
                '!' => self.consume_ignored(),
                // If we find the end of garbage, return
                '>' => return,
                // anything else, ignore
                _ => {},
            }
        }
    }

    fn consume_ignored(&mut self) {
        self.consume_next();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trivial_group() {
        let input = "{}";
        let group = Group::from_str(input).unwrap();
        assert_eq!(group.compute_total_score(), 1);
    }

    #[test]
    fn test_nested_groups() {
        let mut input = "{{{}}}";
        let mut group = Group::from_str(input).unwrap();
        assert_eq!(group.compute_total_score(), 6);

        input = "{{},{}}";
        group = Group::from_str(input).unwrap();
        assert_eq!(group.compute_total_score(), 5);

        input = "{{{},{},{{}}}}";
        group = Group::from_str(input).unwrap();
        assert_eq!(group.compute_total_score(), 16);
    }

    #[test]
    fn test_groups_with_garbage() {

        let mut input = "{<a>,<a>,<a>,<a>}";
        let mut group = Group::from_str(input).unwrap();
        assert_eq!(group.compute_total_score(), 1);

        input = "{{<ab>},{<ab>},{<ab>},{<ab>}}";
        group = Group::from_str(input).unwrap();
        assert_eq!(group.compute_total_score(), 9);
    }

    #[test]
    fn test_groups_with_ignored() {

        let mut input = "{{<!!>},{<!!>},{<!!>},{<!!>}}";
        let mut group = Group::from_str(input).unwrap();
        assert_eq!(group.compute_total_score(), 9);

        input = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
        group = Group::from_str(input).unwrap();
        assert_eq!(group.compute_total_score(), 3);
    }
}
