package math

// BitwiseAnd performs a bitwise AND operation on two integers.
//
// Example:
//
//	BitwiseAnd(5, 3) // 1 (0101 & 0011 = 0001)
func BitwiseAnd(a, b int) int {
	return a & b
}

// BitwiseOr performs a bitwise OR operation on two integers.
//
// Example:
//
//	BitwiseOr(5, 3) // 7 (0101 | 0011 = 0111)
func BitwiseOr(a, b int) int {
	return a | b
}

// BitwiseXor performs a bitwise XOR operation on two integers.
//
// Example:
//
//	BitwiseXor(5, 3) // 6 (0101 ^ 0011 = 0110)
func BitwiseXor(a, b int) int {
	return a ^ b
}

// BitwiseNot performs a bitwise NOT operation on an integer.
//
// Example:
//
//	BitwiseNot(5) // -6
func BitwiseNot(a int) int {
	return ^a
}

// BitwiseLeftShift performs a left bit shift operation.
//
// Example:
//
//	BitwiseLeftShift(1, 3) // 8 (0001 << 3 = 1000)
func BitwiseLeftShift(a, b int) int {
	return a << uint(b)
}

// BitwiseRightShift performs a right bit shift operation.
//
// Example:
//
//	BitwiseRightShift(8, 3) // 1 (1000 >> 3 = 0001)
func BitwiseRightShift(a, b int) int {
	return a >> uint(b)
}
