#[cfg(test)]
mod tests {
    // Примитивы
    use crate::primitives::*;

    #[test]
    fn test_point_add_vector() {
        let mut p = Point(1.0, 2.0, 3.0);
        let v = Vec3(0.5, -1.0, 2.0);
        p.add_vector(&v);
        assert_eq!(p.0, 1.5);
        assert_eq!(p.1, 1.0);
        assert_eq!(p.2, 5.0);
    }

    #[test]
    fn test_point_subtract_vector() {
        let mut p = Point(1.0, 2.0, 3.0);
        let v = Vec3(1.0, 1.0, 1.0);
        p.subtract_vector(&v);
        assert_eq!(p, Point(0.0, 1.0, 2.0));
    }

    #[test]
    fn test_point_add_point() {
        let mut p1 = Point(1.0, 2.0, 3.0);
        let p2 = Point(4.0, 5.0, 6.0);
        p1.add_point(&p2);
        assert_eq!(p1, Point(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_point_subtract_point() {
        let mut p1 = Point(4.0, 5.0, 6.0);
        let p2 = Point(1.0, 2.0, 3.0);
        p1.subtract_point(&p2);
        assert_eq!(p1, Point(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_point_to_vec3() {
        let p = Point(1.0, 2.0, 3.0);
        let v = p.to_vec3();
        assert_eq!(v, Vec3(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_point_set() {
        let mut p = Point(0.0, 0.0, 0.0);
        p.set((9.0, 8.0, 7.0));
        assert_eq!(p, Point(9.0, 8.0, 7.0));
    }

    #[test]
    fn test_vec3_add_vector() {
        let mut v1 = Vec3(1.0, 1.0, 1.0);
        let v2 = Vec3(2.0, 3.0, 4.0);
        v1.add_vector(&v2);
        assert_eq!(v1, Vec3(3.0, 4.0, 5.0));
    }

    #[test]
    fn test_vec3_subtract_vector() {
        let mut v1 = Vec3(5.0, 6.0, 7.0);
        let v2 = Vec3(2.0, 2.0, 2.0);
        v1.subtract_vector(&v2);
        assert_eq!(v1, Vec3(3.0, 4.0, 5.0));
    }

    #[test]
    fn test_vec3_to_point() {
        let v = Vec3(1.1, 2.2, 3.3);
        let p = v.to_point();
        assert_eq!(p, Point(1.1, 2.2, 3.3));
    }

    #[test]
    fn test_vec3_set() {
        let mut v = Vec3::default();
        v.set((3.0, 2.0, 1.0));
        assert_eq!(v, Vec3(3.0, 2.0, 1.0));
    }
    
    #[test]
    fn test_rotate_xy_90_deg() {
        let mut v = Vec3(1.0, 0.0, 0.0);
        v.rotate_xy(90.0);
        let expected = Vec3(0.0, 1.0, 0.0);
        assert_eq!(v, expected);
    }

    #[test]
    fn test_rotate_yz_90_deg() {
        let mut v = Vec3(0.0, 1.0, 0.0);
        v.rotate_yz(90.0);
        let expected = Vec3(0.0, 0.0, 1.0);
        assert_eq!(v, expected);
    }

    #[test]
    fn test_rotate_xz_90_deg() {
        let mut v = Vec3(1.0, 0.0, 0.0);
        v.rotate_xz(90.0);
        let expected = Vec3(0.0, 0.0, 1.0);
        assert_eq!(v, expected);
    }

    #[test]
    fn test_scale() {
        let mut v = Vec3(1.0, 2.0, 3.0);
        v.scale((2.0, 0.5, -1.0));
        let expected = Vec3(2.0, 1.0, -3.0);
        assert_eq!(v, expected);
    }
}
