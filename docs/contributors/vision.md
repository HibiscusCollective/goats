# Project Vision

The goal of this project is to provide a simple mechanism to generate client side types from server side models.
The aim is not to replace protobuf, json, captnproto, smithy or other IDLs.
Rather we aim to provide a minimal alternative to projects which have a need to maintain a consistent API contract but that don't require the complex serialisation and/or API modeling capabilities the IDLs offer.
The guiding principles are simplicity and ease of use. We don't want to introduce a new language into the stack or complex build pipeline steps.
We will instead provide annotations that can be applied to model definitions directly in the server source code and leverage existing code gen facilities in the server language to generate client side types and serializers/deserializers.
Languages that don't natively support codegen are out of scope and server-to-server serialization/deserialization is also out of scope of this project.

## Core Objectives

| Priority | Objective                                  | Success Metric                                                                        |
|----------|--------------------------------------------|---------------------------------------------------------------------------------------|
| P0       | Rust -> Typescript                         | A complete suite of tests, demo application, and a crates.io package with docs        |
| P0       | Go -> Typescript                           | A complete suite of tests, demo application, and docs on pkg.go.dev                   |
| P1       | Kotlin -> Typescript                       | A complete suite of tests and demo application, and a gradle package with docs        |
| P1       | C# -> Typescript                           | A complete suite of tests and demo application, and a nuget package with docs         |
| P1       | Server -> Android (Kotlin)                 | A complete suite of tests and demo application, and packages with docs                |
| P1       | Server -> IOs (Swift)                      | A complete suite of tests and demo application, and packages with docs                |
| P2       | Java -> Typescript                         | A complete suite of tests and demo application, and a maven package with docs         |
| P2       | Python -> Typescript                       | A complete suite of tests and demo application, and a pip package with docs           |
| P3       | Other -> Typescript                        | A complete suite of tests and demo application, some package with docs                |
| P4       | WASM serialization package                 | A documented tool to generate a WASM package to handle serialization/deserialization  |

## Governance

This is a solo project at the moment, but contributions are welcome!
@pfouilloux has final say on decisions but will make best efforts to consult with users and the community.
See the [HibiscusCollective governance model](https://github.com/HibiscusCollective/.github/blob/main/docs/GOVERNANCE.md) for the path to collective ownership of this codebase.

## Out of scope

The following list of features is explicitly out of scope of the project.
Community forks are absolutely welcome and encouraged to provide them, but it would take a very compelling argument to add them to the project.

Note that this is not an exhaustive list. Items may be added or removed as the project evolves.

### Server<->Server serialization

The goal is to facilitate writing frontend code with reliable contracts. If it happens to work between servers, great, but serializing Rust -> Go for example is not a goal.

### Javascript

The benefits of this system plain json are minimal without a strongly typed frontend language. So the focus will be on typescript. It can be transpiled to javascript if needed.

### Server -> Protobuf, Capnproto, Flatbuffers, etc

This is not the goal of this project as ideally these should be a one and done migration once a team is ready to adopt one of those technologies.
