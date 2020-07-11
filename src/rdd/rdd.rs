use std::cmp::Ordering;
// add serde
use std::iter::IntoIterator;

use crate::context::Context;
use crate::dependency::Dependency;

pub(crate) struct RddVals {
    pub id: usize,
    // pub dependencies: Vec<dyn Dependency>,
    should_cache: bool,
    pub context: Context,
}

trait RDD<T> {
    type Item;
    //Transformations
    fn filter(&self, f: fn(T) -> bool) -> Box<dyn RDD<T, Item=Self::Item>>;
    //fn map<T,U>(&self, f: fn(T) -> U) -> Box<dyn RDD>;

    //Actions
    fn reduce(&self, f: fn(T, T) -> T) -> T;
    fn save(&self, path: String);

    fn saveAsTextFile(&self, path: String);
    //def map[U: ClassTag](f: T => U): RDD[U]
}

trait RDDPair<K, V>: RDD<(K, V)> {
    fn groupByKey(&self) -> dyn RDD<K, Item=Self::Item>;
    // where 
        // T: IntoIterator<Item=V>,;
    fn reduceByKey(&self, f: fn(V, V) -> V) -> dyn RDDPair<K,V, Item=Self::Item>;

    fn sort(&self, c: fn(K, K) -> Ordering) -> dyn RDDPair<K,V, Item=Self::Item>;
}
