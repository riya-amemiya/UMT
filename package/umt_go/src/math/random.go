package math

import (
	"crypto/rand"
	"math/big"
)

// Random は指定された範囲内の乱数を生成します
//
// パラメータ:
//
//	max: 最大値（この値を含む）
//	min: 最小値（この値を含む）省略可能、デフォルトは0
//
// 戻り値:
//
//	生成された乱数と発生したエラー（もしあれば）
//
// 使用例:
//
//	n, err := math.Random(10)    // 0から10までの乱数を生成
//	n, err := math.Random(10, 5) // 5から10までの乱数を生成
func Random(max int, min ...int) (int, error) {
	minVal := 0
	if len(min) > 0 {
		minVal = min[0]
		if minVal > max {
			// 最小値が最大値より大きい場合は入れ替え
			minVal, max = max, minVal
		}
	}

	// 範囲の大きさを計算
	diff := big.NewInt(int64(max - minVal + 1))

	// 乱数生成
	n, err := rand.Int(rand.Reader, diff)
	if err != nil {
		return 0, err
	}

	// 最小値を加算して範囲内の値に調整
	return int(n.Int64()) + minVal, nil
}

// MustRandom はRandomのラッパー関数で、エラーが発生した場合はパニックします
//
// パラメータ:
//
//	max: 最大値（この値を含む）
//	min: 最小値（この値を含む）省略可能、デフォルトは0
//
// 戻り値:
//
//	生成された乱数
//
// 使用例:
//
//	n := math.MustRandom(10)    // 0から10までの乱数を生成
//	n := math.MustRandom(10, 5) // 5から10までの乱数を生成
func MustRandom(max int, min ...int) int {
	n, err := Random(max, min...)
	if err != nil {
		panic(err)
	}
	return n
}
