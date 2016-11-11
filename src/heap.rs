#![allow(non_snake_case)]

use vector::Vector;
use vector::Elem;

pub struct Heap {
    vector: Vector
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            vector: Vector::new()
        }
    }
    fn _up_heap(&mut self, pos: usize) -> () {
        let parent_pos = self.parent_index(pos);
        if pos > 0 && self.vector.at(parent_pos).data > self.vector.at(pos).data {
            self._swap(parent_pos, pos);
            self._up_heap(parent_pos);
        }
    }
    fn _down_heap(&mut self, pos: usize) -> () {
        let left_child_pos = self.left_child_index(pos);
        let right_child_pos = self.right_child_index(pos);
        let mut smallest_child_pos = 0;

        if self.vector.at(left_child_pos).data == 0 && self.vector.at(right_child_pos).data != 0 {
            smallest_child_pos = right_child_pos;
        }
        else if self.vector.at(left_child_pos).data != 0 && self.vector.at(right_child_pos).data == 0 {
            smallest_child_pos = left_child_pos;
        }
        else if self.vector.at(left_child_pos).data != 0 && self.vector.at(right_child_pos).data != 0 {
            smallest_child_pos = if self.vector.at(left_child_pos).data < self.vector.at(right_child_pos).data { left_child_pos } else { right_child_pos };
        }
        if smallest_child_pos > 0 && self.vector.at(smallest_child_pos).data < self.vector.at(pos).data {
            self._swap(smallest_child_pos, pos);
            self._down_heap(smallest_child_pos);
        }         
    }    
    fn _swap(&mut self, pos1: usize, pos2: usize) -> () {
        let temp = self.vector.at(pos1);
        let temp2 = self.vector.at(pos2);
        self.vector.replaceAt(pos1, temp2);
        self.vector.replaceAt(pos2, temp);
    }
    pub fn height(&self) -> usize {
        let l = self.vector.n();
        match l {
	        0 => f64::from(0 as u32) as usize,
            1 => f64::from(0 as u32) as usize,
            _ => f64::from((l - (l % 2)) as u32).log2() as usize
        }
    }
    pub fn parent_index(&mut self, pos: usize) -> usize {
        match pos {
            0 => 0,
            _ => pos - f32::from(pos as f32 / 2.0).floor() as usize - 1
        }
    }
    fn _child_index(&mut self, pos: usize) -> usize {
        pos + f32::from(pos as f32 / 2.0).floor() as usize
    }
    pub fn left_child_index(&mut self, pos: usize) -> usize {
        self._child_index(pos) + 1
    }
    pub fn right_child_index(&mut self, pos: usize) -> usize {
        self._child_index(pos) + 2
    }    
    pub fn insert(&mut self, elem: Elem) -> () {
        let length = self.vector.n();
        self.vector.insertAt(length, elem);
        self._up_heap(length);
    }
    pub fn remove(&mut self) -> Elem {
        let e = self.vector.at(0);
        let n = self.vector.n()-1;
        let r = self.vector.at(n);
        self.vector.replaceAt(0, r);
        self.vector.removeAt(n);
        self._down_heap(0);
        e
    }
    pub fn top(&self) -> Elem {
        self.vector.get_elements()[0]
    }
}

/*

0
| \
1  2
|\  \ \
3 4  5 6
|
7

*/

#[cfg(test)]
mod tests {

    use super::*;
    use vector::Elem;

    #[test]
    fn test_heap_height() {
        let mut h = Heap::new();

        assert!(h.height() == 0);

        h.insert(Elem { data: 1 });
        assert!(h.height() == 0);


        h.insert(Elem { data: 2 });
        assert!(h.height() == 1);

        h.insert(Elem { data: 3 });
        assert!(h.height() == 1);

        h.insert(Elem { data: 4 });
        assert!(h.height() == 2);
    }

    #[test]
    fn test_heap_get_parent() {
    
        let mut h = Heap::new();

        h.insert(Elem { data: 1 });
        h.insert(Elem { data: 1 });
        h.insert(Elem { data: 1 });
        h.insert(Elem { data: 1 });

        assert!(h.parent_index(3) == 1);

        h.insert(Elem { data: 1 });

        assert!(h.parent_index(4) == 1);
        
        h.insert(Elem { data: 1 });

        assert!(h.parent_index(5) == 2);

    }

    #[test]
    fn test_heap_insert_upheap() {
    
        let mut h = Heap::new();

        h.insert(Elem { data: 4 });
        h.insert(Elem { data: 3 });
        h.insert(Elem { data: 2 });

        println!("{:?}", h.top().data);
        assert!(h.top().data == 2);

        h.insert(Elem { data: 1 });
        assert!(h.top().data == 1);
    }

    #[test]
    fn test_heap_remove_downheap() {
    
        let mut h = Heap::new();

        h.insert(Elem { data: 4 });
        h.insert(Elem { data: 3 });
        h.insert(Elem { data: 2 });

        println!("{:?}", h.top().data);
        assert!(h.top().data == 2);

        h.remove();
        assert!(h.top().data == 3);
    }


}