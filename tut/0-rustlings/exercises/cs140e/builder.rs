// FIXME: Make me pass! Diff budget: 30 lines.

#[derive(Default)]
struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    fn string<T: AsRef<str>>(&mut self, string: T) -> &mut Self {
        self.string = Option::Some(String::from(string.as_ref()));
        self
    }

    fn number(&mut self, number: usize) -> &mut Self {
        self.number = Option::Some(number);
        self
    }
}

impl ToString for Builder {
    fn to_string(&self) -> String {
        String::from(format!(
            "{}{}{}",
            match &self.string {
                Some(s) => s.clone(),
                None => String::from(""),
            },
            match (&self.string, &self.number) {
                (Some(_), Some(_)) => String::from(" "),
                _ => String::from(""),
            },
            match &self.number {
                Some(n) => n.to_string(),
                None => String::from(""),
            },
        ))
    }
}

// Do not modify this function.
#[test]
fn builder() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default().string("heap!".to_owned()).to_string();

    assert_eq!(c, "heap!");
}
