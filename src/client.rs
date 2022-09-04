pub mod client_data {
    pub struct Client {
        pub state: State,
    }

    pub enum State {
        HandShaking,
        Status,
        Login,
        Play,
    }
}
