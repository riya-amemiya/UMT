package color

import (
	"fmt"
	"math"
	"regexp"
	"strconv"
	"strings"
)

// RGBA represents a color in RGBA color space.
type RGBA struct {
	R, G, B float64 // 0-255
	A       float64 // 0-1
}

// HSLA represents a color in HSLA color space.
type HSLA struct {
	H float64 // 0-360
	S float64 // 0-100
	L float64 // 0-100
	A float64 // 0-1
}

// CMYK represents a color in CMYK color space.
type CMYK struct {
	C, M, Y, K float64 // 0-100
	A           float64 // 0-1
}

func roundTo(v float64, precision int) float64 {
	p := math.Pow(10, float64(precision))
	return math.Round(v*p) / p
}

// HexaToRgba converts a hex color string to RGBA.
// Supports 3, 6, or 8 digit hex codes with #.
func HexaToRgba(hex string) (RGBA, error) {
	re := regexp.MustCompile(`^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$`)
	if !re.MatchString(hex) {
		return RGBA{}, fmt.Errorf("invalid hex code")
	}

	hexCode := strings.TrimPrefix(hex, "#")

	// Convert 3-digit to 6-digit
	if len(hexCode) == 3 {
		expanded := ""
		for _, c := range hexCode {
			expanded += string(c) + string(c)
		}
		hexCode = expanded
	}

	r, _ := strconv.ParseInt(hexCode[0:2], 16, 64)
	g, _ := strconv.ParseInt(hexCode[2:4], 16, 64)
	b, _ := strconv.ParseInt(hexCode[4:6], 16, 64)

	var alphaValue int64 = 255
	if len(hexCode) == 8 {
		alphaValue, _ = strconv.ParseInt(hexCode[6:8], 16, 64)
	}

	a := roundTo(float64(alphaValue)/255.0, 2)

	return RGBA{R: float64(r), G: float64(g), B: float64(b), A: a}, nil
}

// RgbaToHexA converts RGBA color to hex string including alpha channel.
func RgbaToHexA(rgba RGBA) string {
	hex := func(x int) string {
		h := strconv.FormatInt(int64(x), 16)
		if len(h) == 1 {
			return "0" + h
		}
		return h
	}

	r := int(rgba.R)
	g := int(rgba.G)
	b := int(rgba.B)
	a := int(math.Round(rgba.A * 255))

	return "#" + hex(r) + hex(g) + hex(b) + hex(a)
}

// RgbaToHsla converts RGBA to HSLA color space.
func RgbaToHsla(rgba RGBA) HSLA {
	rPrime := rgba.R / 255.0
	gPrime := rgba.G / 255.0
	bPrime := rgba.B / 255.0

	maxVal := math.Max(rPrime, math.Max(gPrime, bPrime))
	minVal := math.Min(rPrime, math.Min(gPrime, bPrime))
	diff := maxVal - minVal

	l := (maxVal + minVal) / 2.0

	var s float64
	if diff == 0 {
		s = 0
	} else {
		s = diff / (1.0 - math.Abs(maxVal+minVal-1.0))
	}

	var h float64
	if diff != 0 {
		switch maxVal {
		case rPrime:
			h = (gPrime - bPrime) / diff
			if gPrime < bPrime {
				h += 6
			}
		case gPrime:
			h = (bPrime-rPrime)/diff + 2
		case bPrime:
			h = (rPrime-gPrime)/diff + 4
		}
		h *= 60
	}

	return HSLA{
		H: roundTo(h, 2),
		S: roundTo(s*100, 2),
		L: roundTo(l*100, 2),
		A: rgba.A,
	}
}

// HslaToRgba converts HSLA to RGBA color space.
func HslaToRgba(hsla HSLA) RGBA {
	h := math.Mod(hsla.H, 360) / 360.0
	s := math.Max(0, math.Min(hsla.S, 100)) / 100.0
	l := math.Max(0, math.Min(hsla.L, 100)) / 100.0
	a := math.Max(0, math.Min(1, hsla.A))

	var r, g, b float64

	if s == 0 {
		r = l
		g = l
		b = l
	} else {
		hueToRgb := func(p, q, t float64) float64 {
			if t < 0 {
				t += 1
			}
			if t > 1 {
				t -= 1
			}
			if t < 1.0/6.0 {
				return p + (q-p)*6*t
			}
			if t < 1.0/2.0 {
				return q
			}
			if t < 2.0/3.0 {
				return p + (q-p)*(2.0/3.0-t)*6
			}
			return p
		}

		var q float64
		if l < 0.5 {
			q = l * (1 + s)
		} else {
			q = l + s - l*s
		}
		p := 2*l - q

		r = hueToRgb(p, q, h+1.0/3.0)
		g = hueToRgb(p, q, h)
		b = hueToRgb(p, q, h-1.0/3.0)
	}

	return RGBA{
		R: roundTo(r*255, 2),
		G: roundTo(g*255, 2),
		B: roundTo(b*255, 2),
		A: a,
	}
}

// RgbaToCmyk converts RGBA to CMYK color model.
func RgbaToCmyk(rgba RGBA) CMYK {
	rPrime := rgba.R / 255.0
	gPrime := rgba.G / 255.0
	bPrime := rgba.B / 255.0

	k := 1.0 - math.Max(rPrime, math.Max(gPrime, bPrime))

	var c, m, y float64
	if k < 1.0 {
		c = (1.0 - rPrime - k) / (1.0 - k)
		m = (1.0 - gPrime - k) / (1.0 - k)
		y = (1.0 - bPrime - k) / (1.0 - k)
	}

	return CMYK{
		C: roundTo(c*100, 2),
		M: roundTo(m*100, 2),
		Y: roundTo(y*100, 2),
		K: roundTo(k*100, 2),
		A: rgba.A,
	}
}

// CmykToRgba converts CMYK to RGBA color space.
func CmykToRgba(c, m, y, k float64, a float64) RGBA {
	cVal := math.Max(0, math.Min(100, c)) / 100.0
	mVal := math.Max(0, math.Min(100, m)) / 100.0
	yVal := math.Max(0, math.Min(100, y)) / 100.0
	kVal := math.Max(0, math.Min(100, k)) / 100.0

	r := 255 * (1 - cVal) * (1 - kVal)
	g := 255 * (1 - mVal) * (1 - kVal)
	b := 255 * (1 - yVal) * (1 - kVal)

	return RGBA{
		R: math.Round(r),
		G: math.Round(g),
		B: math.Round(b),
		A: math.Max(0, math.Min(1, a)),
	}
}
