use std::{io, string::FromUtf8Error};

use json_parse::AmneziaJSON;
use serde_json::Error as JsonError;
pub mod json_parse;

/// Перевод base64 в string
pub fn decode_url_base64_str(url_string: String) -> Result<String, FromUtf8Error>{
    use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
    let bytes_url = engine::GeneralPurpose::new(
                 &alphabet::URL_SAFE,
                 general_purpose::NO_PAD)
        .decode(url_string).unwrap();
    String::from_utf8(bytes_url)
}

/// Перевод base64 в byte
pub fn decode_url_base64_byte(url_string: String) -> Result<Vec<u8>, base64::DecodeError> {
    use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
    engine::GeneralPurpose::new(
                 &alphabet::URL_SAFE,
                 general_purpose::NO_PAD).decode(url_string)
}

/// Перевод byte в base64
pub fn encode_url_base64_byte(data: Vec<u8>) -> String{ 
    use base64::{Engine as _, alphabet, engine::{self, general_purpose}};

    engine::GeneralPurpose::new(
        &alphabet::URL_SAFE,
        general_purpose::NO_PAD).encode(data)
}


/// Распаковка zlib
pub fn decode_zlib(data_to_decode: &[u8]) -> io::Result<String>{
    use flate2::read::ZlibDecoder;
    use std::io::prelude::*;

    let mut z = ZlibDecoder::new(&data_to_decode[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}

/// Запаковка zlib
pub fn encode_zlib(data: String) -> Result<Vec<u8>, io::Error> {
    use std::io::prelude::*;
    use flate2::Compression;
    use flate2::write::ZlibEncoder;

    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(data.as_bytes())?;
    e.finish()
}

/// Обрезка текста конфига
pub fn cut_config(data1: String, cut_number: usize) -> String{
    data1[cut_number..data1.len()].to_string()
}

/// Обрезка текста конфига по байтам
pub fn cut_config_byte(data1: &[u8], cut_number: usize) -> &[u8]{
  &data1[cut_number..data1.len()]
}

/// Поиск и исправление информации
fn json_fix(data: String) -> serde_json::Result<String>{
    let mut config_amnezia = json_parse::AmneziaJSON::new_from_str(&data)?;
    config_amnezia.json_fix();
    let new_data = config_amnezia.to_string1()?;
    Ok(new_data)
}

impl AmneziaJSON {
    fn json_fix(&mut self) {
        let route =  self.containers[0].openvpn.last_config
            .split("route ")
            .last().unwrap()
            .split(" 255.255.255.255")
            .nth(0).unwrap()
            .to_string();
        self.containers[0].cloak.last_config.remote_host = route;
    }

    pub fn replace_dns_server(mut self, dns_address: String) {
        self.dns1 = dns_address
    }

    pub fn to_string(&self) -> Result<String, io::Error>{
        let returna_string = serde_json::to_string(&self)?;
        Ok(returna_string)
    }
}

pub fn get_json(data: String) -> Result<AmneziaJSON, JsonError> {
    Ok(json_parse::AmneziaJSON::new_from_str(&data)?)
}

pub fn unpack_config(string_to_fix: String) -> String {
    let mut data1 = string_to_fix.clone();
    if data1.contains("vpn://"){ //Убираем vpn:// из начала файла (На самом деле не только из начала)
        if data1.find("vpn://").unwrap() == 0_usize {
            data1 = data1.replace("vpn://", "")
        }
    }

    if data1.ends_with("\n") { //И убираем перенос из конца файла. Я не знаю зачем он там нужен, но с ним не переводится из base64
        data1 = data1.replace("\n","");
    }

    let data2 = decode_url_base64_byte(data1.to_string()).unwrap();
    let debase64 = cut_config_byte(&data2, 4);
    decode_zlib(debase64).unwrap()
}

pub fn pack(config_to_return: String) -> String{
    let mut encoded_data = encode_zlib(config_to_return).unwrap();
    /* 
        Вот тут началось шапито. Дело в том, что если ты хочешь преобразовать 
        файлик zlib в qcompress, то тебе нужно добавить 4 байта которые
        показывают длину зашифрованного участкка
        Вот тут про это написано https://doc.qt.io/qt-6/qbytearray.html#qUncompress-1
    */
    encoded_data.reverse();
    encoded_data.push(255);
    encoded_data.push(0);
    encoded_data.push(0);
    encoded_data.push(0);
    encoded_data.reverse();
    encode_url_base64_byte(encoded_data)
}

pub fn fixer(string_to_fix: String) -> String {
    let decode64 = unpack_config(string_to_fix);
    let fixed_data = json_fix(decode64).unwrap();
    pack(fixed_data)
} 