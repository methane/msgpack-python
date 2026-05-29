from pytest import raises


def test_cmsgpack_module_symbols():
    from msgpack import _cmsgpack

    assert _cmsgpack.Packer
    assert _cmsgpack.Unpacker
    assert _cmsgpack.unpackb
    assert _cmsgpack.BufferFull
    assert _cmsgpack.ExtraData
    assert _cmsgpack.FormatError
    assert _cmsgpack.OutOfData
    assert _cmsgpack.StackError


def test_default_read_extended_type():
    from msgpack import _cmsgpack

    with raises(NotImplementedError, match="Cannot decode extended type with typecode=1"):
        _cmsgpack.default_read_extended_type(1, b"data")
