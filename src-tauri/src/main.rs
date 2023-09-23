// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    collections::{BTreeMap, HashMap},
    sync::{Arc, Mutex},
};

use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use tungstenite::{connect as websocket_connect, Message};

const F64_MULT: f64 = 1_00_000_000f64;

#[tauri::command]
fn get_latest_info(state: tauri::State<Arc<Mutex<BookState>>>, depth: usize) -> BookStateOutput {
    let state_lock = state.lock().unwrap();
    let result = state_lock.clone_to_output(depth);
    return result;
}

#[tauri::command]
fn get_latest_trade(state: tauri::State<Arc<Mutex<BookState>>>) -> Vec<(u64, f64, f64)> {
    let mut state_lock = state.lock().unwrap();
    return state_lock
        .trades
        .drain(..)
        .map(|(a, b, c)| (a, b as f64 / F64_MULT, c as f64 / F64_MULT))
        .collect();
}

/// A WebSocket echo server
fn start_binance_ws(state: Arc<Mutex<BookState>>) -> Result<(), Box<dyn std::error::Error>> {
    let message = WsMessage {
        method: WsMessageMethod::Subscribe,
        params: vec![
            "btcusdt@bookTicker".to_string(),
            "btcusdt@depth".to_string(),
            // "btcusdt@aggTrade".to_string(),
            "btcusdt@trade".to_string(),
        ],
        id: rand::thread_rng().gen::<u64>(),
    };
    let message_bytes = serde_json::to_string(&message)?;
    println!("sending to binance: {}", message_bytes);

    let (mut socket, _) =
        websocket_connect(url::Url::parse("wss://stream.binance.com:9443/stream").unwrap())?;

    socket.send(Message::Text(message_bytes))?;

    loop {
        let msg = socket.read()?;
        match msg {
            tungstenite::Message::Text(text) => {
                let mut state_lock = state.lock().unwrap();
                if let Ok(msg_json) = serde_json::from_str::<WsStreamMessage>(&text) {
                    match msg_json.data {
                        WsStreamMesssageData::BookTicker(bt) => {
                            state_lock.insert_price_level(
                                msg_json.stream.clone(),
                                true,
                                bt.best_bid_price,
                                bt.best_bid_quantity,
                                bt.update_id,
                                bt.update_id,
                                true,
                            );
                            state_lock.insert_price_level(
                                msg_json.stream,
                                false,
                                bt.best_ask_price,
                                bt.best_ask_quantity,
                                bt.update_id,
                                bt.update_id,
                                true,
                            );
                        }
                        WsStreamMesssageData::Depth(d) => {
                            for PqPair(price, quantity) in d.bids {
                                state_lock.insert_price_level(
                                    msg_json.stream.clone(),
                                    true,
                                    price,
                                    quantity,
                                    d.first_update_id,
                                    d.last_update_id,
                                    false,
                                )
                            }
                            for PqPair(price, quantity) in d.asks {
                                state_lock.insert_price_level(
                                    msg_json.stream.clone(),
                                    false,
                                    price,
                                    quantity,
                                    d.first_update_id,
                                    d.last_update_id,
                                    false,
                                )
                            }
                        }
                        WsStreamMesssageData::Trade(t) => {
                            state_lock.insert_trade(t.trade_time, t.price, t.quantity)
                        }
                    }
                } else {
                    println!("Unable to deserialize:{}", text);
                }
            }
            Message::Ping(_) => {
                println!("sending pong");
                socket.send(Message::Pong(vec![]))?;
            }
            Message::Pong(_) => {
                println!("received pong");
            }
            _ => {
                println!("ignoring: {:#?}", msg);
            }
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BookState {
    bids: BTreeMap<u64, (u64, u64, u64)>,
    asks: BTreeMap<u64, (u64, u64, u64)>,
    depth_stream_state: HashMap<
        String,
        (
            BTreeMap<u64, (u64, u64, u64)>,
            BTreeMap<u64, (u64, u64, u64)>,
        ),
    >,
    trade_stream_state: HashMap<String, Vec<PqPair>>,
    trades: Vec<(u64, u64, u64)>,
}

impl BookState {
    pub fn insert_trade(&mut self, timestamp: u64, price: u64, quantity: u64) {
        let should_insert = match self.trades.last() {
            Some((old_ts, p, _)) => {
                (timestamp - old_ts > 5_00) || (price.abs_diff(*p) >= 10_000_000)
            }
            None => true,
        };
        if should_insert {
            self.trades.push((timestamp, price, quantity))
        }
    }

    pub fn insert_price_level(
        &mut self,
        stream_name: String,
        is_bid: bool,
        price: u64,
        quantity: u64,
        min_seq_id: u64,
        max_seq_id: u64,
        uncross: bool,
    ) {
        if is_bid {
            if quantity == 0 {
                self.bids.remove(&price);
            } else {
                self.bids.insert(price, (quantity, min_seq_id, max_seq_id));

                if uncross {
                    self.bids.retain(|k, _| *k <= price);
                    self.asks.retain(|k, _| *k > price);
                }
            }
        } else {
            if quantity == 0 {
                self.asks.remove(&price);
            } else {
                self.asks.insert(price, (quantity, min_seq_id, max_seq_id));

                if uncross {
                    self.bids.retain(|k, _| *k < price);
                    self.asks.retain(|k, _| *k >= price);
                }
            }
        }

        if !self.depth_stream_state.contains_key(&stream_name) {
            self.depth_stream_state.insert(
                stream_name.clone(),
                (BTreeMap::default(), BTreeMap::default()),
            );
        }

        let (depth_bids, depth_asks) = self.depth_stream_state.get_mut(&stream_name).unwrap();

        if is_bid {
            if quantity == 0 {
                depth_bids.remove(&price);
            } else {
                depth_bids.insert(price, (quantity, min_seq_id, max_seq_id));

                if uncross {
                    depth_bids.retain(|k, _| *k <= price);
                    depth_asks.retain(|k, _| *k > price);
                }
            }
        } else {
            if quantity == 0 {
                depth_asks.remove(&price);
            } else {
                depth_asks.insert(price, (quantity, min_seq_id, max_seq_id));

                if uncross {
                    depth_bids.retain(|k, _| *k < price);
                    depth_asks.retain(|k, _| *k >= price);
                }
            }
        }
    }

    pub fn clone_to_output(&self, depth: usize) -> BookStateOutput {
        let depth_stream_state = self
            .depth_stream_state
            .iter()
            .map(|(k, v)| {
                let bid_bbo = get_bbo(v.0.iter(), true, depth);
                let ask_bbo = get_bbo(v.1.iter(), false, depth);
                return (k.clone(), (bid_bbo, ask_bbo));
            })
            .collect::<HashMap<_, _>>();

        BookStateOutput {
            symbol: "btcusdt".to_string(),
            bids: get_bbo(self.bids.iter(), true, depth),
            asks: get_bbo(self.asks.iter(), false, depth),
            depth_stream_state,
        }
    }
}

fn get_bbo<
    'a,
    I: Iterator<Item = (&'a u64, &'a (u64, u64, u64))> + DoubleEndedIterator + ExactSizeIterator,
>(
    arr: I,
    is_bid: bool,
    amount: usize,
) -> Vec<PqData> {
    if is_bid {
        // We want the highest prices
        let best_bids = arr
            .rev()
            .take(amount)
            .map(|(p, q)| (*p, *q))
            .collect::<Vec<_>>();
        let max_quantity = best_bids
            .iter()
            .map(|(_, q)| q)
            .max()
            .map(|(v, _, _)| *v as f64 / F64_MULT);
        return best_bids
            .iter()
            .map(|(k, (a, b, c))| PqPair(*k, *a).to_pqdata(max_quantity, *b, *c))
            .collect::<Vec<_>>();
    } else {
        // We want the lowest prices
        let best_asks = arr
            .take(amount)
            .rev()
            .map(|(p, q)| (*p, *q))
            .collect::<Vec<_>>();
        let max_quantity = best_asks
            .iter()
            .map(|(_, q)| q)
            .max()
            .map(|(v, _, _)| *v as f64 / F64_MULT);
        return best_asks
            .iter()
            .map(|(k, (a, b, c))| PqPair(*k, *a).to_pqdata(max_quantity, *b, *c))
            .collect::<Vec<_>>();
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct BookStateOutput {
    symbol: String,
    bids: Vec<PqData>,
    asks: Vec<PqData>,
    #[serde(rename = "depthStream")]
    depth_stream_state: HashMap<String, (Vec<PqData>, Vec<PqData>)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsStreamMessage {
    pub stream: String,
    pub data: WsStreamMesssageData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WsStreamMesssageData {
    BookTicker(BookTicker),
    Depth(Depth),
    Trade(Trade),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqPair(
    #[serde(with = "string_or_float")] u64,
    #[serde(with = "string_or_float")] u64,
);

impl PqPair {
    fn to_pqdata(&self, max_value: Option<f64>, min_seq_id: u64, max_seq_id: u64) -> PqData {
        let p = self.0 as f64 / F64_MULT;
        let q = self.1 as f64 / F64_MULT;
        match max_value {
            Some(max_value) if max_value != 0.0f64 => PqData {
                p,
                q,
                min_seq_id,
                max_seq_id,
                min_color: 0.0f64,
                max_color: 100.0f64 * (q / max_value),
            },
            _ => PqData {
                p,
                q,
                min_seq_id,
                max_seq_id,
                min_color: 0.0f64,
                max_color: 100.0f64,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqData {
    p: f64,
    q: f64,
    min_seq_id: u64,
    max_seq_id: u64,
    min_color: f64,
    max_color: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Depth {
    // #[serde(rename = "e")]
    // E: String,
    #[serde(rename = "E")]
    event_time: u64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "U")]
    first_update_id: u64,
    #[serde(rename = "u")]
    last_update_id: u64,
    #[serde(rename = "b")]
    bids: Vec<PqPair>,
    #[serde(rename = "a")]
    asks: Vec<PqPair>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    // #[serde(rename = "e")]
    // E: String,
    #[serde(rename = "E")]
    event_time: u64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "t")]
    trade_id: u64,
    #[serde(rename = "p")]
    #[serde(with = "string_or_float")]
    price: u64,
    #[serde(rename = "q")]
    #[serde(with = "string_or_float")]
    quantity: u64,
    #[serde(rename = "b")]
    bid_id: u64,
    #[serde(rename = "a")]
    ask_id: u64,
    #[serde(rename = "T")]
    trade_time: u64,
    #[serde(rename = "m")]
    market_maker: bool,
    // #[serde(rename = "M")]
    // M: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookTicker {
    #[serde(rename = "u")]
    update_id: u64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "b")]
    #[serde(with = "string_or_float")]
    best_bid_price: u64,
    #[serde(rename = "B")]
    #[serde(with = "string_or_float")]
    best_bid_quantity: u64,
    #[serde(rename = "a")]
    #[serde(with = "string_or_float")]
    best_ask_price: u64,
    #[serde(rename = "A")]
    #[serde(with = "string_or_float")]
    best_ask_quantity: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WsMessageMethod {
    #[serde(rename = "SUBSCRIBE")]
    Subscribe,
    #[serde(rename = "UNSUBSCRIBE")]
    Unsubscribe,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsMessage {
    method: WsMessageMethod,
    params: Vec<String>,
    id: u64,
}

fn main() {
    env_logger::init();

    let state = Arc::new(Mutex::new(BookState::default()));

    let state_clone = state.clone();
    std::thread::spawn(move || loop {
        let state_clone = state_clone.clone();
        if let Err(err) = start_binance_ws(state_clone) {
            eprintln!("{:#?}", err);
        }
    });
    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![get_latest_info, get_latest_trade])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub(crate) mod string_or_float {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    use crate::F64_MULT;

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        let value_f64 = match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse::<f64>().map_err(de::Error::custom)?,
            StringOrFloat::Float(i) => i,
        };

        return Ok((value_f64 * F64_MULT) as u64);
    }
}
