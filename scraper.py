# use this file to scrap for problem descriptions

from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
import time

problems = ["4sum", "3sum", "3sum-closest", "remove-duplicates-from-sorted-array",
            "combination-sum-ii", "decode-string", "meeting-rooms-ii",
            "merge-two-sorted-lists", "palindrome-permutation", "merge-intervals",
            "house-robber-iii", "edit-distance", "daily-temperatures",
            "remove-element", "min-stack", "asteroid-collision", "plus-one",
            "word-break", "remove-duplicates-from-sorted-list", "jump-game",
            "task-scheduler", "unique-paths", "flatten-binary-tree-to-linked-list",
            "find-duplicate-file-in-system", "reverse-words-in-a-string-ii",
            "maximum-subarray", "move-zeroes", "maximal-square",
            "container-with-most-water", "integer-to-roman",
            "permutation-in-string", "best-time-to-buy-and-sell-stock",
            "diameter-of-binary-tree", "divide-two-integers", "swap-nodes-in-pairs",
            "generate-parentheses", "course-schedule-ii",
            "search-in-rotated-sorted-array", "longest-common-prefix",
            "reconstruct-itinerary", "binary-tree-level-order-traversal",
            "palindromic-substrings", "word-search", "implement-trie-prefix-tree",
            "kth-largest-element-in-an-array", "coin-change", "invert-binary-tree",
            "maximum-product-subarray", "group-anagrams", "find-leaves-of-binary-tree",
            "valid-sudoku", "merge-sorted-array", "lowest-common-ancestor-of-a-binary-tree",
            "search-insert-position", "longest-substring-without-repeating-characters",
            "target-sum", "number-of-islands",
            "construct-binary-tree-from-preorder-and-inorder-traversal",
            "find-the-index-of-the-first-occurrence-in-a-string",
            "longest-palindromic-substring", "binary-tree-right-side-view",
            "two-sum", "add-two-numbers", "string-to-integer-atoi", "lru-cache",
            "jump-game-ii", "permutations-ii", "fizz-buzz", "valid-anagram",
            "perfect-squares", "palindrome-number", "sort-colors", "rotate-array",
            "design-tic-tac-toe", "longest-increasing-subsequence",
            "find-first-and-last-position-of-element-in-sorted-array",
            "combination-sum", "letter-combinations-of-a-phone-number",
            "subarray-sum-equals-k", "insert-delete-getrandom-o1", "add-binary",
            "word-ladder", "remove-nth-node-from-end-of-list", "course-schedule",
            "binary-tree-zigzag-level-order-traversal", "product-of-array-except-self",
            "spiral-matrix", "sort-list", "kth-smallest-element-in-a-bst",
            "validate-binary-search-tree", "find-all-anagrams-in-a-string",
            "path-sum-iii", "roman-to-integer", "partition-equal-subset-sum",
            "next-permutation", "valid-parentheses", "rotate-image", "game-of-life",
            "insert-interval", "house-robber", "reverse-string", "subsets",
            "search-a-2d-matrix-ii", "longest-consecutive-sequence",
            "reverse-linked-list", "min-cost-climbing-stairs", "top-k-frequent-elements",
            "reverse-integer", "find-the-duplicate-number", "permutations",
            "minimum-path-sum", "climbing-stairs", "unique-binary-search-trees"
            ]

# Set up Selenium WebDriver (make sure you have ChromeDriver installed and in PATH)
driver = webdriver.Chrome()

try:
    # Visit the login page
    driver.get('https://leetcode.com/accounts/login/')

    # MANUAL LOGIN HERE
    # Wait for page to load after login
    time.sleep(20)

    for problem in problems:
        url = f'https://leetcode.com/problems/{problem}/description/'
        driver.get(url)
        time.sleep(3)

        # Find the div with data_track_load="description_content"
        description_div = driver.find_element(
            By.CSS_SELECTOR, 'div[data-track-load="description_content"]')

        # can't have package names start with number in Rust
        problem_underscore = problem.replace('-', '_')
        match problem:
            case "4sum":
                problem_underscore = "four_sum"
            case "3sum":
                problem_underscore = "three_sum"
            case "3sum_closest":
                problem_underscore = "three_sum_closest"
 
        filename = "./" + problem_underscore + "/description.html"
        print(filename)
        with open(filename, "w", encoding="utf-8") as f:
            f.write(description_div.get_attribute('innerHTML'))
        print("Description saved to " + problem_underscore)

finally:
    driver.quit()
