use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg, BitXor};

#[derive(Debug)]
struct PhysicalVector {
    x: f64,
    y: f64,
    z: f64,
}

/**
 * 向量加法
 */
impl AddAssign for PhysicalVector {
    fn add_assign(&mut self, rhs: PhysicalVector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

/**
 * 向量减法
 */
impl SubAssign for PhysicalVector {
    fn sub_assign(&mut self, rhs: PhysicalVector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

/**
 * 标量乘法
 */
impl MulAssign<f64> for PhysicalVector {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

/**
 * 标量除法
 */

impl DivAssign<f64> for PhysicalVector {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

/**
 * 共轭
 */

impl<'a> Neg for &'a PhysicalVector {
    type Output = PhysicalVector;
    fn neg(self) -> PhysicalVector {
        return PhysicalVector { x: -self.x, y: -self.y, z: -self.z }
    }
}


impl Add for PhysicalVector {
    type Output = PhysicalVector;

    fn add(self, rhs: PhysicalVector) -> Self {
        PhysicalVector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for PhysicalVector {
    type Output = PhysicalVector;

    fn sub(self, p: PhysicalVector) -> PhysicalVector {
        PhysicalVector {
            x: self.x - p.x,
            y: self.y - p.y,
            z: self.z - p.z,
        }
    }
}

/**
 * ^ 叉积
 * 如果两个向量平行，那么它们的叉积为0，通过这个可以判断向量是否平行
 */

impl BitXor for PhysicalVector {
    type Output = PhysicalVector;
    fn bitxor(self, rhs: Self) -> Self::Output {
        return PhysicalVector {
            x: (self.y * rhs.z - self.z * self.y),
            y: (-self.x * rhs.z + self.z * rhs.x),
            z: (self.x * rhs.y - self.y * rhs.x),
        }
    }
}

// TODO
impl Mul for PhysicalVector {
    type Output = f64;

    fn mul(self, rhs: PhysicalVector) -> f64 {
        return self.x * rhs.x;
    }
}

//TODO
impl Div for PhysicalVector {
    type Output = PhysicalVector;

    fn div(self, rhs: Self) -> Self::Output {
        PhysicalVector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}



impl PhysicalVector {
    fn create0() -> PhysicalVector {
        return PhysicalVector { x: 0.0, y: 0.0, z: 0.0 }
    }

    fn new(xi: f64, yi: f64, zi: f64) -> PhysicalVector {
        return PhysicalVector { x: xi, y: yi, z: zi }
    }

    fn clone(&self) -> PhysicalVector {
        return PhysicalVector { x: self.x, y: self.y, z: self.z }
    }

    fn magnitude(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    fn normalize(&mut self) {
        const TOL: f64 = 0.0001;
        let mut m = self.magnitude();
        if m <= TOL {
            m = 1.0;
        }

        self.x /= m;
        self.y /= m;
        self.z /= m;

        if self.x.abs() < TOL {
            self.x = 0.0;
        }

        if self.y.abs() < TOL {
            self.y = 0.0;
        }

        if self.z.abs() < TOL {
            self.z = 0.0;
        }
    }

    fn reverse(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
}

fn add<T: Add<T, Output=T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let v1: PhysicalVector = PhysicalVector::create0();
    let v2: PhysicalVector = PhysicalVector::new(3.0, 4.0, 0.0);
    let v3: PhysicalVector = PhysicalVector::new(3.2, 2.0, 3.0);
    let v2_value: f64 = v2.magnitude();
    println!("v2`s length is {}", v2_value);
    println!("use add function, will get {:?}", add(v1.clone(), v2.clone()));
    println!("use add function, will get {:?}", add(v1.clone(), v3.clone()));
    println!("call create0 function, will get {:?}", v1);
    println!("call new function, will get {:?}", v2);
    println!("call new function, will get {:?}", v3);
    println!("use '+', will get: {:?}", v1 + v2 + v3);
    let mut v4: PhysicalVector = PhysicalVector::new(6.0, 5.0, 3.0);
    println!("v4 is {:?}", v4);
    v4.normalize();
    println!("v4 normalize is {:?}", v4);
    println!("v4 length is {:?}", v4.magnitude());
    v4.reverse();
    println!("v4 reverse is {:?}", v4);
    
    let mut v5: PhysicalVector = PhysicalVector::new(1.0, 1.0, 2.0);
    let u5: PhysicalVector = PhysicalVector::new(2.0, 2.0, 1.0);
    v5 += u5.clone();
    println!("v5 += u5 is {:?}", v5);
    
    v5 -= u5.clone();
    println!("v5 -= u5 is {:?}", v5);
    
    let mut v6: PhysicalVector = PhysicalVector::new(1.0, 2.0, 3.0);
    println!("v6 is {:?}", v6);
    v6 *= 2.0;
    println!("v6 * 2.0 is {:?}", v6);

    let v7: PhysicalVector = PhysicalVector::new(1.0, 0.0, 0.0);
    let v7_neg = -&v7;
    println!("{:?}", v7_neg);
    println!("{:?}", v7)
}
