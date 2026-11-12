from src.random.random_uuid import _bytes_to_uuid


def test_formats_sequential_bytes_in_canonical_layout():
    assert _bytes_to_uuid(bytes(range(16))) == "00010203-0405-0607-0809-0a0b0c0d0e0f"


def test_formats_high_bytes_as_lowercase_hex():
    assert _bytes_to_uuid(bytes([0xFF] * 16)) == "ffffffff-ffff-ffff-ffff-ffffffffffff"


def test_pads_low_nibbles_with_zero():
    assert _bytes_to_uuid(bytes([0x0A] * 16)) == "0a0a0a0a-0a0a-0a0a-0a0a-0a0a0a0a0a0a"
