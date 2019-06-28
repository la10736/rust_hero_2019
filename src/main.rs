
fn main() {
    let mut data = "2216224441".to_owned();
    let steps = 40;

    for _ in 0..steps {
        data = encode(&data);
    }
    println!("New data after '{}' steps = {}", steps, data);
    println!("Len = {}", data.len());
}

struct CounterState {
    char: char,
    occurrences: usize
}

impl CounterState {
    pub fn new(c: char) -> Self {
        Self { char: c, occurrences: 1 }
    }

    fn increment(&mut self) {
        self.occurrences += 1;
    }

    fn same(&self, other: char) -> bool {
        self.char == other
    }
}

struct Encoder<I: Iterator<Item=char>> {
    stream: I,
    state: Option<CounterState>
}

impl<I: Iterator<Item=char>> Encoder<I> {
    pub fn new(stream: I) -> Self {
        Self {stream, state: None }
    }

    fn replace_state(&mut self, c: char) -> Option<CounterState> {
        std::mem::replace(&mut self.state, Some(CounterState::new(c)))
    }
}

impl<I: Iterator<Item=char>> Iterator for Encoder<I> {
    type Item = CounterState;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.stream.next() {
            match self.state {
                None => self.state = Some(CounterState::new(c)),
                Some(ref mut cs) if cs.same(c) => { cs.increment(); },
                _ => return self.replace_state(c)
            }
        };
        std::mem::replace(&mut self.state, None)
    }
}

pub fn encode(input: &str) -> String {
    Encoder::new(input.chars()).
        map(|cs| format!("{}{}", cs.occurrences, cs.char))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest_parametrize;

    #[rstest_parametrize(input, expected,
        case("1", "11"),
        case("2", "12"),
        case("11", "21"),
        case("31", "1311"),
        case("3211", "131221"),
        case("111223", "312213"),
        case("2216224441", "221116223411"),
    )]
    fn standard_cases(input: &str, expected: &str) {
        assert_eq!(&encode(input), expected)
    }
}
