use simplicity::Value;

pub fn value_to_bitstring(value: &Value) -> Vec<bool> {
    let mut bitstring = Vec::new();
    value.do_each_bit(|bit| {
        bitstring.push(bit);
    });
    bitstring
}

pub fn bytes_to_bitstring<A: AsRef<[u8]>>(bytes: A) -> Vec<bool> {
    let mut bitstring = Vec::new();
    for byte in bytes.as_ref() {
        for i in (0..8).rev() {
            let bit = byte & (1 << i) != 0;
            bitstring.push(bit);
        }
    }
    bitstring
}

pub fn fmt_bitstring(bitstring: &[bool]) -> String {
    let mut s = "".to_string();
    for bit in bitstring {
        s.push_str(if !bit { "0" } else { "1" });
    }
    s
}

pub fn parse_bitstring(s: &str) -> Result<Vec<bool>, String> {
    let mut bitstring = Vec::new();
    for c in s.chars() {
        match c {
            '0' => bitstring.push(false),
            '1' => bitstring.push(true),
            _ => return Err(format!("Illegal character: {c}")),
        }
    }
    Ok(bitstring)
}
