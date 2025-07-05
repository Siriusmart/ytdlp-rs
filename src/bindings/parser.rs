use std::io::{BufRead, BufReader, Read};

/// A function that converts type I to type O
pub struct Parser<I, O>(Box<dyn Fn(I) -> O>);

impl<I: 'static, O: 'static> Parser<I, O> {
    /// Append a processor to the parser
    pub fn map<F, O2>(self, processor: F) -> Parser<I, O2>
    where
        F: Fn(O) -> O2 + 'static,
    {
        Parser(Box::new(move |i: I| processor(self.0(i))))
    }

    /// Pass an input value to the processor
    pub fn pass(&self, input: I) -> O {
        self.0(input)
    }
}

impl<O: 'static> Parser<String, O> {
    /// Read lines into the parser
    pub fn read<R: Read>(&self, reader: R) -> Result<(), std::io::Error> {
        let buf = BufReader::new(reader);

        for line in buf.lines() {
            self.pass(line?);
        }

        Ok(())
    }

    /// Read lines into the parser, collects and returns output
    pub fn read_collect<R: Read>(&self, reader: R) -> Result<Vec<O>, std::io::Error> {
        let buf = BufReader::new(reader);

        let mut collect = Vec::new();

        for line in buf.lines() {
            collect.push(self.pass(line?));
        }

        Ok(collect)
    }
}

/// Parser instantiator
pub struct ParserBuilder;

impl ParserBuilder {
    /// Create a new instance of Parser
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Parser<T, T> {
        Parser(Box::new(|i| i))
    }
}
