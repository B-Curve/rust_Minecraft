macro_rules! vector {
    ($e1:expr, $e2:expr) => (::math::Vec2::new($e1, $e2));
    ($e1:expr, $e2:expr, $e3:expr) => (::math::Vec3::new($e1, $e2, $e3));
    ($e1:expr, $e2:expr, $e3:expr, $e4:expr) => (::math::Vec4::new($e1, $e2, $e3, $e4));
}

macro_rules! mat4 {
    ($e1:expr) => (::math::Mat4::new($e1,$e1,$e1,$e1,$e1,$e1,$e1,$e1,$e1,$e1,$e1,$e1,$e1,$e1,$e1,$e1));
    ($v1:expr, $v2:expr, $v3:expr, $v4:expr) => (::math::Mat4::new(
        $v1.x,$v1.y,$v1.z,$v1.w,
        $v2.x,$v2.y,$v2.z,$v2.w,
        $v3.x,$v3.y,$v3.z,$v3.w,
        $v4.x,$v4.y,$v4.z,$v4.w,
    ));
    ($mat3:expr) => (::math::Mat4::new(
        $mat3[0][0], $mat3[0][1], $mat3[0][2], 0.0,
        $mat3[1][0], $mat3[1][1], $mat3[1][2], 0.0,
        $mat3[2][0], $mat3[2][1], $mat3[2][2], 0.0,
        0.0, 0.0, 0.0, 0.0
    ))
}

macro_rules! null {
    () => (::std::ptr::null());
}

macro_rules! null_mut {
    () => (::std::ptr::null_mut());
}