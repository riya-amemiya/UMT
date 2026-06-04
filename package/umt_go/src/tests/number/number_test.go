package number_test

import (
	"strings"
	"testing"

	umt_number "github.com/riya-amemiya/umt-go/src/number"
)

func intPtr(v int) *int { return &v }

func TestFormatNumber(t *testing.T) {
	tests := []struct {
		name    string
		value   float64
		options umt_number.FormatNumberOptions
		want    string
	}{
		{
			name:    "default options in en-US",
			value:   1234567.89,
			options: umt_number.FormatNumberOptions{Locale: "en-US"},
			want:    "1,234,567.89",
		},
		{
			name:    "specified locale de-DE",
			value:   1234567.89,
			options: umt_number.FormatNumberOptions{Locale: "de-DE"},
			want:    "1.234.567,89",
		},
		{
			name:    "percent style",
			value:   0.75,
			options: umt_number.FormatNumberOptions{Style: "percent", Locale: "en-US"},
			want:    "75%",
		},
		{
			name:  "currency USD",
			value: 1234.5,
			options: umt_number.FormatNumberOptions{
				Style: "currency", Currency: "USD", Locale: "en-US",
			},
			want: "$1,234.50",
		},
		{
			name:  "minimumFractionDigits",
			value: 42,
			options: umt_number.FormatNumberOptions{
				MinimumFractionDigits: intPtr(2), Locale: "en-US",
			},
			want: "42.00",
		},
		{
			name:  "maximumFractionDigits",
			value: 3.14158,
			options: umt_number.FormatNumberOptions{
				MaximumFractionDigits: intPtr(2), Locale: "en-US",
			},
			want: "3.14",
		},
		{
			name:    "zero",
			value:   0,
			options: umt_number.FormatNumberOptions{Locale: "en-US"},
			want:    "0",
		},
		{
			name:    "negative number",
			value:   -1234.56,
			options: umt_number.FormatNumberOptions{Locale: "en-US"},
			want:    "-1,234.56",
		},
		{
			name:  "Japanese yen currency",
			value: 1234,
			options: umt_number.FormatNumberOptions{
				Style: "currency", Currency: "JPY", Locale: "ja-JP",
			},
			want: "￥1,234",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := umt_number.FormatNumber(tt.value, tt.options)
			if got != tt.want {
				t.Errorf("FormatNumber(%v, %+v) = %q, want %q", tt.value, tt.options, got, tt.want)
			}
		})
	}
}

func TestFormatNumberNoOptions(t *testing.T) {
	got := umt_number.FormatNumber(1234.56)
	if !strings.Contains(got, "1") {
		t.Errorf("FormatNumber(1234.56) = %q, want a string containing %q", got, "1")
	}
}

func TestToOrdinal(t *testing.T) {
	tests := []struct {
		value int
		want  string
	}{
		{1, "1st"}, {21, "21st"}, {31, "31st"}, {101, "101st"}, {121, "121st"},
		{2, "2nd"}, {22, "22nd"}, {32, "32nd"}, {102, "102nd"}, {122, "122nd"},
		{3, "3rd"}, {23, "23rd"}, {33, "33rd"}, {103, "103rd"}, {123, "123rd"},
		{11, "11th"}, {12, "12th"}, {13, "13th"},
		{111, "111th"}, {112, "112th"}, {113, "113th"},
		{4, "4th"}, {5, "5th"}, {6, "6th"}, {7, "7th"}, {8, "8th"}, {9, "9th"},
		{10, "10th"}, {20, "20th"}, {100, "100th"},
		{0, "0th"},
	}

	for _, tt := range tests {
		t.Run(tt.want, func(t *testing.T) {
			if got := umt_number.ToOrdinal(tt.value); got != tt.want {
				t.Errorf("ToOrdinal(%d) = %q, want %q", tt.value, got, tt.want)
			}
		})
	}
}

func TestToPercentage(t *testing.T) {
	tests := []struct {
		name     string
		value    float64
		total    float64
		decimals []int
		want     float64
	}{
		{name: "25/100", value: 25, total: 100, want: 25},
		{name: "50/100", value: 50, total: 100, want: 50},
		{name: "100/100", value: 100, total: 100, want: 100},
		{name: "1/3 default", value: 1, total: 3, want: 33.33},
		{name: "2/3 default", value: 2, total: 3, want: 66.67},
		{name: "1/3 decimals 0", value: 1, total: 3, decimals: []int{0}, want: 33},
		{name: "1/3 decimals 1", value: 1, total: 3, decimals: []int{1}, want: 33.3},
		{name: "1/3 decimals 4", value: 1, total: 3, decimals: []int{4}, want: 33.3333},
		{name: "0/0", value: 0, total: 0, want: 0},
		{name: "5/0", value: 5, total: 0, want: 0},
		{name: "-5/0", value: -5, total: 0, want: 0},
		{name: "0/100", value: 0, total: 100, want: 0},
		{name: "150/100", value: 150, total: 100, want: 150},
		{name: "200/100", value: 200, total: 100, want: 200},
		{name: "-25/100", value: -25, total: 100, want: -25},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := umt_number.ToPercentage(tt.value, tt.total, tt.decimals...)
			if got != tt.want {
				t.Errorf("ToPercentage(%v, %v, %v) = %v, want %v", tt.value, tt.total, tt.decimals, got, tt.want)
			}
		})
	}
}
