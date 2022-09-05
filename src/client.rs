pub mod client_data {
    pub struct Client {
        pub state: State,
    }
    #[derive(Debug)]
    pub enum State {
        HandShaking,
        Status,
        Login,
        Play,
    }
}
