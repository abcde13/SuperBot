pub struct InternalApi
{
}

pub enum ApiResponse
{
    User(String),
    Continue(),
    Logout(),
}

impl InternalApi
{
    pub fn new(name: String, channel: String, token: String) -> InternalApi
    {
        let api = InternalApi{};
        api
    }

    pub fn recieve_response(&self) -> ApiResponse
    {
        ApiResponse::User("neji49".to_string())
    }

    pub fn send_music(&self, music: String)
    {
        println!("{}", music);
    }
}
