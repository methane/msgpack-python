from pytest import raises


def test_cmsgpack_module_symbols():
    from msgpack import _cmsgpack

    assert _cmsgpack.pack
    assert _cmsgpack.packb
    assert _cmsgpack.unpack
    assert _cmsgpack.Packer
    assert _cmsgpack.Unpacker
    assert _cmsgpack.unpackb
    assert _cmsgpack.BufferFull
    assert _cmsgpack.ExtraData
    assert _cmsgpack.FormatError
    assert _cmsgpack.OutOfData
    assert _cmsgpack.StackError


def test_packer_is_rust_wrapped():
    from msgpack import _cmsgpack
    from msgpack.fallback import Packer as FallbackPacker

    packer = _cmsgpack.Packer()

    assert type(packer) is _cmsgpack.Packer
    assert not isinstance(packer, FallbackPacker)
    assert packer.pack([1, 2, 3]) == b"\x93\x01\x02\x03"


def test_unpacker_is_rust_wrapped():
    from msgpack import _cmsgpack
    from msgpack.fallback import Unpacker as FallbackUnpacker

    unpacker = _cmsgpack.Unpacker()
    unpacker.feed(b"\x93\x01\x02\x03")

    assert type(unpacker) is _cmsgpack.Unpacker
    assert not isinstance(unpacker, FallbackUnpacker)
    assert unpacker.unpack() == [1, 2, 3]


def test_default_read_extended_type():
    from msgpack import _cmsgpack

    with raises(NotImplementedError, match="Cannot decode extended type with typecode=1"):
        _cmsgpack.default_read_extended_type(1, b"data")


def test_top_level_helpers_use_rust_backend():
    import io
    import os

    import msgpack
    from msgpack import _cmsgpack

    if os.environ.get("MSGPACK_PUREPYTHON"):
        assert msgpack.pack is not _cmsgpack.pack
        assert msgpack.packb is not _cmsgpack.packb
        assert msgpack.unpack is not _cmsgpack.unpack
        assert msgpack.unpackb is not _cmsgpack.unpackb
    else:
        assert msgpack.pack is _cmsgpack.pack
        assert msgpack.packb is _cmsgpack.packb
        assert msgpack.unpack is _cmsgpack.unpack
        assert msgpack.unpackb is _cmsgpack.unpackb

    stream = io.BytesIO()
    msgpack.pack({"value": 1}, stream)
    stream.seek(0)

    assert msgpack.packb([1, 2, 3]) == b"\x93\x01\x02\x03"
    assert msgpack.unpack(stream) == {"value": 1}
