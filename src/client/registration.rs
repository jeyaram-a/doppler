pub struct Registration<'a> {
    pub id: &'a str,
    // waits until x mins. Post which the client is considered dead
    wait_until_sec: i32,
    active: bool,
}

impl<'a> Registration<'a> {
    pub fn new() -> Registration<'a> {
        return Registration {
            id: "hello",
            wait_until_sec: 1,
            active: false,
        };
    }
}
