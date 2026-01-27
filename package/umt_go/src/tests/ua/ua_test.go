package ua_test

import (
	"testing"

	"github.com/riya-amemiya/umt-go/src/ua"
)

func TestParseUserAgentChrome(t *testing.T) {
	uaStr := "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
	info := ua.ParseUserAgent(uaStr)

	if info.Browser.Name != "chrome" {
		t.Errorf("Browser = %q, want %q", info.Browser.Name, "chrome")
	}
	if info.OS.Name != "windows" {
		t.Errorf("OS = %q, want %q", info.OS.Name, "windows")
	}
	if info.Device.Type != "desktop" {
		t.Errorf("Device = %q, want %q", info.Device.Type, "desktop")
	}
}

func TestParseUserAgentSafariIOS(t *testing.T) {
	uaStr := "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1"
	info := ua.ParseUserAgent(uaStr)

	if info.Browser.Name != "safari" {
		t.Errorf("Browser = %q, want %q", info.Browser.Name, "safari")
	}
	if info.OS.Name != "ios" {
		t.Errorf("OS = %q, want %q", info.OS.Name, "ios")
	}
	if info.Device.Type != "mobile" {
		t.Errorf("Device = %q, want %q", info.Device.Type, "mobile")
	}
}

func TestParseUserAgentFirefox(t *testing.T) {
	uaStr := "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/115.0"
	info := ua.ParseUserAgent(uaStr)

	if info.Browser.Name != "firefox" {
		t.Errorf("Browser = %q, want %q", info.Browser.Name, "firefox")
	}
	if info.OS.Name != "linux" {
		t.Errorf("OS = %q, want %q", info.OS.Name, "linux")
	}
	if info.Device.Type != "desktop" {
		t.Errorf("Device = %q, want %q", info.Device.Type, "desktop")
	}
}

func TestParseUserAgentGooglebot(t *testing.T) {
	uaStr := "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"
	info := ua.ParseUserAgent(uaStr)

	if info.Device.Type != "bot" {
		t.Errorf("Device = %q, want %q", info.Device.Type, "bot")
	}
}

func TestParseUserAgentEdge(t *testing.T) {
	uaStr := "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0"
	info := ua.ParseUserAgent(uaStr)

	if info.Browser.Name != "edge" {
		t.Errorf("Browser = %q, want %q", info.Browser.Name, "edge")
	}
	if info.OS.Name != "windows" {
		t.Errorf("OS = %q, want %q", info.OS.Name, "windows")
	}
}

func TestParseUserAgentAndroidMobile(t *testing.T) {
	uaStr := "Mozilla/5.0 (Linux; Android 13; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Mobile Safari/537.36"
	info := ua.ParseUserAgent(uaStr)

	if info.OS.Name != "android" {
		t.Errorf("OS = %q, want %q", info.OS.Name, "android")
	}
	if info.Device.Type != "mobile" {
		t.Errorf("Device = %q, want %q", info.Device.Type, "mobile")
	}
}

func TestParseUserAgentAndroidTablet(t *testing.T) {
	uaStr := "Mozilla/5.0 (Linux; Android 13; SM-X800) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
	info := ua.ParseUserAgent(uaStr)

	if info.OS.Name != "android" {
		t.Errorf("OS = %q, want %q", info.OS.Name, "android")
	}
	if info.Device.Type != "tablet" {
		t.Errorf("Device = %q, want %q", info.Device.Type, "tablet")
	}
}

func TestParseUserAgentIPad(t *testing.T) {
	uaStr := "Mozilla/5.0 (iPad; CPU OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1"
	info := ua.ParseUserAgent(uaStr)

	if info.OS.Name != "ios" {
		t.Errorf("OS = %q, want %q", info.OS.Name, "ios")
	}
}

func TestParseUserAgentMacOS(t *testing.T) {
	uaStr := "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
	info := ua.ParseUserAgent(uaStr)

	if info.OS.Name != "macos" {
		t.Errorf("OS = %q, want %q", info.OS.Name, "macos")
	}
	if info.Device.Type != "desktop" {
		t.Errorf("Device = %q, want %q", info.Device.Type, "desktop")
	}
}

func TestExtractBrowserFromUserAgent(t *testing.T) {
	tests := []struct {
		name     string
		ua       string
		expected string
	}{
		{"chrome", "chrome/120", "chrome"},
		{"firefox", "firefox/115", "firefox"},
		{"edge", "edg/120", "edge"},
		{"safari", "safari/604", "safari"},
		{"ie", "msie 11.0", "ie"},
		{"ie trident", "trident/7.0", "ie"},
		{"unknown", "some-unknown-browser", "other"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := ua.ExtractBrowserFromUserAgent(tt.ua)
			if result.Name != tt.expected {
				t.Errorf("ExtractBrowserFromUserAgent(%q) = %q, want %q", tt.ua, result.Name, tt.expected)
			}
		})
	}
}

func TestExtractOsFromUserAgent(t *testing.T) {
	tests := []struct {
		name     string
		ua       string
		expected string
	}{
		{"ios", "iphone", "ios"},
		{"android", "android", "android"},
		{"macos", "mac os x", "macos"},
		{"windows", "windows nt", "windows"},
		{"linux", "linux x86_64", "linux"},
		{"unknown", "some-unknown-os", "other"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := ua.ExtractOsFromUserAgent(tt.ua)
			if result.Name != tt.expected {
				t.Errorf("ExtractOsFromUserAgent(%q) = %q, want %q", tt.ua, result.Name, tt.expected)
			}
		})
	}
}

func TestExtractDeviceFromUserAgent(t *testing.T) {
	tests := []struct {
		name     string
		ua       string
		expected string
	}{
		{"bot", "googlebot/2.1", "bot"},
		{"mobile", "iphone", "mobile"},
		{"desktop windows", "windows nt 10.0", "desktop"},
		{"desktop mac", "macintosh", "desktop"},
		{"unknown", "some-unknown-device", "other"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := ua.ExtractDeviceFromUserAgent(tt.ua)
			if result.Type != tt.expected {
				t.Errorf("ExtractDeviceFromUserAgent(%q) = %q, want %q", tt.ua, result.Type, tt.expected)
			}
		})
	}
}
