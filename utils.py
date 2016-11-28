'''
    Generic helpers for all days
'''

from os import path

def get_input_filename(day):
    return path.join('input', str(day)+'.txt')
