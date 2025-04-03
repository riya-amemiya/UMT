package math

import (
	"testing"
)

func TestRandom(t *testing.T) {
	tests := []struct {
		name    string
		max     int
		min     []int
		wantMin int
		wantMax int
	}{
		{
			name:    "基本的な範囲テスト（0-10）",
			max:     10,
			min:     nil,
			wantMin: 0,
			wantMax: 10,
		},
		{
			name:    "最小値と最大値を指定（5-10）",
			max:     10,
			min:     []int{5},
			wantMin: 5,
			wantMax: 10,
		},
		{
			name:    "最小値が最大値より大きい場合（10-5→5-10）",
			max:     5,
			min:     []int{10},
			wantMin: 5,
			wantMax: 10,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// 100回試行して範囲内に収まることを確認
			for i := 0; i < 100; i++ {
				got, err := Random(tt.max, tt.min...)
				if err != nil {
					t.Errorf("Random() error = %v", err)
					return
				}
				if got < tt.wantMin || got > tt.wantMax {
					t.Errorf("Random() = %v, want range [%v, %v]", got, tt.wantMin, tt.wantMax)
				}
			}
		})
	}
}

func TestMustRandom(t *testing.T) {
	// 正常系のテスト
	t.Run("正常系", func(t *testing.T) {
		for i := 0; i < 100; i++ {
			got := MustRandom(10, 5)
			if got < 5 || got > 10 {
				t.Errorf("MustRandom() = %v, want range [5, 10]", got)
			}
		}
	})
}

func TestRandomDistribution(t *testing.T) {
	// 分布のテスト（0-9の範囲で1000回試行）
	counts := make([]int, 10)
	trials := 1000

	for i := 0; i < trials; i++ {
		n, err := Random(9)
		if err != nil {
			t.Fatalf("Random() error = %v", err)
		}
		counts[n]++
	}

	// 各数値が全体の8-12%の範囲に収まることを期待
	expectedMin := trials / 10 * 8 / 10  // 8%
	expectedMax := trials / 10 * 12 / 10 // 12%

	for i, count := range counts {
		if count < expectedMin || count > expectedMax {
			t.Logf("Warning: number %d appeared %d times (expected range: %d-%d)", i, count, expectedMin, expectedMax)
		}
	}
}
