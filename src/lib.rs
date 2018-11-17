extern crate rusoto_core;
extern crate rusoto_dynamodb;
#[macro_use]
extern crate serde_json;

use rusoto_dynamodb::{AttributeValue, QueryInput};
use serde_json::Value;
use std::any::Any;
use std::collections::HashMap;
use std::default::Default;

pub fn gateway_response(body: Option<&str>, status: i16) -> Value {
    match body {
        Some(b) => return json!({"statusCode":status,"body":b,"headers":{"Content-Type":"application/json","Access-Control-Allow-Origin":"*"},}),
        None    => return json!({"statusCode":status,"body":"","headers":{"Content-Type":"application/json","Access-Control-Allow-Origin":"*"},}),
    }
}


// note: Vec<u8> will be interpreted as ns, Box<[u8]> will be interpreted 
// as binary; bool will be interpreted as bool, but Some(bool) will be
// interpreted as null. see tests for more details.
pub fn av_from_val<T: Any>(val: T) -> Option<AttributeValue> {
    let t = &val as &Any;
    
    if let Some(v) = t.downcast_ref::<Box<[u8]>>() {
        return Some(AttributeValue { b: Some(v.to_vec()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<bool>() {
        return Some(AttributeValue { bool: Some(v.to_owned()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<Vec<Vec<u8>>>() {
        return Some(AttributeValue { bs: Some(v.to_owned()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<Vec<AttributeValue>>() {
        return Some(AttributeValue { l: Some(v.to_owned()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<HashMap<String, AttributeValue>>() {
        return Some(AttributeValue { m: Some(v.to_owned()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<i64>() {
        return Some(AttributeValue { n: Some(v.to_string()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<i32>() {
        return Some(AttributeValue { n: Some(v.to_string()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<i16>() {
        return Some(AttributeValue { n: Some(v.to_string()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<i8>() {
        return Some(AttributeValue { n: Some(v.to_string()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<u64>() {
        return Some(AttributeValue { n: Some(v.to_string()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<u32>() {
        return Some(AttributeValue { n: Some(v.to_string()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<u16>() {
        return Some(AttributeValue { n: Some(v.to_string()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<u8>() {
        return Some(AttributeValue { n: Some(v.to_string()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<f64>() {
        return Some(AttributeValue { n: Some(v.to_string()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<f32>() {
        return Some(AttributeValue { n: Some(v.to_string()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<Vec<i64>>() {
        return Some(AttributeValue { ns: Some(v.into_iter().map(|i| i.to_string()).collect()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<Vec<i32>>() {
        return Some(AttributeValue { ns: Some(v.into_iter().map(|i| i.to_string()).collect()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<Vec<i16>>() {
        return Some(AttributeValue { ns: Some(v.into_iter().map(|i| i.to_string()).collect()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<Vec<i8>>() {
        return Some(AttributeValue { ns: Some(v.into_iter().map(|i| i.to_string()).collect()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<Vec<u64>>() {
        return Some(AttributeValue { ns: Some(v.into_iter().map(|i| i.to_string()).collect()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<Vec<u32>>() {
        return Some(AttributeValue { ns: Some(v.into_iter().map(|i| i.to_string()).collect()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<Vec<u16>>() {
        return Some(AttributeValue { ns: Some(v.into_iter().map(|i| i.to_string()).collect()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<Vec<u8>>() {
        return Some(AttributeValue { ns: Some(v.into_iter().map(|i| i.to_string()).collect()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<Vec<f64>>() {
        return Some(AttributeValue { ns: Some(v.into_iter().map(|i| i.to_string()).collect()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<Vec<f32>>() {
        return Some(AttributeValue { ns: Some(v.into_iter().map(|i| i.to_string()).collect()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<Option<bool>>() {
        return Some(AttributeValue { null: v.to_owned(), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<String>() {
        return Some(AttributeValue { s: Some(v.to_owned()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<&str>() {
        return Some(AttributeValue { s: Some(v.to_string()), ..Default::default() });
    }
    else if let Some(v) = t.downcast_ref::<Vec<String>>() {
        return Some(AttributeValue { ss: Some(v.to_owned()), ..Default::default() });
    }
    None
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;

    #[test]
    fn test_gateway() {
        println!("\n******\nbodied:\n{:#}\n******", gateway_response(Some("test"), 200));
        println!("no body:\n{:#}\n******\n", gateway_response(None, 200));
    }
    
    #[test]
    fn test_av_from_val() {
        assert_eq!(AttributeValue { bool: Some(true), ..Default::default() }, av_from_val(true)
            .expect("no bool av"));
        
        let test_num_vec: Vec<u64> = vec!(1, 2, 3);
        
        // had some errors from integer size mismatch
        let any_test = &test_num_vec.clone() as &Any;
        if let Some(_) = any_test.downcast_ref::<Vec<u64>>() {
            println!("\ntest vec is vec<u64>");
        }
        assert_eq!(AttributeValue { ns: Some(vec!("1".to_string(), "2".to_string(), "3".to_string())), ..Default::default() }, av_from_val(test_num_vec.clone())
            .expect("no ns av"));
            
        assert_eq!(AttributeValue { s: Some("test".to_string()), ..Default::default() }, av_from_val("test")
            .expect("no &str av"));
            
        let mut m = HashMap::new();
        m.insert("test key".to_string(), av_from_val(Some(false))
            .expect("no null av"));
        
        assert_eq!(AttributeValue { null: Some(false), ..Default::default() }, av_from_val(Some(false))
            .expect("no null av"));
            
        let ref_m  = AttributeValue { m: Some(m.clone()), ..Default::default() };
        let timer  = Instant::now();
        let test_m = av_from_val(m)
                    .expect("no map av");
        println!("{:#?}", timer.elapsed());
        assert_eq!(ref_m, test_m);
        
        let bin_vec = vec![1u8, 2u8, 3u8];
        let ns_av   = av_from_val(bin_vec.clone())
                        .expect("no u8 ns av");
        let bin_av  = av_from_val(bin_vec.clone().into_boxed_slice())
                        .expect("no bin av");
        assert_eq!(AttributeValue { b: Some(bin_vec), ..Default::default() }, bin_av);
        assert_eq!(AttributeValue { ns: Some(vec!("1".to_string(), "2".to_string(), "3".to_string())), ..Default::default() }, ns_av);
    }
}
