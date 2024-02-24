/// Project: euclid
/// Module: kd_tree
///
/// This file contains functions to construct and query a kd-tree data
/// structure.
use crate::primitives::*;
use std::cmp::Ordering;

type Child<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
    pub fn range_query(&self, region: &Region<f32>) -> Vec<&Point<f32>> {
        self.search_tree(&region, &self.root, Vec::new(), 0)
    }

    /// Recursive function call that traverses down the (sub)tree from `node`
    /// and evaluates the parallel axis according to the current `depth` to
    /// collect the points that are contained within the rectangular range
    /// `region`.
    fn search_tree<'a>(
        &'a self,
        region: &Region<f32>,
        node: &'a Node<f32>,
        mut result: Vec<&'a Point<f32>>,
        depth: usize,
    ) -> Vec<&Point<f32>> {
        match node.is_leaf() {
            // report the point that is stored in v
            true => {
                if region.contains_point(&node.data) == true {
                    result.push(&node.data);
                }
            }
            false => {
                match depth % 2 == 0 {
                    // split by vertical axis (x)
                    true => {
                        if let Some(lc) = &node.left {
                            if let Some(left_region) =
                                &region.intersect_left_halfplane(node.data.x())
                            {
                                if region.strictly_contains(left_region) == true {
                                    result = self.report_subtree(lc, result);
                                }
                                // investigate subtree further
                                else if region.intersects(left_region) == true {
                                    result = self.search_tree(region, lc, result, depth + 1)
                                }
                            }
                        }
                        // decide if to traverse the right child (if it exists)
                        if let Some(rc) = &node.right {
                            if let Some(right_region) =
                                &region.intersect_right_halfplane(node.data.x())
                            {
                                if region.strictly_contains(right_region) == true {
                                    result = self.report_subtree(rc, result);
                                }
                                // // investigate subtree further
                                else if region.intersects(right_region) == true {
                                    result = self.search_tree(region, rc, result, depth + 1)
                                }
                            }
                        }
                    }
                    // split by horizontal axis (y)
                    false => {
                        // decide if to traverse the left child (if it exists)
                        if let Some(lc) = &node.left {
                            if let Some(lower_region) =
                                &region.intersect_lower_halfplane(node.data.y())
                            {
                                if region.strictly_contains(lower_region) == true {
                                    result = self.report_subtree(lc, result);
                                }
                                // investigate subtree further
                                else if region.intersects(lower_region) == true {
                                    result = self.search_tree(region, lc, result, depth + 1)
                                }
                            }
                        }
                        // decide if to traverse the right child (if it exists)
                        if let Some(rc) = &node.right {
                            if let Some(upper_region) =
                                &region.intersect_upper_halfplane(node.data.y())
                            {
                                if region.strictly_contains(upper_region) == true {
                                    result = self.report_subtree(rc, result);
                                }
                                // investigate subtree further
                                else if region.intersects(upper_region) == true {
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

    /// Reports all leaves stored below `node` in the tree using an iterative
    /// approach.
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
