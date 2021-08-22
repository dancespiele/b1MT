use wasm_bindgen_futures::spawn_local;
use wasm_request::{get_options, request, Method, RequestError};
use yew::agent::AgentLink;
use yewtil::store::{Store, StoreWrapper};

const COINGEKO_URL: &str = "https://api.coingecko.com/api/v3/coins/1million-token";

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Currency {
    pub eur: f64,
    pub usd: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct MarketData {
    pub current_price: Currency,
    pub market_cap: Currency,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct TokenInfo {
    pub market_data: MarketData,
}

pub enum RequestCoingecko {
    Get1MT,
}

pub enum Action {
    Set1MTInfo(TokenInfo),
    SetError(RequestError),
}

#[derive(Clone)]
pub struct TokenInfoStore {
    pub token_info: Option<TokenInfo>,
    pub token_info_error: Option<RequestError>,
}

impl Store for TokenInfoStore {
    type Action = Action;
    type Input = RequestCoingecko;

    fn new() -> Self {
        Self {
            token_info: None,
            token_info_error: None,
        }
    }

    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            RequestCoingecko::Get1MT => {
                let options = get_options::<TokenInfo>(COINGEKO_URL, Method::Get, None, None);

                spawn_local(async move {
                    match request(options).await {
                        Ok(resp) => {
                            let token_info: TokenInfo = resp.into_serde().unwrap();
                            link.send_message(Action::Set1MTInfo(token_info));
                        }
                        Err(e) => link.send_message(Action::SetError(e)),
                    }
                })
            }
        }
    }

    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Action::Set1MTInfo(token_info) => {
                self.token_info = Some(token_info);
            }
            Action::SetError(error) => {
                self.token_info_error = Some(error);
            }
        }
    }
}
