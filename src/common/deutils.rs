use serde::de::{DeserializeOwned, DeserializeSeed, MapAccess, Visitor};
use serde::{de, Deserialize, Deserializer};
use std::collections::BTreeMap;
use std::collections::Bound::Included;
use std::fmt::{Debug, Display, Write};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Bound::Unbounded;
use std::str::FromStr;

pub type Step = u32;

pub(crate) fn de_int_key<'de, D, K, V>(deserializer: D) -> Result<BTreeMap<K, V>, D::Error>
where
    D: Deserializer<'de>,
    K: Eq + Ord + PartialOrd + FromStr,
    K::Err: Display,
    V: Deserialize<'de>,
{
    struct KeySeed<K> {
        k: PhantomData<K>,
    }

    impl<'de, K> DeserializeSeed<'de> for KeySeed<K>
    where
        K: FromStr,
        K::Err: Display,
    {
        type Value = K;

        fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(self)
        }
    }

    impl<'de, K> Visitor<'de> for KeySeed<K>
    where
        K: FromStr,
        K::Err: Display,
    {
        type Value = K;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            K::from_str(string).map_err(de::Error::custom)
        }
    }

    struct MapVisitor<K, V> {
        k: PhantomData<K>,
        v: PhantomData<V>,
    }

    impl<'de, K, V> Visitor<'de> for MapVisitor<K, V>
    where
        K: Eq + Ord + PartialOrd + FromStr,
        K::Err: Display,
        V: Deserialize<'de>,
    {
        type Value = BTreeMap<K, V>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map")
        }

        fn visit_map<A>(self, mut input: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut map = BTreeMap::new();
            while let Some((k, v)) =
                input.next_entry_seed(KeySeed { k: PhantomData }, PhantomData)?
            {
                map.insert(k, v);
            }
            Ok(map)
        }
    }

    deserializer.deserialize_map(MapVisitor {
        k: PhantomData,
        v: PhantomData,
    })
}
