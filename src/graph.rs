#![allow(non_snake_case)]

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Hash, Eq, PartialEq, Debug)]
struct Vertex<T> {
    id: String,
    elem: T
}

impl<T> Vertex<T> {
    pub fn new(id: String, elem: T) -> Self {
        Vertex {
            id: id,
            elem: elem
        }
    }
}

type Edge<T> = (Vertex<T>, Vertex<T>);

pub struct Graph<T> {
    adj_map: HashMap<Vertex<T>, (Vec<Vertex<T>>, Vec<Vertex<T>>)>
}

impl<T> Graph<T> {
    pub fn new() -> Self 
        where T: Hash + Eq
    {
        Graph {
            adj_map: HashMap::new()
        }
    }

    fn vertices(&self) -> &[Vertex<T>] {
        self.adj_map.keys()
    }

    fn in_degrees(&self, v: Vertex<T>) -> i32 {
        self.in_incident_edges.count()
    }

    fn out_degrees(&self, v: Vertex<T>) -> i32 {
        self.out_incident_edges.count()
    }

    fn in_incident_edges(&self, v: Vertex<T>) -> &[Edge<T>] {
        self.adj_map[v].0.map(|x| (x, v))
    }

    fn out_incident_edges(&self, v: Vertex<T>) -> &[Edge<T>] {
        self.adj_map[v].1.map(|x| (v, x))
    }

    fn in_adjacent_vertices(&self, v: Vertex<T>) -> &[Vertex<T>] {
        self.adj_map[v].0
    }

    fn out_adjacent_vertices(&self, v: Vertex<T>) -> &[Vertex<T>] {
        self.adj_map[v].1
    }

    fn remove_edge(&mut self, e: Edge<T>) -> Option<Edge<T>> {
        let (v, w) = e;
        adj_map.entry(v).0.remove(w);
        adj_map.entry(w).1.remove(v);
        Some(e)
    }

    fn insert_directed_edge(&mut self, v: Vertex<T>, w: Vertex<T>) -> Option<Edge<T>> {
        adj_map.entry(v).0.push(w);
        adj_map.entry(w).1.push(v);
        Some((v, w))
    }    

    fn insert_vertex(&mut self, o: T) -> Option<Vertex<T>> {
        let v = Vertex::new("foo", o);
        self.adj_map.insert(v, (vec![], vec![]));
        Some(v)
    }

    fn remove_vertex(&mut self, v: Vertex<T>) -> Option<Vertex<T>> {
        self.adj_map.remove(v);
        Some(v)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

}