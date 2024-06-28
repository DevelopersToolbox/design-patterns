<!-- markdownlint-disable -->
<p align="center">
    <a href="https://github.com/DevelopersToolbox/">
        <img src="https://cdn.wolfsoftware.com/assets/images/github/organisations/developerstoolbox/black-and-white-circle-256.png" alt="DevelopersToolbox logo" />
    </a>
    <br />
    <a href="https://github.com/DevelopersToolbox/design-patterns/actions/workflows/cicd.yml">
        <img src="https://img.shields.io/github/actions/workflow/status/DevelopersToolbox/design-patterns/cicd.yml?branch=master&label=build%20status&style=for-the-badge" alt="Github Build Status" />
    </a>
    <a href="https://github.com/DevelopersToolbox/design-patterns/blob/master/LICENSE.md">
        <img src="https://img.shields.io/github/license/DevelopersToolbox/design-patterns?color=blue&label=License&style=for-the-badge" alt="License">
    </a>
    <a href="https://github.com/DevelopersToolbox/design-patterns">
        <img src="https://img.shields.io/github/created-at/DevelopersToolbox/design-patterns?color=blue&label=Created&style=for-the-badge" alt="Created">
    </a>
    <br />
    <a href="https://github.com/DevelopersToolbox/design-patterns/releases/latest">
        <img src="https://img.shields.io/github/v/release/DevelopersToolbox/design-patterns?color=blue&label=Latest%20Release&style=for-the-badge" alt="Release">
    </a>
    <a href="https://github.com/DevelopersToolbox/design-patterns/releases/latest">
        <img src="https://img.shields.io/github/release-date/DevelopersToolbox/design-patterns?color=blue&label=Released&style=for-the-badge" alt="Released">
    </a>
    <a href="https://github.com/DevelopersToolbox/design-patterns/releases/latest">
        <img src="https://img.shields.io/github/commits-since/DevelopersToolbox/design-patterns/latest.svg?color=blue&style=for-the-badge" alt="Commits since release">
    </a>
    <br />
    <a href="https://github.com/DevelopersToolbox/design-patterns/blob/master/.github/CODE_OF_CONDUCT.md">
        <img src="https://img.shields.io/badge/Code%20of%20Conduct-blue?style=for-the-badge" />
    </a>
    <a href="https://github.com/DevelopersToolbox/design-patterns/blob/master/.github/CONTRIBUTING.md">
        <img src="https://img.shields.io/badge/Contributing-blue?style=for-the-badge" />
    </a>
    <a href="https://github.com/DevelopersToolbox/design-patterns/blob/master/.github/SECURITY.md">
        <img src="https://img.shields.io/badge/Report%20Security%20Concern-blue?style=for-the-badge" />
    </a>
    <a href="https://github.com/DevelopersToolbox/design-patterns/issues">
        <img src="https://img.shields.io/badge/Get%20Support-blue?style=for-the-badge" />
    </a>
</p>

## Overview

Design patterns are standardized solutions to common problems encountered in software design. They represent best practices refined over time by
experienced developers and architects, providing proven techniques to address recurring challenges. Design patterns help streamline the development
process by offering templates and guidelines for solving specific design issues, thus promoting code reuse, efficiency, and consistency across projects.
They are not concrete implementations but rather conceptual frameworks that can be adapted to fit various scenarios and requirements.

The primary purpose of using design patterns is to improve the overall architecture of a software system. By employing design patterns, developers can
create flexible, scalable, and maintainable code. These patterns help in managing complexity, ensuring that systems are easier to understand and extend.
Selecting the correct design pattern is crucial because it directly impacts the system's efficiency, readability, and adaptability. The wrong pattern can
lead to increased complexity, poor performance, and difficulties in maintenance. Therefore, understanding the specific problem at hand and the context in
which it occurs is essential for choosing the most appropriate design pattern, ultimately leading to more robust and effective software solutions.

## The Patterns 

| Pattern                                                            | Description                                                                                                                                                                            |
| ------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Adapter Pattern](docs/Adapter.md)                                 | The adapter pattern allows incompatible interfaces to work together by converting the interface of one class into another expected by the client.                                      |
| [Builder Pattern](docs/Builder.md)                                 | The builder pattern simplifies the construction of complex objects by separating the construction process from the final representation.                                               |
| [Chain of Responsibility Pattern](docs/Chain-of-Responsibility.md) | The chain of responsibility pattern delegates commands to a chain of processing objects, allowing multiple objects a chance to handle the request.                                     |
| [Command Pattern](docs/Command.md)                                 | The command pattern encapsulates a request as an object, allowing for parameterization, queuing, logging, and supporting undoable operations.                                          |
| [Composite Pattern](docs/Composite.md)                             | The composite pattern allows composing objects into tree structures to represent part-whole hierarchies, treating individual objects and compositions uniformly.                       |
| [Decorator Pattern](docs/Decorator.md)                             | The decorator pattern dynamically adds behaviour to individual objects without affecting the behaviour of other objects from the same class.                                           |
| [Facade Pattern](docs/Facade.md)                                   | The facade pattern provides a simplified interface to a complex subsystem, making it easier for clients to interact with the system.                                                   |
| [Factory Pattern](docs/Factory.md)                                 | The factory pattern defines an interface for creating objects but allows subclasses to alter the type of objects that will be created.                                                 |
| [Flyweight Pattern](docs/Flyweight.md)                             | The flyweight pattern reduces the cost of creating and managing a large number of similar objects by sharing as much data as possible.                                                 |
| [Iterator Pattern](docs/Iterator.md)                               | The iterator pattern provides a way to access elements of an aggregate object sequentially without exposing its underlying representation.                                             |
| [Observer Pattern](docs/Observer.md)                               | The observer pattern defines a one-to-many dependency so that when one object changes state, all its dependents are notified and updated automatically.                                |
| [Proxy Pattern](docs/Proxy.md)                                     | The proxy pattern provides a surrogate or placeholder for another object to control access to it, enhancing control over the underlying object.                                        |
| [Singleton Pattern](docs/Singleton.md)                             | The singleton pattern ensures a class has only one instance and provides a global point of access to it, managing shared resources efficiently.                                        |
| [Strategy Pattern](docs/Strategy.md)                               | The strategy pattern defines a family of algorithms, encapsulates each one, and makes them interchangeable, allowing the algorithm to vary independently from the clients that use it. |

<br />
<p align="right"><a href="https://wolfsoftware.com/"><img src="https://img.shields.io/badge/Created%20by%20Wolf%20on%20behalf%20of%20Wolf%20Software-blue?style=for-the-badge" /></a></p>
