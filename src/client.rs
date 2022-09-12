pub mod client_data {
    use crate::authentication::Profile;

    pub struct Client {
        pub state: State,
        pub username: Option<String>,
        pub profile: Option<Profile>,
        pub uuid: Option<String>,
        pub gamemode: Option<u8>,
        pub previous_gamemode: Option<u8>,
    }

    impl Client {
        pub fn new() -> Client {
            Client {
                state: State::HandShaking,
                username: None,
                profile: None,
                uuid: None,
                gamemode: None,
                previous_gamemode: None,
            }
        }
    }

    #[derive(Debug)]
    pub enum State {
        HandShaking,
        Status,
        Login,
        Play,
    }
}
