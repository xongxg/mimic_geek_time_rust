use std::marker::PhantomData;

#[derive(Debug, Default)]
pub struct Equation<IterMethod> {
    current: u32,
    _phantom: PhantomData<IterMethod>,
}

// 线性增长
#[derive(Debug, Default)]
pub struct Linear;

// 二次增长
#[derive(Debug, Default)]
pub struct Quadratic;

impl Iterator for Equation<Linear> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;

        if self.current >= u32::MAX {
            return None;
        }

        Some(self.current)
    }
}

impl Iterator for Equation<Quadratic> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;

        if self.current >= u16::MAX as u32 {
            return None;
        }

        Some(self.current * self.current)
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_linear() {
        let mut linear = Equation::<Linear>::default();
        assert_eq!(linear.next(), Some(1));
        assert_eq!(linear.next(), Some(2));
        assert_eq!(linear.next(), Some(3));
    }

    #[test]
    fn test_quadratic() {
        let mut quadratic = Equation::<Quadratic>::default();
        assert_eq!(quadratic.next(), Some(1));
        assert_eq!(quadratic.next(), Some(4));
        assert_eq!(quadratic.next(), Some(9));
    }
}