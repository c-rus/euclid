#![allow(dead_code)]
/// Project: euclid
/// Module: kd_tree
///
/// This file contains functions to construct and query a kd-tree data structure.

use crate::primitives::*;
use std::cmp::Ordering;

type Child<T> = Option<Box<Node<T>>>;

struct Node<T: Default + Copy> {
    data: Point<T>,
    left: Child<T>,
    right: Child<T>,
}

impl<T: Default + Copy> Node<T> {
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn new(data: Point<T>, left: Child<T>, right: Child<T>) -> Self {
        Self {
            data: data,
            left: left,
            right: right,
        }
    }
}

pub struct KdTree {
    root: Node<f32>,
}

impl KdTree {
    /// Builds a kd-tree for the set of known points `p`.
    pub fn construct(points: Vec<Point<f32>>) -> Self {
        Self {
            root: Self::build(points, 0),
        }
    }

    /// Recursive function call to build the levels of the kd-tree while alternating
    /// axis coordinate sorting. 
    /// 
    /// At each level in the tree, the remaining subset of points are sorted 
    /// according to the current axis under comparison.
    fn build(points: Vec<Point<f32>>, depth: usize) -> Node<f32> {
        match points.len() {
            // create a new leaf
            1 => Node::new(*points.get(0).unwrap(), None, None),
            // recursively build sub-trees
            _ => {
                let ordering = {
                    match depth % 2 == 0 {
                        // split by x-coordinate (vertical line)
                        true => {
                            // sort the points by increasing x-coordinate, and then increasing y-coordinate (if equal x-coordinate)
                            let mut x_split_points = points;
                            x_split_points.sort_by(|a, b| {
                                match a.x().partial_cmp(&b.x()).unwrap() {
                                    Ordering::Equal => a.y().partial_cmp(&b.y()).unwrap(),
                                    Ordering::Greater => Ordering::Greater,
                                    Ordering::Less => Ordering::Less,
                                }
                            });
                            x_split_points
                        }
                        // split by y-coordinate (horizontal line)
                        false => {
                            // sort the points by increasing y-coordinate, and then increasing x-coordinate (if equal y-coordinate)
                            let mut y_split_points = points;
                            y_split_points.sort_by(|a, b| {
                                match a.y().partial_cmp(&b.y()).unwrap() {
                                    Ordering::Equal => a.x().partial_cmp(&b.x()).unwrap(),
                                    Ordering::Greater => Ordering::Greater,
                                    Ordering::Less => Ordering::Less,
                                }
                            });
                            y_split_points
                        }
                    }
                };
                // partition by the median index
                let median = *ordering.get(ordering.len() / 2).unwrap();
                let (left, right) = ordering.split_at(ordering.len() / 2);

                let left_child = Self::build(Vec::from(left), depth + 1);
                let right_child = Self::build(Vec::from(right), depth + 1);

                // create a new node with median point to describe the split-line
                Node::new(
                    median,
                    Some(Box::new(left_child)),
                    Some(Box::new(right_child)),
                )
            }
        }
    }

    /// Asks a rectangular range query question for the rectangle defined by
    /// `region`.
    pub fn range_query(&self, region: Region<f32>) -> Vec<&Point<f32>> {
        self.search_tree(&region, &self.root, Vec::new(), 0)
    }

    /// Searches a (sub)tree at the root `node` at the level `depth` in the kd-tree to decide which nodes
    /// to visit next and report the points fully contained in the `region`.
    fn search_tree<'a>(
        &'a self,
        region: &Region<f32>,
        node: &'a Node<f32>,
        mut result: Vec<&'a Point<f32>>,
        depth: usize,
    ) -> Vec<&Point<f32>> {
        match node.is_leaf() {
            // add to the results if fully contained in R
            true => {
                if region.contains_point(&node.data) {
                    result.push(&node.data);
                }
            }
            // determine which direction to go down the tree
            false => {
                match depth % 2 == 0 {
                    // check by x-coordinate (vertical line)
                    true => {
                        // decide if to traverse the left child (if it exists)
                        if let Some(lc) = &node.left {
                            if let Some(left_region) =
                                &region.intersect_left_halfplane(node.data.x())
                            {
                                // investigate subtree further
                                if region.intersects(left_region) == true {
                                    result = self.search_tree(region, lc, result, depth + 1)
                                }
                            }
                        }
                        // decide if to traverse the right child (if it exists)
                        if let Some(rc) = &node.right {
                            if let Some(right_region) =
                                &region.intersect_right_halfplane(node.data.x())
                            {
                                // // investigate subtree further
                                if region.intersects(right_region) == true {
                                    result = self.search_tree(region, rc, result, depth + 1)
                                }
                            }
                        }
                    }
                    // check by y-coordinate (horizontal line)
                    false => {
                        // decide if to traverse the left child (if it exists)
                        if let Some(lc) = &node.left {
                            if let Some(lower_region) =
                                &region.intersect_lower_halfplane(node.data.y())
                            {
                                // investigate subtree further
                                if region.intersects(lower_region) == true {
                                    result = self.search_tree(region, lc, result, depth + 1)
                                }
                            }
                        }
                        // decide if to traverse the right child (if it exists)
                        if let Some(rc) = &node.right {
                            if let Some(upper_region) =
                                &region.intersect_upper_halfplane(node.data.y())
                            {
                                // investigate subtree further
                                if region.intersects(upper_region) == true {
                                    result = self.search_tree(region, rc, result, depth + 1)
                                }
                            }
                        }
                    }
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
        if node.is_leaf() == true {
            result.push(&node.data);
        }
        if let Some(left) = &node.left {
            result = self.report_subtree(left, result);
        }
        if let Some(right) = &node.right {
            result = self.report_subtree(right, result);
        }
        result
    }
}