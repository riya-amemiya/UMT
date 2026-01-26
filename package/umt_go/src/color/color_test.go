package color

import (
	"math"
	"testing"
)

func approxEqual(a, b, tolerance float64) bool {
	return math.Abs(a-b) <= tolerance
}

func TestHexaToRgba(t *testing.T) {
	tests := []struct {
		name string
		hex  string
		want RGBA
	}{
		{"black transparent", "#00000000", RGBA{0, 0, 0, 0}},
		{"black opaque", "#000000ff", RGBA{0, 0, 0, 1}},
		{"white opaque", "#ffffffff", RGBA{255, 255, 255, 1}},
		{"red", "#ff0000", RGBA{255, 0, 0, 1}},
		{"3-digit hex", "#fff", RGBA{255, 255, 255, 1}},
		{"6-digit hex", "#ff0000", RGBA{255, 0, 0, 1}},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := HexaToRgba(tt.hex)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if got.R != tt.want.R || got.G != tt.want.G || got.B != tt.want.B || !approxEqual(got.A, tt.want.A, 0.01) {
				t.Errorf("HexaToRgba(%q) = %+v, want %+v", tt.hex, got, tt.want)
			}
		})
	}
}

func TestHexaToRgbaInvalid(t *testing.T) {
	invalids := []string{"", "invalid", "#xyz", "#12345"}
	for _, hex := range invalids {
		_, err := HexaToRgba(hex)
		if err == nil {
			t.Errorf("expected error for HexaToRgba(%q)", hex)
		}
	}
}

func TestRgbaToHexA(t *testing.T) {
	tests := []struct {
		name string
		rgba RGBA
		want string
	}{
		{"black opaque", RGBA{0, 0, 0, 1}, "#000000ff"},
		{"white opaque", RGBA{255, 255, 255, 1}, "#ffffffff"},
		{"red", RGBA{255, 0, 0, 1}, "#ff0000ff"},
		{"transparent", RGBA{0, 0, 0, 0}, "#00000000"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := RgbaToHexA(tt.rgba)
			if got != tt.want {
				t.Errorf("RgbaToHexA(%+v) = %q, want %q", tt.rgba, got, tt.want)
			}
		})
	}
}

func TestRgbaToHsla(t *testing.T) {
	tests := []struct {
		name string
		rgba RGBA
		wantH, wantS, wantL float64
	}{
		{"gray", RGBA{100, 100, 100, 1}, 0, 0, 39.22},
		{"red", RGBA{255, 0, 0, 1}, 0, 100, 50},
		{"green", RGBA{0, 255, 0, 1}, 120, 100, 50},
		{"blue", RGBA{0, 0, 255, 1}, 240, 100, 50},
		{"black", RGBA{0, 0, 0, 1}, 0, 0, 0},
		{"white", RGBA{255, 255, 255, 1}, 0, 0, 100},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := RgbaToHsla(tt.rgba)
			if !approxEqual(got.H, tt.wantH, 1) || !approxEqual(got.S, tt.wantS, 1) || !approxEqual(got.L, tt.wantL, 1) {
				t.Errorf("RgbaToHsla(%+v) = {H:%.2f, S:%.2f, L:%.2f}, want {H:%.2f, S:%.2f, L:%.2f}",
					tt.rgba, got.H, got.S, got.L, tt.wantH, tt.wantS, tt.wantL)
			}
		})
	}
}

func TestHslaToRgba(t *testing.T) {
	tests := []struct {
		name                 string
		hsla                 HSLA
		wantR, wantG, wantB float64
	}{
		{"red", HSLA{0, 100, 50, 1}, 255, 0, 0},
		{"green", HSLA{120, 100, 50, 1}, 0, 255, 0},
		{"blue", HSLA{240, 100, 50, 1}, 0, 0, 255},
		{"black", HSLA{0, 0, 0, 1}, 0, 0, 0},
		{"white", HSLA{0, 0, 100, 1}, 255, 255, 255},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := HslaToRgba(tt.hsla)
			if !approxEqual(got.R, tt.wantR, 2) || !approxEqual(got.G, tt.wantG, 2) || !approxEqual(got.B, tt.wantB, 2) {
				t.Errorf("HslaToRgba(%+v) = {R:%.2f, G:%.2f, B:%.2f}, want {R:%.2f, G:%.2f, B:%.2f}",
					tt.hsla, got.R, got.G, got.B, tt.wantR, tt.wantG, tt.wantB)
			}
		})
	}
}

func TestRgbaToCmyk(t *testing.T) {
	tests := []struct {
		name                     string
		rgba                     RGBA
		wantC, wantM, wantY, wantK float64
	}{
		{"black", RGBA{0, 0, 0, 1}, 0, 0, 0, 100},
		{"white", RGBA{255, 255, 255, 1}, 0, 0, 0, 0},
		{"red", RGBA{255, 0, 0, 1}, 0, 100, 100, 0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := RgbaToCmyk(tt.rgba)
			if !approxEqual(got.C, tt.wantC, 1) || !approxEqual(got.M, tt.wantM, 1) ||
				!approxEqual(got.Y, tt.wantY, 1) || !approxEqual(got.K, tt.wantK, 1) {
				t.Errorf("RgbaToCmyk(%+v) = {C:%.2f, M:%.2f, Y:%.2f, K:%.2f}, want {C:%.2f, M:%.2f, Y:%.2f, K:%.2f}",
					tt.rgba, got.C, got.M, got.Y, got.K, tt.wantC, tt.wantM, tt.wantY, tt.wantK)
			}
		})
	}
}

func TestCmykToRgba(t *testing.T) {
	tests := []struct {
		name                string
		c, m, y, k, a      float64
		wantR, wantG, wantB float64
	}{
		{"black", 0, 0, 0, 100, 1, 0, 0, 0},
		{"white", 0, 0, 0, 0, 1, 255, 255, 255},
		{"red", 0, 100, 100, 0, 1, 255, 0, 0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := CmykToRgba(tt.c, tt.m, tt.y, tt.k, tt.a)
			if !approxEqual(got.R, tt.wantR, 2) || !approxEqual(got.G, tt.wantG, 2) || !approxEqual(got.B, tt.wantB, 2) {
				t.Errorf("CmykToRgba(%.2f,%.2f,%.2f,%.2f,%.2f) = {R:%.2f, G:%.2f, B:%.2f}, want {R:%.2f, G:%.2f, B:%.2f}",
					tt.c, tt.m, tt.y, tt.k, tt.a, got.R, got.G, got.B, tt.wantR, tt.wantG, tt.wantB)
			}
		})
	}
}
