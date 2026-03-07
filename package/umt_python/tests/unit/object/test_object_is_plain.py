import unittest
from datetime import datetime

from src.object import object_is_plain


class TestObjectIsPlain(unittest.TestCase):
    def test_should_return_true_for_object_literals(self):
        self.assertTrue(object_is_plain({}))
        self.assertTrue(object_is_plain({"a": 1}))

    def test_should_return_false_for_non_objects(self):
        self.assertFalse(object_is_plain(None))
        self.assertFalse(object_is_plain(1))
        self.assertFalse(object_is_plain("string"))
        self.assertFalse(object_is_plain(True))

    def test_should_return_false_for_arrays(self):
        self.assertFalse(object_is_plain([]))

    def test_should_return_false_for_date_objects(self):
        self.assertFalse(object_is_plain(datetime.now()))

    def test_should_return_false_for_set(self):
        self.assertFalse(object_is_plain(set()))

    def test_should_return_false_for_custom_classes(self):
        class MyClass:
            pass

        self.assertFalse(object_is_plain(MyClass()))

    def test_should_return_false_for_functions(self):
        def my_func():
            return 1

        self.assertFalse(object_is_plain(my_func))
        self.assertFalse(object_is_plain(lambda: 1))
