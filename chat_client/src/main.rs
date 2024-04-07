use utils::en_de;
use utils::Serializable;

fn main() {
    utils::init();

    let data = "ILikeLayla".to_string();
    let key = "ILikeLaylaForever";
    let encode_type = en_de::EncodeType::Caesar;

    let mut encoder = en_de::Encoder::new(encode_type);
    encoder.set_key(key);
    encoder.store_string(data);
    println!("{}", encoder);

    let serial_encoder = encoder.to_string_serial();
    let mut encoder_new = en_de::Encoder::from_str(&serial_encoder).unwrap();
    println!("{}", encoder_new);

    // encoder_new.key_decode();
    // println!("{}", encoder_new);

    println!("{:?}", encoder_new.pop_string());
    println!("{:?}", encoder.pop_string())
}
