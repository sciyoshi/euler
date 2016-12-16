pub mod linkedmatrix;
pub mod numbers;

#[macro_export]
macro_rules! recurrence {
	( $len:expr, $name:ident [ $index:ident ] : $ty:ty = $body:expr ) => ({
		const SIZE: usize = $len;

		struct Recurrence {
			terms: [$ty; SIZE],
			pos: usize
		}

		impl<'a> ::std::ops::Index<usize> for Recurrence {
			type Output = $ty;

			fn index(&self, index: usize) -> &Self::Output {
				&self.terms[index % SIZE]
			}
		}

		impl Iterator for Recurrence {
			type Item = $ty;

			#[inline]
			fn next(&mut self) -> Option<Self::Item> {
				let next = {
					let $index = self.pos;
					let $name = &self;

					$body
				};

				self.terms[self.pos % SIZE] = next;

				self.pos += 1;

				Some(next)
			}
		}

		Recurrence {
			terms: [0; SIZE],
			pos: 1
		}
	})
}
