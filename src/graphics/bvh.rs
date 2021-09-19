use crate::math::Axis;

use super::{Aabb, HitRecord, Hittable, Ray};

enum BvhNode {
    Branch {left: Box<Bvh>, right: Box<Bvh>},
    Leaf(Box<dyn Hittable>)
}

pub struct Bvh {
    node: BvhNode,
    bounding_box: Aabb,
}

impl Bvh {
    pub fn new(mut src_objects: Vec<Box<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        let random_axis: Axis = rand::random();
        let node;

        let object_span = src_objects.len();
        match object_span {
            0 => {
                panic!("No elements passed into BVH")
            }
            1 => {
                node = src_objects.pop().unwrap();
                match node.bounding_box(time0, time1) {
                    None => panic!("No bounding box in BVH_Node construction"),
                    Some(h) => {
                        Bvh {
                            node: BvhNode::Leaf(node),
                            bounding_box: h,
                        }
                    }
                }
            },
            _ => {
                src_objects.sort_unstable_by(|a, b| Bvh::get_box_axis_value(a, time0, time1, &random_axis).partial_cmp(&Bvh::get_box_axis_value(b, time0, time1, &random_axis)).unwrap());
                let right = Bvh::new(src_objects.drain(object_span / 2..).collect(), time0, time1);
                let left = Bvh::new(src_objects, time0, time1);
                let right_box = right.bounding_box(time0, time1);
                let left_box = left.bounding_box(time0, time1);

                if right_box.is_none() || left_box.is_none() {
                    panic!("No bounding box in BVH_Node construction");
                }

                let full_bounding_box = right_box.unwrap().surrounding_box(&left_box.unwrap());

                Bvh {
                    node: BvhNode::Branch{left: Box::new(left), right: Box::new(right)},
                    bounding_box: full_bounding_box
                }
            }
        }
    }

    fn get_box_axis_value(a: &Box<dyn Hittable>, time0: f64, time1: f64, axis: &Axis) -> f64 {
        let box_option = a.bounding_box(time0, time1);

        match box_option {
            None => panic!("No bounding box in BVH_Node construction"),
            Some(b) => b.minimum[axis],
        }
    }
}

impl Hittable for Bvh {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bounding_box)
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        match &self.node {
            BvhNode::Leaf(n) => n.hit(ray, t_min, t_max),
            BvhNode::Branch{left, right} => {
                let hit_left = left.hit(ray, t_min, t_max);
                let hit_right = right.hit(ray, t_min, t_max);
                match (hit_left, hit_right) {
                    (Some(left_record), Some(right_record)) => {
                        if left_record.t < right_record.t {
                            Some(left_record)
                        } else {
                            Some(right_record)
                        }
                    }
                    (Some(left_record), None) => Some(left_record),
                    (None, Some(right_record)) => Some(right_record),
                    (None, None) => None,
                }
            }
        }
    }
}