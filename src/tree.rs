#![allow(non_snake_case)]

use std::cmp::max;
use std::collections::HashMap;

pub struct Tree {
    nodes: HashMap<usize,u8>,
    a: usize,
    b: usize
}

impl Tree {
    pub fn new(a: usize, b: usize) -> Self {
        Tree {
            nodes: HashMap::new(),
            a: a,
            b: b
        }
    }
    pub fn insertAt(&mut self, index: usize, item: u8) {
        self.nodes.insert(index, item);
    }
    pub fn height(&self, pos: usize) -> usize {
        let children: Vec<usize> = self.children_indexes(pos);
        if self.empty(pos) {
            0
        } else {
            println!("left {:?}, {:?}", children[0], self.nodes.get(&children[0]));
            println!("right {:?}, {:?}", children[1], self.nodes.get(&children[0]));
            1 + children.iter().map(|&x| self.height (x)).collect::<Vec<usize>>().iter().fold(0, |acc, &c| max(acc, c))
        }
    }
    pub fn children_indexes(&self, pos: usize) -> Vec<usize> {
        let f = pos * self.b as usize + 1;
        let mut v: Vec<usize> = Vec::new();
        for i in 0..self.b {
            v.push(f + i);
        };
        v
    }
    pub fn empty(&self, pos: usize) -> bool {
        match self.nodes.get(&pos) {
            Some(_) => false,
            None    => true
        }
    }
    pub fn preorder(&self) -> Vec<i32> {
        Vec::new()
    }
    pub fn postorder(&self) -> Vec<i32> {
        Vec::new()
    }
    pub fn inorder(&self) -> Vec<i32> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use vector::{Vector,Elem};
    use std::cmp::max;
    use std::option::Option;

    fn build_tree(t: &mut Tree, idx: usize, expr_vec: &Vec<u8>, subexpr: Option<&[u8]>) -> () {

        let mut operator = None;
        let mut op_count = 0;
        let mut left_subexpr = None;
        let mut right_subexpr = None; 

        match subexpr {
            Some(expr) => {
                if expr.len() == 0 {
                    return;
                }
                else if expr.len() == 1 && expr[0] <= 48 && expr[0] >= 57 { // 0-9
                    t.insertAt(idx, expr[0]);
                    left_subexpr = None;
                    right_subexpr = None;
                } else {
                    for c in 0..expr.len() {
                        println!("{:?}", expr[c]);
                        println!("{:?}", op_count);
                        if (expr[c] == 43 || expr[c] == 45 || expr[c] == 42 || expr[c] == 47) {
                            operator = Some(expr_vec[c]);
                            left_subexpr = Some(&expr_vec[op_count..c]);
                            right_subexpr = Some(&expr_vec[c+1..expr.len()-op_count]);

                            println!("{:?}", expr[c]);
                            println!("{:?}", left_subexpr);
                            println!("{:?}", right_subexpr);
                            println!("Inserting at {:?}", idx);

                            t.insertAt(idx, operator.unwrap());
                            break;
                        }
                        else if expr[c] == 40 { // (
                            op_count += 1;
                        } else if expr[c] == 44 { // )
                            op_count -= 1;
                        }
                    }

                    println!("idx: {:?}, {:?}", idx, (&t).children_indexes(idx));

                }
                let left_idx = (&t).children_indexes(idx)[0];
                let right_idx = (&t).children_indexes(idx)[1];

                build_tree(t, left_idx, expr_vec, left_subexpr);
                build_tree(t, right_idx, expr_vec, right_subexpr);
            },
            _ => return
            }
            
    }

    #[test]
    fn test_map() {

        let a = [1, 2, 3];

        let m = a.iter().map(|&x| x * 2).collect::<Vec<i32>>().iter().fold(0, |acc, &c| max(acc, c));
        assert_eq!(6, m);

    }    

    #[test]
    fn test_tree() {
        let mut tree = Tree::new(2,2);

        assert_eq!(true, tree.empty(0));

        let expression = "1+((2*((6+8)/2))+7)";
        let expr_vec = String::from(expression).into_bytes();

        build_tree(&mut tree, 0, &expr_vec, Some(&expr_vec[..]));

        assert_eq!(5, tree.height(0));
    }
}