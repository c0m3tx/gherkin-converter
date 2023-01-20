# Gherkin converter

Converts gherkin files into markdown/youtrack compatible formats.

## Build

Just run

    cargo build --release

It will create two files

    target/release/gherkin-to-markdown
    target/release/gherkin-to-youtrack

## Usage

    target/release/gherkin-to-markdown [<file>]

or

    target/release/gherkin-to-youtrack [<file>]

If file is not given, reads from stdin.

## Example

Given a standard feature file like

```feature
Feature: some feature
    Scenario: some scenario
        Given some precondition
        When I do something
        Then I get something
```

The markdown output will be

    ### some feature

    - [ ] some scenario
    ```
    Given some precondition
    When I do something
    Then I get something
    ```

The youtrack output will be

    ## some feature
    - [ ] some scenario
    <pre style="padding-top: 10px; padding-bottom: 10px; margin-bottom: 20px"><span style="color: darkorange">Given</span> some precondition
    <span style="color: darkorange">When</span> I do something
    <span style="color: darkorange">Then</span> I get something</pre>
