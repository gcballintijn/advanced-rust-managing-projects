use rand::{thread_rng, Rng};
use shapes_2_09::{Circle, Feature, Rectangle};
use std::f64::consts::PI;

#[test]
fn test_rectangle_feature_area() {
    for _ in 0..10 {
        let w = thread_rng().gen_range(0.0..100.0);
        let h = thread_rng().gen_range(0.0..100.0);
        let rectangle = Rectangle::new(w, h);
        let result = rectangle.get_feature(Feature::Area);
        assert_eq!(result, w * h);
    }
}

#[test]
fn test_circle_feature_area() {
    for _ in 0..10 {
        let r = thread_rng().gen_range(0.0..100.0);
        let circle = Circle::new(r);
        let result = circle.get_feature(Feature::Area);
        assert_eq!(result, PI * r * r);      
    }
}

#[test]
fn test_rectangle_feature_perimeter() {
    for _ in 0..10 {
        let w = thread_rng().gen_range(0.0..100.0);
        let h = thread_rng().gen_range(0.0..100.0);
        let rectangle = Rectangle::new(w, h);
        let result = rectangle.get_feature(Feature::Perimeter);
        assert_eq!(result, 2.0 * (w + h));        
    }
}

#[test]
fn test_circle_feature_perimeter() {
    for _ in 0..10 {
        let r = thread_rng().gen_range(0.0..100.0);
        let circle = Circle::new(r);
        let result = circle.get_feature(Feature::Perimeter);
        assert_eq!(result, 2.0 * PI * r);
    }
}
