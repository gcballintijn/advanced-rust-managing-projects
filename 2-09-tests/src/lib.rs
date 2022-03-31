use std::f64::consts::PI;

pub struct Rectangle {
    width: f64,
    height: f64,
}

pub struct Circle {
    radius: f64
}

pub enum Feature {
    Area,
    Perimeter,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {width, height}
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_perimeter()
        }
    }

    fn calc_area(&self) -> f64 {
        self.width * self.height
    }

    fn calc_perimeter(&self) -> f64 {
        2.0 * self.width + 2.0 * self.height
    }
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle {radius}
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_perimeter()
        }
    }

    fn calc_area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn calc_perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let rectangle = Rectangle::new(1.0, 2.0);
        let result = rectangle.calc_area();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_rectangle_perimeter() {
        let rectangle = Rectangle::new(1.0, 2.0);
        let result = rectangle.calc_perimeter();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_circle_area() {
        let circle = Circle::new(1.0);
        let result = circle.calc_area();
        assert_eq!(result, PI);
    }

    #[test]
    fn test_circle_perimeter() {
        let circle = Circle::new(1.0);
        let result = circle.calc_perimeter();
        assert_eq!(result, 2.0 * PI);
    }

    #[test]
    fn test_rectangle_feature_area() {
        let rectangle = Rectangle::new(1.0, 2.0);
        let result = rectangle.get_feature(Feature::Area);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_circle_feature_area() {
        let circle = Circle::new(1.0);
        let result = circle.get_feature(Feature::Area);
        assert_eq!(result, PI);
    }

    #[test]
    fn test_rectangle_feature_perimeter() {
        let rectangle = Rectangle::new(1.0, 2.0);
        let result = rectangle.get_feature(Feature::Perimeter);
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_circle_feature_perimeter() {
        let circle = Circle::new(1.0);
        let result = circle.get_feature(Feature::Perimeter);
        assert_eq!(result, 2.0 * PI);
    }
}
