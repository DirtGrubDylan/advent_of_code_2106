"""Day 1 of Advent of Code."""
import unittest


GIVEN_INPUT = [
    'R1', 'R3', 'L2', 'L5', 'L2', 'L1', 'R3', 'L4', 'R2', 'L2', 'L4', 'R2',
    'L1', 'R1', 'L2', 'R3', 'L1', 'L4', 'R2', 'L5', 'R3', 'R4', 'L1', 'R2',
    'L1', 'R3', 'L4', 'R5', 'L4', 'L5', 'R5', 'L3', 'R2', 'L3', 'L3', 'R1',
    'R3', 'L4', 'R2', 'R5', 'L4', 'R1', 'L1', 'L1', 'R5', 'L2', 'R1', 'L2',
    'R188', 'L5', 'L3', 'R5', 'R1', 'L2', 'L4', 'R3', 'R5', 'L3', 'R3', 'R45',
    'L4', 'R4', 'R72', 'R2', 'R3', 'L1', 'R1', 'L1', 'L1', 'R192', 'L1', 'L1',
    'L1', 'L4', 'R1', 'L2', 'L5', 'L3', 'R5', 'L3', 'R3', 'L4', 'L3', 'R1',
    'R4', 'L2', 'R2', 'R3', 'L5', 'R3', 'L1', 'R1', 'R4', 'L2', 'L3', 'R1',
    'R3', 'L4', 'L3', 'L4', 'L2', 'L2', 'R1', 'R3', 'L5', 'L1', 'R4', 'R2',
    'L4', 'L1', 'R3', 'R3', 'R1', 'L5', 'L2', 'R4', 'R4', 'R2', 'R1', 'R5',
    'R5', 'L4', 'L1', 'R5', 'R3', 'R4', 'R5', 'R3', 'L1', 'L2', 'L4', 'R1',
    'R4', 'R5', 'L2', 'L3', 'R4', 'L4', 'R2', 'L2', 'L4', 'L2', 'R5', 'R1',
    'R4', 'R3', 'R5', 'L4', 'L4', 'L5', 'L5', 'R3', 'R4', 'L1', 'L3', 'R2',
    'L2', 'R1', 'L3', 'L5', 'R5', 'R5', 'R3', 'L4', 'L2', 'R4', 'R5', 'R1',
    'R4', 'L3']


class TestDay1(unittest.TestCase):
    """This is the unit tests for Day 1.
    """
    def setUp(self):
        self.test_1 = ['R2', 'L3']
        self.test_2 = ['R2', 'R2', 'R2']
        self.test_3 = ['R5', 'L5', 'R5', 'R3']
        self.test_4 = ['R8', 'R4', 'R4', 'R8']

    def test_longest_block_distances(self):
        """Tests to make sure the longest block distance function works.
        """
        self.assertEqual(longest_block_distance(self.test_1), 5)
        self.assertEqual(longest_block_distance(self.test_2), 2)
        self.assertEqual(longest_block_distance(self.test_3), 12)
        self.assertEqual(longest_block_distance(self.test_4[:2]), 12)
        self.assertEqual(longest_block_distance(self.test_4), 8)

    def test_distance_to_first_repeated_block(self):
        """Tests to make sure the longest block distance function works.
        """
        self.assertEqual(distance_to_first_repeated_block(self.test_4), 4)


def longest_block_distance(directions):
    """Interpret given directions.

    Args:
        directions (list): A list of strings indication turn direction and
                           length of distance.
    Returns:
        (int): Distance, in blocks, from origin.
    """
    index = 0
    ns_parity = 1
    ew_parity = 1
    direction_parity_table = {'R': 1, 'L': -1}

    travel_location = [0, 0]

    for direction in directions:
        direction_parity, distance = (
            direction_parity_table[direction[0]], int(direction[1:]))

        if index % 2 == 0: #heading ew
            if ns_parity == 1: #facing north
                ew_parity = direction_parity
            else: #facing south
                ew_parity = -direction_parity

            travel_location[1] += ew_parity * distance
        else: #heading ns
            if ew_parity == 1: #facing east
                ns_parity = -direction_parity
            else: #facing west
                ns_parity = direction_parity

            travel_location[0] += ns_parity * distance

        index += 1

    return sum([abs(num) for num in travel_location])


def distance_to_first_repeated_block(directions):
    """Returns the block distance to the first block visited twice.

    Args:
        directions (list): A list of strings indication turn direction and
                           length of distance.
    Returns:
        (int): Distance, in blocks, from origin to the first block visited
               twice.
    """
    index = 0
    ns_parity = 1
    ew_parity = 1
    direction_parity_table = {'R': 1, 'L': -1}

    travel_location = [0, 0]
    locations_visited = set()
    locations_visited.add(str(travel_location))

    for direction in directions:
        direction_parity = direction_parity_table[direction[0]]
        _, distance = direction[0], int(direction[1:])

        if index % 2 == 0: #heading ew
            if ns_parity == 1: #facing north
                ew_parity = direction_parity
            else: #facing south
                ew_parity = -direction_parity

            old_ew = travel_location[1]
            travel_location[1] += ew_parity * distance

            for num in range(old_ew + ew_parity, travel_location[1] + ew_parity,
                             ew_parity):
                temp_location = [travel_location[0], num]

                if str(temp_location) in locations_visited:
                    return sum([abs(num) for num in temp_location])

                locations_visited.add(str(temp_location))
        else: #heading ns
            if ew_parity == 1: #facing east
                ns_parity = -direction_parity
            else: #facing west
                ns_parity = direction_parity

            old_ns = travel_location[0]
            travel_location[0] += ns_parity * distance

            for num in range(old_ns + ns_parity, travel_location[0] + ns_parity,
                             ns_parity):
                temp_location = [num, travel_location[1]]

                if str(temp_location) in locations_visited:
                    return sum([abs(num) for num in temp_location])

                locations_visited.add(str(temp_location))

        index += 1


if __name__ == '__main__':
    print('Given directions: {}'.format(GIVEN_INPUT))
    print(
        'Longest distance with given directions: {}'.format(
            longest_block_distance(GIVEN_INPUT)))
    print(
        'Distance to first repeated block with given directions: {}'.format(
            distance_to_first_repeated_block(GIVEN_INPUT)))
    # unittest.main()
