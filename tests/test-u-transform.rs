#[path = "../src/modules/util/transform.rs"]
mod transform;
use transform::*;

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn transform_to_int(){
        let string = "2341".to_string();
        let response = to_int(string);

        assert!(response.is_ok());
        let string2 = "a".to_string();
        let response = to_int(string2);

        assert!(response.is_err());
    }
}