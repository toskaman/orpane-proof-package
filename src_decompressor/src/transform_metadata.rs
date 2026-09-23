use super::{decode_leb128, TransformDesc};

pub(super) fn parse(data: &[u8]) -> Result<Vec<TransformDesc>, String> {
    if data.is_empty() { return Ok(Vec::new()); }
    if data[0] != 0 {
        return serde_json::from_slice(data).map_err(|e| format!("Invalid transform JSON: {e}"));
    }
    let count = *data.get(1).ok_or("Truncated compact transform header")?;
    let mut offset = 2;
    let mut result = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let id = *data.get(offset).ok_or("Truncated compact transform list")?;
        offset += 1;
        let name = match id {
            1 => "stage_1",
            2 => "stage_3",
            _ => return Err(format!("Unknown compact transform identifier: {id}")),
        };
        let (stride, next) = decode_leb128(data, offset)?;
        offset = next;
        let mut params = serde_json::json!({"stride": stride});
        if id == 2 {
            let (order, next) = decode_leb128(data, offset)?;
            offset = next;
            params["order"] = order.into();
        }
        result.push(TransformDesc { name: name.into(), params });
    }
    if offset != data.len() { return Err("Trailing compact transform data".into()); }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compact_chain_and_json() {
        let ts = parse(&[0, 2, 1, 4, 2, 8, 2]).unwrap();
        assert_eq!(ts[0].name, "byte_transpose");
        assert_eq!(ts[1].params["stride"], 8);
        assert_eq!(ts[1].params["order"], 2);
        assert!(parse(b"[]").unwrap().is_empty());
    }
    #[test]
    fn malformed_is_never_an_empty_pipeline() {
        for data in [&[0][..], &[0, 1], &[0, 1, 99], &[0, 1, 2, 1],
                     &[0, 0, 1], b"bad json"] {
            assert!(parse(data).is_err());
        }
    }
}
