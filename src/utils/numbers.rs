use num::PrimInt;

#[inline]
pub fn integer_length<T: PrimInt>(n: T, b: T) -> T {
	if n < b {
		T::one()
	} else {
		T::one() + integer_length(n / b, b)
	}
}

#[inline]
pub fn integer_length_10<T: PrimInt>(n: T) -> T {
	integer_length(n, T::from(10).unwrap())
}

#[inline]
pub fn integer_digits<T: PrimInt>(n: T, b: T) -> Vec<T> {
	if n < b {
		vec![n]
	} else {
		let mut result = integer_digits(n / b, b);

		result.push(n % b);

		result
	}
}

#[inline]
pub fn integer_digits_10<T: PrimInt>(n: T) -> Vec<T> {
	integer_digits(n, T::from(10).unwrap())
}