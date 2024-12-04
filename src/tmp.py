def count_word_occurrences(grid, word):
    rows, cols = len(grid), len(grid[0])
    word_length = len(word)
    directions = [
        (0, 1),  # right
        (1, 0),  # down
        (0, -1),  # left
        (-1, 0),  # up
        (1, 1),  # down-right
        (1, -1),  # down-left
        (-1, 1),  # up-right
        (-1, -1),  # up-left
    ]

    def is_valid(x, y):
        return 0 <= x < rows and 0 <= y < cols

    def check_direction(x, y, dx, dy):
        for i in range(word_length):
            nx, ny = x + i * dx, y + i * dy
            if not is_valid(nx, ny) or grid[nx][ny] != word[i]:
                return False
        return True

    count = 0
    for i in range(rows):
        for j in range(cols):
            for dx, dy in directions:
                if check_direction(i, j, dx, dy):
                    count += 1

    return count


# Example usage
word_search = [
    "MMMSXXMASM",
    "MSAMXMSMSA",
    "AMXSXMAAMM",
    "MSAMASMSMX",
    "XMASAMXAMM",
    "XXAMMXXAMA",
    "SMSMSASXSS",
    "SAXAMASAAA",
    "MAMMMXMMMM",
    "MXMXAXMASX",
]
word_to_find = "XMAS"

# Convert the grid to a list of lists
grid = [list(row) for row in word_search]

result = count_word_occurrences(grid, word_to_find)
print(f"The word '{word_to_find}' occurs {result} times.")
