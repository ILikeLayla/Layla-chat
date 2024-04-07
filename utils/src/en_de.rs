use std::time::SystemTime; 
use super::{format_check, exception::*};
use base64::prelude::*;
use std::env;

/// A simple generator and place of storing for key.
struct Key {
    key: String,

    encode_type: EncodeType,
    print_key: String,
    encoded_key: String,
}

impl Key {
    pub fn new(encode_type: EncodeType) -> Self {
        Self {
            key: String::new(),
            print_key: String::new(),
            encoded_key: String::new(),
            encode_type,
        }
    }

    pub fn buffer(encoded: String) -> Self {
        let mut out = Self::new(EncodeType::None);
        out.encoded_key = encoded;
        out
    }

    pub fn print(&self) -> String {
        self.encode_type.encode(self.key.clone(), &self.print_key).to_string()
    }

    pub fn get(&self) -> String { self.key.clone() }

    pub fn cal(&mut self) {
        todo!()
    }

    pub fn decode(&mut self) -> ErrorStruct<()> {
        match self.encode_type.decode(self.encoded_key.clone(), &self.print_key) {
            ErrorStruct::Ok(s) => { self.key = s; ErrorStruct::Ok(()) },
            ErrorStruct::Err(e) => ErrorStruct::Err(e),
        }
    }

    pub fn set(&mut self, key: &str) -> ErrorStruct<()> {
        if key.is_empty() {
            ErrorStruct::default_err(ErrorType::IllegalArgument)
        } else { 
            self.key = key.to_string();
            ErrorStruct::Ok(())
        }
    }

    pub fn set_print_key(&mut self, print_key: &str) -> ErrorStruct<()> {
        if print_key.is_empty() {
            ErrorStruct::default_err(ErrorType::IllegalArgument)
        } else { 
            self.print_key = print_key.to_string();
            ErrorStruct::Ok(())
        }
    }

    pub fn set_encode_type(&mut self, encode_type: EncodeType) {
        self.encode_type = encode_type;
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key<{}>", self.print())
    }
}

impl super::Serializable for Key {
    fn to_string_serial(&self) -> String {
        format!("{}", self)
    }

    fn from_str(s: &str) -> ErrorStruct<Self> where Self: Sized {
        let data = match format_check(s, "Key") {
            ErrorStruct::Ok(d) => d,
            ErrorStruct::Err(e) => return ErrorStruct::Err(e),
        };
        ErrorStruct::Ok(Self::buffer(data.to_string()))
    }
}

pub struct Encoder {
    key: Key,
    encoded_data: Option<String>,
    encode_type: EncodeType
}

impl Encoder {
    pub fn new(encode_type: EncodeType) -> Self {
        Self {
            encoded_data: None,
            key: Key::new(encode_type.clone()), 
            encode_type
        }
    }

    pub fn key_cal(&mut self) {
        self.key.cal();
    }

    fn key_decode(&mut self) -> ErrorStruct<()> {
        match self.setup_key() {
            ErrorStruct::Ok(()) => (),
            ErrorStruct::Err(e) => return ErrorStruct::Err(e),
        }
        self.key.decode()
    }

    pub fn set_key(&mut self, key: &str) -> ErrorStruct<()> {
        self.key.set(key)
    }

    fn setup_key(&mut self) -> ErrorStruct<()> {
        match &self.encoded_data {
            Some(d) => {
                match self.key.set_print_key(d) {
                    ErrorStruct::Ok(()) => (),
                    ErrorStruct::Err(e) => return ErrorStruct::Err(e),
                }
            },
            None => return ErrorStruct::default_err(ErrorType::DataNotFound),
        };
        self.key.set_encode_type(self.encode_type.clone());
        ErrorStruct::Ok(())
    }

    pub fn set_encode_type(&mut self, encode_type: EncodeType) {
        self.encode_type = encode_type;
    }

    pub fn encode(&self, s: String) -> String {
        self.encode_type.encode(s, &self.key.get())
    }

    pub fn decode(&self, s: String) -> ErrorStruct<String> {
        self.encode_type.decode(s, &self.key.get())
    }

    pub fn store_string(&mut self, s: String) -> ErrorStruct<()> {
        let data = self.encode(s);
        self.encoded_data = Some(data);
        self.setup_key()
    }

    pub fn store<T: super::Serializable>(&mut self , obj: &T) {
        let s = obj.to_string_serial();
        let encoded_data = self.encode(s);
        self.key.set_print_key(&encoded_data);
        self.encoded_data = Some(encoded_data);
    }

    pub fn pop_string(&mut self) -> ErrorStruct<String> {
        match self.key_decode() {
            ErrorStruct::Ok(()) => (),
            ErrorStruct::Err(e) => return ErrorStruct::Err(e)
        };
        if let Some(encoded_data) = self.encoded_data.clone() {
            let out = self.decode(encoded_data);
            self.encoded_data = None;
            out
        } else {
            ErrorStruct::default_err(ErrorType::DataNotFound)
        }
    }

    pub fn pop<T: super::Serializable>(&mut self) -> ErrorStruct<T> {
        self.pop_string().ok_work_with_exception(&|s| {T::from_str(s)} )
    }
}

impl std::fmt::Display for Encoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = if self.encoded_data.is_some() {
            if let Some(data) = &self.encoded_data { data.to_string() } else { panic!("Never be here!") }
        } else { String::new() };
        write!(f, "Encoder<{},{},Data<{}>>", self.encode_type, self.key, s)
    }
}

impl super::Serializable for Encoder {
    fn to_string_serial(&self) -> String {
        format!("{}", self)
    }

    fn from_str(s: &str) -> ErrorStruct<Self> where Self: Sized {
        let data = match format_check(s, "Encoder") {
            ErrorStruct::Ok(d) => d,
            ErrorStruct::Err(e) => return ErrorStruct::Err(e),
        };

        let data_list = data.split(",").collect::<Vec<&str>>();

        let encode_type = match EncodeType::from_str(data_list[0]) {
            ErrorStruct::Ok(d) => d,
            ErrorStruct::Err(e) => return ErrorStruct::Err(e),
        };

        let key = match Key::from_str(data_list[1]) {
            ErrorStruct::Ok(d) => d,
            ErrorStruct::Err(e) => return ErrorStruct::Err(e),
        };

        let encoded_data = match format_check(data_list[2], "Data") {
            ErrorStruct::Ok(d) => d.to_string(),
            ErrorStruct::Err(e) => return ErrorStruct::Err(e),
        };

        ErrorStruct::Ok(
            Self {
                encode_type, key,
                encoded_data: Some(encoded_data)
            }
        )
    }
}

#[derive(Copy, Clone)]
pub enum EncodeType {
    None = 0,
    Base64 = 1,
    Caesar = 2,
}

impl EncodeType {
    fn encode(&self, s: String, key: &str) -> String {
        match self {
            Self::Base64 => Base64Encoder::encode(s, key),
            Self::None => NoneEncoder::encode(s, key),
            Self::Caesar => CaesarEncoder::encode(s, key),
        }
    }

    fn decode(&self, s: String, key: &str) -> ErrorStruct<String> {
        match self {
            Self::Base64 => Base64Encoder::decode(s, key),
            Self::None => NoneEncoder::decode(s, key),
            Self::Caesar => CaesarEncoder::decode(s, key),
        }
    }

    fn from_id(id: usize) -> Self {
        match id {
            0 => Self::None,
            1 => Self::Base64,
            2 => Self::Caesar,
            _ => panic!("Invalid id for EncodeType"),
        }
    }
}

impl std::fmt::Display for EncodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.clone() as usize;
        write!(f, "EncodeType<{}>", id)
    }
}

impl super::Serializable for EncodeType {
    fn to_string_serial(&self) -> String {
        format!("{}", self)
    }

    fn from_str(s: &str) -> ErrorStruct<Self> where Self: Sized {
        let id = match format_check(s, "EncodeType") {
            ErrorStruct::Ok(d) => d.parse::<usize>().unwrap(),
            ErrorStruct::Err(e) => return ErrorStruct::Err(e),
        };
        ErrorStruct::Ok(Self::from_id(id))
    }
}

trait EnDe {
    fn encode(s: String, key: &str) -> String;
    fn decode(s: String, key: &str) -> ErrorStruct<String>;
}

struct NoneEncoder;
struct Base64Encoder;
struct CaesarEncoder;

impl EnDe for NoneEncoder {
    fn encode(s: String, _key: &str) -> String {
        s
    }
    fn decode(s: String,  _key: &str) -> ErrorStruct<String> {
        ErrorStruct::Ok(s)
    }
}

impl EnDe for Base64Encoder {
    fn encode(s: String, _key: &str) -> String {
        let mut output_buf = String::new();
        BASE64_STANDARD.encode_string(s, &mut output_buf);
        output_buf
    }
    fn decode(s: String,  _key: &str) -> ErrorStruct<String> {
        let temp = ErrorStruct::from_result_by_debug(BASE64_STANDARD.decode(s));
        let output = temp.ok_work(&|v| {String::from_utf8_lossy(&v).to_string()});
        output
    }
}

impl EnDe for CaesarEncoder {
    fn encode(s: String, key: &str) -> String {
        let mut cipher_text = String::new();
        let mut key = (str_to_num(key) % 26) as u32;
        if key == 0 { key = 13; }
        for c in s.chars() {
            let shifted_char = (c as u32 + key) % 256;
            cipher_text.push(char::from_u32(shifted_char).unwrap());
        }
        cipher_text
    }

    fn decode(s: String, key: &str) -> ErrorStruct<String> {
        let mut plaintext = String::new();
        let mut key = (str_to_num(key) % 26) as u32;
        if key == 0 { key = 13; }
        for c in s.chars() {
            let shifted_char = (c as u32 - key) % 256;
            plaintext.push(char::from_u32(shifted_char).unwrap());
        }
        ErrorStruct::Ok(plaintext)
    }
}

fn str_to_num(s: &str) -> usize {
    let mut buffer = 0;
    for i in s.as_bytes() {
        buffer += *i as usize
    };
    buffer
}