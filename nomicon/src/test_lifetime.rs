#[cfg(test)]
mod test {
  use std::collections::HashMap;
  use std::hash::Hash;

    #[derive(Debug)]
    struct Foo;

    impl Foo {
        fn mutate_and_share(&mut self) -> &Self {
            &*self
        }
        fn share(&self) {}
    }

    #[cfg(disable)]
    fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m V
    where
        K: Clone + Eq + Hash,
        V: Default,
    {
        let v = map.get(&key);

        if let Some(v) = v {
            v
        }
        else {
          map.insert(key.clone(), V::default());
          map.get_mut(&key).unwrap()
        } 
    }

    #[test]
    fn test_lifetime() {
        let mut foo = Foo;
        foo.share();
        let loan = foo.mutate_and_share();
        println!("{:?}", loan);
    }

    #[test]
    fn test_get_default() {
      let mut kv = HashMap::<&str, String>::new();
      // 
      // let result = get_default(&mut kv, "hello");
    }
}
