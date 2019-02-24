use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};
use serde::ser::{Serialize, Serializer, SerializeMap};
use std::marker::PhantomData; // https://serde.rs/deserialize-map.html
use std::collections::HashMap;
use std::fmt;

struct DataCollector {
    number : f64
}

impl DataCollector {
    fn from<T>(value: T) -> DataCollector
        where f64: std::convert::From<T>
    {
        let value_as_f64 = <f64 as std::convert::From<T>>::from(value);
        println!("DataCollector::from({})", value_as_f64);
        DataCollector {
            number: value_as_f64
        }
    }
}

impl Serialize for DataCollector {
    fn serialize<S: Serializer>(self: &Self, serializer: S) ->
        std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    {
        serializer.serialize_f64(self.number)
    }
}

impl<'de> Deserialize<'de> for DataCollector {
    fn deserialize<D>(deserializer: D) -> Result<DataCollector, D::Error> where D: Deserializer<'de>
    {
        let result = DataCollector { number: 0.0 };
        deserializer.deserialize_any(result)
    }
}

impl<'de> Visitor<'de> for DataCollector {
    type Value = DataCollector;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("data we understand, that is, some number")
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E> where E: de::Error
    { Ok(DataCollector::from(value)) }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E> where E: de::Error
    { Ok(DataCollector::from(value)) }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E> where E: de::Error
    { Ok(DataCollector::from(value)) }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> where E: de::Error
    { Ok(DataCollector::from(value as u32)) }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E> where E: de::Error
    { Ok(DataCollector::from(value)) }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E> where E: de::Error
    { Ok(DataCollector::from(value)) }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E> where E: de::Error
    { Ok(DataCollector::from(value)) }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> where E: de::Error
    { Ok(DataCollector::from(value as i32)) }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E> where E: de::Error
    { Ok(DataCollector::from(value)) }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E> where E: de::Error
    { Ok(DataCollector::from(value)) }
}

struct MyMap
{
    map: HashMap<String, DataCollector>
}

impl MyMap
{
    fn new() -> Self {
        MyMap { map: HashMap::new() }
    }

    fn insert(&mut self, k: String, v: DataCollector) -> Option<DataCollector> {
        self.map.insert(k, v)
    }

    fn len(&self) -> usize {
        self.map.len()
    }
}

impl Serialize for MyMap {
    fn serialize<S: Serializer>(self: &Self, serializer: S) ->
        std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    {
        let mut serialization = serializer.serialize_map(Some(self.map.len()))?;
        for (k, v) in self.map.iter() {
            serialization.serialize_entry(&k, &v)?;
        }
        serialization.end()
    }
}

impl<'de> Visitor<'de> for MyMap {
    type Value = MyMap;

    // Format a message stating what data this Visitor expects to receive.
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a very special map")
    }

    // Deserialize MyMap from an abstract "map" provided by the
    // Deserializer. The MapAccess input is a callback provided by
    // the Deserializer to let us see each entry in the map.
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        println!("visit_map");
        let mut map = MyMap::new();

        // While there are entries remaining in the input, add them
        // into our map.
        while let Some((key, value)) = access.next_entry()? {
            println!("Insert key: {}", key);
            map.insert(key, value);
        }

        println!("map done: {}", map.len());

        Ok(map)
    }
}

impl<'de> Deserialize<'de> for MyMap {
    fn deserialize<D>(deserializer: D) -> Result<MyMap, D::Error> where D: Deserializer<'de>
    {
        let result = MyMap::new();
        deserializer.deserialize_any(result)
    }
}

fn main() {
    println!("helo");
    let stdin = std::io::stdin();
    let map : MyMap = serde_json::from_reader(stdin).unwrap();
    let json : String = serde_json::to_string(&map).unwrap();
    println!("{}", json);
}
