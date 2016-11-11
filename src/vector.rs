#![allow(non_snake_case)]

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Elem {
    pub data: i32
}

pub struct Vector {
    elements: [Elem; 100],
    num_elements: usize,
    iter_count: usize
}

impl Vector {
    pub fn new() -> Self {
        Vector {
            elements: [Elem { data: 0 }; 100],
            num_elements: 0,
            iter_count: 0
        }
    }
    pub fn insertAt(&mut self, pos: usize, elem: Elem) -> () {
        for i in (pos..self.n()).rev() {
            self.elements[i+1] = self.elements[i];
        };
        self.elements[pos] = elem;
        self.num_elements += 1;
    }
    pub fn replaceAt(&mut self, pos: usize, elem: Elem) -> () {
        self.elements[pos] = elem;
    }
    pub fn removeAt(&mut self, pos: usize) -> Elem {
        let elem = self.elements[pos];
        for i in pos..self.n() {
            self.elements[i] = self.elements[i+1];
        };
        self.num_elements -= 1;
        elem
    }
    pub fn n(&self) -> usize {
        self.num_elements
    }
    pub fn max_size(&self) -> usize {
        self.elements.len()
    }
    pub fn get_elements(&self) -> [Elem; 100] {
        self.elements
    }
    pub fn at(&self, pos: usize) -> Elem {
        self.elements[pos]
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vector() {
        let mut v = Vector::new();
        let es: [Elem; 3] = [Elem { data: 1 }, Elem { data: 2 }, Elem { data: 3 }];

        for e in 0..es.len() {
            v.insertAt(e, es[e]);
        }

        assert!(v.max_size() == 100);
        assert!(v.n() == 3);

        v.insertAt(3, Elem { data: 4 });


        assert!(v.n() == 4);
        assert!(v.max_size() == 100);

        let x = v.removeAt(0);

        assert!(x.data == 1);
        assert!(v.n() == 3);
        assert!(v.max_size() == 100);

        v.insertAt(0, Elem { data: 5 });

        assert!(v.n() == 4);
        assert!(v.get_elements()[0].data == 5);

    }
}