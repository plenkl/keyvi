

    def get (self, key, default = None):
        assert isinstance(key, bytes), 'arg in_0 wrong type'
    
        cdef _Match * _r = new _Match(deref(self.inst.get())[(<const_char *>key)])

        if _r.IsEmpty():
            return default
        cdef Match py_result = Match.__new__(Match)
        py_result.inst = shared_ptr[_Match](_r)
        return py_result


    def __getitem__ (self, key):
        assert isinstance(key, bytes), 'arg in_0 wrong type'
    
        cdef _Match * _r = new _Match(deref(self.inst.get())[(<const_char *>key)])

        if _r.IsEmpty():
            raise KeyError(key)
        cdef Match py_result = Match.__new__(Match)
        py_result.inst = shared_ptr[_Match](_r)
        return py_result

    def _key_iterator_wrapper(self, iterator):
        for m in iterator:
            yield m.GetMatchedString()

    def _value_iterator_wrapper(self, iterator):
        for m in iterator:
            yield m.GetRawValueAsString()

    def _item_iterator_wrapper(self, iterator):
        for m in iterator:
            yield (m.GetMatchedString(), m.GetRawValueAsString())

    def GetAllKeys(self):
        cdef _MatchIteratorPair _r = self.inst.get().GetAllItems()
        cdef MatchIterator py_result = MatchIterator.__new__(MatchIterator)
        py_result.it = _r.begin()
        py_result.end = _r.end()
        return self._key_iterator_wrapper(py_result)

    def GetAllValues(self):
        cdef _MatchIteratorPair _r = self.inst.get().GetAllItems()
        cdef MatchIterator py_result = MatchIterator.__new__(MatchIterator)
        py_result.it = _r.begin()
        py_result.end = _r.end()
        return self._value_iterator_wrapper(py_result)

    def GetAllItems(self):
        cdef _MatchIteratorPair _r = self.inst.get().GetAllItems()
        cdef MatchIterator py_result = MatchIterator.__new__(MatchIterator)
        py_result.it = _r.begin()
        py_result.end = _r.end()
        return self._item_iterator_wrapper(py_result)
