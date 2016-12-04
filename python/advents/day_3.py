"""This is day 3 of Advent of Code."""

import os


def is_valid_triangle(triangle_sides):
    """Returns a boolean inicating whether the triangle sides can form a valid
       triangle.

    Args:
        triangle_sides (tuple): The three sides to a triangle.

    Returns:
        (bool): True if the triangle_sides can form valid triangle.
    """
    return (
        (triangle_sides[0] + triangle_sides[1] > triangle_sides[2]) and
        (triangle_sides[1] + triangle_sides[2] > triangle_sides[0]) and
        (triangle_sides[0] + triangle_sides[2] > triangle_sides[1]))


def num_of_valid_triangles(list_of_triangles):
    """Returns the number of valid triangle from a list containing tuples of
       triangle side lengths.

    Args:
        list_of_triangles (list): A list of tuples, which are sides to
                                  triangles.

    Returns:
        (int): Number of valid triangle in list of tuples of triangle side
               lengths.
    """
    num_valid = 0

    for triangle_sides in list_of_triangles:
        if is_valid_triangle(triangle_sides):
            num_valid += 1

    return num_valid


def triangles_from_file(file_path):
    """Returns an array of triangles from file.

    Args:
        file_path (str): The file name, including the path, to the data file.

    Returns:
        (list): A list of tuples, which are sides to triangles.
    """
    list_of_triangle_sides = []

    with open(file_path, 'r') as data_file:
        for line in data_file:
            stripped_line = line.rstrip('\r\n').split(' ')

            list_of_triangle_sides.append(
                tuple([int(num) for num in stripped_line if num != '']))

    return list_of_triangle_sides


def main():
    """The main of the module.
    """
    file_name = os.path.join(
        os.path.join(os.path.dirname(__file__), 'data'), 'day3_input.txt')
    print(num_of_valid_triangles(triangles_from_file(file_name)))


if __name__ == '__main__':
    main()
