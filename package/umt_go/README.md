# UMT Go

Partial Go port of TypeScript [`umt`](https://github.com/riya-amemiya/UMT/tree/main/package/main). `package/main` is the source of truth; this package mirrors many of those modules under `src/` with Go-style names. It is not a line-for-line API clone — several signatures differ (see IP helpers below).

Module path: `github.com/riya-amemiya/umt-go`. Import packages from `src/<name>` (there is no root `math` package).

```bash
go get github.com/riya-amemiya/umt-go
```

Requires Go 1.24.1 (`go.mod`). There is no GitHub Actions workflow for this package; run the Makefile targets locally.

## Quick start

```go
package main

import (
	"fmt"

	"github.com/riya-amemiya/umt-go/src/ip"
	"github.com/riya-amemiya/umt-go/src/math"
)

func main() {
	n, err := math.Random(10) // inclusive 0..10
	if err != nil {
		panic(err)
	}
	fmt.Println(n)

	n = math.MustRandom(10, 5) // inclusive 5..10; panics on error

	long, err := ip.IpToLong("192.168.1.1")
	if err != nil {
		panic(err)
	}
	fmt.Println(long)          // 3232235777
	fmt.Println(ip.LongToIp(long)) // "192.168.1.1"
}
```

## Modules

Public code lives in `src/<package>`. Tests are `src/tests/<package>/*_test.go` and import `github.com/riya-amemiya/umt-go/src/<package>`.

| Package | Path | Examples |
| --- | --- | --- |
| `advance` | `src/advance` | `RangeAdvance` |
| `array` | `src/array` | `Chunk`, `Unique`, `UltraNumberSort` |
| `async` | `src/async` | `Sleep`, `Parallel`, `Timeout` |
| `color` | `src/color` | `HexaToRgba`, `RgbaToHsla` |
| `consts` | `src/consts` | `OneDayMs`, HTTP status maps |
| `cryptoutil` | `src/cryptoutil` | `EncodeBase32`, `DecodeBase58` |
| `datastructure` | `src/datastructure` | `LRUCache`, `TTLCache`, `PriorityQueue` |
| `date` | `src/date` | `StartOf`, `IsBusinessDay`, `AddDuration` |
| `errutil` | `src/errutil` | Result helpers |
| `function` | `src/function` | `Debounce`, `Memoize`, `Throttle` |
| `ip` | `src/ip` | `IpToLong`, `IsInRange`, `IsPrivateIp` |
| `iterator` | `src/iterator` | `LazyMap`, `LazyFilter`, `LazyTake` |
| `maputil` | `src/maputil` | `GroupByToMap`, `ZipToMap` |
| `math` | `src/math` | `Random`, `MustRandom`, `GCD` |
| `number` | `src/number` | `FormatNumber`, `ToOrdinal` |
| `object` | `src/object` | `Get`, `DeepClone`, `FlattenObject` |
| `predicate` | `src/predicate` | `Every`, `Some`, `IsNullish` |
| `random` | `src/random` | `RandomChoice`, `SeededRandom` |
| `simple` | `src/simple` | `BirthdaySimple`, `NowSimple` |
| `str` | `src/str` | `FormatString`, `Slugify` |
| `timeutil` | `src/timeutil` | `ConvertTime` |
| `tool` | `src/tool` | `Pipe`, `Unwrap`, `ParseJson` |
| `ua` | `src/ua` | `ParseUserAgent` |
| `unit` | `src/unit` | `ToCelsius`, `ToKelvin` |
| `urlutil` | `src/urlutil` | `BuildUrl`, `ParseQueryString` |
| `validate` | `src/validate` | `IsNumber`, `ArrayOf` |

Not every TypeScript helper is ported. There is no `IsBetween`, `AddBusinessDays`, `FromUnix` / `ToUnix`, `WeekOfYear`, `GetQuarter`, or `IsSame`.

## Date helpers

Calendar helpers in `src/date`. Week boundaries are Sunday-start (`time.Weekday`, Sunday=0), matching TypeScript `Date#getDay()`, not ISO weeks.

| Function | Notes |
| --- | --- |
| `StartOf` / `EndOf` | `DateBoundaryUnit`: second, minute, hour, day, week, month, quarter, year. Unknown unit returns the date unchanged. `EndOf` uses millisecond `.999`. |
| `AddDuration` / `SubDuration` / `Diff` | Fixed units (`ms`, `s`, `m`, `h`, `d`, `w`) use millisecond arithmetic. `M` / `y` are calendar-aware and clamp end-of-month (Jan 31 + 1 month → Feb 28/29). |
| `IsWeekend` / `IsSameDay` / `IsBusinessDay` | Compared in each value's own location. `IsBusinessDay` treats optional holidays as calendar days. |

## IP helpers

IPv4 dotted-decimal only. Unlike TypeScript, most helpers validate input. Several signatures **do not** match `package/main/src/IP`:

| Function | Go | TypeScript / Python / Rust |
| --- | --- | --- |
| `CidrToLong` | `CidrToLong("192.168.1.0/24")` → `[2]int64{start, end}` | prefix `0`–`32` → mask number |
| `IsInRange` | `IsInRange(ip, "192.168.1.0/24")` | `(ip, network, cidrPrefix)` |
| `GetNetworkAddress` | dotted string (`"192.168.1.0"`) | unsigned 32-bit number |
| `CidrToSubnetMask` | panics if prefix is outside `0`–`32` | Python/Rust return an error |
| `LongToIp` | panics if the value is outside `0..0xFFFFFFFF` | Python returns a string; Rust always returns `String` |

Other constraints (verified against `src/ip/ip.go` and `src/tests/ip/ip_test.go`):

- Leading-zero octets (`"192.168.01.1"`) are rejected.
- `SubnetMaskToCidr` requires contiguous `1` bits then `0` bits (`"255.0.255.0"` errors). TypeScript `subnetMaskToCidr` only counts set bits.
- `IsPrivateIp` is RFC 1918 only (`10/8`, `172.16/12`, `192.168/16`). Loopback and link-local are not private. Invalid input returns `false` (no error).
- `GetIpClass("0.0.0.0")` is `""`.

```go
import "github.com/riya-amemiya/umt-go/src/ip"

long, _ := ip.IpToLong("192.168.1.1")           // 3232235777
_ = ip.LongToIp(3232235777)                     // "192.168.1.1"
_ = ip.CidrToSubnetMask(24)                     // "255.255.255.0"
in, _ := ip.IsInRange("192.168.1.2", "192.168.1.0/24")
net, _ := ip.GetNetworkAddress("192.168.1.1", "255.255.255.0") // "192.168.1.0"
```

## Development

```bash
cd package/umt_go
make fmt      # go fmt ./...
make check    # go vet ./...
make test     # go test -v -race ./...
make build    # go build ./...
make coverage
make all      # fmt, check, test, build
```

## Versioning

Semantic Versioning via Git tags. Pin with Go modules:

```bash
go get github.com/riya-amemiya/umt-go@v1.0.0
```

## License

MIT License
