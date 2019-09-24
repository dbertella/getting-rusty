pub trait Hashing
{
    fn hashing(&self) -> usize;
}

impl Hashing for String
{
    fn hashing(&self) -> usize
    {
        let mut sum = 0;
        for (i, byte) in self.bytes().enumerate()
        {
            sum += (byte as usize) * i;
        }
        sum
    }
}
