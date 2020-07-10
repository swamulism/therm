use std::cmp::Ordering;
// add serde
use std::iter::IntoIterator;

use crate::context::Context;
use crate::dependency::Dependency;

pub(crate) struct RddVals {
    pub id: usize,
    pub dependencies: Vec<dyn Dependency>,
    should_cache: bool,
    pub context: Context,
}

trait RDD<T> {
    
    //Transformations
    fn filter(&self, f: fn(T) -> bool) -> dyn RDD<T>;
    fn map<U>(&self, f: fn(T) -> U) -> dyn RDD<U>;

    //Actions
    fn reduce(f: fn(T, T) -> T) -> T;
    fn save(path: String);

    fn saveAsTextFile(path: String);
    //def map[U: ClassTag](f: T => U): RDD[U]
}

trait RDDPair<K, V>: RDD<(K,V)> {
    fn groupByKey<T>() -> dyn RDDPair<K, dyn T>
    where 
        T: IntoIterator,
        T::Item:V,;
    fn reduceByKey(f: fn(V, V) -> V) -> dyn RDDPair<K, V>;

    fn sort(c: fn(K, K) -> Ordering) -> dyn RDDPair<K, V>;
}
