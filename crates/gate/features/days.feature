Feature: Service should respond OK on days endpoint

  Scenario:
    Given configured days endpoint
    And mocked storage provider with empty store
    When request is sent
    Then response is valid
    And body contains empty days list

  Scenario:
    Given configured days endpoint
    And mocked storage provider with some days in store
    When request is sent
    Then response is valid
    And body contains days list