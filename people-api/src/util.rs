use anyhow::Result;
use http::HeaderValue;

pub(crate) fn get_id_from_path(header_value: &HeaderValue) -> Result<Option<i32>>{
    match header_value.to_str()?.split("/").last() {
        Some(str) => match str.parse::<i32>() {
            Ok(value) => Ok(Some(value)),
            _ => Ok(None)
        },
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_get_id_from_path() -> Result<()> {
        let id = get_id_from_path(&HeaderValue::from_static("/people/1"))?;
        assert_eq!(id, Some(1));
        Ok(())
    }
    
    #[test]
    fn test_get_no_id_from_path() -> Result<()> {
        let id = get_id_from_path(&HeaderValue::from_static("/people"))?;
        assert_eq!(id, None);
        Ok(())
    }
    
    #[test]
    fn test_get_no_id_from_base_path() -> Result<()> {
        let id = get_id_from_path(&HeaderValue::from_static("/"))?;
        assert_eq!(id, None);
        Ok(())
    }
}