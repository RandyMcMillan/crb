use ratatui::Frame;
use reqwest::Client;
use reqwest::Url;
use serde_json::{from_str, Value};

pub struct AppState {
    crabs: usize,
	blockheight: String,
}

impl AppState {
    pub fn new() -> Self {
        Self { crabs: 0, blockheight: 0.to_string() }
    }

    pub fn get_blockheight(&mut self) {

    let url = Url::parse("https://mempool.space/api/blocks/tip/height").unwrap();
    let res = reqwest::blocking::get(url).unwrap();

//		let client = Client::new();
//		let url = "https://mempool.space/api/blocks/tip/height";
//		let response = client.get(url).send();
//		if response.status().is_success() {
//			let json_data = response.text();
//			let data: Value = from_str(&json_data).expect("REASON");
//			println!("API response: {}", data);
//    } else {
//		println!("Error: {}", response.status());
//    }
        self.blockheight = format!("{:?}", res);
        //format!("{:?}", res)
    }

    pub fn plus_zero(&mut self) {
        self.crabs = 0;
    }
    pub fn plus_one(&mut self) {
        self.crabs += 1;
    }
    pub fn minus_one(&mut self) {
		if self.crabs >= 1 {
        self.crabs -= 1;
		}
	}

    pub fn render(&self, frame: &mut Frame<'_>) {
        let mut welcome_text = String::from("Hello, CRaBs! ");
        let blockheight = String::from(&self.blockheight);
        for _ in 0..self.crabs {
            welcome_text.push('🦀');
        }
        let text_string = format!("{}:{}", blockheight, welcome_text);
        frame.render_widget(text_string, frame.area());
    }
}
