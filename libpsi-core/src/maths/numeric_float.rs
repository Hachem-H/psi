use super::{Complex, Float};

impl Float for f32 {
    fn sqrt(self) -> Self {
        libm::sqrtf(self)
    }

    fn atan2(y: Self, x: Self) -> Self {
        libm::atan2f(y, x)
    }
}

impl Float for f64 {
    fn sqrt(self) -> Self {
        libm::sqrt(self)
    }

    fn atan2(y: Self, x: Self) -> Self {
        libm::atan2(y, x)
    }
}

impl Float for Complex<f32> {
    fn sqrt(self) -> Self {
        let r = self.abs();
        let theta = self.phase();

        let sqrt_r = libm::sqrtf(r);
        let sqrt_theta = theta / 2.0;

        Complex::new(
            sqrt_r * libm::cosf(sqrt_theta),
            sqrt_r * libm::sinf(sqrt_theta),
        )
    }

    fn atan2(y: Self, x: Self) -> Self {
        Complex::new(
            libm::atan2f(y.real, x.real),
            libm::atan2f(y.imaginary, x.imaginary),
        )
    }
}

impl Float for Complex<f64> {
    fn sqrt(self) -> Self {
        let r = self.abs();
        let theta = self.phase();

        let sqrt_r = libm::sqrt(r);
        let sqrt_theta = theta / 2.0;

        Complex::new(
            sqrt_r * libm::cos(sqrt_theta),
            sqrt_r * libm::sin(sqrt_theta),
        )
    }

    fn atan2(y: Self, x: Self) -> Self {
        Complex::new(
            libm::atan2(y.real, x.real),
            libm::atan2(y.imaginary, x.imaginary),
        )
    }
}
