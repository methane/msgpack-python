#cython: embedsignature=True, c_string_encoding=ascii, language_level=3
#cython: freethreading_compatible = True
import cython
import datetime
cdef object utc = datetime.timezone.utc
cdef object epoch = datetime.datetime(1970, 1, 1, tzinfo=utc)
cdef object timedelta = datetime.timedelta

include "_packer.pyx"
include "_unpacker.pyx"
