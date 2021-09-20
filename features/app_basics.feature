@app_basics_app @srs-Ex0001
Feature: Sum function
  Scenario: User to sum two integers together
    Given the numbers "2" and "3"
     When the user sums them
     Then the user gets "5" as the result

  # Scenario Outline: User wants to sum two integers
  #   Given <first_int> and <second_int>
  #    When sum is called with those integers
  #    Then the user receives <sum>

  #    Examples:
  #      | first_int | second_int |   sum   |
  #      |      0    |       2    |     2   |
  #      |     -7    |      17    |    10   |
