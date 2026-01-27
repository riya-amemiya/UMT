package ua

import (
	"regexp"
	"strings"
)

// BrowserInfo contains browser information extracted from a user agent string.
type BrowserInfo struct {
	// Name is the detected browser name.
	// Possible values: "edge", "chrome", "firefox", "safari", "ie", "other".
	Name string
}

// OsInfo contains operating system information extracted from a user agent string.
type OsInfo struct {
	// Name is the detected operating system name.
	// Possible values: "ios", "android", "macos", "windows", "linux", "other".
	Name string
}

// DeviceInfo contains device type information extracted from a user agent string.
type DeviceInfo struct {
	// Type is the detected device type.
	// Possible values: "bot", "mobile", "tablet", "desktop", "other".
	Type string
}

// UserAgentInfo contains the combined parsed information from a user agent string.
type UserAgentInfo struct {
	OS      OsInfo
	Browser BrowserInfo
	Device  DeviceInfo
}

// Pre-compiled regular expressions for browser detection.
var (
	reEdge    = regexp.MustCompile(`(?i)edg(e)?`)
	reIE      = regexp.MustCompile(`(?i)msie|trident`)
	reFirefox = regexp.MustCompile(`(?i)firefox|fxios`)
	reOpera   = regexp.MustCompile(`(?i)opr/`)
	reChrome  = regexp.MustCompile(`(?i)chrome|crios`)
	reSafari  = regexp.MustCompile(`(?i)safari`)
)

// Pre-compiled regular expressions for OS detection.
var (
	reIOS     = regexp.MustCompile(`(?i)iphone|ipad|ipod`)
	reAndroid = regexp.MustCompile(`(?i)android`)
	reMacOS   = regexp.MustCompile(`(?i)mac os x`)
	reWindows = regexp.MustCompile(`(?i)windows|win32`)
	reLinux   = regexp.MustCompile(`(?i)linux`)
)

// Pre-compiled regular expressions for device detection.
var (
	reBot             = regexp.MustCompile(`(?i)bot|googlebot|crawler|spider|robot|crawling`)
	reMobileDevice    = regexp.MustCompile(`(?i)iphone|ipod|webos|blackberry|iemobile|opera mini`)
	reMobileFlag      = regexp.MustCompile(`(?i)mobile`)
	reTabletIPad      = regexp.MustCompile(`(?i)ipad`)
	reDesktop         = regexp.MustCompile(`(?i)windows|macintosh|linux`)
)

// ExtractBrowserFromUserAgent extracts browser information from a user agent string.
//
// Detection order matters: Edge is checked before Chrome because Edge includes
// "Chrome" in its UA string. Opera is checked before Chrome because Opera also
// includes "Chrome". Safari is checked last because Chrome/Firefox on iOS include "Safari".
//
// Returns BrowserInfo with Name set to one of: "edge", "chrome", "firefox", "safari", "ie", "other".
func ExtractBrowserFromUserAgent(uaString string) BrowserInfo {
	if reEdge.MatchString(uaString) {
		return BrowserInfo{Name: "edge"}
	}
	if reIE.MatchString(uaString) {
		return BrowserInfo{Name: "ie"}
	}
	if reFirefox.MatchString(uaString) {
		return BrowserInfo{Name: "firefox"}
	}
	// Opera uses Chromium, so check for OPR before Chrome
	if reOpera.MatchString(uaString) {
		return BrowserInfo{Name: "other"}
	}
	if reChrome.MatchString(uaString) {
		return BrowserInfo{Name: "chrome"}
	}
	// Safari check should be last as Chrome/Firefox on iOS also include Safari in UA
	if reSafari.MatchString(uaString) {
		return BrowserInfo{Name: "safari"}
	}
	return BrowserInfo{Name: "other"}
}

// ExtractOsFromUserAgent extracts operating system information from a user agent string.
//
// iOS devices (iPhone, iPad, iPod) are checked before Android because some
// iOS user agents may include "like Mac OS X".
//
// Returns OsInfo with Name set to one of: "ios", "android", "macos", "windows", "linux", "other".
func ExtractOsFromUserAgent(uaString string) OsInfo {
	if reIOS.MatchString(uaString) {
		return OsInfo{Name: "ios"}
	}
	if reAndroid.MatchString(uaString) {
		return OsInfo{Name: "android"}
	}
	if reMacOS.MatchString(uaString) {
		return OsInfo{Name: "macos"}
	}
	if reWindows.MatchString(uaString) {
		return OsInfo{Name: "windows"}
	}
	if reLinux.MatchString(uaString) {
		return OsInfo{Name: "linux"}
	}
	return OsInfo{Name: "other"}
}

// ExtractDeviceFromUserAgent extracts device type information from a user agent string.
//
// Detection order: bots first, then mobile-specific devices, then Android
// (with mobile/tablet distinction), then iPad/tablet, then desktop.
//
// Returns DeviceInfo with Type set to one of: "bot", "mobile", "tablet", "desktop", "other".
func ExtractDeviceFromUserAgent(uaString string) DeviceInfo {
	if reBot.MatchString(uaString) {
		return DeviceInfo{Type: "bot"}
	}

	if reMobileDevice.MatchString(uaString) {
		return DeviceInfo{Type: "mobile"}
	}

	if reAndroid.MatchString(uaString) {
		if reMobileFlag.MatchString(uaString) {
			return DeviceInfo{Type: "mobile"}
		}
		return DeviceInfo{Type: "tablet"}
	}

	if reTabletIPad.MatchString(uaString) {
		return DeviceInfo{Type: "tablet"}
	}

	if reDesktop.MatchString(uaString) {
		return DeviceInfo{Type: "desktop"}
	}

	return DeviceInfo{Type: "other"}
}

// ParseUserAgent parses a user agent string into structured information
// containing the detected browser, device type, and operating system.
//
// The input string is converted to lowercase before analysis.
//
// Example:
//
//	info := ParseUserAgent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) ...")
//	// info.OS.Name == "windows"
//	// info.Browser.Name == "chrome"
//	// info.Device.Type == "desktop"
func ParseUserAgent(uaString string) UserAgentInfo {
	ua := strings.ToLower(uaString)
	return UserAgentInfo{
		OS:      ExtractOsFromUserAgent(ua),
		Browser: ExtractBrowserFromUserAgent(ua),
		Device:  ExtractDeviceFromUserAgent(ua),
	}
}
