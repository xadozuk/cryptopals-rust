use std::ops;
use std::cmp;
use std::fmt;

pub struct Matrix<T>
{
    width:  usize,
    height: usize,

    values: Vec<Vec<T>>
}

impl<T> Matrix<T> where T: Default + Copy
{
    pub fn new(height: usize, width: usize) -> Self
    {
        let mut values = Vec::new();
        values.resize(height as usize, vec![]);

        for row in values.iter_mut()
        {
            row.resize(width as usize, T::default())
        }

        Matrix { width: width, height: height, values: values }
    }

    pub fn width(&self) -> usize
    {
        return self.width;
    }

    pub fn height(&self) -> usize
    {
        return self.height;
    }

    pub fn transpose(&self) -> Self
    {
        let mut transposed = Self::new(self.width, self.height);
        
        for j in 0..self.width
        {
            for i in 0..self.height
            {
                transposed[j][i] = self[i][j];
            }
        }

        return transposed;
    }
}

impl<T> Clone for Matrix<T> where T: Copy
{
    fn clone(&self) -> Self
    {
        Matrix {
            width: self.width,
            height: self.height,
            values: self.values.clone()
        }
    }
}

impl<T> From<&Vec<Vec<T>>> for Matrix<T> where T: Default + Copy
{
    fn from(values: &Vec<Vec<T>>) -> Self
    {
        assert_size(values);

        let height = values.len();
        let width  = values[0].len();

        let mut matrix = Self::new(height, width);

        for i in 0..height
        {
            for j in 0..width
            {
                matrix.values[i][j] = values[i][j];
            }
        }

        return matrix;
    }
}

impl<T> ops::Index<usize> for Matrix<T>
{
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output
    {
        &self.values[index]
    }
}

impl<T> ops::IndexMut<usize> for Matrix<T>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    { 
        self.values.get_mut(index).unwrap()
    }
}

impl<T> cmp::PartialEq for Matrix<T> where T: PartialEq
{
    fn eq(&self, rhs: &Matrix<T>) -> bool
    { 
        self.values == rhs.values
    }
}

impl<T> fmt::Debug for Matrix<T> where T: fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), fmt::Error>
    { 
        write!(f, "\n")?;

        for i in 0..self.height
        {
            write!(f, "[")?;

            for j in 0..self.width
            {
                write!(f, " {}", self[i][j])?;
            }

            write!(f, " ]\n")?;
        }

        write!(f, "")
    }
}

fn assert_size<T>(values: &Vec<Vec<T>>)
{
    if values.len() == 0 { panic!("A 0x0 matrix is not supported (use ::new)") }
    
    let mut last_width = 0;

    for i in 0..values.len()
    {
        if i == 0 { last_width = values[i].len() }
        else
        {
            if last_width != values[i].len() { panic!("All rows must have the same size in a matrix") }
        }
    }

    if last_width == 0 { panic!("A Nx0 matrix is not supported"); }
}

#[cfg(test)]
mod tests
{
    use super::Matrix;

    #[test]
    fn from()
    {
        let matrix = Matrix::<u8>::from(&vec![
            vec![11, 12],
            vec![21, 22]
        ]);

        assert_eq!(11, matrix[0][0]);
        assert_eq!(12, matrix[0][1]);
        assert_eq!(21, matrix[1][0]);
        assert_eq!(22, matrix[1][1]);
    }

    #[test]
    fn transpose()
    {
        let matrix = Matrix::<u8>::from(&vec![
            vec![11, 12],
            vec![21, 22]
        ]);

        let transposed = matrix.transpose();

        assert_eq!(
            Matrix::<u8>::from(&vec![
                vec![11, 21],
                vec![12, 22]
            ]),
            transposed
        )
    }
}