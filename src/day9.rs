use std::str::Chars;

pub struct Group {
    score: u32,
    sub_groups: Vec<Option<Group>>,
    garbage_count: u32,
}

impl Group {
    fn from_consumer<'a, T>(consumer: &'a mut T, parent_score: u32) -> Option<Group>
        where T: GroupConsumer
     {
        let score = parent_score + 1;
        let mut sub_groups = Vec::new();
        let mut garbage_count = 0;
        while let Some(value) = consumer.consume_next() {
            match value {
                // If we find the start of a new group, recurse and store the result as a child
                '{' => sub_groups.push(Group::from_consumer(consumer, score)),
                // If we find garbage, consume it
                '<' => { garbage_count += consumer.consume_garbage() },
                // If we find an ignored character, consume the ignored data
                '!' => consumer.consume_ignored(),
                // If we find the end of a group, then we're done processing this group, return it
                '}' => return Some(Group { score, sub_groups, garbage_count }),
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

    /// Computes the total amount of garbage in this group and all its sub-groups
    pub fn compute_total_garbage(&self) -> u32 {
        fn compute_total(group: &Option<Group>) -> u32 {
            if let &Some(ref group) = group { 
                group.compute_total_garbage() 
            } else { 
                0 
            }
        }
        self.garbage_count + self.sub_groups.iter().map(compute_total).sum::<u32>()
    }
}

/// A type which can consume portions of a stream while processing groups
trait GroupConsumer {
    fn consume_next(&mut self) -> Option<char>;
    /// Consumes garbage and returns the number of garbage characters returned
    fn consume_garbage(&mut self) -> u32;
    fn consume_ignored(&mut self);
}

impl <'a> GroupConsumer for Chars<'a> {
    fn consume_next(&mut self) -> Option<char> {
        self.next()
    }

    fn consume_garbage(&mut self) -> u32 {
        let mut garbage_count = 0;
        while let Some(value) = self.consume_next() {
            match value {
                // If we find an ignore character, consume the ignored data
                '!' => { self.consume_ignored(); continue; },
                // If we find the end of garbage, return
                '>' => return garbage_count,
                // anything else, ignore
                _ => {},
            }
            garbage_count += 1;
        }
        garbage_count
    }

    fn consume_ignored(&mut self) {
        self.consume_next();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_consume_garbage() {
        // Consume garbage would normally start after the '<'
        //  so we remove it here to simulate how it would actually be called

        let mut input = ">";
        assert_eq!(input.chars().consume_garbage(), 0);
        
        input = "random characters>";
        assert_eq!(input.chars().consume_garbage(), 17);

        input = "<<<>";
        assert_eq!(input.chars().consume_garbage(), 3);

        input = "{!>}>";
        assert_eq!(input.chars().consume_garbage(), 2);

        input = "!!>";
        assert_eq!(input.chars().consume_garbage(), 0);

        input = "!!!>>";
        assert_eq!(input.chars().consume_garbage(), 0);

        input = "{o\"i!a,<{i<a>";
        assert_eq!(input.chars().consume_garbage(), 10);
    }

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
