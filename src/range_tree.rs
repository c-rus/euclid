#![allow(dead_code)]
/// Project: euclid
/// Module: kd_tree
///
/// This file contains functions to construct and query a kd-tree data structure.
use crate::primitives::*;
use std::cmp::Ordering;

type Child<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq)]
struct Node<T: Default + Copy> {
    data: Point<T>,
    // the associated structure (takes you to the next dimension!!)
    assoc: Child<T>,
    left: Child<T>,
    right: Child<T>,
}

impl<T: Default + Copy> Node<T> {
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn is_last_dimension(&self) -> bool {
        self.assoc.is_none()
    }

    pub fn next_dimension(&self) -> &Child<T> {
        &self.assoc
    }

    pub fn new(data: Point<T>, left: Child<T>, right: Child<T>, assoc: Child<T>) -> Self {
        Self {
            data: data,
            left: left,
            right: right,
            assoc: assoc,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RangeTree {
    root: Node<f32>,
}

impl RangeTree {
    /// Builds a range-tree for the set of known points `p`.
    pub fn construct(points: Vec<Point<f32>>) -> Self {
        // build the main tree T on the x-coordinate axis
        let mut x_sorted: Vec<&Point<f32>> = points.iter().collect();
        x_sorted.sort_by(|a, b| match a.x().partial_cmp(&b.x()).unwrap() {
            Ordering::Equal => a.y().partial_cmp(&b.y()).unwrap(),
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        });

        // build the main tree
        Self {
            root: Self::build_main_struct(x_sorted),
        }
    }

    /// Builds the main search tree T (assumes points are already sorted).
    fn build_main_struct(points: Vec<&Point<f32>>) -> Node<f32> {
        // build the associated structure T_assoc according to y
        let mut y_sorted: Vec<&&Point<f32>> = points.iter().collect();
        y_sorted.sort_by(|a, b| match a.y().partial_cmp(&b.y()).unwrap() {
            Ordering::Equal => a.x().partial_cmp(&b.x()).unwrap(),
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        });

        let t_assoc = Self::build_assoc_struct(y_sorted);

        match points.len() {
            1 => Node::new(
                **points.get(0).unwrap(),
                None,
                None,
                Some(Box::new(t_assoc)),
            ),
            _ => {
                // partition by the middle index
                let middle = **points.get(points.len() / 2).unwrap();
                let (left, right) = points.split_at(points.len() / 2);

                let left_child = Self::build_main_struct(Vec::from(left));
                let right_child = Self::build_main_struct(Vec::from(right));

                // create a new node with median point to describe the split-line
                Node::new(
                    middle,
                    Some(Box::new(left_child)),
                    Some(Box::new(right_child)),
                    Some(Box::new(t_assoc)),
                )
            }
        }
    }

    /// Builds the binary search tree T_assoc (assumes points are already sorted).
    fn build_assoc_struct(points: Vec<&&Point<f32>>) -> Node<f32> {
        match points.len() {
            1 => Node::new(***points.get(0).unwrap(), None, None, None),
            _ => {
                // partition by the middle index
                let middle = ***points.get(points.len() / 2).unwrap();
                let (left, right) = points.split_at(points.len() / 2);

                let left_child = Self::build_assoc_struct(Vec::from(left));
                let right_child = Self::build_assoc_struct(Vec::from(right));

                // create a new node with median point to describe the split-line
                Node::new(
                    middle,
                    Some(Box::new(left_child)),
                    Some(Box::new(right_child)),
                    None,
                )
            }
        }
    }

    /// Asks a rectangular range query question for the rectangle defined by
    /// `region`.
    pub fn range_query(&self, region: &Region<f32>) -> Vec<&Point<f32>> {
        self.search_tree(&region, &self.root, Vec::new())
    }

    fn find_split_node_x<'a>(node: &'a Node<f32>, min_x: f32, max_x: f32) -> &'a Node<f32> {
        let mut v = node;
        while v.is_leaf() == false && (v.data.x() > max_x || v.data.x() < min_x) {
            // too far right... try to go left
            if v.data.x() > max_x {
                v = v.left.as_ref().unwrap();
            // too far left... try to go right
            } else {
                v = v.right.as_ref().unwrap();
            }
        }
        v
    }

    fn find_split_node_y<'a>(node: &'a Node<f32>, min_y: f32, max_y: f32) -> &'a Node<f32> {
        let mut v = node;
        while v.is_leaf() == false && (v.data.y() > max_y || v.data.y() < min_y) {
            // too far right... try to go left
            if v.data.y() > max_y {
                v = v.left.as_ref().unwrap();
            // too far left... try to go right
            } else {
                v = v.right.as_ref().unwrap();
            }
        }
        v
    }

    fn range_query_1d<'a>(
        &'a self,
        node: &'a Node<f32>,
        region: &Region<f32>,
        mut result: Vec<&'a Point<f32>>,
    ) -> Vec<&'a Point<f32>> {
        let v_split = Self::find_split_node_y(node, region.l_y(), region.r_y());
        match v_split.is_leaf() {
            true => {
                if region.contains_point(&v_split.data) == true {
                    result.push(&v_split.data);
                }
            }
            false => {
                // follow the path to lowermost boundary and report the points in the subtrees right of the path
                let mut v = v_split.left.as_ref().unwrap();
                while v.is_leaf() == false {
                    if v.data.y() >= region.l_y() {
                        result = self.report_subtree(v.right.as_ref().unwrap(), result);
                        v = v.left.as_ref().unwrap();
                    } else {
                        v = v.right.as_ref().unwrap();
                    }
                }
                // check if the point stored in the leaf v must be reported
                if region.contains_point(&v.data) == true {
                    result.push(&v.data);
                }

                // follow the path to uppermost boundary and report the points in the subtrees left of the path
                v = v_split.right.as_ref().unwrap();
                while v.is_leaf() == false {
                    if v.data.y() <= region.r_y() {
                        result = self.report_subtree(v.left.as_ref().unwrap(), result);
                        v = v.right.as_ref().unwrap();
                    } else {
                        v = v.left.as_ref().unwrap();
                    }
                }
                // check if the point stored in the leaf v must be reported
                if region.contains_point(&v.data) == true {
                    result.push(&v.data);
                }
            }
        }
        result
    }

    fn search_tree<'a>(
        &'a self,
        region: &Region<f32>,
        node: &'a Node<f32>,
        mut result: Vec<&'a Point<f32>>,
    ) -> Vec<&Point<f32>> {
        let v_split = Self::find_split_node_x(node, region.l_x(), region.r_x());
        match v_split.is_leaf() {
            true => {
                if region.contains_point(&v_split.data) == true {
                    result.push(&v_split.data);
                }
            }
            false => {
                // follow the path to leftmost boundary and report the points in the subtrees right of the path
                let mut v = v_split.left.as_ref().unwrap();
                while v.is_leaf() == false {
                    if v.data.x() >= region.l_x() {
                        result = self.range_query_1d(
                            v.right.as_ref().unwrap().next_dimension().as_ref().unwrap(),
                            region,
                            result,
                        );
                        v = v.left.as_ref().unwrap();
                    } else {
                        v = v.right.as_ref().unwrap();
                    }
                }
                // check if the point stored in the leaf v must be reported
                if region.contains_point(&v.data) == true {
                    result.push(&v.data);
                }

                // follow the path to rightmost boundary and report the points in the subtrees left of the path
                v = v_split.right.as_ref().unwrap();
                while v.is_leaf() == false {
                    if v.data.x() <= region.r_x() {
                        result = self.range_query_1d(
                            v.left.as_ref().unwrap().next_dimension().as_ref().unwrap(),
                            region,
                            result,
                        );
                        v = v.right.as_ref().unwrap();
                    } else {
                        v = v.left.as_ref().unwrap();
                    }
                }
                // check if the point stored in the leaf v must be reported
                if region.contains_point(&v.data) == true {
                    result.push(&v.data);
                }
            }
        }
        result
    }

    /// Reports all nodes stored below `node` in the tree.
    fn report_subtree<'a>(
        &'a self,
        node: &'a Node<f32>,
        mut result: Vec<&'a Point<f32>>,
    ) -> Vec<&Point<f32>> {
        let mut stack = vec![node];
        while stack.is_empty() == false {
            let n = *stack.pop().as_ref().unwrap();
            match n.is_leaf() {
                true => {
                    result.push(&n.data);
                }
                false => {
                    if let Some(left) = &n.left {
                        stack.push(left);
                    }
                    if let Some(right) = &n.right {
                        stack.push(right);
                    }
                }
            }
        }
        result
    }
}
