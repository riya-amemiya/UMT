package color_test

import (
	"math"
	"regexp"
	"strings"
	"testing"

	"github.com/riya-amemiya/umt-go/src/color"
)

func approxEqual(a, b, tolerance float64) bool {
	return math.Abs(a-b) <= tolerance
}

func assertPanic(t *testing.T, msg string, fn func()) {
	t.Helper()
	defer func() {
		r := recover()
		if r == nil {
			t.Fatalf("expected panic containing %q, but no panic occurred", msg)
		}
		s, ok := r.(string)
		if !ok {
			t.Fatalf("expected panic string containing %q, got non-string: %v", msg, r)
		}
		if !strings.Contains(s, msg) {
			t.Fatalf("expected panic containing %q, got %q", msg, s)
		}
	}()
	fn()
}

// =============================================================================
// HexaToRgba
// =============================================================================

func TestHexaToRgba6Digit(t *testing.T) {
	tests := []struct {
		hex  string
		want color.RGBA
	}{
		{"#FF0000", color.RGBA{R: 255, G: 0, B: 0, A: 1}},
		{"#00FF00", color.RGBA{R: 0, G: 255, B: 0, A: 1}},
		{"#0000FF", color.RGBA{R: 0, G: 0, B: 255, A: 1}},
		{"#FFFFFF", color.RGBA{R: 255, G: 255, B: 255, A: 1}},
		{"#000000", color.RGBA{R: 0, G: 0, B: 0, A: 1}},
		{"#FFA500", color.RGBA{R: 255, G: 165, B: 0, A: 1}},
	}
	for _, tt := range tests {
		t.Run(tt.hex, func(t *testing.T) {
			got, err := color.HexaToRgba(tt.hex)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if got.R != tt.want.R || got.G != tt.want.G || got.B != tt.want.B || !approxEqual(got.A, tt.want.A, 0.01) {
				t.Errorf("HexaToRgba(%q) = %+v, want %+v", tt.hex, got, tt.want)
			}
		})
	}
}

func TestHexaToRgba8DigitWithAlpha(t *testing.T) {
	tests := []struct {
		hex  string
		want color.RGBA
	}{
		{"#FFA50099", color.RGBA{R: 255, G: 165, B: 0, A: 0.6}},
		{"#FF0000FF", color.RGBA{R: 255, G: 0, B: 0, A: 1}},
		{"#FF000080", color.RGBA{R: 255, G: 0, B: 0, A: 0.5}},
		{"#FF000000", color.RGBA{R: 255, G: 0, B: 0, A: 0}},
	}
	for _, tt := range tests {
		t.Run(tt.hex, func(t *testing.T) {
			got, err := color.HexaToRgba(tt.hex)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if got.R != tt.want.R || got.G != tt.want.G || got.B != tt.want.B || !approxEqual(got.A, tt.want.A, 0.01) {
				t.Errorf("HexaToRgba(%q) = %+v, want %+v", tt.hex, got, tt.want)
			}
		})
	}
}

func TestHexaToRgba3Digit(t *testing.T) {
	tests := []struct {
		hex  string
		want color.RGBA
	}{
		{"#F00", color.RGBA{R: 255, G: 0, B: 0, A: 1}},
		{"#0F0", color.RGBA{R: 0, G: 255, B: 0, A: 1}},
		{"#00F", color.RGBA{R: 0, G: 0, B: 255, A: 1}},
		{"#FFF", color.RGBA{R: 255, G: 255, B: 255, A: 1}},
		{"#000", color.RGBA{R: 0, G: 0, B: 0, A: 1}},
	}
	for _, tt := range tests {
		t.Run(tt.hex, func(t *testing.T) {
			got, err := color.HexaToRgba(tt.hex)
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
	invalids := []string{"#12345", "#1234567", "123456", ""}
	for _, hex := range invalids {
		t.Run(hex, func(t *testing.T) {
			_, err := color.HexaToRgba(hex)
			if err == nil {
				t.Fatalf("expected error for HexaToRgba(%q)", hex)
			}
			if !strings.Contains(err.Error(), "Invalid hex code") {
				t.Errorf("expected error containing %q, got %q", "Invalid hex code", err.Error())
			}
		})
	}
}

func TestHexaToRgbaBlackTransparentOpaque(t *testing.T) {
	got, err := color.HexaToRgba("#00000000")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if got.R != 0 || got.G != 0 || got.B != 0 || got.A != 0 {
		t.Errorf("HexaToRgba(#00000000) = %+v, want {0,0,0,0}", got)
	}

	got, err = color.HexaToRgba("#000000ff")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if got.R != 0 || got.G != 0 || got.B != 0 || got.A != 1 {
		t.Errorf("HexaToRgba(#000000ff) = %+v, want {0,0,0,1}", got)
	}

	got, err = color.HexaToRgba("#ffffffff")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if got.R != 255 || got.G != 255 || got.B != 255 || got.A != 1 {
		t.Errorf("HexaToRgba(#ffffffff) = %+v, want {255,255,255,1}", got)
	}
}

// =============================================================================
// RgbaToHexA
// =============================================================================

func TestRgbaToHexAValid(t *testing.T) {
	tests := []struct {
		name string
		rgba color.RGBA
		want string
	}{
		{"red", color.RGBA{R: 255, G: 0, B: 0, A: 1}, "#ff0000ff"},
		{"green", color.RGBA{R: 0, G: 255, B: 0, A: 1}, "#00ff00ff"},
		{"blue", color.RGBA{R: 0, G: 0, B: 255, A: 1}, "#0000ffff"},
		{"white", color.RGBA{R: 255, G: 255, B: 255, A: 1}, "#ffffffff"},
		{"black", color.RGBA{R: 0, G: 0, B: 0, A: 1}, "#000000ff"},
		{"orange", color.RGBA{R: 255, G: 165, B: 0, A: 1}, "#ffa500ff"},
		{"white half alpha", color.RGBA{R: 255, G: 255, B: 255, A: 0.5}, "#ffffff80"},
		{"transparent", color.RGBA{R: 0, G: 0, B: 0, A: 0}, "#00000000"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := color.RgbaToHexA(tt.rgba)
			if got != tt.want {
				t.Errorf("RgbaToHexA(%+v) = %q, want %q", tt.rgba, got, tt.want)
			}
		})
	}
}

func TestRgbaToHexAPanicsInvalid(t *testing.T) {
	tests := []struct {
		name string
		rgba color.RGBA
	}{
		{"r>255", color.RGBA{R: 256, G: 0, B: 0, A: 1}},
		{"g>255", color.RGBA{R: 0, G: 256, B: 0, A: 1}},
		{"b>255", color.RGBA{R: 0, G: 0, B: 256, A: 1}},
		{"a>1", color.RGBA{R: 0, G: 0, B: 0, A: 1.5}},
		{"r<0", color.RGBA{R: -1, G: 0, B: 0, A: 1}},
		{"g<0", color.RGBA{R: 0, G: -1, B: 0, A: 1}},
		{"b<0", color.RGBA{R: 0, G: 0, B: -1, A: 1}},
		{"a<0", color.RGBA{R: 0, G: 0, B: 0, A: -0.5}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assertPanic(t, "Invalid rgba value", func() {
				color.RgbaToHexA(tt.rgba)
			})
		})
	}
}

// =============================================================================
// RgbaToHsla
// =============================================================================

func TestRgbaToHslaPanicsInvalid(t *testing.T) {
	tests := []struct {
		name string
		rgba color.RGBA
	}{
		{"r<0", color.RGBA{R: -1, G: 0, B: 0, A: 1}},
		{"r>255", color.RGBA{R: 256, G: 0, B: 0, A: 1}},
		{"g<0", color.RGBA{R: 0, G: -1, B: 0, A: 1}},
		{"g>255", color.RGBA{R: 0, G: 256, B: 0, A: 1}},
		{"b<0", color.RGBA{R: 0, G: 0, B: -1, A: 1}},
		{"b>255", color.RGBA{R: 0, G: 0, B: 256, A: 1}},
		{"a<0", color.RGBA{R: 0, G: 0, B: 0, A: -0.1}},
		{"a>1", color.RGBA{R: 0, G: 0, B: 0, A: 1.1}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assertPanic(t, "Invalid rgba value", func() {
				color.RgbaToHsla(tt.rgba)
			})
		})
	}
}

func TestRgbaToHslaEdgeCases(t *testing.T) {
	// Black
	got := color.RgbaToHsla(color.RGBA{R: 0, G: 0, B: 0, A: 1})
	if got.H != 0 || got.S != 0 || got.L != 0 || got.A != 1 {
		t.Errorf("RgbaToHsla(black) = %+v, want {H:0, S:0, L:0, A:1}", got)
	}

	// White
	got = color.RgbaToHsla(color.RGBA{R: 255, G: 255, B: 255, A: 1})
	if got.H != 0 || got.S != 0 || got.L != 100 || got.A != 1 {
		t.Errorf("RgbaToHsla(white) = %+v, want {H:0, S:0, L:100, A:1}", got)
	}
}

func TestRgbaToHslaConversions(t *testing.T) {
	tests := []struct {
		name  string
		input color.RGBA
		want  color.HSLA
	}{
		{"gray", color.RGBA{R: 100, G: 100, B: 100, A: 1}, color.HSLA{H: 0, S: 0, L: 39.22, A: 1}},
		{"red", color.RGBA{R: 255, G: 0, B: 0, A: 1}, color.HSLA{H: 0, S: 100, L: 50, A: 1}},
		{"green", color.RGBA{R: 0, G: 255, B: 0, A: 1}, color.HSLA{H: 120, S: 100, L: 50, A: 1}},
		{"blue", color.RGBA{R: 0, G: 0, B: 255, A: 1}, color.HSLA{H: 240, S: 100, L: 50, A: 1}},
		{"red half alpha", color.RGBA{R: 255, G: 0, B: 0, A: 0.5}, color.HSLA{H: 0, S: 100, L: 50, A: 0.5}},
		{"yellow", color.RGBA{R: 255, G: 255, B: 0, A: 1}, color.HSLA{H: 60, S: 100, L: 50, A: 1}},
		{"blue with alpha", color.RGBA{R: 0, G: 0, B: 255, A: 0.7}, color.HSLA{H: 240, S: 100, L: 50, A: 0.7}},
		{"light blue", color.RGBA{R: 173, G: 216, B: 230, A: 1}, color.HSLA{H: 194.74, S: 53.27, L: 79.02, A: 1}},
		{"pink", color.RGBA{R: 255, G: 0, B: 100, A: 1}, color.HSLA{H: 336.47, S: 100, L: 50, A: 1}},
		{"purple", color.RGBA{R: 100, G: 0, B: 255, A: 1}, color.HSLA{H: 263.53, S: 100, L: 50, A: 1}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := color.RgbaToHsla(tt.input)
			if got.H != tt.want.H || got.S != tt.want.S || got.L != tt.want.L || got.A != tt.want.A {
				t.Errorf("RgbaToHsla(%+v) = %+v, want %+v", tt.input, got, tt.want)
			}
		})
	}
}

// =============================================================================
// HslaToRgba
// =============================================================================

func TestHslaToRgbaPrimaryColors(t *testing.T) {
	tests := []struct {
		name  string
		input color.HSLA
		want  color.RGBA
	}{
		{"red", color.HSLA{H: 0, S: 100, L: 50, A: 1}, color.RGBA{R: 255, G: 0, B: 0, A: 1}},
		{"green", color.HSLA{H: 120, S: 100, L: 50, A: 1}, color.RGBA{R: 0, G: 255, B: 0, A: 1}},
		{"blue", color.HSLA{H: 240, S: 100, L: 50, A: 1}, color.RGBA{R: 0, G: 0, B: 255, A: 1}},
		{"yellow", color.HSLA{H: 60, S: 100, L: 50, A: 1}, color.RGBA{R: 255, G: 255, B: 0, A: 1}},
		{"cyan", color.HSLA{H: 180, S: 100, L: 50, A: 1}, color.RGBA{R: 0, G: 255, B: 255, A: 1}},
		{"magenta", color.HSLA{H: 300, S: 100, L: 50, A: 1}, color.RGBA{R: 255, G: 0, B: 255, A: 1}},
		{"white", color.HSLA{H: 0, S: 0, L: 100, A: 1}, color.RGBA{R: 255, G: 255, B: 255, A: 1}},
		{"black", color.HSLA{H: 0, S: 0, L: 0, A: 1}, color.RGBA{R: 0, G: 0, B: 0, A: 1}},
		{"red half alpha", color.HSLA{H: 0, S: 100, L: 50, A: 0.5}, color.RGBA{R: 255, G: 0, B: 0, A: 0.5}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := color.HslaToRgba(tt.input)
			if got.R != tt.want.R || got.G != tt.want.G || got.B != tt.want.B || got.A != tt.want.A {
				t.Errorf("HslaToRgba(%+v) = %+v, want %+v", tt.input, got, tt.want)
			}
		})
	}
}

func TestHslaToRgbaDefaultAlpha(t *testing.T) {
	tests := []struct {
		name  string
		input color.HSLA
		want  color.RGBA
	}{
		{"black", color.HSLA{H: 0, S: 0, L: 0, A: 1}, color.RGBA{R: 0, G: 0, B: 0, A: 1}},
		{"white h=360", color.HSLA{H: 360, S: 100, L: 100, A: 1}, color.RGBA{R: 255, G: 255, B: 255, A: 1}},
		{"red", color.HSLA{H: 0, S: 100, L: 50, A: 1}, color.RGBA{R: 255, G: 0, B: 0, A: 1}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := color.HslaToRgba(tt.input)
			if got.R != tt.want.R || got.G != tt.want.G || got.B != tt.want.B || got.A != tt.want.A {
				t.Errorf("HslaToRgba(%+v) = %+v, want %+v", tt.input, got, tt.want)
			}
		})
	}
}

func TestHslaToRgbaFractionalValues(t *testing.T) {
	tests := []struct {
		name  string
		input color.HSLA
		want  color.RGBA
	}{
		{"blue 25%", color.HSLA{H: 240, S: 100, L: 25, A: 1}, color.RGBA{R: 0, G: 0, B: 127.5, A: 1}},
		{"green 25%", color.HSLA{H: 120, S: 100, L: 25, A: 1}, color.RGBA{R: 0, G: 127.5, B: 0, A: 1}},
		{"red 25%", color.HSLA{H: 0, S: 100, L: 25, A: 1}, color.RGBA{R: 127.5, G: 0, B: 0, A: 1}},
		{"yellow 25%", color.HSLA{H: 60, S: 100, L: 25, A: 1}, color.RGBA{R: 127.5, G: 127.5, B: 0, A: 1}},
		{"cyan 25%", color.HSLA{H: 180, S: 100, L: 25, A: 1}, color.RGBA{R: 0, G: 127.5, B: 127.5, A: 1}},
		{"magenta 25%", color.HSLA{H: 300, S: 100, L: 25, A: 1}, color.RGBA{R: 127.5, G: 0, B: 127.5, A: 1}},
		{"gray 25%", color.HSLA{H: 0, S: 0, L: 25, A: 1}, color.RGBA{R: 63.75, G: 63.75, B: 63.75, A: 1}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := color.HslaToRgba(tt.input)
			if got.R != tt.want.R || got.G != tt.want.G || got.B != tt.want.B || got.A != tt.want.A {
				t.Errorf("HslaToRgba(%+v) = %+v, want %+v", tt.input, got, tt.want)
			}
		})
	}
}

func TestHslaToRgbaPanicsInvalidHue(t *testing.T) {
	assertPanic(t, "Hue must be between 0 and 360 degrees", func() {
		color.HslaToRgba(color.HSLA{H: -60, S: 100, L: 50, A: 1})
	})
	assertPanic(t, "Hue must be between 0 and 360 degrees", func() {
		color.HslaToRgba(color.HSLA{H: 420, S: 100, L: 50, A: 1})
	})
}

func TestHslaToRgbaPanicsInvalidSaturation(t *testing.T) {
	assertPanic(t, "Saturation must be between 0 and 100 percent", func() {
		color.HslaToRgba(color.HSLA{H: 0, S: -50, L: 50, A: 1})
	})
	assertPanic(t, "Saturation must be between 0 and 100 percent", func() {
		color.HslaToRgba(color.HSLA{H: 0, S: 150, L: 50, A: 1})
	})
}

func TestHslaToRgbaPanicsInvalidLightness(t *testing.T) {
	assertPanic(t, "Lightness must be between 0 and 100 percent", func() {
		color.HslaToRgba(color.HSLA{H: 0, S: 100, L: -20, A: 1})
	})
	assertPanic(t, "Lightness must be between 0 and 100 percent", func() {
		color.HslaToRgba(color.HSLA{H: 0, S: 100, L: 120, A: 1})
	})
}

func TestHslaToRgbaPanicsInvalidAlpha(t *testing.T) {
	assertPanic(t, "Alpha must be between 0 and 1", func() {
		color.HslaToRgba(color.HSLA{H: 0, S: 100, L: 50, A: -0.5})
	})
	assertPanic(t, "Alpha must be between 0 and 1", func() {
		color.HslaToRgba(color.HSLA{H: 0, S: 100, L: 50, A: 1.5})
	})
}

// =============================================================================
// RgbaToCmyk
// =============================================================================

func TestRgbaToCmykPanicsInvalid(t *testing.T) {
	tests := []struct {
		name string
		rgba color.RGBA
	}{
		{"r<0", color.RGBA{R: -1, G: 0, B: 0, A: 1}},
		{"r>255", color.RGBA{R: 256, G: 0, B: 0, A: 1}},
		{"g<0", color.RGBA{R: 0, G: -1, B: 0, A: 1}},
		{"g>255", color.RGBA{R: 0, G: 256, B: 0, A: 1}},
		{"b<0", color.RGBA{R: 0, G: 0, B: -1, A: 1}},
		{"b>255", color.RGBA{R: 0, G: 0, B: 256, A: 1}},
		{"a<0", color.RGBA{R: 0, G: 0, B: 0, A: -0.1}},
		{"a>1", color.RGBA{R: 0, G: 0, B: 0, A: 1.1}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			assertPanic(t, "Invalid rgba value", func() {
				color.RgbaToCmyk(tt.rgba)
			})
		})
	}
}

func TestRgbaToCmykConversions(t *testing.T) {
	tests := []struct {
		name  string
		input color.RGBA
		want  color.CMYK
	}{
		{"white", color.RGBA{R: 255, G: 255, B: 255, A: 1}, color.CMYK{C: 0, M: 0, Y: 0, K: 0, A: 1}},
		{"black", color.RGBA{R: 0, G: 0, B: 0, A: 1}, color.CMYK{C: 0, M: 0, Y: 0, K: 100, A: 1}},
		{"red half alpha", color.RGBA{R: 255, G: 0, B: 0, A: 0.5}, color.CMYK{C: 0, M: 100, Y: 100, K: 0, A: 0.5}},
		{"green with alpha", color.RGBA{R: 0, G: 255, B: 0, A: 0.7}, color.CMYK{C: 100, M: 0, Y: 100, K: 0, A: 0.7}},
		{"blue with alpha", color.RGBA{R: 0, G: 0, B: 255, A: 0.3}, color.CMYK{C: 100, M: 100, Y: 0, K: 0, A: 0.3}},
		{"white default alpha", color.RGBA{R: 255, G: 255, B: 255, A: 1}, color.CMYK{C: 0, M: 0, Y: 0, K: 0, A: 1}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := color.RgbaToCmyk(tt.input)
			if got.C != tt.want.C || got.M != tt.want.M || got.Y != tt.want.Y || got.K != tt.want.K || got.A != tt.want.A {
				t.Errorf("RgbaToCmyk(%+v) = %+v, want %+v", tt.input, got, tt.want)
			}
		})
	}
}

// =============================================================================
// CmykToRgba
// =============================================================================

func TestCmykToRgbaConversions(t *testing.T) {
	tests := []struct {
		name                string
		c, m, y, k, a      float64
		wantR, wantG, wantB float64
		wantA               float64
	}{
		{"dark blue", 100, 100, 0, 60.78, 1, 0, 0, 100, 1},
		{"white", 0, 0, 0, 0, 1, 255, 255, 255, 1},
		{"red half alpha", 0, 100, 100, 0, 0.5, 255, 0, 0, 0.5},
		{"green with alpha", 100, 0, 100, 0, 0.7, 0, 255, 0, 0.7},
		{"blue with alpha", 100, 100, 0, 0, 0.3, 0, 0, 255, 0.3},
		{"dark gray", 50, 50, 50, 50, 1, 64, 64, 64, 1},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := color.CmykToRgba(tt.c, tt.m, tt.y, tt.k, tt.a)
			if got.R != tt.wantR || got.G != tt.wantG || got.B != tt.wantB || got.A != tt.wantA {
				t.Errorf("CmykToRgba(%.2f,%.2f,%.2f,%.2f,%.2f) = %+v, want {R:%.0f, G:%.0f, B:%.0f, A:%.2f}",
					tt.c, tt.m, tt.y, tt.k, tt.a, got, tt.wantR, tt.wantG, tt.wantB, tt.wantA)
			}
		})
	}
}

func TestCmykToRgbaClampingInvalidValues(t *testing.T) {
	// Negative C value is clamped to 0
	got := color.CmykToRgba(-1, 100, 100, 0, 1)
	if got.R != 255 || got.G != 0 || got.B != 0 || got.A != 1 {
		t.Errorf("CmykToRgba(-1,100,100,0,1) = %+v, want {255,0,0,1}", got)
	}

	// All 100 gives black
	got = color.CmykToRgba(100, 100, 100, 100, 1)
	if got.R != 0 || got.G != 0 || got.B != 0 || got.A != 1 {
		t.Errorf("CmykToRgba(100,100,100,100,1) = %+v, want {0,0,0,1}", got)
	}

	// Negative alpha is clamped to 0
	got = color.CmykToRgba(100, 100, 100, 100, -1)
	if got.R != 0 || got.G != 0 || got.B != 0 || got.A != 0 {
		t.Errorf("CmykToRgba(100,100,100,100,-1) = %+v, want {0,0,0,0}", got)
	}
}

// =============================================================================
// Integration: Round-trip conversions
// =============================================================================

func TestRoundTripRgbaHslaRgba(t *testing.T) {
	testColors := []color.RGBA{
		{R: 255, G: 0, B: 0, A: 1},       // Red
		{R: 0, G: 255, B: 0, A: 0.5},     // Green with alpha
		{R: 0, G: 0, B: 255, A: 1},       // Blue
		{R: 128, G: 128, B: 128, A: 1},   // Gray
		{R: 255, G: 255, B: 255, A: 0.8}, // White with alpha
	}

	for _, original := range testColors {
		hsla := color.RgbaToHsla(original)
		converted := color.HslaToRgba(hsla)

		if !approxEqual(converted.R, original.R, 0.5) ||
			!approxEqual(converted.G, original.G, 0.5) ||
			!approxEqual(converted.B, original.B, 0.5) ||
			!approxEqual(converted.A, original.A, 0.01) {
			t.Errorf("RGBA->HSLA->RGBA round trip failed: %+v -> %+v -> %+v", original, hsla, converted)
		}
	}
}

func TestRoundTripRgbaCmykRgba(t *testing.T) {
	testColors := []color.RGBA{
		{R: 255, G: 0, B: 0, A: 1},       // Red
		{R: 0, G: 255, B: 0, A: 1},       // Green
		{R: 0, G: 0, B: 255, A: 1},       // Blue
		{R: 100, G: 150, B: 200, A: 0.7}, // Custom color with alpha
	}

	for _, original := range testColors {
		cmyk := color.RgbaToCmyk(original)
		converted := color.CmykToRgba(cmyk.C, cmyk.M, cmyk.Y, cmyk.K, cmyk.A)

		if !approxEqual(converted.R, original.R, 0.5) ||
			!approxEqual(converted.G, original.G, 0.5) ||
			!approxEqual(converted.B, original.B, 0.5) ||
			!approxEqual(converted.A, original.A, 0.01) {
			t.Errorf("RGBA->CMYK->RGBA round trip failed: %+v -> %+v -> %+v", original, cmyk, converted)
		}
	}
}

func TestRoundTripRgbaHexRgba(t *testing.T) {
	testColors := []color.RGBA{
		{R: 255, G: 0, B: 0, A: 1},       // Red
		{R: 0, G: 255, B: 0, A: 0.5},     // Green with alpha
		{R: 128, G: 64, B: 192, A: 0.75}, // Purple with alpha
	}

	for _, original := range testColors {
		hex := color.RgbaToHexA(original)
		converted, err := color.HexaToRgba(hex)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		if converted.R != original.R || converted.G != original.G || converted.B != original.B ||
			!approxEqual(converted.A, original.A, 0.01) {
			t.Errorf("RGBA->HEX->RGBA round trip failed: %+v -> %q -> %+v", original, hex, converted)
		}
	}
}

func TestMultiStepColorTransformations(t *testing.T) {
	originalRgba := color.RGBA{R: 75, G: 150, B: 225, A: 0.9}

	// RGBA -> HSLA
	hsla := color.RgbaToHsla(originalRgba)

	// HSLA -> RGBA -> CMYK
	rgbaFromHsla := color.HslaToRgba(hsla)
	cmyk := color.RgbaToCmyk(rgbaFromHsla)

	// CMYK -> RGBA -> HEX
	rgbaFromCmyk := color.CmykToRgba(cmyk.C, cmyk.M, cmyk.Y, cmyk.K, cmyk.A)
	hex := color.RgbaToHexA(rgbaFromCmyk)

	// HEX -> RGBA
	finalRgba, err := color.HexaToRgba(hex)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if !approxEqual(finalRgba.R, originalRgba.R, 0.5) ||
		!approxEqual(finalRgba.G, originalRgba.G, 0.5) ||
		!approxEqual(finalRgba.B, originalRgba.B, 0.5) ||
		!approxEqual(finalRgba.A, originalRgba.A, 0.01) {
		t.Errorf("multi-step round trip failed: %+v -> %+v", originalRgba, finalRgba)
	}
}

func TestGrayscaleConversions(t *testing.T) {
	grayLevels := []float64{0, 64, 128, 192, 255}

	for _, gray := range grayLevels {
		rgba := color.RGBA{R: gray, G: gray, B: gray, A: 1}

		// Test HSLA conversion
		hsla := color.RgbaToHsla(rgba)
		if hsla.S != 0 {
			t.Errorf("gray(%v): saturation should be 0, got %.2f", gray, hsla.S)
		}

		rgbaFromHsla := color.HslaToRgba(hsla)
		if !approxEqual(rgbaFromHsla.R, gray, 0.5) ||
			!approxEqual(rgbaFromHsla.G, gray, 0.5) ||
			!approxEqual(rgbaFromHsla.B, gray, 0.5) {
			t.Errorf("gray(%v) HSLA round trip: %+v -> %+v -> %+v", gray, rgba, hsla, rgbaFromHsla)
		}

		// Test CMYK conversion
		cmyk := color.RgbaToCmyk(rgba)
		rgbaFromCmyk := color.CmykToRgba(cmyk.C, cmyk.M, cmyk.Y, cmyk.K, cmyk.A)
		if !approxEqual(rgbaFromCmyk.R, gray, 0.5) ||
			!approxEqual(rgbaFromCmyk.G, gray, 0.5) ||
			!approxEqual(rgbaFromCmyk.B, gray, 0.5) {
			t.Errorf("gray(%v) CMYK round trip: %+v -> %+v -> %+v", gray, rgba, cmyk, rgbaFromCmyk)
		}
	}
}

func TestAlphaPreservation(t *testing.T) {
	alphaValues := []float64{0, 0.25, 0.5, 0.75, 1}
	baseColor := color.RGBA{R: 100, G: 150, B: 200, A: 0}

	for _, alpha := range alphaValues {
		rgba := baseColor
		rgba.A = alpha

		// Test through HSLA
		hsla := color.RgbaToHsla(rgba)
		if !approxEqual(hsla.A, alpha, 0.01) {
			t.Errorf("alpha %.2f: HSLA.A = %.2f", alpha, hsla.A)
		}

		// Test through CMYK
		cmyk := color.RgbaToCmyk(rgba)
		if !approxEqual(cmyk.A, alpha, 0.01) {
			t.Errorf("alpha %.2f: CMYK.A = %.2f", alpha, cmyk.A)
		}

		// Test through HEX
		hex := color.RgbaToHexA(rgba)
		rgbaFromHex, err := color.HexaToRgba(hex)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if !approxEqual(rgbaFromHex.A, alpha, 0.01) {
			t.Errorf("alpha %.2f: HEX round trip A = %.2f", alpha, rgbaFromHex.A)
		}
	}
}

func TestEdgeCaseConversions(t *testing.T) {
	edgeCases := []color.RGBA{
		{R: 0, G: 0, B: 0, A: 1},       // Black
		{R: 255, G: 255, B: 255, A: 1},  // White
		{R: 255, G: 0, B: 255, A: 0},    // Magenta with 0 alpha
	}

	for _, c := range edgeCases {
		// Test all conversion paths
		hsla := color.RgbaToHsla(c)
		cmyk := color.RgbaToCmyk(c)
		hex := color.RgbaToHexA(c)

		fromHsla := color.HslaToRgba(hsla)
		fromCmyk := color.CmykToRgba(cmyk.C, cmyk.M, cmyk.Y, cmyk.K, cmyk.A)
		fromHex, err := color.HexaToRgba(hex)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		for _, converted := range []color.RGBA{fromHsla, fromCmyk, fromHex} {
			if !approxEqual(converted.R, c.R, 0.5) ||
				!approxEqual(converted.G, c.G, 0.5) ||
				!approxEqual(converted.B, c.B, 0.5) ||
				!approxEqual(converted.A, c.A, 0.01) {
				t.Errorf("edge case %+v round trip failed: got %+v", c, converted)
			}
		}
	}
}

func TestColorMixingThroughConversions(t *testing.T) {
	color1 := color.RGBA{R: 255, G: 0, B: 0, A: 1} // Red
	color2 := color.RGBA{R: 0, G: 0, B: 255, A: 1} // Blue

	// Convert to HSLA for mixing
	hsla1 := color.RgbaToHsla(color1)
	hsla2 := color.RgbaToHsla(color2)

	// Mix in HSLA space
	mixedHsla := color.HSLA{
		H: (hsla1.H + hsla2.H) / 2,
		S: (hsla1.S + hsla2.S) / 2,
		L: (hsla1.L + hsla2.L) / 2,
		A: (hsla1.A + hsla2.A) / 2,
	}

	mixedRgba := color.HslaToRgba(mixedHsla)
	mixedHex := color.RgbaToHexA(mixedRgba)

	// Verify the mixed color has valid properties
	if mixedRgba.R < 0 {
		t.Errorf("mixed R should be >= 0, got %.2f", mixedRgba.R)
	}
	if mixedRgba.B < 0 {
		t.Errorf("mixed B should be >= 0, got %.2f", mixedRgba.B)
	}

	hexPattern := regexp.MustCompile(`^#[0-9a-f]{8}$`)
	if !hexPattern.MatchString(mixedHex) {
		t.Errorf("mixed hex %q does not match pattern #[0-9a-f]{8}", mixedHex)
	}
}
