import numpy as np

def dag_this(func):
    def wrapper(*args, **kwargs):
        print('Dagging this function')
        return func(*args, **kwargs)
    return wrapper

def check_if_dagged(func):
    return hasattr(func, '__dagged__')

@dag_this
def add(a, b):
    return a + b