# Gherkin converter

Converts gherkin files into markdown/youtrack compatible formats.

## Install

Download latest version from [releases](https://github.com/c0m3tx/gherkin-converter/releases) for your OS/arch, make it executable

    chmod +x gherkin-converter

and put the file anywhere in your path. Binaries for windows and many architectures are not provided, but it's pretty easy to build it yourself.

## Build

Just run

    cargo build --release

It will create a file

    target/release/gherkin-converter

## Usage

    target/release/gherkin-converter --format markdown [<file>]

or

    target/release/gherkin-converter --format youtrack [<file>]

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
