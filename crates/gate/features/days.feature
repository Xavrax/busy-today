Feature: Service should respond OK on days endpoint

  Scenario: Service called with empty storage response
    Given configured /days endpoint
    And valid request
    And mocked storage provider with empty store
    When request is sent
    Then response is valid
    And body contains empty days list

  Scenario: Service called with some days in storage response
    Given configured /days endpoint
    And valid request
    And mocked storage provider with some days in store
    When request is sent
    Then response is valid
    And body contains days list